# typpkg: a converter for [typst](https://typst.app) package repos to typst packages

This tool translates a project with `typst.toml` to the corresponding `{package name}/{package version}` directory. It makes it convenient to maintain (version controlled) local packages and makes publishing packages easier and less error-prone.

- local usage: specifying no `output` directory, places it into the `@local` folder
- publish usage: specifying the forked [typst packages](https://github.com/typst/packages) repo appended by `packages/preview`, places it inside the correct directory

## Features
- correct folder creation and placement
- exclude files from publishing
  - include files otherwise excluded
- replace import statements with the `@local`/`@preview` correspondent
- compares documents inside `tests` to new version with perceptual hash

## Non-features
- watching on file changes, you can use an external tool, e.g. [watchexec](https://github.com/watchexec/watchexec)

## typst.toml options
To configure this tool's options, one can utilize the `[tool.typpkg]` table inside `typst.toml`. The following options are provided:
- `exclude`: globs to exclude from publishing. `.git`, `.git/**/*` and `.gitignore` are ignored by default.
- `include`: globs to include even though they have been excluded by in `exclude` field
- `replace_imports`: globs to files, where to replace the imports
  - `replace_locals`: also replaces `@local` with `@preview`, default is false.
- `typst_compile`: array of tables:
  - `path`: glob to file to compile
  - `root`: by default is the package directory
  - `format`: one of {`pdf`, `png`, `svg`}, is by default `pdf`
- `postscript`: script which executes after this tool in the output directory. It is a table with the field `command` and the field `arguments`.

> [!WARNING]
> The globs in the config use the [wax](https://glob.guide/) library. 

If you wanted to exclude the `example` folder in your project, you would have to write
```toml
[tool.typpkg]
exclude = ["examples", "examples/**/*"]
```

If you wanted to call `echo hello` in nushell as the prescript, use
```toml
[tool.typpkg]
prescript = {command="nu", arguments=['-c "echo hello"']}
```

## TODO
- [] files to compile
- [] tests to run