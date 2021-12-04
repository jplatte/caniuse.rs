# [caniuse.rs]

Rust feature search

## About

This site allows you to quickly which version of Rust stabilized a certain
feature, or whether it is still unstable. Head over to [caniuse.rs] to try it!

[caniuse.rs]: https://caniuse.rs/

## Contributing

### Running the site locally

First you need to be able to run the site locally of course, to try your
changes.

Pre-requisites:

- cargo
- Python 3
- [wasm-pack][] (website has install instructions, to build from source use `cargo install wasm-pack`)
- [rollup][] (`npm install --global rollup` if you have [npm installed][npm])

[wasm-pack]: https://rustwasm.github.io/wasm-pack/installer/
[rollup]: https://www.rollupjs.org/guide/en/
[npm]: https://www.npmjs.com/get-npm

Once those are installed, just run `./x.py serve` and the site will be built
and served at <http://localhost:8000/>.

### Adding data

You may want to look at issue [#16][] for missing data. These fields can be
specified for a feature:

* `title`: Short description to identify the feature. Should fit into
  "can i use {title}?".
* `flag`: The feature flag, if any – you can most often find this in the diff of
  the stabilization or implementation PR, for library features look for
  `#[stable]` and `#[rustc_const_stable]` attributes.
* `rfc_id`: RFC ID, if applicable – the number of the PR that added the RFC.
  Also the first part of the filename of the RFC after being merged.
* `impl_pr_id`: Implementation PR ID, if applicable – the number of the PR that
  added this feature. For features where is no clear single implementation PR,
  leave out this field.
* `tracking_issue_id`: Tracking issue ID, if applicable.
* `stabilization_pr_id`: Stabilization PR ID, if applicable.
* `doc_path`: Documentation path, if applicable –
  `https://doc.rust-lang.org/{path}`.
* `edition_guide_path`: Edition guide path, if applicable –
  `https://doc.rust-lang.org/edition-guide/{path}`
* `unstable_book_path`: Unstable book path, if applicable –
  `https://doc.rust-lang.org/unstable-book/{path}`
* `items`: Language items (functions, structs, modules) that are part of this
  feature – do not specify if this feature is exactly one item and that item
  is already used as the title
* `aliases`: Alternatives to the `title`

[#16]: https://github.com/jplatte/caniuse.rs/issues/16

## Related Projects

- [alfred-caniuse-rs](https://github.com/robjtede/alfred-caniuse-rs): caniuse.rs lookups as an Alfred workflow (macOS)
