use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;
use rs_html_parser_tokens::Token;

static OPTIONS: ParserOptions = ParserOptions {
    xml_mode: false,
    tokenizer_options: TokenizerOptions {
        xml_mode: None,
        decode_entities: None,
    },
};

pub fn parser_test<'a>(data: &'a str) -> Vec<Token<'a>> {
    let mut log: Vec<Token<'a>> = Vec::new();

    let tokenizer = Parser::new(data, &OPTIONS);

    for token in tokenizer {
        log.push(token);
    }

    log
}
