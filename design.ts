import { Enum } from "./enum";

// enum: factory, match
type Result<Ok, Err> = Enum<{
  Ok: Ok;
  Err: Err;
}>;

// ---- enum-ts generated ----

// name = "Result"
// generics = "Ok, Err"
// variants = { t: "Ok", c: "Ok" }, { t: "Err", c: "Err" }
namespace Result {
  export function Ok<Ok, Err>(contents: Ok): Result<Ok, Err> {
    return { t: "Ok", c: contents };
  }
  export function Err<Ok, Err>(contents: Err): Result<Ok, Err> {
    return { t: "Err", c: contents };
  }
  export function apply<Ok, Err, R>(fns: {
    Ok(content: Ok): R;
    Err(content: Err): R;
  }): (value: Result<Ok, Err>) => R {
    return function matchResultApply(value) {
      // @ts-ignore
      return fns[value.t](value.c);
    };
  }
  export function match<Ok, Err, R>(
    value: Result<Ok, Err>,
    fns: {
      Ok(content: Ok): R;
      Err(content: Err): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
