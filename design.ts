import { Enum } from "./enum";

// enum: factory, match
type Result<Ok, Err> = Enum<{
  Ok: Ok;
  Err: Err;
}>;

export function findLargestIntInList(list: string[]): number {
  return list.map(parseInteger).reduce(
    (acc, next) =>
      Result.match(next, {
        Err(err) {
          console.error(err);
          return acc;
        },
        Ok(num) {
          if (num > acc) return num;
          else return acc;
        },
      }),
    0
  );
}

function parseInteger(input: string): Result<number, string> {
  try {
    return Result.Ok(parseInt(input));
  } catch (err) {
    return Result.Err(
      `Failed to parse "${input}" as an integer.\n${err.message}`
    );
  }
}

//#region enum-ts generated <cae8a3797082e66>
namespace Result {
  export function Ok<Ok, Err>(contents: Ok): Result<Ok, Err> {
    return ["Ok", contents];
  }
  export function Err<Ok, Err>(contents: Err): Result<Ok, Err> {
    return ["Err", contents];
  }
  export function apply<Ok, Err, R>(fns: {
    Ok(content: Ok): R;
    Err(content: Err): R;
  }): (value: Result<Ok, Err>) => R {
    return function matchResultApply([name, contents]) {
      // @ts-ignore
      return fns[name](contents);
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
//#endregion
