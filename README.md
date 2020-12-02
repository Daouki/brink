# The Brink Programming Language

Brink is a general-purpose, impurely functional programming language that focuses
on performance, elegance and ease of use.

## Code examples

### Fibonacci

```
let rec fibonacci n =
  match n
  | 0 | 1 -> 1
  | n -> fibonacci (n - 1) + fibonacci (n - 2)
```

## License

Brink is distributed under the terms of the MIT license. See [LICENSE](LICENSE)
for details.
