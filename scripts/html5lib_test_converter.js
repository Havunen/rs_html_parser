import {writeFileSync} from 'node:fs';
const testFiles = ["test1.test"];

async function load() {
    for (const file of testFiles) {
        const fileResponse = await fetch("https://raw.githubusercontent.com/html5lib/html5lib-tests/master/tokenizer/" + file);
        const fileData = await fileResponse.json();
        console.log(fileData);


        writeFileSync('../crates/rs_html_parser/tests/html5lib_' + file, );
    }
}

console.log("Starting!")

load().then(() => {
    console.log("Loading finished!")
}).catch(() => {
    console.log("Loading ERROR!")
})
