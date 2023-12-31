mod test_utils;

mod tests {
    use crate::test_utils::*;

    #[test]
    fn basic_element() {
        insta::assert_debug_snapshot!(parser_test("<div></div>"))
    }

    #[test]
    fn basic_element_with_text() {
        insta::assert_debug_snapshot!(parser_test("<span>Hello World!</span>"))
    }

    #[test]
    fn special_script_tag() {
        /*
         * However, a Script Element is never a void or a parametric Element,
         * because script tag before anything else, is a Browser Instruction,
         * not a Data Description declaration.
         * -- this is why div is inside script tag
         */
        insta::assert_debug_snapshot!(parser_test("<script /><div>1</div>"))
    }

    #[test]
    fn special_style_tag() {
        insta::assert_debug_snapshot!(parser_test("<style /><div></div>"))
    }

    #[test]
    fn special_title_tag() {
        insta::assert_debug_snapshot!(parser_test("<title /><div></div>"))
    }

    #[test]
    fn no_value_attribute() {
        insta::assert_debug_snapshot!(parser_test("<div aaaaaaa >"))
    }

    #[test]
    fn no_quote_attribute() {
        insta::assert_debug_snapshot!(parser_test("<div aaa=aaa >"))
    }

    #[test]
    fn single_quote_attribute() {
        insta::assert_debug_snapshot!(parser_test("<div aaa='a' >"))
    }

    #[test]
    fn double_quote_attribute() {
        insta::assert_debug_snapshot!(parser_test("<div aaa=\"a\" >"))
    }

    #[test]
    fn for_normal_special_tag() {
        insta::assert_debug_snapshot!(parser_test("<style>a{}</style>&apos;<br/>"))
    }

    #[test]
    fn for_normal_special_tag2() {
        insta::assert_debug_snapshot!(parser_test("<style>a{}</style>&apos; 1234&apos;dsa<br/>"))
    }

    #[test]
    fn for_normal_self_closing_special_tag() {
        insta::assert_debug_snapshot!((parser_test("<style />&apos;<br/>")))
    }

    #[test]
    fn entities_for_xml_entities() {
        insta::assert_debug_snapshot!(
            (parser_test("&amp;&gt;&amp&lt;&uuml;&#x61;&#x62&#99;&#100&#101"))
        )
    }

    #[test]
    fn entities_for_xml_incorrect_after_valid() {
        insta::assert_debug_snapshot!(parser_test("&amp;&gt;&amp&sometext;&uuml"))
    }

    #[test]
    fn entities_for_attributes() {
        insta::assert_debug_snapshot!(parser_test(
            "<img src=\"?&image_uri=1&&image;=2&image=3\"/>?&image_uri=1&&image;=2&image=3"
        ))
    }

    #[test]
    fn for_trailing_legacy_entity() {
        insta::assert_debug_snapshot!(parser_test("&timesbar;&timesbar"))
    }

    #[test]
    fn for_multi_byte_entities() {
        insta::assert_debug_snapshot!(parser_test("&NotGreaterFullEqual;"))
    }
}
