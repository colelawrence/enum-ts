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

//#region enum-ts generated <3aba71cab09d1668>
type Ok<Ok, Err> = Ok;
type Err<Ok, Err> = Err;
function Ok<Ok, Err>(contents: Ok<Ok, Err>): { Ok: Ok<Ok, Err> } {
  return { Ok: contents };
}
function Err<Ok, Err>(contents: Err<Ok, Err>): { Err: Err<Ok, Err> } {
  return { Err: contents };
}
function isOk<Ok, Err>(item: Result<Ok, Err>): item is { Ok: Ok<Ok, Err> } {
  return item != null && "Ok" in item;
}
function isErr<Ok, Err>(item: Result<Ok, Err>): item is { Err: Err<Ok, Err> } {
  return item != null && "Err" in item;
}
namespace Result {
  const unexpected = "Unexpected Enum variant for Result<Ok, Err>";
  export function apply<Ok, Err, R>(fns: {
    Ok(content: Ok<Ok, Err>): R;
    Err(content: Err<Ok, Err>): R;
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
      Ok(content: Ok<Ok, Err>): R;
      Err(content: Err<Ok, Err>): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
//#endregion
