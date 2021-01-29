import { Enum } from "../enum";

export type Result<O, E> = Enum<{
  Ok: O;
  Err: E;
}>;
