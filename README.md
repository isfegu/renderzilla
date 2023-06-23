# Renderzilla

This repository is a proof of concept on how to use Rust to transform an SVG to PNG using [resvg](https://github.com/RazrFalcon/resvg), [usvg](https://github.com/RazrFalcon/resvg/tree/master/crates/usvg) and [tiny-skia](https://github.com/RazrFalcon/tiny-skia). All these (and many more) crates has been created by [RazrFalcon](https://github.com/RazrFalcon).

## Usage

Clone this repository and run:

```bash
renderzilla$ cargo run
```

This command creates the `equation.png` file using the `equation.svg` file as source.

## Contributing

If for some reason you want to contribute to this repository, please:

* If you use Visual Studio Code, open the [renderzilla.code-workspace](./renderzilla.code-workspace) workspace and install all recommended extensions.
* Use [Conventional Commits](https://www.conventionalcommits.org/).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a pull request to `main`.
* Use [Semantic Versioning](https://semver.org/). Each change done MUST update the crate's version.
