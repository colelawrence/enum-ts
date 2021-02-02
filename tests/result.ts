import { Enum } from "../enum";

export type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;

//#region enum-ts generated <143964222eca0ea6>
export namespace Result {
  export function Ok<O, E>(contents: O): Result<O, E> {
    return { Ok: contents };
  }
  export function Err<O, E>(contents: E): Result<O, E> {
    return { Err: contents };
  }
  export function isOk<O, E>(item: Result<O, E>): item is { Ok: O } {
    return item != null && "Ok" in item;
  }
  export function isErr<O, E>(item: Result<O, E>): item is { Err: E } {
    return item != null && "Err" in item;
  }
  const unexpected = "Unexpected Enum variant for Result<O, E>";
  export function apply<O, E, R>(fns: {
    Ok(content: O): R;
    Err(content: E): R;
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
      Ok(content: O): R;
      Err(content: E): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
//#endregion
