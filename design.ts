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

//#region enum-ts generated <afded0c975c7a073>
namespace Result {
  export function Ok<Ok, Err>(contents: Ok): Result<Ok, Err> {
    return { Ok: contents };
  }
  export function Err<Ok, Err>(contents: Err): Result<Ok, Err> {
    return { Err: contents };
  }
  export function isOk<Ok, Err>(item: Result<Ok, Err>): item is { Ok: Ok } {
    return item != null && "Ok" in item;
  }
  export function isErr<Ok, Err>(item: Result<Ok, Err>): item is { Err: Err } {
    return item != null && "Err" in item;
  }
  const unexpected = "Unexpected Enum variant for Result<Ok, Err>";
  export function apply<Ok, Err, R>(fns: {
    Ok(content: Ok): R;
    Err(content: Err): R;
  }): (value: Result<Ok, Err>) => R {
    return function matchResultApply(item) {
      return "Ok" in item
        ? fns.Ok(item.Ok)
        : "Err" in item
        ? fns.Err(item.Err)
        : (console.assert(false, unexpected, item) as never);
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
