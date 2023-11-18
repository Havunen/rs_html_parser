
#[cfg(test)]
mod tests {
    use rs_html_parser_tokenizer::{TokenizerOptions, Tokenizer};
    use rs_html_parser_tokenizer_tokens::TokenizerToken;

    fn tokenize(data: &str) -> Vec<TokenizerToken> {
        let mut log: Vec<TokenizerToken> = Vec::new();

        let options = TokenizerOptions {
            xml_mode: Option::from(false),
            decode_entities: Option::from(true),
        };

        let tokenizer = Tokenizer::new(&data.as_bytes(), options);

        for token in tokenizer {
            log.push(token);
        }

        return log;
    }

    #[test]
    fn basic_element() {
        insta::assert_debug_snapshot!(tokenize("<a a=a&>foo"))
    }
}
