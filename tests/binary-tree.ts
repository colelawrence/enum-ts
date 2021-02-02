import { Enum } from "../enum";

export type BinaryTree<T> = Enum<{
  Leaf: T;
  Branch: {
    left: BinaryTree<T>;
    right: BinaryTree<T>;
  };
}>;

//#region enum-ts generated <40b84c8ed7eda02f>
export type Leaf<T> = T;
export type Branch<T> = {
  left: BinaryTree<T>;
  right: BinaryTree<T>;
};
export function Leaf<T>(contents: Leaf<T>): { Leaf: Leaf<T> } {
  return { Leaf: contents };
}
export function Branch<T>(contents: Branch<T>): { Branch: Branch<T> } {
  return { Branch: contents };
}
export function isLeaf<T>(item: BinaryTree<T>): item is { Leaf: Leaf<T> } {
  return item != null && "Leaf" in item;
}
export function isBranch<T>(item: BinaryTree<T>): item is { Branch: Branch<T> } {
  return item != null && "Branch" in item;
}
export namespace BinaryTree {
  const unexpected = "Unexpected Enum variant for BinaryTree<T>";
  export function apply<T, R>(fns: {
    Leaf(content: Leaf<T>): R;
    Branch(content: Branch<T>): R;
  }): (value: BinaryTree<T>) => R {
    return function matchBinaryTreeApply(item) {
      return "Leaf" in item
        ? fns.Leaf(item.Leaf)
        : "Branch" in item
        ? fns.Branch(item.Branch)
        : (console.assert(false, unexpected, item) as never);
    };
  }
  export function match<T, R>(
    value: BinaryTree<T>,
    fns: {
      Leaf(content: Leaf<T>): R;
      Branch(content: Branch<T>): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
//#endregion
