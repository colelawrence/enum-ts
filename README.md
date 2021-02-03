# `enum-ts`

Place this definition somewhere in your project:

```typescript
/**
 * This special type can help generate pattern matchers for you!
 * Just use it as so:
 *
 *    type Result<Ok, Err> = Enum<{
 *      Ok: Ok,
 *      Err: Err,
 *    }>
 */
export type Enum<T extends { [Variant: string]: any }> = {
  [P in keyof T]: Record<P, T[P]>;
}[keyof T];
```

Now whenever you write a type with `Enum` definition at the root of a file (cannot be nested in another block), then
`enum-ts` will be able to generate exhaustive pattern matchers and constructor code for you!

## Install

This is currently just an executable you can install through compiling through cargo, or use by installing through npm.

```sh
cargo install enum-ts
# build from source hosted on crates.io


npm install --global enum-ts-bin
# install from npm registry with npm


yarn global add enum-ts-bin
# install from npm registry with yarn


enum-ts --version
# print version
```

Then, you can test it out with one of the examples in `tests/` or in this README:

```sh
cat ./tests/result.ts | enum-ts
```

Which will print the generated enum matcher helpers!

### CLI

```sh
enum-ts
# no arguments puts enum-ts into pipe-mode

cat my-file.ts | enum-ts
# prints what would be generated from this file (mostly for debugging purposes)

enum-ts --write .
# recursively walks down the directory looking for *.ts & *.tsx files
# to write updates to directly

enum-ts .
# "dry-run" will only print out what it would have rewritten the files to if given the `--write` flag.
```

## Examples

### Result

**Input**

```typescript //input(result)
type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;
```

<details>
  <summary><b>Generated</b></summary>

```typescript //generated(result)
export type Ok<O, E> = O;
export type Err<O, E> = E;
export function Ok<O, E>(contents: Ok<O, E>): { Ok: Ok<O, E> } {
  return { Ok: contents };
}
export function Err<O, E>(contents: Err<O, E>): { Err: Err<O, E> } {
  return { Err: contents };
}
export function isOk<O, E>(item: Result<O, E>): item is { Ok: Ok<O, E> } {
  return item != null && "Ok" in item;
}
export function isErr<O, E>(item: Result<O, E>): item is { Err: Err<O, E> } {
  return item != null && "Err" in item;
}
export namespace Result {
  const unexpected = "Unexpected Enum variant for Result<O, E>";
  export function apply<O, E, R>(fns: {
    Ok(content: Ok<O, E>): R;
    Err(content: Err<O, E>): R;
  }): (value: Result<O, E>) => R {
    return function matchResultApply(item) {
      return "Ok" in item
        ? fns.Ok(item.Ok)
        : "Err" in item
        ? fns.Err(item.Err)
        : (console.assert(false, unexpected, item) as never);
    };
  }
  export function match<O, E, R>(
    value: Result<O, E>,
    fns: {
      Ok(content: Ok<O, E>): R;
      Err(content: Err<O, E>): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
```

</details>

**Usage**

```typescript
const res = Ok<string, any>("okay value");
Result.match(res, {
  Ok(value) {
    // do something with value
  },
  Err(err) {
    // do something with err
  },
});
if (isOk(res)) {
  console.assert(res.Ok === "okay value");
}
```

### `BinaryTree<T>`

**Input**

```typescript //input(binary-tree)
export type BinaryTree<T> = Enum<{
  Leaf: T;
  Branch: {
    left: BinaryTree<T>;
    right: BinaryTree<T>;
  };
}>;
```

<details>
  <summary><b>Generated</b></summary>

```typescript //generated(binary-tree)
export type Leaf<T> = T;
export type Branch<T> = {
  left: BinaryTree<T>;
  right: BinaryTree<T>;
};
export function Leaf<T>(contents: Leaf<T>): { Leaf: Leaf<T> } {
  return { Leaf: contents };
}
export function Branch<T>(contents: Branch<T>): { Branch: Branch<T> } {
  return { Branch: contents };
}
export function isLeaf<T>(item: BinaryTree<T>): item is { Leaf: Leaf<T> } {
  return item != null && "Leaf" in item;
}
export function isBranch<T>(item: BinaryTree<T>): item is { Branch: Branch<T> } {
  return item != null && "Branch" in item;
}
export namespace BinaryTree {
  const unexpected = "Unexpected Enum variant for BinaryTree<T>";
  export function apply<T, R>(fns: {
    Leaf(content: Leaf<T>): R;
    Branch(content: Branch<T>): R;
  }): (value: BinaryTree<T>) => R {
    return function matchBinaryTreeApply(item) {
      return "Leaf" in item
        ? fns.Leaf(item.Leaf)
        : "Branch" in item
        ? fns.Branch(item.Branch)
        : (console.assert(false, unexpected, item) as never);
    };
  }
  export function match<T, R>(
    value: BinaryTree<T>,
    fns: {
      Leaf(content: Leaf<T>): R;
      Branch(content: Branch<T>): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
```

</details>

##### Development

`cargo install --path ./enum-ts`: Install locally after cloning repo

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
