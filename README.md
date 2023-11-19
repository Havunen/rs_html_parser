
Very compatible HTML tokenizer and parser.

Differences to HTML specification.

- Errors are not thrown for invalid input
- Just like web browsers, this parser automatically handles missing closing tags 
- `<?` and `<!` always opens a new processing instruction
- Orphan end tags are ignored ( This might still change )


Notes:
- According to the spec attribute handling is case-insensitive
