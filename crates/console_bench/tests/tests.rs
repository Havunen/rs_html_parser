#[cfg(test)]
mod tests {
    use console_bench::runner::read_all_test_file_data;
    use rs_html_parser::{Parser, ParserOptions};
    use rs_html_parser_tokenizer::TokenizerOptions;
    use rs_html_parser_tokens::Token;

    fn parser<'a>(data: &'a str, options: &'a ParserOptions) -> Vec<Token> {
        let mut log: Vec<Token> = Vec::new();

        let tokenizer = Parser::new(data, options);

        for token in tokenizer {
            log.push(token);
        }

        log
    }

    #[test]
    fn test_all_html_files() {
        let test_data = read_all_test_file_data("./../../test_data/");

        let options = ParserOptions {
            xml_mode: false,
            tokenizer_options: TokenizerOptions {
                xml_mode: None,
                decode_entities: None,
                ignore_whitespace_between_tags: Some(true),
            },
        };

        for test_data in &test_data {
            let result = parser(test_data, &options);

            insta::assert_debug_snapshot!(result);
        }
    }
}
