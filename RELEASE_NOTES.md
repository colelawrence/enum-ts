# v0.2.3: Wiring up CI to Publish

- Add `-v` and `--version` flags to print the crate's current version to stderr.
- fix: Exclude all non-source files from `cargo publish`
- Update `binary-install`

# v0.2.0: Externally tagged enums

- Use externally tagged enums (e.g. `{ "Variant1": contents }`) [see `serde`'s enum representations](https://serde.rs/enum-representations.html#externally-tagged)
- Expose variant creator checks at the same level as the parent type (a-la Elm / FSharp names)
- Add type guards
- Fix nested variant types and declare type aliases for variants

# v0.1.10

- Fix type guards with generics
- Prepare binary releases through `enum-ts-bin` and a library through `enum-ts-lib` packages
- Leverage the redistributable binaries in the VS Code Extension
