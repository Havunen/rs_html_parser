use rs_html_parser_tokens::Token;
use rs_html_parser_tokenizer::{Tokenizer, TokenizerOptions};

pub struct ParserOptions {
    /**
     * Indicates whether special tags (`<script>`, `<style>`, and `<title>`) should get special treatment
     * and if "empty" tags (eg. `<br>`) can have children.  If `false`, the content of special tags
     * will be text only. For feeds and other XML content (documents that don't consist of HTML),
     * set this to `true`.
     *
     * @default false
     */
    xmlMode: bool,

    /**
     * Decode entities within the document.
     *
     * @default true
     */
    decodeEntities: bool,

    /**
     * If set to true, all tags will be lowercased.
     *
     * @default !xmlMode
     */
    lowerCaseTags: bool,

    /**
     * If set to `true`, all attribute names will be lowercased. This has noticeable impact on speed.
     *
     * @default !xmlMode
     */
    lowerCaseAttributeNames: bool,

    /**
     * If set to true, CDATA sections will be recognized as text even if the xmlMode option is not enabled.
     * NOTE: If xmlMode is set to `true` then CDATA sections will always be recognized as text.
     *
     * @default xmlMode
     */
    recognizeCDATA: bool,

    /**
     * If set to `true`, self-closing tags will trigger the onclosetag event even if xmlMode is not set to `true`.
     * NOTE: If xmlMode is set to `true` then self-closing tags will always be recognized.
     *
     * @default xmlMode
     */
    recognizeSelfClosing: bool,

    tokenizer_options: TokenizerOptions,
}


pub struct Parser<'a> {
    /** The start index of the last event. */
    startIndex: u32,
    /** The end index of the last event. */
    endIndex: u32,
    /**
     * Store the start index of the current open tag,
     * so we can update the start index for attributes.
     */
    openTagStar: u32,

    // tagname = "";
    // attribname = "";
    // attribvalue = "";
    // attribs: null | { [key: string]: string } = null;
    // stack: string[] = [];
    // /** Determines whether self-closing tags are recognized. */
    // foreignContext: boolean[];
    // cbs: Partial<Handler>;
    // lowerCaseTagNames: boolean;
    // lowerCaseAttributeNames: boolean;
    // recognizeSelfClosing: boolean;
    // /** We are parsing HTML. Inverse of the `xmlMode` option. */
    htmlMode: bool,
    // tokenizer: Tokenizer;
    //
    // buffers: string[] = [];
    // bufferOffset = 0;
    // /** The index of the last written buffer. Used when resuming after a `pause()`. */
    // writeIndex = 0;
    // /** Indicates whether the parser has finished running / `.end` has been called. */
    // ended = false;

    tokenizer: Tokenizer<'a>
}

impl Parser<'static> {
    pub fn new(html: &str, options: ParserOptions) -> Parser  {
        Parser {
            startIndex: 0,
            endIndex: 0,
            openTagStar: 0,
            htmlMode: !options.xmlMode,
            tokenizer: Tokenizer::new(html, options.tokenizer_options)
        }
    }

    fn parse_next(&mut self) -> Option<Token> {
        return None;
    }
}


impl Iterator for Parser<'static> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.parse_next()
    }
}
