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

    #[test]
    fn special_script_tag() {
        insta::assert_debug_snapshot!(tokenize("<script /><div></div>"))
    }

    #[test]
    fn special_style_tag() {
        insta::assert_debug_snapshot!(tokenize("<style /><div></div>"))
    }

    #[test]
    fn special_title_tag() {
        insta::assert_debug_snapshot!(tokenize("<style /><div></div>"))
    }

    #[test]
    fn no_value_attribute() {
        insta::assert_debug_snapshot!(tokenize("<div aaaaaaa >"))
    }

    #[test]
    fn no_quote_attribute() {
        insta::assert_debug_snapshot!(tokenize("<div aaa=aaa >"))
    }

    #[test]
    fn single_quote_attribute() {
        insta::assert_debug_snapshot!(tokenize("<div aaa='a' >"))
    }

    #[test]
    fn double_quote_attribute() {
        insta::assert_debug_snapshot!(tokenize("<div aaa=\"a\" >"))
    }
}
