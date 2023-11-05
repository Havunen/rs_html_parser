#[cfg(test)]
mod tests {
    use rs_html_parser_tokenizer::{Options, Tokenizer};
    use rs_html_parser_tokens::Token;

    fn tokenize(data: &str) -> Vec<Token> {
        let mut log: Vec<Token> = Vec::new();

        let options = Options {
            xml_mode: false,
            decode_entities: false,
        };

        let tokenizer = Tokenizer::new(data, options);

        for token in tokenizer {
            log.push(token);
        }

        return log;
    }

    #[test]
    fn basic_element() {
        insta::assert_debug_snapshot!(tokenize("<div></div>"))
    }

    #[test]
    fn basic_element_with_text() {
        insta::assert_debug_snapshot!(tokenize("<span>Hello World!</span>"))
    }
}
