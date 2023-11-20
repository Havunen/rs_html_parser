mod test_utils;

mod tests {
    use crate::test_utils::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn wiki_page() {
        let file_path = Path::new("./benches/bench_data/wiki.html");
        let contents = fs::read_to_string(file_path).expect("Should have wiki page test data");

        insta::assert_debug_snapshot!(parser_test(&contents))
    }
}
