import {writeFileSync} from 'node:fs';
const testFiles = ["test1.test", "test2.test", "test3.test", "test4.test"];

function sanitizeTestName(description, usedNames) {
    let testName = description
        .replaceAll(' ', '_')
        .replaceAll('<', '_lt_')
        .replaceAll('>', '_gt_')
        .replaceAll('w//', '_with_')
        .replaceAll('-', '_hyphen_')
        .replaceAll(',', '_comma_')
        .replaceAll('.', '_dot_')
        .replaceAll('[', '_opn_bracket_')
        .replaceAll(']', '_cls_bracket_')
        .replaceAll('+', '_plus_')
        .replaceAll('&', '_amp_')
        .replaceAll('?', '_qmark_')
        .replaceAll('@', '_at_mark_')
        .replaceAll('=', '_equals_')
        .replaceAll('"', '_dbl_quote_')
        .replaceAll('\'', '_apos1_')
        .replaceAll('%', '_percent_')
        .replaceAll('`', '_apos2_')
        .replaceAll('!', '_excl_mark_')
        .replace(/[<>\/\\(),.&!#_{}]+/g,'_')
        .replace(/_+$/,'')
        .replace(/^_+/, '')
        .toLowerCase();

    if (testName.match(/^\d/)) {
        testName = `num_${testName}`
    }

    if (usedNames.has(testName)) {
        let num = usedNames.get(testName);
        usedNames.set(testName, ++num);

        testName += ('_gen_' + num);
    } else {
        usedNames.set(testName, 0);
    }

    return testName;
}

function sanitizeTestInput(input) {
    return input
        .replaceAll('\r', '\r\n') // It would be nice to test only carriage returns but Rust does not allow it in the string
}

async function load() {
    for (const file of testFiles) {
        console.log("Generating " + file);

        const usedNames = new Map();
        const fileResponse = await fetch("https://raw.githubusercontent.com/html5lib/html5lib-tests/master/tokenizer/" + file);
        const fileData = await fileResponse.json();
        const tests = fileData.tests.filter(d => !d.initialStates);

        let rustTestFile =
`// AUTOGENERATED FILE
mod test_utils;

mod tests {
    use insta::{assert_debug_snapshot, with_settings};
    use crate::test_utils::*;

`;
        const testsNoErrors = tests.filter(d => !d.errors);
        const errorTests = tests.filter(d => d.errors);

        rustTestFile += `
// Spec valid tests
`

        for (const testData of testsNoErrors) {

            rustTestFile += `
    #[test]
    fn ${sanitizeTestName(testData.description, usedNames)}() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"${sanitizeTestInput(testData.input)}"####));
        });
    }
`
        }

        rustTestFile += `
// Spec error tests
`

        for (const testData of errorTests) {

            rustTestFile += `
    #[test]
    fn ${sanitizeTestName(testData.description, usedNames)}() {
        with_settings!({sort_maps =>true}, {
            assert_debug_snapshot!(parser_test(r####"${sanitizeTestInput(testData.input)}"####));
        });
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
}).catch((ex) => {
    console.log("Loading ERROR! " + ex)
})
