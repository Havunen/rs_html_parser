use criterion::{criterion_group, criterion_main, Criterion};
use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;
use std::fs;
use std::path::Path;

fn benchmark_wiki_page(c: &mut Criterion) {
    let file_path = Path::new(
        "/home/sampo/git/rs_html_parser/crates/rs_html_parser/benches/bench_data/wiki.html",
    );
    let contents = fs::read_to_string(file_path).expect("Should have wiki page test data");

    c.bench_function("benchmarks", |b| {
        b.iter(|| {
            let options = ParserOptions {
                xml_mode: false,
                tokenizer_options: TokenizerOptions {
                    xml_mode: None,
                    decode_entities: None,
                },
            };

            let tokenizer = Parser::new(&contents, &options);

            for _token in tokenizer {}
        });
    });
}

criterion_group!(benches, benchmark_wiki_page);
criterion_main!(benches);
