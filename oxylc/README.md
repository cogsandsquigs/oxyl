# oxylc

The compiler (library!) for Oxyl, my silly little programming language.

## What's inside?

This compiler exposes a public API with access to:

- The parser
- FST (full syntax tree) produced by the parser
- The type solver on the FST
- AST produced by the type solver
- The lower-er into LLIR
- And more!

## TODO

- [ ] Baby's first llir/asm output
- [ ] "main" function
- [ ] Comment support
  - [ ] Parse comments out of main AST
  - [ ] Keep comments inside FST (for use in LSP/formatter/etc.)

## Resources

- [Subtype Inference by Example Part 1: Introducing CubiML](https://blog.polybdenum.com/2020/07/04/subtype-inference-by-example-part-1-introducing-cubiml.html)
- [A thoughtful introduction to the pest parser](https://pest.rs/book)
