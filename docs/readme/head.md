![Gen-changelog — Generate a change log based on the git commits compatible with keep-a-changelog and using conventional commits to categorise commits][splash]

[splash]: https://raw.githubusercontent.com/jerus-org/gen-changelog/main/assets/splash.svg

[![Rust 1.85+][version-badge]][version-url]
[![circleci-badge]][circleci-url]
[![Crates.io][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE licensed][apache-badge]][apache-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/gen-changelog.svg
[crates-url]: https://crates.io/crates/gen-changlog
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/gen-changelog/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[apache-url]: https://github.com/jerusdp/gen-changelog/blob/main/LICENSE-APACHE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/gen-changelog/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/gen-changelog/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.85+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/gen-changelog/badge.svg
[docs-url]:  https://docs.rs/gen-changelog
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

**gen-changelog** is a release tool that generates changelogs in the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. It analyses a repository's commit history and uses conventional commit types to categorise and filter commits for inclusion in the changelog.

## Main Features

- **Commit Categorization**: Uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification to automatically categorise commits and filter them for changelog inclusion
- **Summary Counts**: Displays summary counts for each commit category in releases, including uncategorised (non-conventional) commits
- **Detailed Commit Summaries**: Shows commit details for Added, Fixed, Changed, and Security categories
- **Security Classification**: Automatically classifies commits made to the dependency scope as Security commits, regardless of their conventional commit type
- **Flexible Configuration**: Configurable mapping of commit types to headings, customizable heading display options, and optional commit summary counts
