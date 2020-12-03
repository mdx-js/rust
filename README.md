# MDX

Rust implementation of MDX

> I found this repo, should I use this?

no.

> I'm going to anyway

ok, we look forward to working with you.

## Developing

You must have Rust installed. You probably want [rustup](https://rustup.rs/).

### Command Cheatsheet:

```shell
cargo build
cargo build --release
cargo test
cargo bench
```

### repo organization

what files matter where

#### integrations tests

tests against the public package API, and the public API only.

- in `tests/`

#### unit tests

testing internals and such in a small way.

- in `src/*.rs`

#### benchmark tests

Can we be speedy? how speedy? Did a recent change cause a regression?

- in `benches/mdx_benchmark.rs`

### Troubleshooting

> I have a println/dgb/etc in my test and I'm not seeing the output

Use `cargo test -- --nocapture` to show output in tests. cargo swallows the output by default.