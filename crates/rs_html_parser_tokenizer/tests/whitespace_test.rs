#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use rs_html_parser_tokenizer::{Tokenizer, TokenizerOptions};
    use rs_html_parser_tokenizer_tokens::TokenizerToken;

    fn tokenize(data: &str) -> Vec<TokenizerToken> {
        let mut log: Vec<TokenizerToken> = Vec::new();

        let options = TokenizerOptions {
            xml_mode: Some(false),
            decode_entities: Some(true),
            ignore_whitespace_between_tags: Some(true),
        };

        let tokenizer = Tokenizer::new(data.as_bytes(), &options);

        for token in tokenizer {
            log.push(token);
        }

        log
    }

    #[test]
    fn whitespace_between_tags_removed() {
        assert_debug_snapshot!(tokenize(r#"
            <template>
                <h2>Flower</h2>
                <img src="img_white_flower.jpg" width="214" height="204">
            </template>"#)
        )
    }
}
