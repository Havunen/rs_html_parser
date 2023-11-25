use htmlize::Context;
use rs_html_parser_tokenizer_tokens::{QuoteType, TokenizerToken, TokenizerTokenLocation};
use std::iter::Iterator;
use std::ops::Range;
use std::str;

struct CharCodes {}

impl CharCodes {
    const TAB: u8 = 9; // "\t"
    const NEW_LINE: u8 = 10; // "\n"
    const FORM_FEED: u8 = 2; // "\f"
    const CARRIAGE_RETURN: u8 = 3; // "\r"
    const SPACE: u8 = 32; // " "
    const EXCLAMATION_MARK: u8 = 33; // "!"
    const HASH: u8 = 35; // "#"
    const AMP: u8 = 38; // "&"
    const SINGLE_QUOTE: u8 = 39; // "'"
    const DOUBLE_QUOTE: u8 = 34; // '"'
    const DASH: u8 = 45; // "-"
    const SLASH: u8 = 47; // "/"
                          // const ZERO: u8 = 48; // "0"
                          // const NINE: u8 = 57; // "9"
    const SEMI: u8 = 59; // ";"
    const LT: u8 = 60; // "<"
    const EQ: u8 = 61; // "="
    const GT: u8 = 62; // ">"
    const QUESTION_MARK: u8 = 63; // "?"
                                  // const UPPER_A: u8 = 65; // "A"
    const UNDERSCORE: u8 = 95; // "_"
                               // const LOWER_A: u8 = 97; // "a"
                               // const UPPER_F: u8 = 70; // "F"
                               // const LOWER_F: u8 = 102; // "f"
                               // const UPPER_Z: u8 = 90; // "Z"
                               // const LOWER_Z: u8 = 122; // "z"
                               // const LOWER_X: u8 = 120; // "x"
    const OPENING_SQUARE_BRACKET: u8 = 91; // "["
}

struct Sequences {}

impl Sequences {
    const EMPTY: [u8; 0] = [];
    const CDATA: [u8; 6] = [0x43, 0x44, 0x41, 0x54, 0x41, 0x5b]; // CDATA[
    const CDATA_END: [u8; 3] = [0x5d, 0x5d, 0x3e]; // ]]>
    const COMMENT_END: [u8; 3] = [0x2d, 0x2d, 0x3e]; // `-->`
    const SCRIPT_END: [u8; 8] = [0x3c, 0x2f, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74]; // `</script`
    const STYLE_END: [u8; 7] = [0x3c, 0x2f, 0x73, 0x74, 0x79, 0x6c, 0x65]; // `</style`
    const TITLE_END: [u8; 7] = [0x3c, 0x2f, 0x74, 0x69, 0x74, 0x6c, 0x65]; // `</title`
}

/** All the states the tokenizer can be in. */
#[derive(Clone, Copy, PartialEq)]
enum State {
    Text = 1,
    BeforeTagName, // After <
    InTagName,
    InSelfClosingTag,
    BeforeClosingTagName,
    InClosingTagName,
    AfterClosingTagName,

    // Attributes
    BeforeAttributeName,
    InAttributeName,
    InAttributeAfterDataSingleQuote,
    InAttributeAfterDataDoubleQuote,
    AfterAttributeName,
    AfterAttributeData,
    BeforeAttributeValue,
    InAttributeValueDq, // "
    InAttributeValueSq, // '
    InAttributeValueNq,

    // Declarations
    BeforeDeclaration, // !
    InDeclaration,

    // Processing instructions
    InProcessingInstruction, // ?

    // Comments & CDATA
    BeforeComment,
    CDATASequence,
    InSpecialComment,
    InCommentLike,

    // Special tags
    BeforeSpecialS, // Decide if we deal with `<script` or `<style`
    SpecialStartSequence,
    InSpecialTag,

    InEntity,
    AfterReadEntityText,
    AfterReadEntityAttr,

    End,
}

pub struct Tokenizer<'a> {
    state: State,
    buffer: &'a [u8],
    section_start: usize,
    index: i32,
    entity_start: usize,
    base_state: State,
    is_special: bool,
    code: u32,
    prev_quote_type: QuoteType,

    xml_mode: bool,
    decode_entities: bool,

    current_sequence: &'a [u8],
    sequence_index: usize,
    has_ended: bool,
}

pub struct TokenizerOptions {
    pub xml_mode: Option<bool>,
    pub decode_entities: Option<bool>,
}

fn is_whitespace(c: u8) -> bool {
    matches!(
        c,
        CharCodes::SPACE
            | CharCodes::NEW_LINE
            | CharCodes::TAB
            | CharCodes::FORM_FEED
            | CharCodes::CARRIAGE_RETURN
    )
}

fn is_end_of_tag_section(c: u8) -> bool {
    matches!(
        c,
        CharCodes::SLASH
            | CharCodes::GT
            | CharCodes::SPACE
            | CharCodes::NEW_LINE
            | CharCodes::TAB
            | CharCodes::FORM_FEED
            | CharCodes::CARRIAGE_RETURN
    )
}

impl Tokenizer<'_> {
    pub fn new<'a>(buffer: &'a [u8], options: &'a TokenizerOptions) -> Tokenizer<'a> {
        Tokenizer {
            state: State::Text,
            buffer,
            section_start: 0,
            index: 0,
            code: 0,
            entity_start: 0,
            base_state: State::Text,
            is_special: false,
            xml_mode: options.xml_mode.unwrap_or(false),
            decode_entities: options.decode_entities.unwrap_or(true),
            current_sequence: Default::default(),
            sequence_index: 0,
            has_ended: false,
            prev_quote_type: QuoteType::NoValue,
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Text;
        self.buffer = &Sequences::EMPTY;
        self.section_start = 0;
        self.index = 0;
        self.base_state = State::Text;
        self.current_sequence = Default::default();
    }

    fn fast_forward_to(&mut self, c: u8) -> bool {
        while self.index < (self.buffer.len() - 1) as i32 {
            self.index += 1;

            if self.buffer[self.index as usize] == c {
                return true;
            }
        }

        self.index = (self.buffer.len() - 1) as i32;

        false
    }

    fn state_text(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::LT || (!self.decode_entities && self.fast_forward_to(CharCodes::LT)) {
            let token = if self.index > self.section_start as i32 {
                Some(TokenizerToken {
                    start: self.section_start,
                    end: self.index as usize,
                    location: TokenizerTokenLocation::Text,
                    code: 0,
                    quote: QuoteType::NoValue,
                })
            } else {
                None
            };

            self.state = State::BeforeTagName;
            self.section_start = self.index as usize;

            return token;
        }
        if self.decode_entities && c == CharCodes::AMP {
            self.start_entity();
        }

        None
    }

    /**
     * Comments and CDATA end with `-->` and `]]>`.
     *
     * Their common qualities are:
     * - Their end sequences have a distinct character they start with.
     * - That character is then repeated, so we have to check multiple repeats.
     * - All characters but the start character of the sequence can be skipped.
     */
    fn state_in_comment_like(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == self.current_sequence[self.sequence_index] {
            self.sequence_index += 1;

            if self.sequence_index == self.current_sequence.len() {
                let end_index = if self.index - 2 > self.buffer.len() as i32 {
                    self.section_start
                } else {
                    self.index as usize - 2
                };

                let token = TokenizerToken {
                    start: self.section_start,
                    end: if self.section_start > end_index {
                        self.section_start
                    } else {
                        end_index
                    },
                    location: if self.current_sequence == Sequences::CDATA_END {
                        TokenizerTokenLocation::CData
                    } else {
                        TokenizerTokenLocation::Comment
                    },
                    code: 0,
                    quote: QuoteType::NoValue,
                };

                self.sequence_index = 0;
                self.section_start = (self.index + 1) as usize;
                self.state = State::Text;

                return Some(token);
            }
        } else if self.sequence_index == 0 {
            // Fast-forward to the first character of the sequence
            if self.fast_forward_to(self.current_sequence[0]) {
                self.sequence_index = 1;
            }
        } else if c != self.current_sequence[self.sequence_index - 1] {
            // Allow long sequences, eg. --->, ]]]>
            self.sequence_index = 0;
        }

        None
    }

    fn is_tag_start_char(&self, c: u8) -> bool {
        if self.xml_mode {
            is_end_of_tag_section(c)
        } else {
            c.is_ascii_alphabetic()
        }
    }

    fn start_special(&mut self, sequence: &'static [u8], offset: i32) {
        self.is_special = true;
        self.current_sequence = sequence;
        self.sequence_index = offset as usize;
        self.state = State::SpecialStartSequence;
    }

    fn state_before_tag_name(&mut self, c: u8) -> Option<TokenizerToken> {
        match c {
            CharCodes::EXCLAMATION_MARK => {
                self.state = State::BeforeDeclaration;
                self.section_start = (self.index + 1) as usize;
            }
            CharCodes::QUESTION_MARK => {
                self.state = State::InProcessingInstruction;
                self.section_start = (self.index + 1) as usize;
            }
            _ if self.is_tag_start_char(c) => {
                let lower = c | 0x20;
                self.section_start = self.index as usize;
                if !self.xml_mode && lower == Sequences::TITLE_END[2] {
                    self.start_special(&Sequences::TITLE_END, 3);
                } else {
                    self.state = if !self.xml_mode && lower == Sequences::SCRIPT_END[2] {
                        State::BeforeSpecialS
                    } else {
                        State::InTagName
                    };
                }
            }
            CharCodes::SLASH => {
                self.state = State::BeforeClosingTagName;
            }
            _ => {
                self.state = State::Text;
                return self.state_text(c);
            }
        }

        None
    }
    fn state_in_tag_name(&mut self, c: u8) -> Option<TokenizerToken> {
        if is_end_of_tag_section(c) {
            let token = Some(TokenizerToken {
                start: self.section_start,
                end: self.index as usize,
                location: TokenizerTokenLocation::OpenTagName,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.section_start = 0;
            self.state = State::BeforeAttributeName;
            self.index -= 1; // continue

            return token;
        }

        None
    }
    fn state_before_closing_tag_name(&mut self, c: u8) -> Option<TokenizerToken> {
        if is_whitespace(c) {
            // Ignore
        } else if c == CharCodes::GT {
            self.state = State::Text;
        } else {
            self.state = if self.is_tag_start_char(c) {
                State::InClosingTagName
            } else {
                State::InSpecialComment
            };

            self.section_start = self.index as usize;
        }

        None
    }
    fn state_in_closing_tag_name(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::GT || is_whitespace(c) {
            let token = Some(TokenizerToken {
                start: self.section_start,
                end: self.index as usize,
                location: TokenizerTokenLocation::CloseTag,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.section_start = 0;
            self.state = State::AfterClosingTagName;
            self.state_after_closing_tag_name(c);

            return token;
        }

        None
    }
    fn state_after_closing_tag_name(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::GT || self.fast_forward_to(CharCodes::GT) {
            self.state = State::Text;
            self.section_start = (self.index + 1) as usize;
        }

        None
    }
    fn state_before_attribute_name(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::GT {
            let token = Some(TokenizerToken {
                start: self.index as usize,
                end: self.index as usize,
                location: TokenizerTokenLocation::OpenTagEnd,
                code: 0,
                quote: QuoteType::NoValue,
            });

            if self.is_special {
                self.state = State::InSpecialTag;
                self.sequence_index = 0;
            } else {
                self.state = State::Text;
            }
            self.section_start = (self.index + 1) as usize;

            return token;
        } else if c == CharCodes::SLASH {
            self.state = State::InSelfClosingTag;
        } else if !is_whitespace(c) {
            self.state = State::InAttributeName;
            self.section_start = self.index as usize;
        }

        None
    }
    fn state_in_self_closing_tag(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::GT {
            let token = Some(TokenizerToken {
                start: self.index as usize,
                end: self.index as usize,
                location: TokenizerTokenLocation::SelfClosingTag,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.state = State::Text;
            self.section_start = (self.index + 1) as usize;
            self.is_special = false;

            return token;
        } else if !is_whitespace(c) {
            self.state = State::BeforeAttributeName;
            self.index -= 1; // continue
        }

        None
    }
    fn state_in_attribute_name(&mut self, c: u8) -> Option<TokenizerToken> {
        let token: Option<TokenizerToken>;
        if c == CharCodes::EQ || is_end_of_tag_section(c) {
            token = Some(TokenizerToken {
                start: self.section_start,
                end: self.index as usize,
                location: TokenizerTokenLocation::AttrName,
                code: 0,
                quote: QuoteType::NoValue,
            });
            self.section_start = self.index as usize;
            self.state = State::AfterAttributeName;
            self.index -= 1; // continue
        } else {
            token = None;
        }

        token
    }
    fn state_after_attribute_name(&mut self, c: u8) -> Option<TokenizerToken> {
        let token: Option<TokenizerToken>;

        if c == CharCodes::EQ {
            self.state = State::BeforeAttributeValue;
            token = None;
        } else if c == CharCodes::SLASH || c == CharCodes::GT {
            token = Some(TokenizerToken {
                start: self.section_start,
                end: self.section_start,
                location: TokenizerTokenLocation::AttrEnd,
                code: 0,
                quote: QuoteType::NoValue,
            });
            self.section_start = 0;
            self.state = State::BeforeAttributeName;
            self.index -= 1; // continue
        } else if !is_whitespace(c) {
            token = Some(TokenizerToken {
                start: self.section_start,
                end: self.section_start,
                location: TokenizerTokenLocation::AttrEnd,
                code: 0,
                quote: QuoteType::NoValue,
            });
            self.state = State::InAttributeName;
            self.section_start = self.index as usize;
        } else {
            token = None;
        };

        token
    }
    fn state_before_attribute_value(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::DOUBLE_QUOTE {
            self.state = State::InAttributeValueDq;
            self.prev_quote_type = QuoteType::Double;
            self.section_start = (self.index + 1) as usize;
        } else if c == CharCodes::SINGLE_QUOTE {
            self.state = State::InAttributeValueSq;
            self.prev_quote_type = QuoteType::Single;
            self.section_start = (self.index + 1) as usize;
        } else if !is_whitespace(c) {
            self.prev_quote_type = QuoteType::Unquoted;
            self.section_start = self.index as usize;
            self.state = State::InAttributeValueNq;

            return self.state_in_attribute_value_no_quotes(c);
        }

        None
    }
    fn handle_in_attribute_value(&mut self, c: u8, quote: u8) -> Option<TokenizerToken> {
        if c == quote || (!self.decode_entities && self.fast_forward_to(quote)) {
            self.state = if quote == CharCodes::DOUBLE_QUOTE {
                State::InAttributeAfterDataDoubleQuote
            } else {
                State::InAttributeAfterDataSingleQuote
            };

            let token = Some(TokenizerToken {
                start: self.section_start,
                end: self.index as usize,
                location: TokenizerTokenLocation::AttrData,
                code: 0,
                quote: self.prev_quote_type,
            });

            self.index -= 1; // Continue

            return token;
        } else if self.decode_entities && c == CharCodes::AMP {
            self.start_entity();
        }

        None
    }
    fn state_in_attribute_value_double_quotes(&mut self, c: u8) -> Option<TokenizerToken> {
        self.handle_in_attribute_value(c, CharCodes::DOUBLE_QUOTE)
    }
    fn state_in_attribute_value_single_quotes(&mut self, c: u8) -> Option<TokenizerToken> {
        self.handle_in_attribute_value(c, CharCodes::SINGLE_QUOTE)
    }
    fn state_in_attribute_value_no_quotes(&mut self, c: u8) -> Option<TokenizerToken> {
        if is_whitespace(c) || c == CharCodes::GT {
            let token = if self.section_start < self.index as usize {
                Some(TokenizerToken {
                    start: self.section_start,
                    end: self.index as usize,
                    location: TokenizerTokenLocation::AttrData,
                    code: 0,
                    quote: QuoteType::Unquoted,
                })
            } else {
                None
            };

            self.index -= 1; // continue
            self.state = State::AfterAttributeData;
            self.section_start = 0;

            return token;
        } else if self.decode_entities && c == CharCodes::AMP {
            self.start_entity();
        }

        None
    }
    fn state_before_declaration(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::OPENING_SQUARE_BRACKET {
            self.state = State::CDATASequence;
            self.sequence_index = 0;
        } else {
            self.state = if c == CharCodes::DASH {
                State::BeforeComment
            } else {
                State::InDeclaration
            };
        }

        None
    }

    fn fast_get_until_gt(
        &mut self,
        c: u8,
        location: TokenizerTokenLocation,
    ) -> Option<TokenizerToken> {
        if c == CharCodes::GT || self.fast_forward_to(CharCodes::GT) {
            let token = Some(TokenizerToken {
                start: self.section_start,
                end: self.index as usize,
                location,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.state = State::Text;
            self.section_start = (self.index + 1) as usize;

            return token;
        }

        None
    }

    fn state_before_comment(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == CharCodes::DASH {
            self.state = State::InCommentLike;
            self.current_sequence = &Sequences::COMMENT_END;
            // Allow short comments (eg. <!-->)
            self.sequence_index = 2;
            self.section_start = (self.index + 1) as usize;
        } else {
            self.state = State::InDeclaration;
        }

        None
    }

    fn state_before_special_s(&mut self, c: u8) -> Option<TokenizerToken> {
        let lower = c | 0x20;
        if lower == Sequences::SCRIPT_END[3] {
            self.start_special(&Sequences::SCRIPT_END, 4);

            return None;
        }
        if lower == Sequences::STYLE_END[3] {
            self.start_special(&Sequences::STYLE_END, 4);

            return None;
        }

        self.state = State::InTagName;

        self.state_in_tag_name(c)
    }

    fn start_entity(&mut self) {
        self.base_state = self.state;
        self.state = State::InEntity;
        self.entity_start = self.index as usize;
        self.index -= 1; // continue
    }

    fn find_end_of_html_entity(&mut self) -> i32 {
        let start_pos = self.index as usize;
        let loop_until = self.buffer.len();
        let mut count: usize = start_pos + 1; // the first is always &

        while count < loop_until {
            let char = self.buffer[count];

            if (char > 47 && char < 58) || // number
                (char > 96 && char < 123) || // lower case letter
                (char > 64 && char < 91) || // upper case letter
                char == CharCodes::UNDERSCORE ||
                char == CharCodes::HASH
            {
                count += 1;
                continue;
            }
            if char == CharCodes::SEMI {
                count += 1;
                break;
            }

            break;
        }

        if count <= loop_until {
            return count as i32;
        }

        -1
    }

    fn state_in_entity(&mut self) -> Option<TokenizerToken> {
        let index = self.find_end_of_html_entity();

        if index >= 0 {
            let range: Range<usize> = Range {
                start: self.index as usize,
                end: index as usize,
            };
            let is_attr = self.base_state != State::Text && self.base_state != State::InSpecialTag;
            let code = htmlize::unescape_bytes_in(
                &self.buffer[range],
                if is_attr {
                    Context::Attribute
                } else {
                    Context::General
                },
            );

            if code.len() > 0 {
                if code.len() > 1 && code[0] == CharCodes::AMP {
                    self.state = self.base_state;
                    return None;
                }

                let text = unsafe { str::from_utf8_unchecked(&code) };

                if let Some(character) = text.chars().next() {
                    self.code = character as u32
                } else {
                    self.state = self.base_state;
                    return None;
                }

                let token: Option<TokenizerToken>;

                if self.section_start < self.entity_start {
                    token = Some(TokenizerToken {
                        start: self.section_start,
                        end: self.entity_start,
                        location: if is_attr {
                            TokenizerTokenLocation::AttrData
                        } else {
                            TokenizerTokenLocation::Text
                        },
                        code: 0,
                        quote: if is_attr {
                            self.prev_quote_type
                        } else {
                            QuoteType::NoValue
                        },
                    });
                    self.section_start = self.index as usize;
                } else {
                    token = None;
                }

                self.index = index - 1;

                self.state = if is_attr {
                    State::AfterReadEntityAttr
                } else {
                    State::AfterReadEntityText
                };

                return token;
            } else {
                self.index = (self.buffer.len() - 1) as i32;
            }
        } else {
            self.index = (self.buffer.len() - 1) as i32;
        }

        None
    }

    fn tokenizer_next(&mut self) -> Option<TokenizerToken> {
        while self.index < self.buffer.len() as i32 {
            let c = self.buffer[self.index as usize];

            let token_or_empty: Option<TokenizerToken> = match self.state {
                State::Text => self.state_text(c),
                State::SpecialStartSequence => self.state_special_start_sequence(c),
                State::InSpecialTag => self.state_in_special_tag(c),
                State::CDATASequence => self.state_cdata_sequence(c),
                State::InAttributeValueDq => self.state_in_attribute_value_double_quotes(c),
                State::InAttributeName => self.state_in_attribute_name(c),
                State::InAttributeAfterDataSingleQuote => {
                    self.state_in_attribute_after_data(QuoteType::Single)
                }
                State::InAttributeAfterDataDoubleQuote => {
                    self.state_in_attribute_after_data(QuoteType::Double)
                }
                State::InCommentLike => self.state_in_comment_like(c),
                State::InSpecialComment => {
                    self.fast_get_until_gt(c, TokenizerTokenLocation::Comment)
                }
                State::BeforeAttributeName => self.state_before_attribute_name(c),
                State::InTagName => self.state_in_tag_name(c),
                State::InClosingTagName => self.state_in_closing_tag_name(c),
                State::BeforeTagName => self.state_before_tag_name(c),
                State::AfterAttributeName => self.state_after_attribute_name(c),
                State::InAttributeValueSq => self.state_in_attribute_value_single_quotes(c),
                State::BeforeAttributeValue => self.state_before_attribute_value(c),
                State::BeforeClosingTagName => self.state_before_closing_tag_name(c),
                State::AfterClosingTagName => self.state_after_closing_tag_name(c),
                State::BeforeSpecialS => self.state_before_special_s(c),
                State::InAttributeValueNq => self.state_in_attribute_value_no_quotes(c),
                State::AfterAttributeData => self.state_after_attribute_data(),
                State::InSelfClosingTag => self.state_in_self_closing_tag(c),
                State::InDeclaration => {
                    self.fast_get_until_gt(c, TokenizerTokenLocation::Declaration)
                }
                State::BeforeDeclaration => self.state_before_declaration(c),
                State::BeforeComment => self.state_before_comment(c),
                State::InProcessingInstruction => {
                    self.fast_get_until_gt(c, TokenizerTokenLocation::ProcessingInstruction)
                }
                State::InEntity => self.state_in_entity(),
                State::AfterReadEntityText => {
                    self.state_after_entity(TokenizerTokenLocation::TextEntity)
                }
                State::AfterReadEntityAttr => {
                    self.state_after_entity(TokenizerTokenLocation::AttrEntity)
                }
                State::End => self.state_end(),
            };

            self.index += 1;

            if token_or_empty.is_some() {
                return token_or_empty;
            }
        }

        if self.state == State::InEntity {
            self.state = self.base_state;
        }

        let option_trailing_token: Option<TokenizerToken> = self.handle_trailing_data();
        self.state = State::End;
        if option_trailing_token.is_some() {
            return option_trailing_token;
        }

        self.state_end()
    }

    fn state_end(&mut self) -> Option<TokenizerToken> {
        if !self.has_ended {
            self.has_ended = true;

            let i = self.buffer.len();

            return Some(TokenizerToken {
                start: i,
                end: i,
                location: TokenizerTokenLocation::End,
                code: 0,
                quote: QuoteType::NoValue,
            });
        }

        None
    }

    fn handle_trailing_data(&mut self) -> Option<TokenizerToken> {
        if self.state == State::End {
            return None;
        }

        let end_index = self.buffer.len();
        if self.section_start >= end_index {
            return None;
        }

        if self.state == State::InCommentLike || self.state == State::InSpecialComment {
            Some(TokenizerToken {
                start: self.section_start,
                end: end_index,
                location: if self.current_sequence == Sequences::CDATA_END {
                    TokenizerTokenLocation::CData
                } else {
                    TokenizerTokenLocation::Comment
                },
                code: 0,
                quote: QuoteType::NoValue,
            })
        } else {
            match &self.state {
                State::InTagName
                | State::BeforeAttributeName
                | State::BeforeAttributeValue
                | State::AfterAttributeName
                | State::InAttributeName
                | State::InAttributeValueSq
                | State::InAttributeValueDq
                | State::InAttributeValueNq
                | State::InClosingTagName => None,
                State::AfterReadEntityText => {
                    self.state_after_entity(TokenizerTokenLocation::TextEntity)
                }
                State::AfterReadEntityAttr => {
                    self.state_after_entity(TokenizerTokenLocation::AttrData)
                }
                _ => Some(TokenizerToken {
                    start: self.section_start,
                    end: end_index,
                    location: TokenizerTokenLocation::Text,
                    code: 0,
                    quote: QuoteType::NoValue,
                }),
            }
        }
    }

    fn state_special_start_sequence(&mut self, c: u8) -> Option<TokenizerToken> {
        let is_end = self.sequence_index == self.current_sequence.len();
        let is_match = if is_end {
            // If we are at the end of the sequence, make sure the tag name has ended
            is_end_of_tag_section(c)
        } else {
            // Otherwise, do a case-insensitive comparison
            (c | 0x20) == self.current_sequence[self.sequence_index]
        };

        if !is_match {
            self.is_special = false;
        } else if !is_end {
            self.sequence_index += 1;

            return None;
        }

        self.sequence_index = 0;
        self.state = State::InTagName;

        self.state_in_tag_name(c)
    }
    fn state_cdata_sequence(&mut self, c: u8) -> Option<TokenizerToken> {
        if c == Sequences::CDATA[self.sequence_index] {
            self.sequence_index += 1;
            if self.sequence_index == Sequences::CDATA.len() {
                self.state = State::InCommentLike;
                self.current_sequence = &Sequences::CDATA_END;
                self.sequence_index = 0;
                self.section_start = (self.index + 1) as usize;
            }
            None
        } else {
            self.sequence_index = 0;
            self.state = State::InDeclaration;

            self.fast_get_until_gt(c, TokenizerTokenLocation::Declaration) // Reconsume the character
        }
    }
    fn state_in_special_tag(&mut self, c: u8) -> Option<TokenizerToken> {
        if self.sequence_index == self.current_sequence.len() {
            if c == CharCodes::GT || is_whitespace(c) {
                let end_of_text = self.index - self.current_sequence.len() as i32;
                let token: Option<TokenizerToken>;

                if self.section_start < end_of_text as usize {
                    // Spoof the index so that reported locations match up.
                    let actual_index = self.index;
                    self.index = end_of_text;
                    token = Some(TokenizerToken {
                        start: self.section_start,
                        end: end_of_text as usize,
                        location: TokenizerTokenLocation::Text,
                        code: 0,
                        quote: QuoteType::NoValue,
                    });
                    self.index = actual_index;
                } else {
                    token = None;
                }

                self.is_special = false;
                self.section_start = (end_of_text + 2) as usize; // Skip over the `</`
                self.state = State::InClosingTagName;
                self.index -= 1; // continue
                                 // self.state_in_closing_tag_name(c);

                return token; // We are done; skip the rest of the function.
            }

            self.sequence_index = 0;
        }

        if (c | 0x20) == self.current_sequence[self.sequence_index] {
            self.sequence_index += 1;
        } else if self.sequence_index == 0 {
            if self.current_sequence == Sequences::TITLE_END {
                // We have to parse entities in <title> tags.
                if self.decode_entities && c == CharCodes::AMP {
                    self.start_entity();
                }
            } else if self.fast_forward_to(CharCodes::LT) {
                // Outside of <title> tags, we can fast-forward.
                self.sequence_index = 1;
                // self.index -= 1; // continue
            }
        } else {
            // If we see a `<`, set the sequence index to 1; useful for eg. `<</script>`.
            if c == CharCodes::LT {
                self.sequence_index = 1;
            } else {
                self.sequence_index = 0;
            }
        }

        None
    }

    #[inline]
    fn state_after_attribute_data(&mut self) -> Option<TokenizerToken> {
        let token = Some(TokenizerToken {
            start: self.index as usize,
            end: self.index as usize,
            location: TokenizerTokenLocation::AttrEnd,
            code: 0,
            quote: QuoteType::Unquoted,
        });
        self.state = State::BeforeAttributeName;
        self.index -= 1; // continue

        token
    }

    #[inline]
    fn state_in_attribute_after_data(&mut self, quote_type: QuoteType) -> Option<TokenizerToken> {
        self.section_start = 0;
        self.state = State::BeforeAttributeName;

        Some(TokenizerToken {
            start: (self.index + 1) as usize,
            end: (self.index + 1) as usize,
            location: TokenizerTokenLocation::AttrEnd,
            code: 0,
            quote: quote_type,
        })
    }

    #[inline]
    fn state_after_entity(&mut self, location: TokenizerTokenLocation) -> Option<TokenizerToken> {
        self.state = self.base_state;

        let token = Some(TokenizerToken {
            start: self.section_start,
            end: self.index as usize,
            location,
            code: self.code,
            quote: if location == TokenizerTokenLocation::AttrEntity {
                self.prev_quote_type
            } else {
                QuoteType::NoValue
            },
        });

        self.section_start = self.index as usize;
        self.index -= 1;
        self.code = 0;

        token
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = TokenizerToken;

    fn next(&mut self) -> Option<TokenizerToken> {
        self.tokenizer_next()
    }
}
