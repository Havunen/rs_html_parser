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
}
