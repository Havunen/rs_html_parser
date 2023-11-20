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
        assert_debug_snapshot!(tokenize("<div></div>"))
    }

    #[test]
    fn basic_element_with_text() {
        assert_debug_snapshot!(tokenize("<span>Hello World!</span>"))
    }

    #[test]
    fn special_script_tag() {
        assert_debug_snapshot!(tokenize("<script /><div></div>"))
    }

    #[test]
    fn special_style_tag() {
        assert_debug_snapshot!(tokenize("<style /><div></div>"))
    }

    #[test]
    fn special_title_tag() {
        assert_debug_snapshot!(tokenize("<title /><div></div>"))
    }

    #[test]
    fn no_value_attribute() {
        assert_debug_snapshot!(tokenize("<div aaaaaaa >"))
    }

    #[test]
    fn no_quote_attribute() {
        assert_debug_snapshot!(tokenize("<div aaa=aaa >"))
    }

    #[test]
    fn single_quote_attribute() {
        assert_debug_snapshot!(tokenize("<div aaa='a' >"))
    }

    #[test]
    fn double_quote_attribute() {
        assert_debug_snapshot!(tokenize("<div aaa=\"a\" >"))
    }

    #[test]
    fn for_normal_special_tag() {
        assert_debug_snapshot!(tokenize("<style>a{}</style>&apos;<br/>"))
    }

    #[test]
    fn for_normal_special_tag2() {
        assert_debug_snapshot!(tokenize("<style>a{}</style>&apos; 1234&apos;dsa<br/>"))
    }

    #[test]
    fn for_normal_self_closing_special_tag() {
        assert_debug_snapshot!((tokenize("<style />&apos;<br/>")))
    }

    #[test]
    fn entities_for_xml_entities() {
        assert_debug_snapshot!((tokenize("&amp;&gt;&amp&lt;&uuml;&#x61;&#x62&#99;&#100&#101")))
    }

    #[test]
    fn entities_for_xml_incorrect_after_valid() {
        assert_debug_snapshot!(tokenize("&amp;&gt;&amp&sometext;&uuml"))
    }

    #[test]
    fn entities_for_attributes() {
        assert_debug_snapshot!(tokenize("<img src=\"?&image_uri=1&&image;=2&image=3\"/>?&image_uri=1&&image;=2&image=3"))
    }

    #[test]
    fn for_trailing_legacy_entity() {
        assert_debug_snapshot!(tokenize("&timesbar;&timesbar"))
    }

    #[test]
    fn for_multi_byte_entities() {
        assert_debug_snapshot!(tokenize("&NotGreaterFullEqual;"))
    }
}
