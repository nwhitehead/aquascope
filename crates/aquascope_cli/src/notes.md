Notes on JSON response from aquascope.

```
X = private_us_as_methods_instead
```

Wow there are a lot of values, and they are weird. Pointers, numbers, variants.

```ts
type filepos = { column: number, line: number }
type private = { private_use_as_methods_instead: 0 }
type span = { start: filepos, end: filepos, filename: private }
type Type = "Int"
type Value = number

{
    code: string,
    result: {
        type: "Success",
    },
    steps: [
        heap: {
            locations: [

            ],
        }
        stack: {
            frames: [
                name: string,
                body_span: span,
                location: span,
                locals: [
                    name: string,
                    moved_paths: [],
                    value: [
                        type: Type,
                        value: Value,
                    ],
                ],
            ],
        },
    ],
}
```