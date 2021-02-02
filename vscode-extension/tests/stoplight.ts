import { Enum } from "./enum";

export type Stoplight = Enum<{
  Green: 0;
  Yellow: 0;
  Red: 0;
}>;

//#region enum-ts generated <174a05206a9aaca1>
export namespace Stoplight {
  export function Green(contents: 0): Stoplight {
    return { Green: contents };
  }
  export function Yellow(contents: 0): Stoplight {
    return { Yellow: contents };
  }
  export function Red(contents: 0): Stoplight {
    return { Red: contents };
  }
  export function isGreen(item: Stoplight): item is { Green: 0 } {
    return item != null && "Green" in item;
  }
  export function isYellow(item: Stoplight): item is { Yellow: 0 } {
    return item != null && "Yellow" in item;
  }
  export function isRed(item: Stoplight): item is { Red: 0 } {
    return item != null && "Red" in item;
  }
  const unexpected = "Unexpected Enum variant for Stoplight";
  export function apply<R>(fns: {
    Green(content: 0): R;
    Yellow(content: 0): R;
    Red(content: 0): R;
  }): (value: Stoplight) => R {
    return function matchStoplightApply(item) {
      return "Green" in item
        ? fns.Green(item.Green)
        : "Yellow" in item
        ? fns.Yellow(item.Yellow)
        : "Red" in item
        ? fns.Red(item.Red)
        : (console.assert(false, unexpected, item) as never);
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
