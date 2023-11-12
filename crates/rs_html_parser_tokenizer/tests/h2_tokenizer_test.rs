#[cfg(test)]
mod tests {
    use rs_html_parser_tokenizer::{TokenizerOptions, Tokenizer};
    use rs_html_parser_tokenizer_tokens::Token;

    fn tokenize(data: &str) -> Vec<Token> {
        let mut log: Vec<Token> = Vec::new();

        let options = TokenizerOptions {
            xml_mode: Option::from(false),
            decode_entities: Option::from(true),
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

    #[test]
    fn for_normal_special_tag() {
        insta::assert_debug_snapshot!(tokenize("<style>a{}</style>&apos;<br/>"))
    }

    #[test]
    fn for_normal_special_tag2() {
        insta::assert_debug_snapshot!(tokenize("<style>a{}</style>&apos; 1234&apos;dsa<br/>"))
    }

    #[test]
    fn for_normal_self_closing_special_tag() {
        insta::assert_debug_snapshot!((tokenize("<style />&apos;<br/>")))
    }

    #[test]
    fn entities_for_xml_entities() {
        insta::assert_debug_snapshot!((tokenize("&amp;&gt;&amp&lt;&uuml;&#x61;&#x62&#99;&#100&#101")))
    }

    #[test]
    fn entities_for_xml_incorrect_after_valid() {
        insta::assert_debug_snapshot!(tokenize("&amp;&gt;&amp&sometext;&uuml"))
    }

    #[test]
    fn entities_for_attributes() {
        insta::assert_debug_snapshot!(tokenize("<img src=\"?&image_uri=1&&image;=2&image=3\"/>?&image_uri=1&&image;=2&image=3"))
    }

    #[test]
    fn for_trailing_legacy_entity() {
        insta::assert_debug_snapshot!(tokenize("&timesbar;&timesbar"))
    }

    #[test]
    fn for_multi_byte_entities() {
        insta::assert_debug_snapshot!(tokenize("&NotGreaterFullEqual;"))
    }
}
