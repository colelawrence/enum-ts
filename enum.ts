/**
 * This special type can help generate pattern matchers for you!
 * Just use it as so:
 *
 *    // gen: enum-matcher
 *    type Result<Ok, Err> = Enum<{
 *      Ok: Ok,
 *      Err: Err,
 *    }>
 */
export type Enum<T extends { [Variant: string]: any }> = {
  [P in keyof T]: { t: P; c: T[P] };
}[keyof T];
