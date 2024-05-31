// re2rust $INPUT -o $OUTPUT

pub fn lex(s: &[u8]) -> bool {
  /*!re2c
    re2c:define:YYCTYPE = char;
    re2c:yyfill:enable = 0;
    //..Other rules...

    keyword = "let" | "meta";
    // "as" | "async" | "await" | "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger" | "default" | "delete" | "do" | "else" | "enum" | "export" | "extends" | "false" | "finally" | "for" | "from" | "function" | "get" | "if" | "import" | "implements" | "in" | "instanceof" | "interface" | "let" | "meta" | "new" | "null" | "of" | "package" | "private" | "protected" | "public" | "rel" | "return" | "set" | "static" | "super" | "switch" | "target" | "this" | "throw" | "true" | "try" | "typeof" | "void" | "while" | "with" | "yield";
    * { /* Ignore other characters */ }
    keyword { println!("Found keyword: {}",yytext()); }
  */
}

pub fn main() {
  assert!(lex(b"let Test String here!"))
}