#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use rs_html_parser_tokenizer::{Callbacks, Options, Tokenizer};

    fn tokenize(data: &str) {

        // let mut log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        let mut log: Vec<String> = Vec::new();

        let callbacks = Callbacks {
            onattribdata: &(|start, end_index| {
               log.push(format!("onattribdata-{start}-{end_index}"));
            }),
            onattribentity: &(|code_point |{
               log.push(format!("onattribentity-{code_point}"));
            }),
             onattribend: &(|quote, end_index|{
                 let q = quote as i32;

                log.push(format!("onattribend-{q}-{end_index}"));
             }),
             onattribname: &(|start: i32, end_index: i32|{
                log.push(format!("onattribname-{start}-{end_index}"));
             }),
             oncdata: &(|start: i32, end_index: i32, end_offset: i32|{
                log.push(format!("oncdata-{start}-{end_index}-{end_offset}"));
             }),
             onclosetag: &(|start: i32, end_index: i32|{
                log.push(format!("onclosetag-{start}-{end_index}"));
             }),
             oncomment: &(|start: i32, end_index: i32, end_offset: i32|{
                log.push(format!("oncomment-{start}-{end_index}-{end_offset}"));
             }),
             ondeclaration: &(|start: i32, end_index: i32|{
                log.push(format!("ondeclaration-{start}-{end_index}"));
             }),
             onend:&(||{
                log.push(format!("onend"));
             }),
             onopentagend:&(|end_index: i32|{
                log.push(format!("onopentagend-{end_index}"));
             }),
             onopentagname:&(|start: i32, end_index: i32|{
                log.push(format!("onopentagname-{start}-{end_index}"));
             }),
             onprocessinginstruction:&(|start: i32, end_index: i32|{
                log.push(format!("onprocessinginstruction-{start}-{end_index}"));
             }),
             onselfclosingtag:&(|end_index: i32|{
                log.push(format!("onselfclosingtag-{end_index}"));
             }),
             ontext:&(|start: i32, end_index: i32|{
                log.push(format!("ontext-{start}-{end_index}"));
             }),
             ontextentity:&(|codepoint: u8, end_index: i32|{
                log.push(format!("ontextentity-{end_index}"));
             }),
        };

        let options = Options {
            xml_mode: false,
            decode_entities: false
        };

        let tokenizer = Tokenizer::new(
            options,
            callbacks
        );
    }

    #[test]
    fn it_works() {

        // assert_eq!(result, 4);
    }
}
