import { Enum } from "../enum";

export type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;

//#region enum-ts generated <d4f13e68116377e4>
export namespace Result {
  export function Ok<O, E>(contents: O): Result<O, E> {
    return ["Ok", contents];
  }
  export function Err<O, E>(contents: E): Result<O, E> {
    return ["Err", contents];
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
