# Gistlist

[![Rust build status](https://img.shields.io/github/actions/workflow/status/travisbrown/gistlist/ci.yaml?branch=main)](https://github.com/travisbrown/gistlist/actions)
[![Coverage status](https://img.shields.io/codecov/c/github/travisbrown/gistlist/main.svg)](https://codecov.io/github/travisbrown/gistlist)

This is a tiny Rust project that just dumps the JSON values returned from the GitHub API for all of your gists.

I wanted to be able to clone all of my gists to a single place, to sort by number of comments, etc.,
and I didn't really like that the official CLI tool makes these kinds of operations difficult or impossible by only returning a few fixed fields in a fixed, not very pipeable format.

To use it you need a [personal access token][tokens].
If you want the output to include your private gists, the token will need to have the `gist` scope (the documentation just says that this grants write access, but it also is needed for read access to your private gists).

```bash
$ cargo build --release
$ target/release/gistlist -vvvvv --token ghp_aAAAaaAAaaaaAAAAAAAAAaaA list > gists.ndjson
```

Then you can run for example something like the following to list your most commented-on gists:

```bash
$ jq -r "[first(.files[]).filename, .comments] | @csv" < gists.ndjson | sort -t, -k2,2nr | less
```

Or to make a script to clone everything:

```bash
$ jq -r ".git_pull_url" < gists.ndjson | sed "s/^/git clone /"
```

Please note that otherwise the code provided in this repository is **not** "open source",
but the source is available for use and modification by individuals, non-profit organizations, and worker-owned cooperatives
(see the [license section](#license) below for details).

## License

This software is published under the [Anti-Capitalist Software License][acsl] (v. 1.4).

[acsl]: https://anticapitalist.software/
[github]: https://github.com/
[scopes]: https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps
[tokens]: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens
