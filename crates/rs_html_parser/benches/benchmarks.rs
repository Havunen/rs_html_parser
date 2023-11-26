use criterion::{criterion_group, criterion_main, Criterion};
use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io};

fn get_files_in_folder(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}

pub fn read_all_test_file_data(path: &str) -> Vec<String> {
    let mut test_files: Vec<String> = Vec::new();
    match get_files_in_folder(path) {
        Ok(files) => {
            for file in files {
                if file.is_dir() {
                    println!("{} is a directory", file.display());
                    continue;
                }
                if file.is_symlink() {
                    println!("{} is a symlink", file.display());
                    continue;
                }

                let Ok(m) = file.metadata() else {
                    println!("Could not get metadata for {}", file.display());
                    continue;
                };

                if m.len() == 0 {
                    println!("{} is an empty file", file.display());
                    continue;
                }

                let file_name = file.to_str().unwrap().to_owned();

                if file_name.ends_with(".html") {
                    test_files.push(file_name)
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    };

    let mut test_data: Vec<String> = Vec::new();

    for entry in test_files {
        test_data.push(fs::read_to_string(entry).unwrap())
    }
    test_data
}

fn benchmark_test_data(c: &mut Criterion) {
    let test_data = read_all_test_file_data("./../../test_data/");

    c.bench_function("benchmarks", |b| {
        b.iter(|| {
            let options = ParserOptions {
                xml_mode: false,
                tokenizer_options: TokenizerOptions {
                    xml_mode: None,
                    decode_entities: None,
                },
            };

            for test_data in &test_data {
                let tokenizer = Parser::new(test_data, &options);

                for _token in tokenizer {}
            }
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = benchmark_test_data
);
criterion_main!(benches);
