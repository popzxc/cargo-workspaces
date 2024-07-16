<!-- omit from toc -->
# cargo-workspaces

Inspired by [Lerna](https://lerna.js.org/)

A tool that optimizes the workflow around cargo workspaces with `git` and `cargo` by providing utilities to
version, publish, execute commands and more.

I made this to work on [clap](https://github.com/clap-rs/clap) and other projects that rely on workspaces.
But this will also work on single crates because by default every individual crate is a workspace.

1. [Installation](#installation)
2. [Usage](#usage)
   1. [Init](#init)
   2. [Create](#create)
   3. [List](#list)
   4. [Changed](#changed)
   5. [Exec](#exec)
   6. [Version](#version)
      1. [Fixed or Independent](#fixed-or-independent)
   7. [Publish](#publish)
   8. [Rename](#rename)
3. [Config](#config)
4. [Changelog](#changelog)

## Installation

```
cargo install cargo-workspaces
```

## Usage

The installed tool can be called by `cargo workspaces` or `cargo ws`. Both of them point to the same.

You can use `cargo ws help` or `cargo ws help <subcmd>` anytime to understand allowed options.

The basic commands available for this tool are given below. Assuming you run them inside a cargo workspace.

### Init

Initializes a new cargo workspace in the given directory. Creates `Cargo.toml` if it does not exist and
fills the `members` with the all the crates that can be found in that directory.

```
USAGE:
    cargo workspaces init [PATH]

ARGS:
    <PATH>    Path to the workspace root [default: .]

OPTIONS:
    -h, --help    Print help information
```

### Create

Interactively creates a new crate in the workspace. *We recommend using this instead of `cargo new`*. All
the crates start with `0.0.0` version because the [version](#version) is responsible for determining the
version.

```
USAGE:
    cargo workspaces create [OPTIONS] <PATH>

ARGS:
    <PATH>    Path for the crate relative to the workspace manifest

OPTIONS:
        --bin                  Whether this is a binary crate
        --edition <EDITION>    The crate edition [possible values: 2015, 2018, 2021]
    -h, --help                 Print help information
        --lib                  Whether this is a library crate
        --name <NAME>          The name of the crate
```

### List

Lists crates in the workspace.

```
USAGE:
    cargo workspaces list [OPTIONS]

OPTIONS:
    -a, --all     Show private crates that are normally hidden
    -h, --help    Print help information
        --json    Show information as a JSON array
    -l, --long    Show extended information
```

Several aliases are available.

* `cargo ws ls` implies `cargo ws list`
* `cargo ws ll` implies `cargo ws list --long`
* `cargo ws la` implies `cargo ws list --all`

### Changed

List crates that have changed since the last git tag. This is useful to see the list of crates that
would be the subjects of the next [version](#version) or [publish](#publish) command.

```
USAGE:
    cargo workspaces changed [OPTIONS]

OPTIONS:
    -a, --all                         Show private crates that are normally hidden
        --error-on-empty              Return non-zero exit code if no changes detected
        --force <pattern>             Always include targeted crates matched by glob even when there are no changes
    -h, --help                        Print help information
        --ignore-changes <pattern>    Ignore changes in files matched by glob
        --include-merged-tags         Include tags from merged branches
        --json                        Show information as a JSON array
    -l, --long                        Show extended information
        --since <SINCE>               Use this git reference instead of the last tag
```

### Exec

Executes an arbitrary command in each crate of the workspace.

```
USAGE:
    cargo workspaces exec [OPTIONS] <ARGS>...

ARGS:
    <ARGS>...

OPTIONS:
    -h, --help                Print help information
        --ignore <pattern>    Ignore the crates matched by glob
        --ignore-private      Ignore private crates
        --no-bail             Continue executing command despite non-zero exit in a given crate
```

For example, if you want to run `ls -l` in each crate, you can simply do `cargo ws exec ls -l`.

### Version

Bump versions of the crates in the workspace. This command does the following:

1. Identifies crates that have been updated since the previous tagged release
2. Prompts for a new version according to the crate
3. Modifies crate manifest to reflect new release
4. Update intra-workspace dependency version constraints if needed
5. Commits those changes
6. Tags the commit
7. Pushes to the git remote

You can influence the above steps with the flags and options for this command.

```
USAGE:
    cargo workspaces version [OPTIONS] [ARGS]

OPTIONS:
    -h, --help    Print help information

VERSION ARGS:
    <BUMP>      Increment all versions by the given explicit semver keyword while skipping the prompts for them
                [possible values: major, minor, patch, premajor, preminor, prepatch, skip, prerelease, custom]
    <CUSTOM>    Specify custom version value when 'bump' is set to 'custom'

VERSION OPTIONS:
    -a, --all                         Also do versioning for private crates (will not be published)
        --exact                       Specify inter dependency version numbers exactly with `=`
        --force <pattern>             Always include targeted crates matched by glob even when there are no changes
        --ignore-changes <pattern>    Ignore changes in files matched by glob
        --include-merged-tags         Include tags from merged branches
        --pre-id <identifier>         Specify prerelease identifier
    -y, --yes                         Skip confirmation prompt

GIT OPTIONS:
        --allow-branch <pattern>            Specify which branches to allow from [default: master]
        --amend                             Amend the existing commit, instead of generating a new one
        --git-remote <remote>               Push git changes to the specified remote [default: origin]
        --individual-tag-prefix <prefix>    Customize prefix for individual tags (should contain `%n`) [default: %n@]
    -m, --message <MESSAGE>                 Use a custom commit message when creating the version commit [default: Release %v]
        --no-git-commit                     Do not commit version changes
        --no-git-push                       Do not push generated commit and tags to git remote
        --no-git-tag                        Do not tag generated commit
        --no-global-tag                     Do not create a global tag for a workspace
        --no-individual-tags                Do not tag individual versions for crates
        --tag-prefix <prefix>               Customize tag prefix (can be empty) [default: v]
```

#### Fixed or Independent

By default, all the crates in the workspace will share a single version. But if you want the crate to have
it's version be independent of the other crates, you can add the following to that crate:

```toml
[package.metadata.workspaces]
independent = true
```

For more details, check [Config](#config) section below.

### Publish

Publish all the crates from the workspace in the correct order according to the dependencies. By default,
this command runs [version](#version) first. If you do not want that to happen, you can supply the
`--from-git` option.

> Note: dev-dependencies are not taken into account when building the dependency
> graph used to determine the proper publishing order. This is because
> dev-dependencies are ignored by `cargo publish` - as such, a dev-dependency on a
> local crate (with a `path` attribute), should _not_ have a `version` field.

```
USAGE:
    cargo workspaces publish [OPTIONS] [ARGS]

OPTIONS:
    -h, --help    Print help information

VERSION ARGS:
    <BUMP>      Increment all versions by the given explicit semver keyword while skipping the prompts for them [possible values: major, minor, patch,
                premajor, preminor, prepatch, prerelease, custom]
    <CUSTOM>    Specify custom version value when 'bump' is set to 'custom'

VERSION OPTIONS:
    -a, --all                         Also do versioning for private crates (will not be published)
        --exact                       Specify inter dependency version numbers exactly with `=`
        --force <pattern>             Always include targeted crates matched by glob even when there are no changes
        --ignore-changes <pattern>    Ignore changes in files matched by glob
        --include-merged-tags         Include tags from merged branches
        --pre-id <identifier>         Specify prerelease identifier
    -y, --yes                         Skip confirmation prompt

GIT OPTIONS:
        --allow-branch <pattern>            Specify which branches to allow from [default: master]
        --amend                             Amend the existing commit, instead of generating a new one
        --git-remote <remote>               Push git changes to the specified remote [default: origin]
        --individual-tag-prefix <prefix>    Customize prefix for individual tags (should contain `%n`) [default: %n@]
    -m, --message <MESSAGE>                 Use a custom commit message when creating the version commit [default: Release %v]
        --no-git-commit                     Do not commit version changes
        --no-git-push                       Do not push generated commit and tags to git remote
        --no-git-tag                        Do not tag generated commit
        --no-global-tag                     Do not create a global tag for a workspace
        --no-individual-tags                Do not tag individual versions for crates
        --tag-prefix <prefix>               Customize tag prefix (can be empty) [default: v]

PUBLISH OPTIONS:
        --allow-dirty            Allow dirty working directories to be published
        --no-remove-dev-deps     Don't remove dev-dependencies while publishing
        --no-verify              Skip crate verification (not recommended)
        --publish-as-is          Publish crates from the current commit without versioning
        --registry <REGISTRY>    The Cargo registry to use for publishing
        --token <TOKEN>          The token to use for publishing
```

### Rename

Rename crates in the project. You can run this command when you might want to publish the crates with a standard prefix.

```
USAGE:
    cargo workspaces rename [OPTIONS] <TO>

ARGS:
    <TO>    The value that should be used as new name (should contain `%n`)

OPTIONS:
    -a, --all                 Rename private crates too
    -f, --from <crate>        Rename only a specific crate
    -h, --help                Print help information
        --ignore <pattern>    Ignore the crates matched by glob
```

## Config

There are two kind of options.

* **Workspace**: Options that are specified in the workspace with `[workspace.metadata.workspaces]`
* **Package**: Options that are specified in the package with `[package.metadata.workspaces]`

If an option is allowed to exist in both places, it means that the value specified in the **Package**
overrides the value specified in **Workspace**.

| Name | Type | Workspace | Package | Used in Commands |
| --- | --- | :---: | :---: | --- |
| `allow_branch` | `String` | Yes | No | `version`, `publish` |
| `independent` | `bool` | No | Yes | `version`, `publish` |
| `no_individual_tags` | `bool` | Yes | No | `version`, `publish` |

<!-- omit from toc -->
## Contributors
Here is a list of [Contributors](http://github.com/pksunkara/cargo-workspaces/contributors)

<!-- omit from toc -->
### TODO

## Changelog
Please see [CHANGELOG.md](CHANGELOG.md).

<!-- omit from toc -->
## License
MIT/X11

<!-- omit from toc -->
## Bug Reports
Report [here](http://github.com/pksunkara/cargo-workspaces/issues).

<!-- omit from toc -->
## Creator
Pavan Kumar Sunkara (pavan.sss1991@gmail.com)

Follow me on [github](https://github.com/users/follow?target=pksunkara), [twitter](http://twitter.com/pksunkara)
