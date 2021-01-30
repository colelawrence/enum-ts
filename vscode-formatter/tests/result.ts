import { Enum } from "./enum";

export type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;

//#region enum-ts generated <fbe449f794ffcab5>
export namespace Result {
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
//#endregion