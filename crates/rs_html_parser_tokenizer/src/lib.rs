use rs_html_parser_tokenizer_tokens::{QuoteType, Token, TokenLocation};
use std::iter::Iterator;
use std::ops::Range;
use htmlize;
use htmlize::Context;
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
    const UPPER_A: u8 = 65; // "A"
    const UNDERSCORE: u8 = 95; // "_"
    const LOWER_A: u8 = 97; // "a"
                            // const UPPER_F: u8 = 70; // "F"
                            // const LOWER_F: u8 = 102; // "f"
    const UPPER_Z: u8 = 90; // "Z"
    const LOWER_Z: u8 = 122; // "z"
                             // const LOWER_X: u8 = 120; // "x"
    const OPENING_SQUARE_BRACKET: u8 = 91; // "["
}

struct Sequences {}

impl Sequences {
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
}

pub struct Tokenizer<'a> {
    state: State,
    buffer: Vec<u8>,
    section_start: i32,
    index: i32,
    entity_start: i32,
    base_state: State,
    is_special: bool,
    offset: i32,
    code: u32,

    xml_mode: bool,
    decode_entities: bool,

    current_sequence: &'a [u8],
    sequence_index: usize,
}

pub struct TokenizerOptions {
    pub xml_mode: Option<bool>,
    pub decode_entities: Option<bool>,
}

fn is_whitespace(c: u8) -> bool {
    match c {
        CharCodes::SPACE
        | CharCodes::NEW_LINE
        | CharCodes::TAB
        | CharCodes::FORM_FEED
        | CharCodes::CARRIAGE_RETURN => true,
        _ => false,
    }
}

fn is_end_of_tag_section(c: u8) -> bool {
    return c == CharCodes::SLASH || c == CharCodes::GT || is_whitespace(c);
}

fn is_ascii_alpha(c: u8) -> bool {
    return (c >= CharCodes::LOWER_A && c <= CharCodes::LOWER_Z)
        || (c >= CharCodes::UPPER_A && c <= CharCodes::UPPER_Z);
}


impl Tokenizer<'static> {
    pub fn new(html: &str, options: TokenizerOptions) -> Tokenizer<'static> {
        let buffer = html.as_bytes().to_vec();

        Tokenizer {
            state: State::Text,
            buffer,
            section_start: 0,
            index: 0,
            code: 0,
            entity_start: 0,
            base_state: State::Text,
            is_special: false,
            // running: false,
            offset: 0,
            xml_mode: options.xml_mode.unwrap_or(false),
            decode_entities: options.decode_entities.unwrap_or(true),
            current_sequence: Default::default(),
            sequence_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Text;
        self.buffer = vec![];
        self.section_start = 0;
        self.index = 0;
        self.base_state = State::Text;
        self.current_sequence = Default::default();
        self.offset = 0;
    }

    fn fast_forward_to(&mut self, c: u8) -> bool {
        while self.index < self.buffer.len() as i32 + self.offset - 1 {
            self.index += 1;

            if self.buffer[(self.index - self.offset) as usize] == c {
                return true;
            }
        }

        self.index = self.buffer.len() as i32 + self.offset - 1;

        return false;
    }

    fn state_text(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::LT || (!self.decode_entities && self.fast_forward_to(CharCodes::LT)) {
            let token = if self.index > self.section_start {
                Some(Token {
                    start: self.section_start,
                    end: self.index,
                    offset: 0,
                    location: TokenLocation::Text,
                    code: 0,
                    quote: QuoteType::NoValue,
                })
            } else {
                None
            };

            self.state = State::BeforeTagName;
            self.section_start = self.index;

            return token;
        }
        if self.decode_entities && c == CharCodes::AMP {
            self.start_entity();
        }

        return None;
    }

    /**
     * Comments and CDATA end with `-->` and `]]>`.
     *
     * Their common qualities are:
     * - Their end sequences have a distinct character they start with.
     * - That character is then repeated, so we have to check multiple repeats.
     * - All characters but the start character of the sequence can be skipped.
     */
    fn state_in_comment_like(&mut self, c: u8) -> Option<Token> {
        if c == self.current_sequence[self.sequence_index] {
            self.sequence_index += 1;

            if self.sequence_index == self.current_sequence.len() {
                let token = if self.current_sequence == &Sequences::CDATA_END {
                    Token {
                        start: self.section_start,
                        end: self.index,
                        offset: 2,
                        location: TokenLocation::CData,
                        code: 0,
                        quote: QuoteType::NoValue,
                    }
                } else {
                    Token {
                        start: self.section_start,
                        end: self.index,
                        offset: 2,
                        location: TokenLocation::Comment,
                        code: 0,
                        quote: QuoteType::NoValue,
                    }
                };

                self.sequence_index = 0;
                self.section_start = self.index + 1;
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

        return None;
    }

    fn is_tag_start_char(&self, c: u8) -> bool {
        if self.xml_mode {
            return is_end_of_tag_section(c);
        }

        return is_ascii_alpha(c);
    }

    fn start_special(&mut self, sequence: &'static [u8], offset: i32) {
        self.is_special = true;
        self.current_sequence = sequence;
        self.sequence_index = offset as usize;
        self.state = State::SpecialStartSequence;
    }

    fn state_before_tag_name(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::EXCLAMATION_MARK {
            self.state = State::BeforeDeclaration;
            self.section_start = self.index + 1;
        } else if c == CharCodes::QUESTION_MARK {
            self.state = State::InProcessingInstruction;
            self.section_start = self.index + 1;
        } else if self.is_tag_start_char(c) {
            let lower = c | 0x20;
            self.section_start = self.index;
            if !self.xml_mode && lower == Sequences::TITLE_END[2] {
                self.start_special(&Sequences::TITLE_END, 3);
            } else {
                self.state = if !self.xml_mode && lower == Sequences::SCRIPT_END[2] {
                    State::BeforeSpecialS
                } else {
                    State::InTagName
                };
            }
        } else if c == CharCodes::SLASH {
            self.state = State::BeforeClosingTagName;
        } else {
            self.state = State::Text;

            return self.state_text(c);
        }

        return None;
    }
    fn state_in_tag_name(&mut self, c: u8) -> Option<Token> {
        if is_end_of_tag_section(c) {
            let token = Some(Token {
                start: self.section_start,
                end: self.index,
                offset: 0,
                location: TokenLocation::OpenTagName,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.section_start = -1;
            self.state = State::BeforeAttributeName;
            self.index -= 1; // continue

            return token;
        }

        return None;
    }
    fn state_before_closing_tag_name(&mut self, c: u8) -> Option<Token> {
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

            self.section_start = self.index;
        }

        return None;
    }
    fn state_in_closing_tag_name(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::GT || is_whitespace(c) {
            let token = Some(Token {
                start: self.section_start,
                end: self.index,
                offset: 0,
                location: TokenLocation::CloseTag,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.section_start = -1;
            self.state = State::AfterClosingTagName;
            self.state_after_closing_tag_name(c);

            return token;
        }

        return None;
    }
    fn state_after_closing_tag_name(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::GT || self.fast_forward_to(CharCodes::GT) {
            self.state = State::Text;
            self.section_start = self.index + 1;
        }

        return None;
    }
    fn state_before_attribute_name(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::GT {
            let token = Some(Token {
                start: self.index,
                end: self.index,
                offset: 0,
                location: TokenLocation::OpenTagEnd,
                code: 0,
                quote: QuoteType::NoValue,
            });

            if self.is_special {
                self.state = State::InSpecialTag;
                self.sequence_index = 0;
            } else {
                self.state = State::Text;
            }
            self.section_start = self.index + 1;

            return token;
        } else if c == CharCodes::SLASH {
            self.state = State::InSelfClosingTag;
        } else if !is_whitespace(c) {
            self.state = State::InAttributeName;
            self.section_start = self.index;
        }

        return None;
    }
    fn state_in_self_closing_tag(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::GT {
            let token = Some(Token {
                start: self.index,
                end: self.index,
                offset: 0,
                location: TokenLocation::SelfClosingTag,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.state = State::Text;
            self.section_start = self.index + 1;
            self.is_special = false;

            return token;
        } else if !is_whitespace(c) {
            self.state = State::BeforeAttributeName;
            self.index -= 1; // continue
        }

        return None;
    }
    fn state_in_attribute_name(&mut self, c: u8) -> Option<Token> {
        let token: Option<Token>;
        if c == CharCodes::EQ || is_end_of_tag_section(c) {
            token = Some(Token {
                start: self.section_start,
                end: self.index,
                offset: 0,
                location: TokenLocation::AttrName,
                code: 0,
                quote: QuoteType::NoValue,
            });
            self.section_start = self.index;
            self.state = State::AfterAttributeName;
            self.index -= 1; // continue
        } else {
            token = None;
        }

        return token;
    }
    fn state_after_attribute_name(&mut self, c: u8) -> Option<Token> {
        let token: Option<Token>;

        if c == CharCodes::EQ {
            self.state = State::BeforeAttributeValue;
            token = None;
        } else if c == CharCodes::SLASH || c == CharCodes::GT {
            token = Some(Token {
                start: self.section_start,
                end: self.section_start,
                offset: 0,
                location: TokenLocation::AttrEnd,
                code: 0,
                quote: QuoteType::NoValue,
            });
            self.section_start = -1;
            self.state = State::BeforeAttributeName;
            self.index -= 1; // continue
                             // self.state_before_attribute_name(c);
        } else if !is_whitespace(c) {
            token = Some(Token {
                start: self.section_start,
                end: self.section_start,
                offset: 0,
                location: TokenLocation::AttrEnd,
                code: 0,
                quote: QuoteType::NoValue,
            });
            self.state = State::InAttributeName;
            self.section_start = self.index;
        } else {
            token = None;
        };

        return token;
    }
    fn state_before_attribute_value(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::DOUBLE_QUOTE {
            self.state = State::InAttributeValueDq;
            self.section_start = self.index + 1;
        } else if c == CharCodes::SINGLE_QUOTE {
            self.state = State::InAttributeValueSq;
            self.section_start = self.index + 1;
        } else if !is_whitespace(c) {
            self.section_start = self.index;
            self.state = State::InAttributeValueNq;

            return self.state_in_attribute_value_no_quotes(c);
        }

        return None;
    }
    fn handle_in_attribute_value(&mut self, c: u8, quote: u8) -> Option<Token> {
        if c == quote || (!self.decode_entities && self.fast_forward_to(quote)) {
            self.state = if quote == CharCodes::DOUBLE_QUOTE {
                State::InAttributeAfterDataDoubleQuote
            } else {
                State::InAttributeAfterDataSingleQuote
            };

            let token = Some(Token {
                start: self.section_start,
                end: self.index,
                offset: 0,
                location: TokenLocation::AttrData,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.index -= 1; // Continue

            return token;
        } else if self.decode_entities && c == CharCodes::AMP {
            self.start_entity();
        }

        return None;
    }
    fn state_in_attribute_value_double_quotes(&mut self, c: u8) -> Option<Token> {
        return self.handle_in_attribute_value(c, CharCodes::DOUBLE_QUOTE);
    }
    fn state_in_attribute_value_single_quotes(&mut self, c: u8) -> Option<Token> {
        return self.handle_in_attribute_value(c, CharCodes::SINGLE_QUOTE);
    }
    fn state_in_attribute_value_no_quotes(&mut self, c: u8) -> Option<Token> {
        if is_whitespace(c) || c == CharCodes::GT {
            let token = Some(Token {
                start: self.section_start,
                end: self.index,
                offset: 0,
                location: TokenLocation::AttrData,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.state = State::AfterAttributeData;
            self.section_start = -1;

            return token;
        } else if self.decode_entities && c == CharCodes::AMP {
            self.start_entity();
        }

        return None;
    }
    fn state_before_declaration(&mut self, c: u8) -> Option<Token> {
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

        return None;
    }

    fn fast_get_until_gt(&mut self, c: u8, location: TokenLocation) -> Option<Token> {
        if c == CharCodes::GT || self.fast_forward_to(CharCodes::GT) {
            let token = Some(Token {
                start: self.section_start,
                end: self.index,
                offset: 0,
                location,
                code: 0,
                quote: QuoteType::NoValue,
            });

            self.state = State::Text;
            self.section_start = self.index + 1;

            return token;
        }

        return None;
    }

    fn state_before_comment(&mut self, c: u8) -> Option<Token> {
        if c == CharCodes::DASH {
            self.state = State::InCommentLike;
            self.current_sequence = &Sequences::COMMENT_END;
            // Allow short comments (eg. <!-->)
            self.sequence_index = 2;
            self.section_start = self.index + 1;
        } else {
            self.state = State::InDeclaration;
        }

        return None;
    }

    fn state_before_special_s(&mut self, c: u8) -> Option<Token> {
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

        return self.state_in_tag_name(c);
    }

    fn start_entity(&mut self) {
        self.base_state = self.state;
        self.state = State::InEntity;
        self.entity_start = self.index;
        self.index -= 1; // continue
    }

    fn find_end_of_html_entity(&mut self) -> i32 {
        let start_pos = self.index - self.offset;
        let loop_until = self.buffer.len() as i32 + self.offset;
        let mut count: i32 = start_pos + 1; // the first is always &

        while count < loop_until {
            let char = self.buffer[count as usize];

            if (char > 47 && char < 58) || // number
                (char > 96 && char < 123) || // lower case letter
                (char > 64 && char < 91) || // upper case letter
                char == CharCodes::UNDERSCORE ||
                char == CharCodes::HASH {
                count += 1;
                continue;
            }
            if char == CharCodes::SEMI {
                count += 1;
                break;
            }

            break;
        }

        if count < loop_until {
            return count;
        }
        if count == loop_until {
            return loop_until;
        }

        return -1;
    }

    fn state_in_entity(&mut self) -> Option<Token> {
        let index = self.find_end_of_html_entity();

        if index >= 0 {
            let range: Range<usize> = Range {
                start: (self.index - self.offset) as usize,
                end: index as usize
            };
            let is_attr = self.base_state != State::Text && self.base_state != State::InSpecialTag;
            let code = htmlize::unescape_bytes_in(
                &self.buffer[range],
                if is_attr {Context::Attribute} else {Context::General}
            );

            if code.len() > 0 {
                if code.len() > 1 && code[0] == CharCodes::AMP {
                    self.state = self.base_state;
                    return None;
                }

                let text = unsafe {
                    str::from_utf8_unchecked(&code)
                };

                if let Some(character) = text.chars().next() {
                    self.code = character as u32
                } else {
                    self.state = self.base_state;
                    return None;
                }

                let token: Option<Token>;

                if self.section_start < self.entity_start {
                    token = Some(Token {
                        start: self.section_start,
                        end: self.entity_start,
                        offset: 0,
                        location: if is_attr { TokenLocation::AttrData } else { TokenLocation::Text },
                        code: 0,
                        quote: QuoteType::NoValue,
                    });
                    self.section_start = self.index;
                } else {
                    token = None;
                }

                self.index = index - 1;

                self.state = if is_attr {
                     State::AfterReadEntityAttr
                }
                else {
                     State::AfterReadEntityText
                };

                return token;
            } else {
                self.index = self.offset + self.buffer.len() as i32 - 1;
            }
        } else {
            self.index = self.offset + self.buffer.len() as i32 - 1;
        }

        return None;
    }

    fn parse_next(&mut self) -> Option<Token> {
        while self.index < self.buffer.len() as i32 + self.offset {
            let c = self.buffer[(self.index - self.offset) as usize];

            let token_or_empty: Option<Token> = match self.state {
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
                State::InSpecialComment => self.fast_get_until_gt(c, TokenLocation::Comment),
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
                State::InDeclaration => self.fast_get_until_gt(c, TokenLocation::Declaration),
                State::BeforeDeclaration => self.state_before_declaration(c),
                State::BeforeComment => self.state_before_comment(c),
                State::InProcessingInstruction => self.fast_get_until_gt(c, TokenLocation::ProcessingInstruction),
                State::InEntity => self.state_in_entity(),
                State::AfterReadEntityText => self.state_after_entity(TokenLocation::TextEntity),
                State::AfterReadEntityAttr => self.state_after_entity(TokenLocation::AttrEntity)
            };

            self.index += 1;

            if token_or_empty.is_some() {
                return token_or_empty;
            }
        }

        if self.state == State::InEntity {
            self.state = self.base_state;
        }

        return self.handle_trailing_data();
    }

    fn handle_trailing_data(&mut self) -> Option<Token> {
        let end_index = self.buffer.len() as i32 + self.offset;
        if self.section_start >= end_index {
            return None;
        }

        return if self.state == State::InCommentLike {
            Some(Token {
                start: self.section_start,
                end: end_index,
                offset: 0,
                location: if self.current_sequence == &Sequences::CDATA_END {
                    TokenLocation::CData
                } else {
                    TokenLocation::Comment
                },
                code: 0,
                quote: QuoteType::NoValue,
            })
        } else {
            let token = match &self.state {
                State::InTagName
                | State::BeforeAttributeName
                | State::BeforeAttributeValue
                | State::AfterAttributeName
                | State::InAttributeName
                | State::InAttributeValueSq
                | State::InAttributeValueDq
                | State::InAttributeValueNq
                | State::InClosingTagName => {
                    None
                },
                State::AfterReadEntityText => self.state_after_entity(TokenLocation::TextEntity),
                State::AfterReadEntityAttr => self.state_after_entity(TokenLocation::AttrData),
                _ => Some(Token {
                    start: self.section_start,
                    end: end_index,
                    offset: 0,
                    location: TokenLocation::Text,
                    code: 0,
                    quote: QuoteType::NoValue,
                }),
            };

            self.state = State::InClosingTagName;

            token
        }
    }

    fn state_special_start_sequence(&mut self, c: u8) -> Option<Token> {
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

        return self.state_in_tag_name(c);
    }
    fn state_cdata_sequence(&mut self, c: u8) -> Option<Token> {
        return if c == Sequences::CDATA[self.sequence_index] {
            self.sequence_index += 1;
            if self.sequence_index == Sequences::CDATA.len() {
                self.state = State::InCommentLike;
                self.current_sequence = &Sequences::CDATA_END;
                self.sequence_index = 0;
                self.section_start = self.index + 1;
            }
            None
        } else {
            self.sequence_index = 0;
            self.state = State::InDeclaration;

            self.fast_get_until_gt(c, TokenLocation::Declaration) // Reconsume the character
        }
    }
    fn state_in_special_tag(&mut self, c: u8) -> Option<Token> {
        if self.sequence_index == self.current_sequence.len() {
            if c == CharCodes::GT || is_whitespace(c) {
                let end_of_text = self.index - self.current_sequence.len() as i32;
                let token: Option<Token>;

                if self.section_start < end_of_text {
                    // Spoof the index so that reported locations match up.
                    let actual_index = self.index;
                    self.index = end_of_text;
                    token = Some(Token {
                        start: self.section_start,
                        end: end_of_text,
                        offset: 0,
                        location: TokenLocation::Text,
                        code: 0,
                        quote: QuoteType::NoValue,
                    });
                    self.index = actual_index;
                } else {
                    token = None;
                }

                self.is_special = false;
                self.section_start = end_of_text + 2; // Skip over the `</`
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
            if self.current_sequence == &Sequences::TITLE_END {
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

        return None;
    }
    fn state_after_attribute_data(&mut self) -> Option<Token> {
        let token = Some(Token {
            start: self.index,
            end: self.index,
            offset: 0,
            location: TokenLocation::AttrEnd,
            code: 0,
            quote: QuoteType::Unquoted,
        });
        self.state = State::BeforeAttributeName;
        self.index -= 1; // continue

        return token;
    }

    fn state_in_attribute_after_data(&mut self, quote_type: QuoteType) -> Option<Token> {
        self.section_start = -1;
        self.state = State::BeforeAttributeName;

        return Some(Token {
            start: self.index + 1,
            end: self.index + 1,
            offset: 0,
            location: TokenLocation::AttrEnd,
            code: 0,
            quote: quote_type,
        });
    }
    fn state_after_entity(&mut self, location: TokenLocation) -> Option<Token> {
        self.state = self.base_state;

        let token = Some(Token {
            start: self.section_start,
            end: self.index,
            offset: 0,
            location,
            code: self.code,
            quote: QuoteType::NoValue,
        });

        self.section_start = self.index;
        self.index -= 1;
        self.code = 0;

        return token;
    }
}

impl Iterator for Tokenizer<'static> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.parse_next()
    }
}
