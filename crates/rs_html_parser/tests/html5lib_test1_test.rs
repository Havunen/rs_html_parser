mod test_utils;

mod tests {
    use insta::{assert_debug_snapshot, with_settings};
    use crate::test_utils::*;


// Spec valid tests

    #[test]
    fn correct_doctype_lowercase() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!DOCTYPE html>"#));
        });
    }

    #[test]
    fn correct_doctype_uppercase() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!DOCTYPE HTML>"#));
        });
    }

    #[test]
    fn correct_doctype_mixed_case() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!DOCTYPE HtMl>"#));
        });
    }

    #[test]
    fn doctype_in_error() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!DOCTYPE foo>"#));
        });
    }

    #[test]
    fn single_start_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h>"#));
        });
    }

    #[test]
    fn start_tag_w_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='b'>"#));
        });
    }

    #[test]
    fn start_tag_w_attribute_no_quotes() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a=b>"#));
        });
    }

    #[test]
    fn start_end_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h></h>"#));
        });
    }

    #[test]
    fn two_unclosed_start_tags() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<p>One<p>Two"#));
        });
    }

    #[test]
    fn multiple_atts() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='b' c='d'>"#));
        });
    }

    #[test]
    fn simple_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!--comment-->"#));
        });
    }

    #[test]
    fn commentcomma_central_dash_no_space() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!----->"#));
        });
    }

    #[test]
    fn commentcomma_two_central_dashes() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- --comment -->"#));
        });
    }

    #[test]
    fn commentcomma_central_lesshyphen_than_bang() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!--<!-->"#));
        });
    }

    #[test]
    fn short_comment_three() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!---->"#));
        });
    }

    #[test]
    fn lt_in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- <test-->"#));
        });
    }

    #[test]
    fn lt_lt_in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!--<<-->"#));
        });
    }

    #[test]
    fn lt_excl_mark_in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- <!test-->"#));
        });
    }

    #[test]
    fn lt_excl_mark_hyphen_in_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- <!-test-->"#));
        });
    }

    #[test]
    fn ampersand_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&"#));
        });
    }

    #[test]
    fn ampersand_ampersand_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&&"#));
        });
    }

    #[test]
    fn ampersand_space_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"& "#));
        });
    }

    #[test]
    fn unfinished_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&f"#));
        });
    }

    #[test]
    fn entity_with_trailing_semicolon_1() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"I'm &not;it"#));
        });
    }

    #[test]
    fn entity_with_trailing_semicolon_2() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"I'm &notin;"#));
        });
    }

    #[test]
    fn partial_entity_match_at_end_of_file() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"I'm &no"#));
        });
    }

    #[test]
    fn nonhyphen_ascii_character_reference_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&¬;"#));
        });
    }

    #[test]
    fn ascii_decimal_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&#0036;"#));
        });
    }

    #[test]
    fn ascii_hexadecimal_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&#x3f;"#));
        });
    }

    #[test]
    fn hexadecimal_entity_in_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='&#x3f;'></h>"#));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon_ending_in_x() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='&notx'>"#));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon_ending_in_1() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='&not1'>"#));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon_ending_in_i() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='&noti'>"#));
        });
    }

    #[test]
    fn unquoted_attribute_ending_in_ampersand() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<s o=& t>"#));
        });
    }

    #[test]
    fn unquoted_attribute_at_end_of_tag_with_final_character_of_amp_comma_with_tag_followed_by_characters() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<a a=a&>foo"#));
        });
    }

    #[test]
    fn plaintext_element() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<plaintext>foobar"#));
        });
    }

// Spec error tests

    #[test]
    fn correct_doctype_case_with_eof() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!DOCTYPE HtMl"#));
        });
    }

    #[test]
    fn truncated_doctype_start() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!DOC>"#));
        });
    }

    #[test]
    fn empty_end_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"</>"#));
        });
    }

    #[test]
    fn empty_start_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<>"#));
        });
    }

    #[test]
    fn end_tag_w_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h></h a='b'>"#));
        });
    }

    #[test]
    fn multiple_atts_no_space() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='b'c='d'>"#));
        });
    }

    #[test]
    fn repeated_attr() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='b' a='d'>"#));
        });
    }

    #[test]
    fn unfinished_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!--comment"#));
        });
    }

    #[test]
    fn unfinished_comment_after_start_of_nested_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- <!--"#));
        });
    }

    #[test]
    fn start_of_a_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-"#));
        });
    }

    #[test]
    fn short_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-->"#));
        });
    }

    #[test]
    fn short_comment_two() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!--->"#));
        });
    }

    #[test]
    fn nested_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- <!--test-->"#));
        });
    }

    #[test]
    fn nested_comment_with_extra_lt() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<!-- <<!--test-->"#));
        });
    }

    #[test]
    fn ampersandcomma_number_sign() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&#"#));
        });
    }

    #[test]
    fn unfinished_numeric_entity() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"&#x"#));
        });
    }

    #[test]
    fn entity_without_trailing_semicolon_1() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"I'm &notit"#));
        });
    }

    #[test]
    fn entity_without_trailing_semicolon_2() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"I'm &notin"#));
        });
    }

    #[test]
    fn entity_in_attribute_without_semicolon() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<h a='&COPY'>"#));
        });
    }

    #[test]
    fn open_angled_bracket_in_unquoted_attribute_value_state() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r#"<a a=f<>"#));
        });
    }
}