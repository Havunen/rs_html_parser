
#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use rs_html_parser_tokenizer::{TokenizerOptions, Tokenizer};
    use rs_html_parser_tokenizer_tokens::TokenizerToken;

    fn tokenize(data: &str) -> Vec<TokenizerToken> {
        let mut log: Vec<TokenizerToken> = Vec::new();

        let options = TokenizerOptions {
            xml_mode: Option::from(false),
            decode_entities: Option::from(true),
        };

        let tokenizer = Tokenizer::new(data.as_bytes(), &options);

        for token in tokenizer {
            log.push(token);
        }

        log
    }

    #[test]
    fn basic_element() {
        assert_debug_snapshot!(tokenize("<a a=a&>foo"));
    }

    #[test]
    fn short_comment() {
        assert_debug_snapshot!(tokenize("<!--->"));
    }

    #[test]
    fn dash_in_comment() {
        assert_debug_snapshot!(tokenize("<!----->"));
    }

    #[test]
    fn invalid_end_comment() {
        assert_debug_snapshot!(tokenize("</0"));
    }

    #[test]
    fn open_comment_nothing_else() {
        // This should have a comment node, but in reality having only this content is super rare
        assert_debug_snapshot!(tokenize("<!--"));
    }

    #[test]
    fn orphan_end_tag() {
        // This could change, there is open and then EOF
        assert_debug_snapshot!(tokenize(r####"<z
"####));
    }
}
