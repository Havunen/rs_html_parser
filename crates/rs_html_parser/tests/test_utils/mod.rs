use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;
use rs_html_parser_tokens::Token;

static OPTIONS: ParserOptions = ParserOptions {
    xml_mode: false,
    tokenizer_options: TokenizerOptions {
        xml_mode: None,
        decode_entities: None,
        ignore_whitespace_between_tags: Some(true),
    },
};

pub fn parser_test(data: &str) -> Vec<Token> {
    let mut log: Vec<Token> = Vec::new();

    let tokenizer = Parser::new(data, &OPTIONS);

    for token in tokenizer {
        log.push(token);
    }

    log
}
