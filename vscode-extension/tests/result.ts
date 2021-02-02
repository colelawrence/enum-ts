import { Enum } from "./enum";

export type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;

//#region enum-ts generated <2cf4312647586153>
export namespace Result {
  export function Ok<O, E>(contents: O): Result<O, E> {
    return ["Ok", contents];
  }
  export function Err<O, E>(contents: E): Result<O, E> {
    return ["Err", contents];
  }
  export function isOk<O, E>(item: Result<O, E>): item is ["Ok", O] {
    return item != null && item[0] === "Ok";
  }
  export function isErr<O, E>(item: Result<O, E>): item is ["Err", E] {
    return item != null && item[0] === "Err";
  }
  export function apply<O, E, R>(fns: {
    Ok(content: O): R;
    Err(content: E): R;
  }): (value: Result<O, E>) => R {
    return function matchResultApply([name, contents]) {
      // @ts-ignore
      return fns[name](contents);
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