#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use rs_html_parser_tokenizer::{Tokenizer, TokenizerOptions};
    use rs_html_parser_tokenizer_tokens::TokenizerToken;

    fn tokenize(data: &str) -> Vec<TokenizerToken> {
        let mut log: Vec<TokenizerToken> = Vec::new();

        let options = TokenizerOptions {
            xml_mode: Option::from(false),
            decode_entities: Option::from(true),
            ignore_whitespace_between_tags: Some(true)
        };

        let tokenizer = Tokenizer::new(data.as_bytes(), &options);

        for token in tokenizer {
            log.push(token);
        }

        log
    }

    #[test]
    fn textarea_only_text() {
        assert_debug_snapshot!(tokenize("<textarea>asd</textarea>"))
    }

    #[test]
    fn textarea_tags_should_be_text() {
        assert_debug_snapshot!(tokenize("<textarea><div>asd</div><p>1</p></textarea>"))
    }

    #[test]
    fn ensure_textarea_does_not_invalidate_template() {
        assert_debug_snapshot!(tokenize(r#"<template><h2>Flower</h2><img src="img_white_flower.jpg" width="214" height="204"></template>"#))
    }
}
