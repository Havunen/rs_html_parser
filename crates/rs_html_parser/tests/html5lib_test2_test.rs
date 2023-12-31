// AUTOGENERATED FILE
mod test_utils;

mod tests {
    use insta::{assert_debug_snapshot, with_settings};
    use crate::test_utils::*;


// Spec valid tests

    #[test]
    fn doctype_with_publicid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML Transitional 4.01//EN">"####));
        });
    }

    #[test]
    fn doctype_with_systemid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html SYSTEM "-//W3C//DTD HTML Transitional 4.01//EN">"####));
        });
    }

    #[test]
    fn doctype_with_single_hyphen_quoted_systemid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html SYSTEM '-//W3C//DTD HTML Transitional 4.01//EN'>"####));
        });
    }

    #[test]
    fn doctype_with_publicid_and_systemid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML Transitional 4.01//EN" "-//W3C//DTD HTML Transitional 4.01//EN">"####));
        });
    }

    #[test]
    fn hexadecimal_entity_with_mixed_uppercase_and_lowercase() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&#xaBcD;"####));
        });
    }

    #[test]
    fn entity_without_a_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&;"####));
        });
    }

    #[test]
    fn unescaped_ampersand_in_attribute_value() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h a='&'>"####));
        });
    }

    #[test]
    fn starttag_containing_lt() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<a<b>"####));
        });
    }

    #[test]
    fn non_hyphen_void_element_containing_trailing() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h/>"####));
        });
    }

    #[test]
    fn void_element_with_permitted_slash() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<br/>"####));
        });
    }

    #[test]
    fn void_element_with_permitted_slash_with_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<br foo='bar'/>"####));
        });
    }

    #[test]
    fn double_hyphen_quoted_attribute_value() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h a="b">"####));
        });
    }

    #[test]
    fn entity_plus_newline() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"
x
&gt;
"####));
        });
    }

    #[test]
    fn start_tag_with_no_attributes_but_space_before_the_greater_hyphen_than_sign() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h >"####));
        });
    }

    #[test]
    fn empty_attribute_followed_by_uppercase_attribute() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h a B=''>"####));
        });
    }

// Spec error tests

    #[test]
    fn doctype_without_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE>"####));
        });
    }

    #[test]
    fn doctype_without_space_before_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPEhtml>"####));
        });
    }

    #[test]
    fn incorrect_doctype_without_a_space_before_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPEfoo>"####));
        });
    }

    #[test]
    fn doctype_with_eof_after_public() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC"####));
        });
    }

    #[test]
    fn doctype_with_eof_after_public_apos1() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC '"####));
        });
    }

    #[test]
    fn doctype_with_eof_after_public_apos1_x() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC 'x"####));
        });
    }

    #[test]
    fn doctype_with_gt_in_double_hyphen_quoted_publicid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC ">x"####));
        });
    }

    #[test]
    fn doctype_with_gt_in_single_hyphen_quoted_publicid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC '>x"####));
        });
    }

    #[test]
    fn doctype_with_gt_in_double_hyphen_quoted_systemid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC "foo" ">x"####));
        });
    }

    #[test]
    fn doctype_with_gt_in_single_hyphen_quoted_systemid() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html PUBLIC 'foo' '>x"####));
        });
    }

    #[test]
    fn incomplete_doctype() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!DOCTYPE html "####));
        });
    }

    #[test]
    fn numeric_entity_representing_the_nul_character() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&#0000;"####));
        });
    }

    #[test]
    fn hexadecimal_entity_representing_the_nul_character() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&#x0000;"####));
        });
    }

    #[test]
    fn numeric_entity_representing_a_codepoint_after_1114111_u_plus_10ffff() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&#2225222;"####));
        });
    }

    #[test]
    fn hexadecimal_entity_representing_a_codepoint_after_1114111_u_plus_10ffff() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&#x1010FFFF;"####));
        });
    }

    #[test]
    fn hexadecimal_entity_pair_representing_a_surrogate_pair() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"&#xD869;&#xDED6;"####));
        });
    }

    #[test]
    fn starttag_containing() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h/a='b'>"####));
        });
    }

    #[test]
    fn unescaped_lt() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"</"####));
        });
    }

    #[test]
    fn illegal_end_tag_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"</1>"####));
        });
    }

    #[test]
    fn simili_processing_instruction() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<?namespace>"####));
        });
    }

    #[test]
    fn a_bogus_comment_stops_at_gt_comma_even_if_preceded_by_two_dashes() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<?foo-->"####));
        });
    }

    #[test]
    fn unescaped_lt_gen_1() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"foo < bar"####));
        });
    }

    #[test]
    fn null_byte_replacement() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####" "####));
        });
    }

    #[test]
    fn comment_with_dash() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<!---x"####));
        });
    }

    #[test]
    fn double_hyphen_quote_after_attribute_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h a ">"####));
        });
    }

    #[test]
    fn single_hyphen_quote_after_attribute_name() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"<h a '>"####));
        });
    }

    #[test]
    fn empty_end_tag_with_following_characters() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"a</>bc"####));
        });
    }

    #[test]
    fn empty_end_tag_with_following_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"a</><b>c"####));
        });
    }

    #[test]
    fn empty_end_tag_with_following_comment() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"a</><!--b-->c"####));
        });
    }

    #[test]
    fn empty_end_tag_with_following_end_tag() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"a</></b>c"####));
        });
    }
}