import { Enum } from "../enum";

export type CompletionDropdownItemView = Enum<{
  NavHintView: {
    /** Documentation comment */
    text: string;
    key: string;
    icon_url: string;
  };
  CompletionView: {
    replacement: string;
    displayOverrides: string;
    /** Documentation comment */
    diagnostics: string | null;
  };
  CategoryView: {
    displayOverrides: string;
  };
}>;

//#region enum-ts generated <2c652eb64b89014>
export type NavHintView = {
  /** Documentation comment */
  text: string;
  key: string;
  icon_url: string;
};
export type CompletionView = {
  replacement: string;
  displayOverrides: string;
  /** Documentation comment */
  diagnostics: string | null;
};
export type CategoryView = {
  displayOverrides: string;
};
export function NavHintView(contents: NavHintView): { NavHintView: NavHintView } {
  return { NavHintView: contents };
}
export function CompletionView(contents: CompletionView): { CompletionView: CompletionView } {
  return { CompletionView: contents };
}
export function CategoryView(contents: CategoryView): { CategoryView: CategoryView } {
  return { CategoryView: contents };
}
export function isNavHintView(item: CompletionDropdownItemView): item is { NavHintView: NavHintView } {
  return item != null && "NavHintView" in item;
}
export function isCompletionView(item: CompletionDropdownItemView): item is { CompletionView: CompletionView } {
  return item != null && "CompletionView" in item;
}
export function isCategoryView(item: CompletionDropdownItemView): item is { CategoryView: CategoryView } {
  return item != null && "CategoryView" in item;
}
export namespace CompletionDropdownItemView {
  const unexpected = "Unexpected Enum variant for CompletionDropdownItemView";
  export function apply<R>(fns: {
    NavHintView(content: NavHintView): R;
    CompletionView(content: CompletionView): R;
    CategoryView(content: CategoryView): R;
  }): (value: CompletionDropdownItemView) => R {
    return function matchCompletionDropdownItemViewApply(item) {
      return "NavHintView" in item
        ? fns.NavHintView(item.NavHintView)
        : "CompletionView" in item
        ? fns.CompletionView(item.CompletionView)
        : "CategoryView" in item
        ? fns.CategoryView(item.CategoryView)
        : (console.assert(false, unexpected, item) as never);
    };
  }
  export function match<R>(
    value: CompletionDropdownItemView,
    fns: {
      NavHintView(content: NavHintView): R;
      CompletionView(content: CompletionView): R;
      CategoryView(content: CategoryView): R;
    }
  ): R {
    return apply(fns)(value);
  }
}
//#endregion
