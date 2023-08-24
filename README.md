# git-assist

<!-- [![Build Status](https://img.shields.io/github/actions/workflow/status/regexident/git-assist/ci.yml?branch=main&style=flat-square)](https://github.com/regexident/git-assist/actions/workflows/ci.yml?query=branch%3Amain) -->
[![Docs.rs](https://img.shields.io/docsrs/git-assist?style=flat-square)](https://docs.rs/git-assist)
[![Crates.io](https://img.shields.io/crates/v/git-assist?style=flat-square)](https://crates.io/crates/git-assist)
[![Downloads](https://img.shields.io/crates/d/git-assist.svg?style=flat-square)](https://crates.io/crates/git-assist/)
[![Version](https://img.shields.io/crates/v/git-assist.svg?style=flat-square)](https://crates.io/crates/git-assist/)

## Synopsis

A friendly git assistant.

<!-- ## Motivation -->

## Installation

Install `git-assist` via:

```bash
cargo install git-assist
```

## Usage

`git-assist` currently implements assistive features for the following git commands:

### `git bisect`

`git-assist` currently implements the following commands related to `git bisect`:

#### `git assist bisect skip-pull-requests [OPTIONS]`

```terminal
Usage: git-assist bisect skip-pull-requests [OPTIONS]

Options:
      --remote-url <REMOTE_URL>  Remote url to fetch pull requests from
      --good <GOOD>              A known "good" commit
      --bad <BAD>                A known "bad" commit
      --dry-run                  Perform a "dry" run
  -h, --help                     Print help
```

Most options can either be passed as command-line arguments or entered interactively, later on.

Why is the `skip-pull-requests`` sub-command useful?

Github supports three merging schemes:

- **Merge**: When you click the default "Merge" pull request option on a pull request on GitHub.com, all commits from the feature branch are added to the base branch in a merge commit. The pull request is merged using the --no-ff option.
- **Squash and merge**: When you select the "Squash and merge" option on a pull request on GitHub.com, the pull request's commits are squashed into a single commit.
- **Rebase and merge**: When you select the Rebase and merge option on a pull request on GitHub.com, all commits from the topic branch (or head branch) are added onto the base branch individually without a merge commit.

All of these come with their pros and cons, when it comes to running git bisect to find the commit that introduced a bug.

- **Merge**: No clear linear and thus ambiguous history.
- **Squash and merge**: You basically lose a ton of useful information in the process of squashing, at the cost of git bisect running smoothly.
- **Rebase and merge**: Unless all your commits leave your project in a buildable state at all times running git bisect is a noisy and labor-intensive business.

So what if one could combine the convenience of "Squash and merge" with the history-preserving nature of "Rebase and merge"?

What the `skip-pull-requests` sub-command does:

1. fetch all pull requests associated with the repository's remote URL.
2. filter out any pull request that doesn't overlap with the `good..bad` commit range.
3. runs (or merely prints, in case of `--dry-run`) `git bisect skip base..head^` for each pull request.

The general usage of the `skip-pull-requests` sub-command looks something like this:

```terminal
git bisect start
git assist bisect skip-pull-requests --good <GOOD> --bad <BAD> ...
git bisect good <GOOD>
git bisect bad <BAD>
...
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),  
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/git-assist/tags).

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
