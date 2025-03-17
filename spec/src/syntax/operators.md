The defined operators in the Oxyl programming language are:

> **<sup>Syntax:</sup>**\
> _Operator_:\
> &emsp; `-`\
> &emsp; `+`\
> &emsp; `/`\
> &emsp; `*`\
> &emsp; `|>`\
> &emsp; `.`\
> &emsp; `::`

The operators should be defined and implemented as follows:

- `+`, `-`, `*` and `/`: These correspond to their respective integer/numerical mathematical operations. Similar
  operations on other (mathematical) objects may use the respective operator.
- `|>`: This is the function application/"pipeline" operator. Anything that takes in a single input and produces an
  output of some sort can use this operator. Note that it is generally desired to be used in accordance to the
  definition of it's functional usage, A.K.A. `a |> (a -> b)`.
- `.`: This corresponds to a field access of an object. This is automatically implemented for structures. It may be used
  on tuples, list-like objects or other things with individual access of fields.
- `::`: This corresponds to a namespace access of a module/thing/whatever. This is automatically implemented for modules.
  It may be used wherever there is a namespace being accessed, i.e. methods on types, "hidden"/"child" types, etc.

These operators were listed from least to most priority. Additionally, every operator except `.` and `::` are
left-associative. `.` and `::` are right-associative.
