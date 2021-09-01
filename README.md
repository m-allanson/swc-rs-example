# About

Experimenting with swc and rust. 

## Getting Started

- have Rust installed
- `cargo run` 

I think that's it, but I might have missed some steps.

## Run output

```sh
swc-rs on  main [?]
❯ cargo run
   Compiling swc-rs v0.1.0 (/Users/mike/d/temp/swc-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 9.47s
     Running `target/debug/swc-rs`
Hello, world!
[src/main.rs:166] "const thing = 3;\n/* now what*/" = "const thing = 3;\n/* now what*/"
[src/main.rs:48] &result = Module(
    Module {
        span: Span {
            lo: BytePos(
                0,
            ),
            hi: BytePos(
                16,
            ),
            ctxt: #0,
        },
        body: [],
        shebang: None,
    },
)
[src/main.rs:173] result = ""
```