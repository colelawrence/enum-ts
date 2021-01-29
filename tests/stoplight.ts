import { Enum } from "../enum";

export type Stoplight = Enum<{
  Green: 0;
  Yellow: 0;
  Red: 0;
}>;

//#region enum-ts generated <992d80798e9f9a29>
export namespace Stoplight {
  export function Green(contents: 0): Stoplight {
    return { t: "Green", c: contents };
  }
  export function Yellow(contents: 0): Stoplight {
    return { t: "Yellow", c: contents };
  }
  export function Red(contents: 0): Stoplight {
    return { t: "Red", c: contents };
  }
  export function apply<R>(fns: {
    Green(content: 0): R;
    Yellow(content: 0): R;
    Red(content: 0): R;
  }): (value: Stoplight) => R {
    return function matchStoplightApply(value) {
      // @ts-ignore
      return fns[value.t](value.c);
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
