pub mod runner;

use crate::runner::read_all_test_file_data;
use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;

fn main() {
    println!("RS Console bench!");

    let test_data = read_all_test_file_data("./test_data/");

    println!("Files loaded");

    let options = ParserOptions {
        xml_mode: false,
        tokenizer_options: TokenizerOptions {
            xml_mode: None,
            decode_entities: None,
            ignore_whitespace_between_tags: Some(true),
        },
    };

    println!("Running");

    for i in 0..10 {
        println!("{}", i);

        for test_data in &test_data {
            let tokenizer = Parser::new(test_data, &options);

            for _token in tokenizer {}
        }
    }

    println!("The end");
}
