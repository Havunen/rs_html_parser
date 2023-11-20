mod element_info;

use std::str;
use std::collections::{BTreeMap, VecDeque};
use lazy_static::lazy_static;
use rs_html_parser_tokens::{Token, TokenKind};
use rs_html_parser_tokenizer::{Tokenizer, TokenizerOptions};
use rs_html_parser_tokenizer_tokens::{QuoteType, TokenizerToken, TokenizerTokenLocation};
use regex::Regex;
use crate::element_info::{is_foreign_context_elements, is_html_integration_elements, is_void_elements, open_implies_close};

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

    /**
     * Decode entities within the document.
     *
     * @default true
     */
    pub decode_entities: bool,

    /**
     * If set to true, all tags will be lowercased.
     *
     * @default !xml_mode
     */
    pub lower_case_tags: bool,

    /**
     * If set to `true`, all attribute names will be lowercased. self has noticeable impact on speed.
     *
     * @default !xml_mode
     */
    pub lower_case_attribute_names: bool,

    /**
     * If set to true, CDATA sections will be recognized as text even if the xml_mode option is not enabled.
     * NOTE: If xml_mode is set to `true` then CDATA sections will always be recognized as text.
     *
     * @default xml_mode
     */
    pub recognize_cdata: bool,

    pub tokenizer_options: TokenizerOptions,
}

lazy_static! {
    static ref RE_NAME_END: Regex =  Regex::new(r"/\s|\//").unwrap();
}

pub struct Parser<'a> {
    html_mode: bool,

    buffer: &'a [u8],

    tokenizer_iterator: Tokenizer<'a>,
    tag_name: String,
    next_nodes: VecDeque<Token>,
    stack: VecDeque<String>,
    foreign_context: VecDeque<bool>,
    attribs: BTreeMap<String, Option<(String, QuoteType)>>,
    attrib_value: Option<String>,
    attrib_name: String,
}

impl Parser<'_> {
    pub fn new<'a>(html: &'a str, options: &'a ParserOptions) -> Parser<'a>  {
        let bytes = html.as_bytes();

        Parser {
            buffer: bytes,
            html_mode: !options.xml_mode,
            tokenizer_iterator: Tokenizer::new(bytes, &options.tokenizer_options).into_iter(),
            tag_name: "".to_string(),
            next_nodes: Default::default(),
            stack: Default::default(),
            foreign_context: VecDeque::from([options.xml_mode]),
            attribs: Default::default(),
            attrib_value: None,
            attrib_name: "".to_string(),
        }
    }

    fn ontext(&mut self, tokenizer_token: TokenizerToken) {
        self.next_nodes.push_back(
        Token {
            data: str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap().to_owned(),
            attrs: None,
            kind: TokenKind::Text,
            is_implied: false,
        });
    }

    /** @internal */
    fn ontextentity(&mut self, tokenizer_token: TokenizerToken) {
        let data_string = char::from_u32(tokenizer_token.code).unwrap().to_string();

        self.next_nodes.push_back(Token {
            data: data_string,
            attrs: None,
            kind: TokenKind::Text,
            is_implied: false,
        });
    }

    /**
     * Checks if the current tag is a void element. Override self if you want
     * to specify your own additional void elements.
     */
    fn is_void_element(&self, name: &str) -> bool {
        return self.html_mode && is_void_elements(name);
    }

    /** @internal */
    fn onopentagname(&mut self, tokenizer_token: TokenizerToken) {
        let name = str::from_utf8(&self.buffer[tokenizer_token.start .. tokenizer_token.end]).unwrap();

        self.emit_open_tag(name.to_lowercase());
    }

    fn emit_open_tag(&mut self, name: String) {
        let name2 = name.clone();
        self.tag_name = name;

        let open_implies_close_option: Option<fn(tag_name: &str) -> bool>= open_implies_close(&name2);

        if let Some(open_implies_close_fn) = open_implies_close_option {
            while self.stack.len() > 0 && open_implies_close_fn(&self.stack[0]) {
                let element = self.stack.pop_front().unwrap();

                self.next_nodes.push_back(Token {
                    data: element,
                    attrs: None,
                    kind: TokenKind::CloseTag,
                    is_implied: true
                });
            }
        }
        if !self.is_void_element(&name2) {
            self.stack.push_front(name2.clone());

            if self.html_mode {
                if is_foreign_context_elements(&name2) {
                    self.foreign_context.push_front(true);
                } else if is_html_integration_elements(&name2) {
                    self.foreign_context.push_front(false);
                }
            }
        }
    }

    fn end_open_tag(&mut self, is_implied: bool) {
        let tag_name: &str = self.tag_name.as_ref();

        self.next_nodes.push_back(
            Token {
                data: tag_name.to_string(),
                attrs: if self.attribs.is_empty() { None } else { Some(self.attribs.to_owned()) },
                kind: TokenKind::OpenTag,
                is_implied,
            }
        );
        self.attribs.clear();

        if self.is_void_element(tag_name) {
            self.next_nodes.push_back(
                Token {
                    data: tag_name.to_string(),
                    attrs: None,
                    kind: TokenKind::CloseTag,
                    is_implied: true,
                }
            );
        }

        self.tag_name = "".into();
    }

    /** @internal */
    fn onopentagend(&mut self, tokenizer_token: TokenizerToken) {
        self.end_open_tag(false);
    }

    /** @internal */
    fn onclosetag(&mut self, tokenizer_token: TokenizerToken) {
        let name = &*str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap().to_lowercase();

        if is_foreign_context_elements(name) || is_html_integration_elements(name) {
            self.foreign_context.pop_front();
        }

        if !self.is_void_element(name) {
            let pos = self.stack.iter().position(|n| {
                n == name
            });
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
                self.emit_open_tag(String::from("p"));
                self.close_current_tag(true);
            }
        } else if self.html_mode && name == "br" {
            // We can't use `emit_open_tag` for implicit open, as `br` would be implicitly closed.
            self.next_nodes.push_back(Token {
                data: "br".to_string(),
                attrs: None,
                kind: TokenKind::OpenTag,
                is_implied: false,
            });
            self.next_nodes.push_back(Token {
                data: "br".to_string(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: false,
            });
        }
    }

    /** @internal */
    fn onselfclosingtag(&mut self, tokenizer_token: TokenizerToken) {
        if self.foreign_context[0] {
            self.close_current_tag(false);
        } else {
            // Ignore the fact that the tag is self-closing.
            self.onopentagend(tokenizer_token);
        }
    }

    fn close_current_tag(&mut self, is_open_implied: bool) {
        self.end_open_tag(is_open_implied);

        // Self-closing tags will be on the top of the stack
        if &self.stack[0] == &self.tag_name {
            // If the opening tag isn't implied, the closing tag has to be implied.
            self.next_nodes.push_back(Token {
                data: self.tag_name.to_owned(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: !is_open_implied,
            });
            self.stack.pop_front();
        }
    }

    /** @internal */
    fn onattribname(&mut self, tokenizer_token: TokenizerToken) {
        let name = str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap();

        self.attrib_name = name.to_lowercase();
    }

    /** @internal */
    fn onattribdata(&mut self, tokenizer_token: TokenizerToken) {
        let new_value = str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap();
        if self.attrib_value.is_some() {
            let mut existing_value = self.attrib_value.clone().unwrap();
            existing_value += new_value;
            self.attrib_value = Some(existing_value);
        } else {
            self.attrib_value = Some(new_value.to_string())
        }
    }

    /** @internal */
    fn onattribentity(&mut self, tokenizer_token: TokenizerToken) {
        let new_value = &*char::from_u32(tokenizer_token.code).unwrap().to_string();
        if self.attrib_value.is_some() {
            let mut existing_value = self.attrib_value.clone().unwrap();
            existing_value += new_value;
            self.attrib_value = Some(existing_value);
        } else {
            self.attrib_value = Some(new_value.to_string())
        }
    }

    /** @internal */
    fn onattribend(&mut self, tokenizer_token: TokenizerToken) {
        if !self.attribs.contains_key(&self.attrib_name) {
            let new_attribute: Option<(String, QuoteType)> = if let Some(attrib_value) = self.attrib_value.as_deref_mut() {
                Some((attrib_value.to_owned(), tokenizer_token.quote))
            } else {
                None
            };

            self.attribs.insert(
                self.attrib_name.to_owned(),
                new_attribute
            );
        }
        self.attrib_value = None;
    }

    fn get_instruction_name(&mut self, value: &str) -> String {

        // Use the regex search method to find the index
        if let Some(index) = RE_NAME_END.find(value) {
            // Extract the substring up to the match index
            let name = &value[..index.start()].to_string();

            return name.to_lowercase();
        }

        return value.to_string()
    }

    /** @internal */
    fn ondeclaration(&mut self, tokenizer_token: TokenizerToken) {
        let value: &str = str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap();
        let name: &str = &self.get_instruction_name(value);

        self.next_nodes.push_back(Token {
            data: "".to_string(),
            attrs: Some(BTreeMap::from([
                (format!("!{name}"), None)
            ])),
            kind: TokenKind::ProcessingInstruction,
            is_implied: false,
        });
    }

    /** @internal */
    fn onprocessinginstruction(&mut self, tokenizer_token: TokenizerToken) {
        let value = String::from_utf8(self.buffer[tokenizer_token.start..tokenizer_token.end].to_owned()).unwrap();
        let name = self.get_instruction_name(&value);

        self.next_nodes.push_back(Token {
            data: "".to_string(),
            attrs: Some(BTreeMap::from([
                (format!("?{name}"), None)
            ])),
            kind: TokenKind::ProcessingInstruction,
            is_implied: false,
        });
    }

    /** @internal */
    fn oncomment(&mut self, tokenizer_token: TokenizerToken) {
        self.next_nodes.push_back(Token {
            data: String::from_utf8(self.buffer[tokenizer_token.start..tokenizer_token.end].to_owned()).unwrap(),
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

    /** @internal */
    fn oncdata(&mut self, tokenizer_token: TokenizerToken) {
        self.oncomment(tokenizer_token);
    }

    /** @internal */
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
    fn parse_next(&mut self, tokenizer_token: TokenizerToken) -> Option<Token> {
        match tokenizer_token.location {
            TokenizerTokenLocation::AttrData => self.onattribdata(tokenizer_token),
            TokenizerTokenLocation::AttrEntity => self.onattribentity(tokenizer_token),
            TokenizerTokenLocation::AttrEnd => self.onattribend(tokenizer_token),
            TokenizerTokenLocation::AttrName => self.onattribname(tokenizer_token),
            TokenizerTokenLocation::CData => self.oncdata(tokenizer_token),
            TokenizerTokenLocation::CloseTag => self.onclosetag(tokenizer_token),
            TokenizerTokenLocation::Comment => self.oncomment(tokenizer_token),
            TokenizerTokenLocation::Declaration => self.ondeclaration(tokenizer_token),
            TokenizerTokenLocation::OpenTagEnd => self.onopentagend(tokenizer_token),
            TokenizerTokenLocation::OpenTagName => self.onopentagname(tokenizer_token),
            TokenizerTokenLocation::ProcessingInstruction => self.onprocessinginstruction(tokenizer_token),
            TokenizerTokenLocation::SelfClosingTag => self.onselfclosingtag(tokenizer_token),
            TokenizerTokenLocation::Text => self.ontext(tokenizer_token),
            TokenizerTokenLocation::TextEntity => self.ontextentity(tokenizer_token),
            TokenizerTokenLocation::End => self.onend()
        }

        self.next()
    }
}


impl<'i> Iterator for Parser<'i> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(existing_node) = self.next_nodes.pop_front() {
            return Some(existing_node);
        }
        let possible_token = self.tokenizer_iterator.next();

        match possible_token {
            None => return None,
            Some(tokenizer_token) => {
                self.parse_next(tokenizer_token)
            }
        }
    }
}
