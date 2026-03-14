# Chapter 1

This is some great contetn.

```rust
 let x = 5;
 println!("x = {}", x);
```

```aquascope,interpreter
#fn main() {
 let mut s = String::from("yo ");`[]`
 s.push_str("world");`[]`
#}
```

And now, a _quiz_:

{{#quiz ../quizzes/rust-variables.toml}}

```aquascope,interpreter,stepper,shouldFail,horizontal
#fn main() {
 let mut v = vec![1, 2, 3];
 let n = &v[0];
 v.push(4);
 let a = *n;
#}
```
