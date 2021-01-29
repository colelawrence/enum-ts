# `enum-ts`

Place this definition somewhere in your project:

```typescript
/**
 * This special type can help generate pattern matchers for you!
 * Just use it as so:
 *    // enum-ts
 *    type Result<Ok, Err> = Enum<{
 *      Ok: Ok,
 *      Err: Err,
 *    }>
 */
export type Enum<T extends { [Variant: string]: any }> = {
  [P in keyof T]: { t: P; c: T[P] };
}[keyof T];
```

Now whenever you write a type with `Enum` definition at the root of a file (cannot be nested in another block), then
`enum-ts` will be able to generate exhaustive pattern matchers and constructor code for you!

## Install

This is currently just an executable you can install with
```sh
cargo install enum-ts
```

Then, you can test it out with one of the examples in `tests/` or in this README:

```sh
cat ./tests/result.ts | enum-ts
```

Which will print the generated enum matcher helpers!

## Examples

### Result

**Input**

```typescript
type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;
```

<details>
  <summary><b>Generated</b></summary>

```typescript
namespace Result {
  export function Ok<O, E>(contents: O): Result<O, E> {
    return { t: "Ok", c: contents };
  }
  export function Err<O, E>(contents: E): Result<O, E> {
    return { t: "Err", c: contents };
  }
  export function apply<O, E, R>(fns: {
    Ok(content: O): R;
    Err(content: E): R;
  }): (value: Result<O, E>) => R {
    return function matchResultApply(value) {
      // @ts-ignore
      return fns[value.t](value.c);
    };
  }
  export function match<O, E, R>(
    value: Result<O, E>,
    fns: {
      Ok(content: O): R;
      Err(content: E): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
```

</details>

**Usage**

```typescript
const res = Result.Ok<string, any>("okay value")
Result.match(res, {
  Ok(value) {
    // do something with value
  },
  Err(err) {
    // do something with err
  },
})
```


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
