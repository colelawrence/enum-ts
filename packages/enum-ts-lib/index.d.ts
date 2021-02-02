/**
 * This special type can help generate pattern matchers for you!
 * Just use it as so:
 *
 * ```ts
 * type Result<Ok, Err> = Enum<{
 *   Ok: Ok,
 *   Err: Err,
 * }>
 * ```
 */
export type Enum<T extends { [Variant: string]: any }> = {
  [P in keyof T]: Record<P, T[P]>;
}[keyof T];
