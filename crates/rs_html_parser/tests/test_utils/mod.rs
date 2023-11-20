use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;
use rs_html_parser_tokens::Token;

pub fn parser_test(data: &str) -> Vec<Token> {
    let mut log: Vec<Token> = Vec::new();

    let options = ParserOptions {
        xml_mode: false,
        tokenizer_options: TokenizerOptions {
            xml_mode: None,
            decode_entities: None,
        },
    };

    let tokenizer = Parser::new(&data, &options);

    for token in tokenizer {
        log.push(token);
    }

    return log;
}
