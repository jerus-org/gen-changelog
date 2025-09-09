# Gen-changelog

Generate a change log based on the git commits compatible with keep-a-changelog and using conventional commits to categorise commits.

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


## Gen-changelog library documentation 

The library provides the `ChangeLogConfig` and `ChangeLog` structs to configure and construct the change log document. 

### Usage

Add the library to your program's `Cargo.toml` using `Cargo add` 

```bash
$ cargo add gen-changelog

```

or by configuring the dependencies in `Cargo.toml`

```toml
[dependencies]
gen-changelog = "0.0.4"
```

## Gen-changelog executable documentation 


## License

 Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
 at your option.

## Contribution

 Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
