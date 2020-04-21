# `::mini_paste`

Like [`::paste`] (MIT / Apache licensed), but without any dependency on [`::syn`]
nor [`::quote`], for (significantly) fast(er) compile-from-scratch times.

  - It does not, however, currently offer the fancier features of case
    conversion that [`::paste`] does:

    > When in doubt, do use [`::paste`] instead.

      - Only use `::mini_paste` when the compile-from-scratch time matters to you.


### Seamlessly replacing `::paste` with `::mini_paste`

You can achieve that with the following line in your `Cargo.toml`:

```toml
[dependencies]
paste = { version = "0.1.0", package = "mini_paste" }
```

This will mock / shadow [`::paste`] so that all the `::paste::item!` and
`::paste::expr!` macro calls _Just Work_.

[`::paste`]: https://docs.rs/paste
[`::quote`]: https://docs.rs/quote
[`::syn`]: https://docs.rs/syn
