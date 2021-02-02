import { Enum } from "../enum";

export type Stoplight = Enum<{
  Green: 0;
  Yellow: 0;
  Red: 0;
}>;

//#region enum-ts generated <193cc55841409219>
export type Green = 0;
export type Yellow = 0;
export type Red = 0;
export function Green(contents: Green): { Green: Green } {
  return { Green: contents };
}
export function Yellow(contents: Yellow): { Yellow: Yellow } {
  return { Yellow: contents };
}
export function Red(contents: Red): { Red: Red } {
  return { Red: contents };
}
export function isGreen(item: Stoplight): item is { Green: Green } {
  return item != null && "Green" in item;
}
export function isYellow(item: Stoplight): item is { Yellow: Yellow } {
  return item != null && "Yellow" in item;
}
export function isRed(item: Stoplight): item is { Red: Red } {
  return item != null && "Red" in item;
}
export namespace Stoplight {
  const unexpected = "Unexpected Enum variant for Stoplight";
  export function apply<R>(fns: {
    Green(content: Green): R;
    Yellow(content: Yellow): R;
    Red(content: Red): R;
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
      Green(content: Green): R;
      Yellow(content: Yellow): R;
      Red(content: Red): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
//#endregion
