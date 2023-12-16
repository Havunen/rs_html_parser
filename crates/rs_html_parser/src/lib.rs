mod element_info;

use crate::element_info::{
    is_foreign_context_elements, is_html_integration_elements, is_void_elements, open_implies_close,
};
use lazy_static::lazy_static;
use regex::Regex;
use rs_html_parser_tokenizer::{Tokenizer, TokenizerOptions};
use rs_html_parser_tokenizer_tokens::{QuoteType, TokenizerToken, TokenizerTokenLocation};
use rs_html_parser_tokens::{Token, TokenKind};
use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque};
use std::mem::take;
use std::str;
use unicase::UniCase;

pub struct ParserOptions {
    /**
     * Indicates whether special tags (`<script>`, `<style>`, and `<title>`) should get special treatment
     * and if "empty" tags (eg. `<br>`) can have children.  If `false`, the content of special tags
     * will be text only. For feeds and other XML content (documents that don't consist of HTML),
     * set self to `true`.
     *
     * @default false
     */
    pub xml_mode: bool,

    pub tokenizer_options: TokenizerOptions,
}

lazy_static! {
    static ref RE_NAME_END: Regex = Regex::new(r"/\s|\//").unwrap();
}

pub struct Parser<'a> {
    html_mode: bool,

    buffer: &'a [u8],

    tokenizer: Tokenizer<'a>,
    tag_name: &'a str,
    next_nodes: VecDeque<Token>,
    stack: VecDeque<Box<str>>,
    foreign_context: VecDeque<bool>,
    attribs: BTreeMap<UniCase<Box<str>>, Option<(Box<str>, QuoteType)>>,
    attrib_value: Option<String>,
    attrib_name: UniCase<Box<str>>,
}

fn get_instruction_name(value: &str) -> Cow<str> {
    // Use the regex search method to find the index
    if let Some(index) = RE_NAME_END.find(value) {
        // Extract the substring up to the match index
        let name = value[..index.start()].to_string();

        return Cow::Owned(name);
    }

    Cow::Borrowed(value)
}

impl<'i> Parser<'i> {
    pub fn new<'a>(html: &'a str, options: &'a ParserOptions) -> Parser<'a> {
        let bytes = html.as_bytes();

        Parser {
            buffer: bytes,
            html_mode: !options.xml_mode,
            tokenizer: Tokenizer::new(&bytes, &options.tokenizer_options),
            tag_name: "".into(),
            next_nodes: Default::default(),
            stack: Default::default(),
            foreign_context: VecDeque::from([options.xml_mode]),
            attribs: Default::default(),
            attrib_value: None,
            attrib_name: Default::default(),
        }
    }

    unsafe fn on_text(&mut self, tokenizer_token: TokenizerToken) {
        self.next_nodes.push_back(Token {
            data: String::from_utf8_unchecked(
                self.buffer[tokenizer_token.start..tokenizer_token.end].to_owned(),
            ).into_boxed_str(),
            attrs: None,
            kind: TokenKind::Text,
            is_implied: false,
        });
    }

    fn on_text_entity(&mut self, tokenizer_token: TokenizerToken) {
        let data_string = char::from_u32(tokenizer_token.code).unwrap();

        self.next_nodes.push_back(Token {
            data: data_string.to_string().into_boxed_str(),
            attrs: None,
            kind: TokenKind::Text,
            is_implied: false,
        });
    }

    fn is_void_element(&self, name: &str) -> bool {
        self.html_mode && is_void_elements(name)
    }

    unsafe fn on_open_tag_name(&mut self, tokenizer_token: TokenizerToken) {
        let name = str::from_utf8_unchecked(
            &self.buffer[tokenizer_token.start..tokenizer_token.end],
        );

        self.emit_open_tag(name);
    }

    fn emit_open_tag(&mut self, name: &'i str) {
        self.tag_name = name;

        let open_implies_close_option: Option<fn(tag_name: &str) -> bool> =
            open_implies_close(&self.tag_name);

        if let Some(open_implies_close_fn) = open_implies_close_option {
            while !self.stack.is_empty() && open_implies_close_fn(&self.stack[0]) {
                let element = self.stack.pop_front().unwrap();

                self.next_nodes.push_back(Token {
                    data: element,
                    attrs: None,
                    kind: TokenKind::CloseTag,
                    is_implied: true,
                });
            }
        }
        if !self.is_void_element(&self.tag_name) {
            self.stack.push_front(self.tag_name.to_string().into_boxed_str());

            if self.html_mode {
                if is_foreign_context_elements(&self.tag_name) {
                    self.foreign_context.push_front(true);
                } else if is_html_integration_elements(&self.tag_name) {
                    self.foreign_context.push_front(false);
                }
            }
        }
    }

    fn end_open_tag(&mut self, is_implied: bool) {
        let is_void = self.is_void_element(&self.tag_name);

        let close_node_option = if is_void {
            Some(Token {
                data: self.tag_name.to_string().into_boxed_str(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: true,
            })
        } else {
            None
        };

        self.next_nodes.push_back(Token {
            data: self.tag_name.to_string().into_boxed_str(),
            attrs: if self.attribs.is_empty() {
                None
            } else {
                Some(take(&mut self.attribs))
            },
            kind: TokenKind::OpenTag,
            is_implied,
        });

        if let Some(close_node) = close_node_option {
            self.next_nodes.push_back(close_node);
        }
    }

    fn on_open_tag_end(&mut self) {
        self.end_open_tag(false);
    }

    unsafe fn on_close_tag(&mut self, tokenizer_token: TokenizerToken) {
        let name: &str =
            str::from_utf8_unchecked(&self.buffer[tokenizer_token.start..tokenizer_token.end]);

        if is_foreign_context_elements(name) || is_html_integration_elements(name) {
            self.foreign_context.pop_front();
        }

        if !self.is_void_element(name) {
            let pos = self.stack.iter().position(|n| &**n == name);
            if let Some(index) = pos {
                for i in 0..index + 1 {
                    let tag = self.stack.pop_front().unwrap();
                    self.next_nodes.push_back(Token {
                        data: tag,
                        attrs: None,
                        kind: TokenKind::CloseTag,
                        is_implied: i != index,
                    });
                }
            } else if self.html_mode && name == "p" {
                // Implicit open before close
                self.emit_open_tag("p");
                self.close_current_tag(true);
            }
        } else if self.html_mode && name == "br" {
            // We can't use `emit_open_tag` for implicit open, as `br` would be implicitly closed.
            self.next_nodes.push_back(Token {
                data: "br".to_string().into_boxed_str(),
                attrs: None,
                kind: TokenKind::OpenTag,
                is_implied: false,
            });
            self.next_nodes.push_back(Token {
                data: "br".to_string().into_boxed_str(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: false,
            });
        }
    }

    fn on_self_closing_tag(&mut self) {
        if self.foreign_context[0] {
            self.close_current_tag(false);
        } else {
            // Ignore the fact that the tag is self-closing.
            self.on_open_tag_end();
        }
    }

    fn close_current_tag(&mut self, is_open_implied: bool) {
        self.end_open_tag(is_open_implied);

        // Self-closing tags will be on the top of the stack
        if &*self.stack[0] == self.tag_name {
            // If the opening tag isn't implied, the closing tag has to be implied.
            self.next_nodes.push_back(Token {
                data: self.tag_name.to_string().into_boxed_str(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: !is_open_implied,
            });
            self.stack.pop_front();
        }
    }

    unsafe fn on_attrib_name(&mut self, tokenizer_token: TokenizerToken) {
        let name: &str =
            str::from_utf8_unchecked(&self.buffer[tokenizer_token.start..tokenizer_token.end]);

        self.attrib_name = UniCase::new(name.to_string().into_boxed_str());
    }

    unsafe fn on_attrib_data(&mut self, tokenizer_token: TokenizerToken) {
        let new_attrib = match self.attrib_value.take() {
            None => Some(String::from_utf8_unchecked(
                self.buffer[tokenizer_token.start..tokenizer_token.end].to_owned(),
            )),
            Some(existing_value) => {
                let mut modified_cow = existing_value;

                modified_cow.push_str(str::from_utf8_unchecked(
                    &self.buffer[tokenizer_token.start..tokenizer_token.end],
                ));

                Some(modified_cow)
            }
        };

        self.attrib_value = new_attrib;
    }

    fn on_attrib_entity(&mut self, tokenizer_token: TokenizerToken) {
        let c = char::from_u32(tokenizer_token.code).unwrap();

        let new_attrib = match self.attrib_value.take() {
            None => Some(c.to_string()),
            Some(existing_value) => {
                let mut owned_value = existing_value;
                owned_value.push(c);

                Some(owned_value)
            }
        };

        self.attrib_value = new_attrib;
    }

    fn on_attrib_end(&mut self, tokenizer_token: TokenizerToken) {
        if !self.attribs.contains_key(&self.attrib_name) {
            let new_attribute: Option<(Box<str>, QuoteType)> = self
                .attrib_value
                .as_mut()
                .map(|attrib_value| (attrib_value.clone().into_boxed_str(), tokenizer_token.quote));

            self.attribs.insert(self.attrib_name.to_owned(), new_attribute);
        }
        self.attrib_value = None;
    }

    unsafe fn on_declaration<'a>(&'a mut self, tokenizer_token: TokenizerToken) {
        let value: &str =
            str::from_utf8_unchecked(&self.buffer[tokenizer_token.start..tokenizer_token.end]);
        let name = get_instruction_name(&value);

        self.next_nodes.push_back(Token {
            data: name.to_string().into_boxed_str(),
            attrs: None,
            kind: TokenKind::ProcessingInstruction,
            is_implied: false,
        });
    }

    unsafe fn on_processing_instruction(&mut self, tokenizer_token: TokenizerToken) {
        let value: &str =
            str::from_utf8_unchecked(&self.buffer[tokenizer_token.start..tokenizer_token.end]);
        let name = get_instruction_name(value);

        self.next_nodes.push_back(Token {
            data: name.to_string().into_boxed_str(),
            attrs: None,
            kind: TokenKind::ProcessingInstruction,
            is_implied: false,
        });
    }

    unsafe fn on_comment(&mut self, tokenizer_token: TokenizerToken) {
        self.next_nodes.push_back(Token {
            data: String::from_utf8_unchecked(
                self.buffer[tokenizer_token.start..tokenizer_token.end].to_owned(),
            ).into_boxed_str(),
            attrs: None,
            kind: TokenKind::Comment,
            is_implied: false,
        });
        self.next_nodes.push_back(Token {
            data: "".into(),
            attrs: None,
            kind: TokenKind::CommentEnd,
            is_implied: false,
        });
    }

    unsafe fn on_cdata(&mut self, tokenizer_token: TokenizerToken) {
        self.on_comment(tokenizer_token);
    }

    fn onend(&mut self) {
        // Set the end index for all remaining tags
        let stack_iter = self.stack.iter();
        for item in stack_iter {
            self.next_nodes.push_back(Token {
                data: item.to_owned(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: true,
            })
        }

        self.stack.clear();
    }
    unsafe fn parser_next(&mut self) -> Option<Token> {
        loop {
            if let Some(existing_node) = self.next_nodes.pop_front() {
                return Some(existing_node);
            }

            let possible_token = self.tokenizer.next();

            match possible_token {
                None => return None,
                Some(tokenizer_token) => match tokenizer_token.location {
                    TokenizerTokenLocation::AttrData => self.on_attrib_data(tokenizer_token),
                    TokenizerTokenLocation::AttrEntity => self.on_attrib_entity(tokenizer_token),
                    TokenizerTokenLocation::AttrEnd => self.on_attrib_end(tokenizer_token),
                    TokenizerTokenLocation::AttrName => self.on_attrib_name(tokenizer_token),
                    TokenizerTokenLocation::CData => self.on_cdata(tokenizer_token),
                    TokenizerTokenLocation::CloseTag => self.on_close_tag(tokenizer_token),
                    TokenizerTokenLocation::Comment => self.on_comment(tokenizer_token),
                    TokenizerTokenLocation::Declaration => self.on_declaration(tokenizer_token),
                    TokenizerTokenLocation::OpenTagEnd => self.on_open_tag_end(),
                    TokenizerTokenLocation::OpenTagName => self.on_open_tag_name(tokenizer_token),
                    TokenizerTokenLocation::ProcessingInstruction => {
                        self.on_processing_instruction(tokenizer_token)
                    }
                    TokenizerTokenLocation::SelfClosingTag => self.on_self_closing_tag(),
                    TokenizerTokenLocation::Text => self.on_text(tokenizer_token),
                    TokenizerTokenLocation::TextEntity => self.on_text_entity(tokenizer_token),
                    TokenizerTokenLocation::End => self.onend(),
                },
            }
        }
    }
}

impl <'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        unsafe { self.parser_next() }
    }
}
