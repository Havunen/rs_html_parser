mod test_utils;

mod tests {
    use insta::{assert_debug_snapshot, with_settings};
    use crate::test_utils::*;


// Spec valid tests

    #[test]
    fn correct_doctype_lowercase() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!DOCTYPE html>"));
        });
    }

    #[test]
    fn correct_doctype_uppercase() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!DOCTYPE HTML>"));
        });
    }

    #[test]
    fn correct_doctype_mixed_case() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!DOCTYPE HtMl>"));
        });
    }

    #[test]
    fn doctype_in_error() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!DOCTYPE foo>"));
        });
    }

    #[test]
    fn single_start_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h>"));
        });
    }

    #[test]
    fn start_tag_w_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='b'>"));
        });
    }

    #[test]
    fn start_tag_w_attribute_no_quotes() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a=b>"));
        });
    }

    #[test]
    fn start_end_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h></h>"));
        });
    }

    #[test]
    fn two_unclosed_start_tags() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<p>One<p>Two"));
        });
    }

    #[test]
    fn multiple_atts() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='b' c='d'>"));
        });
    }

    #[test]
    fn simple_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!--comment-->"));
        });
    }

    #[test]
    fn commentcomma__central_dash_no_space() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!----->"));
        });
    }

    #[test]
    fn commentcomma__two_central_dashes() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- --comment -->"));
        });
    }

    #[test]
    fn commentcomma__central_less_than_bang() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!--<!-->"));
        });
    }

    #[test]
    fn short_comment_three() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!---->"));
        });
    }

    #[test]
    fn lt__in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- <test-->"));
        });
    }

    #[test]
    fn lt_lt__in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!--<<-->"));
        });
    }

    #[test]
    fn lt_exclmark__in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- <!test-->"));
        });
    }

    #[test]
    fn lt_exclmark___in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- <!-test-->"));
        });
    }

    #[test]
    fn ampersand_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&"));
        });
    }

    #[test]
    fn ampersand_ampersand_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&&"));
        });
    }

    #[test]
    fn ampersand_space_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("& "));
        });
    }

    #[test]
    fn unfinished_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&f"));
        });
    }

    #[test]
    fn entity_with_trailing_semicolon__1_() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("I'm &not;it"));
        });
    }

    #[test]
    fn entity_with_trailing_semicolon__2_() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("I'm &notin;"));
        });
    }

    #[test]
    fn partial_entity_match_at_end_of_file() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("I'm &no"));
        });
    }

    #[test]
    fn non_ascii_character_reference_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&¬;"));
        });
    }

    #[test]
    fn ascii_decimal_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&#0036;"));
        });
    }

    #[test]
    fn ascii_hexadecimal_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&#x3f;"));
        });
    }

    #[test]
    fn hexadecimal_entity_in_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='&#x3f;'></h>"));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon_ending_in_x() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='&notx'>"));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon_ending_in_1() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='&not1'>"));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon_ending_in_i() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='&noti'>"));
        });
    }

    #[test]
    fn unquoted_attribute_ending_in_ampersand() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<s o=& t>"));
        });
    }

    #[test]
    fn unquoted_attribute_at_end_of_tag_with_final_character_of_amp_comma__with_tag_followed_by_characters() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<a a=a&>foo"));
        });
    }

    #[test]
    fn plaintext_element() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<plaintext>foobar"));
        });
    }

// Spec error tests

    #[test]
    fn correct_doctype_case_with_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!DOCTYPE HtMl"));
        });
    }

    #[test]
    fn truncated_doctype_start() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!DOC>"));
        });
    }

    #[test]
    fn empty_end_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("</>"));
        });
    }

    #[test]
    fn empty_start_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<>"));
        });
    }

    #[test]
    fn end_tag_w_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h></h a='b'>"));
        });
    }

    #[test]
    fn multiple_atts_no_space() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='b'c='d'>"));
        });
    }

    #[test]
    fn repeated_attr() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='b' a='d'>"));
        });
    }

    #[test]
    fn unfinished_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!--comment"));
        });
    }

    #[test]
    fn unfinished_comment_after_start_of_nested_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- <!--"));
        });
    }

    #[test]
    fn start_of_a_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-"));
        });
    }

    #[test]
    fn short_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-->"));
        });
    }

    #[test]
    fn short_comment_two() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!--->"));
        });
    }

    #[test]
    fn nested_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- <!--test-->"));
        });
    }

    #[test]
    fn nested_comment_with_extra_lt_() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<!-- <<!--test-->"));
        });
    }

    #[test]
    fn ampersandcomma__number_sign() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&#"));
        });
    }

    #[test]
    fn unfinished_numeric_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("&#x"));
        });
    }

    #[test]
    fn entity_without_trailing_semicolon__1_() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("I'm &notit"));
        });
    }

    #[test]
    fn entity_without_trailing_semicolon__2_() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("I'm &notin"));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<h a='&COPY'>"));
        });
    }

    #[test]
    fn open_angled_bracket_in_unquoted_attribute_value_state() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test("<a a=f<>"));
        });
    }
}