mod element_info;

use std::str;
use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use phf::Set;
use rs_html_parser_tokens::{Token, TokenKind};
use rs_html_parser_tokenizer::{Tokenizer, TokenizerOptions};
use rs_html_parser_tokenizer_tokens::{TokenizerToken, TokenizerTokenLocation};
use regex::Regex;
use crate::element_info::{FOREIGN_CONTEXT_ELEMENTS, HTML_INTEGRATION_ELEMENTS, OPEN_IMPLIES_CLOSE, VOID_ELEMENTS};

pub struct ParserOptions {
    /**
     * Indicates whether special tags (`<script>`, `<style>`, and `<title>`) should get special treatment
     * and if "empty" tags (eg. `<br>`) can have children.  If `false`, the content of special tags
     * will be text only. For feeds and other XML content (documents that don't consist of HTML),
     * set self to `true`.
     *
     * @default false
     */
    xmlMode: bool,

    /**
     * Decode entities within the document.
     *
     * @default true
     */
    decodeEntities: bool,

    /**
     * If set to true, all tags will be lowercased.
     *
     * @default !xmlMode
     */
    lowerCaseTags: bool,

    /**
     * If set to `true`, all attribute names will be lowercased. self has noticeable impact on speed.
     *
     * @default !xmlMode
     */
    lowerCaseAttributeNames: bool,

    /**
     * If set to true, CDATA sections will be recognized as text even if the xmlMode option is not enabled.
     * NOTE: If xmlMode is set to `true` then CDATA sections will always be recognized as text.
     *
     * @default xmlMode
     */
    recognizeCDATA: bool,

    /**
     * If set to `true`, self-closing tags will trigger the onclosetag event even if xmlMode is not set to `true`.
     * NOTE: If xmlMode is set to `true` then self-closing tags will always be recognized.
     *
     * @default xmlMode
     */
    // recognizeSelfClosing: bool,

    tokenizer_options: TokenizerOptions,
}

lazy_static! {
    static ref RE_NAME_END: Regex =  Regex::new(r"/\s|\//").unwrap();
}

pub struct Parser<'a> {
    /** The start index of the last event. */
    startIndex: usize,
    /** The end index of the last event. */
    endIndex: usize,
    /**
     * Store the start index of the current open tag,
     * so we can update the start index for attributes.
     */
    openTagStar: usize,

    // tagname = "";
    // attribname = "";
    // attribvalue = "";
    // attribs: null | { [key: string]: string } = null;
    // stack: string[] = [];
    // /** Determines whether self-closing tags are recognized. */
    // foreignContext: bool[];
    // cbs: Partial<Handler>;
    // lowerCaseTagNames: bool;
    // lowerCaseAttributeNames: bool;
    // recognizeSelfClosing: bool;
    // /** We are parsing HTML. Inverse of the `xmlMode` option. */
    htmlMode: bool,
    // tokenizer: Tokenizer;
    //
    // buffers: string[] = [];
    // bufferOffset = 0;
    // /** The index of the last written buffer. Used when resuming after a `pause()`. */
    // writeIndex = 0;
    // /** Indicates whether the parser has finished running / `.end` has been called. */
    // ended = false;

    buffer: &'a [u8],

    tokenizer_iterator: Tokenizer<'a>,
    openTagStart: usize,
    tagname: String,
    next_nodes: VecDeque<Token>,
    stack: VecDeque<String>,
    foreignContext: VecDeque<bool>,
    attribs: HashMap<String, String>,
    attribvalue: String,
    attribname: String,
}

impl Parser<'_> {
    pub fn new(html: &str, options: ParserOptions) -> Parser  {
        let bytes = html.as_bytes();

        Parser {
            buffer: bytes,
            startIndex: 0,
            endIndex: 0,
            openTagStar: 0,
            htmlMode: !options.xmlMode,
            tokenizer_iterator: Tokenizer::new(bytes, options.tokenizer_options).into_iter(),
            openTagStart: 0,
            tagname: "".to_string(),
            next_nodes: Default::default(),
            stack: Default::default(),
            foreignContext: Default::default(),
            attribs: Default::default(),
            attribvalue: "".to_string(),
            attribname: "".to_string(),
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

        self.endIndex = (tokenizer_token.end - 1) as usize;
        self.startIndex = tokenizer_token.end as usize;
    }

    /** @internal */
    fn ontextentity(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = (tokenizer_token.end - 1) as usize;

        let data_string = char::from_u32(tokenizer_token.code).unwrap().to_string();

        self.next_nodes.push_back(Token {
            data: data_string,
            attrs: None,
            kind: TokenKind::Text,
            is_implied: false,
        });

        self.startIndex = tokenizer_token.end;
    }

    /**
     * Checks if the current tag is a void element. Override self if you want
     * to specify your own additional void elements.
     */
    fn is_void_element(&self, name: &str) -> bool {
        return self.htmlMode && VOID_ELEMENTS.contains(name);
    }

    /** @internal */
    fn onopentagname(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;

        let name = str::from_utf8(&self.buffer[tokenizer_token.start .. tokenizer_token.end]).unwrap();

        self.emitOpenTag(name.to_lowercase());
    }

    fn emitOpenTag(&mut self, name: String) {
        self.openTagStart = self.startIndex;
        let name2 = name.clone();
        self.tagname = name;

        let implies_close_option: Option<&Set<&'static str>> = OPEN_IMPLIES_CLOSE.get(&*name2);

        if let Some(implies_closed) = implies_close_option {
            while self.stack.len() > 0 && implies_closed.contains(&self.stack[0]) {
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

            if self.htmlMode {
                if FOREIGN_CONTEXT_ELEMENTS.contains(&name2) {
                    self.foreignContext.push_front(true);
                } else if HTML_INTEGRATION_ELEMENTS.contains(&name2) {
                    self.foreignContext.push_front(false);
                }
            }
        }

        self.next_nodes.push_back(Token {
            data: name2.clone(),
            attrs: None,
            kind: TokenKind::OpenTag,
            is_implied: false,
        });
    }

    fn endOpenTag(&mut self, isImplied: bool) {
        let tag_name: &str = self.tagname.as_ref();
        self.startIndex = self.openTagStart;

        if !self.attribs.is_empty() {
            self.next_nodes.push_back(
                Token {
                    data: tag_name.to_string(),
                    attrs: Some(self.attribs.to_owned()),
                    kind: TokenKind::OpenTag,
                    is_implied: isImplied,
                }
            );
            self.attribs = Default::default();
        }

        if self.is_void_element(tag_name) {
            self.next_nodes.push_back(
                Token {
                    data: tag_name.to_string(),
                    attrs: Some(self.attribs.to_owned()),
                    kind: TokenKind::CloseTag,
                    is_implied: isImplied,
                }
            );
        }

        self.tagname = "".into();
    }

    /** @internal */
    fn onopentagend(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;
        self.endOpenTag(false);

        // Set `startIndex` for next node
        self.startIndex = (tokenizer_token.end + 1) as usize;
    }

    /** @internal */
    fn onclosetag(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;

        let name = &*str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap().to_lowercase();

        if FOREIGN_CONTEXT_ELEMENTS.contains(name) || HTML_INTEGRATION_ELEMENTS.contains(name) {
            self.foreignContext.pop_front();
        }

        if !self.is_void_element(name) {
            let pos = self.stack.iter().position(|n| {
                n == name
            });
            if let Some(index) = pos {
                for i in 0..index {
                    let tag = self.stack.pop_front().unwrap();
                    self.next_nodes.push_back(Token {
                        data: tag,
                        attrs: None,
                        kind: TokenKind::CloseTag,
                        is_implied: i != index,
                    });
                }
            } else if self.htmlMode && name == "p" {
                // Implicit open before close
                self.emitOpenTag(String::from("p"));
                self.closeCurrentTag(true);
            }
        } else if self.htmlMode && name == "br" {
            // We can't use `emitOpenTag` for implicit open, as `br` would be implicitly closed.
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

        // Set `startIndex` for next node
        self.startIndex = (tokenizer_token.end + 1) as usize;
    }

    /** @internal */
    fn onselfclosingtag(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;
        if self.foreignContext[0] {
            self.closeCurrentTag(false);

            // Set `startIndex` for next node
            self.startIndex = (tokenizer_token.end + 1) as usize;
        } else {
            // Ignore the fact that the tag is self-closing.
            self.onopentagend(tokenizer_token);
        }
    }

    fn closeCurrentTag(&mut self, isOpenImplied: bool) {
        self.endOpenTag(isOpenImplied);

        // Self-closing tags will be on the top of the stack
        if &self.stack[0] == &self.tagname {
            // If the opening tag isn't implied, the closing tag has to be implied.
            self.next_nodes.push_back(Token {
                data: self.tagname.to_owned(),
                attrs: None,
                kind: TokenKind::CloseTag,
                is_implied: !isOpenImplied,
            });
            self.stack.pop_front();
        }
    }

    /** @internal */
    fn onattribname(&mut self, tokenizer_token: TokenizerToken) {
        self.startIndex = tokenizer_token.start as usize;
        let name = str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap();

        self.attribname = name.to_lowercase();
    }

    /** @internal */
    fn onattribdata(&mut self, tokenizer_token: TokenizerToken) {
        self.attribvalue += str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap();
    }

    /** @internal */
    fn onattribentity(&mut self, tokenizer_token: TokenizerToken) {
        self.attribvalue += &*char::from_u32(tokenizer_token.code).unwrap().to_string();
    }

    /** @internal */
    fn onattribend(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;

        // self.cbs.onattribute?.(
        //     self.attribname,
        //     self.attribvalue,
        //     quote == QuoteType.Double
        //         ? '"'
        //         : quote == QuoteType.Single
        //         ? "'"
        //         : quote == QuoteType.NoValue
        //         ? undefined
        //         : null,
        // );

        if !self.attribs.contains_key(&self.attribname) {
            self.attribs.insert(self.attribname.to_owned(), self.attribvalue.to_owned());
        }
        self.attribvalue = String::from("");
    }

    fn getInstructionName(&mut self, value: &str) -> String {

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
        self.endIndex = tokenizer_token.end as usize;
        let value: &str = str::from_utf8(&self.buffer[tokenizer_token.start..tokenizer_token.end]).unwrap();

        let name: &str = &self.getInstructionName(value);

        self.next_nodes.push_back(Token {
            data: "".to_string(),
            attrs: Some(HashMap::from([
                (format!("!${name}"), format!("!{value}"))
            ])),
            kind: TokenKind::ProcessingInstruction,
            is_implied: false,
        });

        // Set `startIndex` for next node
        self.startIndex = (tokenizer_token.end + 1) as usize;
    }

    /** @internal */
    fn onprocessinginstruction(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;
        let value = String::from_utf8(self.buffer[tokenizer_token.start..tokenizer_token.end].to_owned()).unwrap();

        let name = self.getInstructionName(&value);

        self.next_nodes.push_back(Token {
            data: "".to_string(),
            attrs: Some(HashMap::from([
                (format!("?${name}"), format!("?{value}"))
            ])),
            kind: TokenKind::ProcessingInstruction,
            is_implied: false,
        });

        // Set `startIndex` for next node
        self.startIndex = (tokenizer_token.end + 1) as usize;
    }

    /** @internal */
    fn oncomment(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;

        self.next_nodes.push_back(Token {
            data: String::from_utf8(self.buffer[tokenizer_token.start..tokenizer_token.end - tokenizer_token.offset].to_owned()).unwrap(),
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

        // Set `startIndex` for next node
        self.startIndex = (tokenizer_token.end + 1) as usize;
    }

    /** @internal */
    fn oncdata(&mut self, tokenizer_token: TokenizerToken) {
        self.endIndex = tokenizer_token.end as usize;

        self.next_nodes.push_back(Token {
            data: String::from_utf8(self.buffer[tokenizer_token.start..tokenizer_token.end - tokenizer_token.offset].to_owned()).unwrap(),
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

        // Set `startIndex` for next node
        self.startIndex = (tokenizer_token.end + 1) as usize;
    }

    /** @internal */
    fn onend(&mut self) {
        // Set the end index for all remaining tags
        self.endIndex = self.startIndex;

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
