```rust
pub fn Parser(sess: @mut ParseSess,
              +cfg: ast::crate_cfg,
              +rdr: @reader)
           -> Parser {
    let tok0 = copy rdr.next_token();
    let interner = rdr.interner();

    Parser {
        reader: rdr,
        interner: interner,
        sess: sess,
        cfg: cfg,
        token: @mut copy tok0.tok,
        span: @mut copy tok0.sp,
        last_span: @mut copy tok0.sp,
        buffer: @mut ([copy tok0, .. 4]),
        buffer_start: @mut 0,
        buffer_end: @mut 0,
        tokens_consumed: @mut 0,
        restriction: @mut UNRESTRICTED,
        quote_depth: @mut 0,
        keywords: token::keyword_table(),
        strict_keywords: token::strict_keyword_table(),
        reserved_keywords: token::reserved_keyword_table(),
        obsolete_set: @mut LinearSet::new(),
        mod_path_stack: @mut ~[],
    }
}
```
