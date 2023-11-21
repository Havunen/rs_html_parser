use rs_html_parser::{Parser, ParserOptions};
use rs_html_parser_tokenizer::TokenizerOptions;
use std::path::PathBuf;
use std::{env, fs, io};

fn get_files_in_folder(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}

fn main() {
    println!("RS Console bench!");
    println!("Current dir = {:?}", env::current_dir());

    let mut test_files: Vec<String> = Vec::new();
    match get_files_in_folder("./crates/console_bench/test_data/") {
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

                test_files.push(file.to_str().unwrap().to_owned())
            }
        }
        Err(e) => println!("Error: {}", e),
    };

    let mut test_data: Vec<String> = Vec::new();

    for entry in test_files {
        test_data.push(fs::read_to_string(entry).unwrap())
    }

    println!("Files loaded");

    let options = ParserOptions {
        xml_mode: false,
        tokenizer_options: TokenizerOptions {
            xml_mode: None,
            decode_entities: None,
        },
    };

    println!("Running");

    for i in 0..100 {
        println!("{}", i);

        for test_data in &test_data {
            let tokenizer = Parser::new(test_data, &options);

            for _token in tokenizer {}
        }
    }

    println!("The end");
}
