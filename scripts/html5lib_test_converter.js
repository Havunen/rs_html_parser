import {writeFileSync} from 'node:fs';
const testFiles = ["test1.test", "test2.test"];

function sanitizeTestName(description, usedNames) {
    let testName = description
        .replaceAll(' ', '_')
        .replaceAll('<', 'lt_')
        .replaceAll('>', 'gt_')
        .replaceAll('w//', 'with_')
        .replaceAll('-', 'hyphen_')
        .replaceAll(',', 'comma_')
        .replaceAll('.', 'dot_')
        .replaceAll('+', 'plus_')
        .replaceAll('&', 'amp_')
        .replaceAll('\'', 'apos1_')
        .replaceAll('`', 'apos2_')
        .replaceAll('!', 'excl_mark_')
        .replace(/[<>\/(),.&!#_]+/g,'_')
        .replace(/_+$/,'')
        .toLowerCase();

    if (usedNames.has(testName)) {
        let num = usedNames.get(testName);

        testName += num;

        usedNames.set(testName, ++num);
    } else {
        usedNames.set(testName, 1);
    }

    return testName;
}

async function load() {
    for (const file of testFiles) {
        console.log("Generating " + file);

        const usedNames = new Map();
        const fileResponse = await fetch("https://raw.githubusercontent.com/html5lib/html5lib-tests/master/tokenizer/" + file);
        const fileData = await fileResponse.json();
        const tests = fileData.tests.filter(d => !d.initialStates);

        let rustTestFile =
`mod test_utils;

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
            assert_debug_snapshot!(parser_test(r#"${testData.input}"#));
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
            assert_debug_snapshot!(parser_test(r#"${testData.input}"#));
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
