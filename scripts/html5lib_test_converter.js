import {writeFileSync} from 'node:fs';
const testFiles = ["test1.test"];

function santizieTestName(description) {
    return description
        .replaceAll(' ', '_')
        .replaceAll('<', 'lt_')
        .replaceAll('>', 'gt_')
        .replaceAll('w//', 'with_')
        .replaceAll('/', '_')
        .replaceAll('-', '_')
        .replaceAll('(', '_')
        .replaceAll(')', '_')
        .replaceAll(',', 'comma_')
        .replaceAll('.', 'dot_')
        .replaceAll('&', 'amp_')
        .replaceAll('!', 'exclmark_')
        .toLowerCase();
}

async function load() {
    for (const file of testFiles) {
        console.log("Generating " + file);

        const fileResponse = await fetch("https://raw.githubusercontent.com/html5lib/html5lib-tests/master/tokenizer/" + file);
        const fileData = await fileResponse.json().filter(d => !d.initialStates);

        let rustTestFile =
`mod test_utils;

mod tests {
    use crate::test_utils::*;

`;
        const testsNoErrors = fileData.tests.filter(d => !d.errors);
        const errorTests = fileData.tests.filter(d => d.errors);

        rustTestFile += `
// Spec valid tests
`

        for (const testData of testsNoErrors) {

            rustTestFile += `
    #[test]
    fn ${santizieTestName(testData.description)}() {
        insta::assert_debug_snapshot!(parser_test("${testData.input}"));
    }

`
        }

        rustTestFile += `
// Spec error tests
`

        for (const testData of errorTests) {

            rustTestFile += `
    #[test]
    fn ${santizieTestName(testData.description)}() {
        insta::assert_debug_snapshot!(parser_test("${testData.input}"));
    }

`
        }

        rustTestFile += '}';

        writeFileSync('../crates/rs_html_parser/tests/html5lib_' + file.replace('.', '_') + '.rs', rustTestFile);

    }
}

console.log("Starting!")

load().then(() => {
    console.log("Loading finished!")
}).catch(() => {
    console.log("Loading ERROR!")
})
