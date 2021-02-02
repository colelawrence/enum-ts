import { Enum } from "./enum";

export type Stoplight = Enum<{
  Green: 0;
  Yellow: 0;
  Red: 0;
}>;

//#region enum-ts generated <fe4264a17e80cd23>
export namespace Stoplight {
  export function Green(contents: 0): Stoplight {
    return ["Green", contents];
  }
  export function Yellow(contents: 0): Stoplight {
    return ["Yellow", contents];
  }
  export function Red(contents: 0): Stoplight {
    return ["Red", contents];
  }
  export function apply<R>(fns: {
    Green(content: 0): R;
    Yellow(content: 0): R;
    Red(content: 0): R;
  }): (value: Stoplight) => R {
    return function matchStoplightApply([name, contents]) {
      // @ts-ignore
      return fns[name](contents);
    };
  }
  export function match<R>(
    value: Stoplight,
    fns: {
      Green(content: 0): R;
      Yellow(content: 0): R;
      Red(content: 0): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
//#endregion
