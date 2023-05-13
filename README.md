Lindskou’s Rust Journey
================

- <a href="#primer" id="toc-primer">Primer</a>
- <a href="#concepts-and-reserved-language-keywords"
  id="toc-concepts-and-reserved-language-keywords">Concepts and Reserved
  Language Keywords</a>
  - <a href="#traits" id="toc-traits">Traits</a>
  - <a href="#impl" id="toc-impl"><code>impl</code></a>

Let’s see how long the journey will be; depends on the kids bedtime and
the relevance for Rust at my work.

# Primer

A lot of the material is from the
[documentaion](https://doc.rust-lang.org/rust-by-example/) which I have
either just run as is or changed to understand.

# Concepts and Reserved Language Keywords

This section serves as my own explanation/interpretation of concepts or
just some explanation that I found on the web which I liked. Concepts
are in bold and reserved language keywords are in `code`.

### Traits

https://doc.rust-lang.org/rust-by-example/trait.html?highlight=traits#traits

### `impl`

https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

``` rust
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
```