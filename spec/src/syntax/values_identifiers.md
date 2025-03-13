# Values and Identifiers

> **<sup>Syntax:</sup>**\
> _Value_:\
> &emsp;`[0-9]+`\
> &emsp;| `[0-9]+` `"."` `[0-9]*`\
> &emsp;| ( `"True"` | `"False"` )\
> &emsp;| _Ident_\
> &emsp;| _[Function](./function_decl.md)_

Values can either be integers, floats, ~~strings,~~ or booleans (NOTE: Those aren't implemented yet!)

> **<sup>Syntax:</sup>**\
> _Ident_:\
> &emsp;`( [a-zA-Z] | "_" )` | `( [a-zA-Z0-9] | "_" )*`

Identifiers are Rust-like and are used to identify values, functions (which are values!), modules, etc. Anything with a
name needs an identifier.
