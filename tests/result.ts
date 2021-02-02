import { Enum } from "../enum";

export type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;

//#region enum-ts generated <670abf0a9f424d5a>
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
//#endregion
