![Gen-changelog — Generate a change log based on the git commits compatible with keep-a-changelog and using conventional commits to categorise commits][splash]

[splash]: https://raw.githubusercontent.com/jerus-org/gen-changelog/main/assets/splash2.svg

[![Rust 1.87+][version-badge]][version-url]
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
[version-badge]: https://img.shields.io/badge/rust-1.87+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/gen-changelog/badge.svg
[docs-url]:  https://docs.rs/gen-changelog
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

**Gen-changelog** is a release tool that generates changelogs in the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. It analyses a repository's commit history and uses conventional commit types to categorise and filter commits for inclusion in the changelog.

## Main Features

- **Commit Categorization**: Uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification to automatically categorise commits and filter them for changelog inclusion
- **Summary Counts**: Displays summary counts for each commit category in releases, including uncategorised (non-conventional) commits
- **Detailed Commit Summaries**: Shows commit details for Added, Fixed, Changed, and Security categories
- **Security Classification**: Automatically classifies commits made to the dependency scope as Security commits, regardless of their conventional commit type
- **Flexible Configuration**: Configurable mapping of commit types to headings, customizable heading display options, and optional commit summary counts
## Gen-changelog Library Documentation

The `gen-changelog` library provides comprehensive changelog generation from Git repositories using [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) messages. The library centres around the `ChangeLogConfig` and `ChangeLog` structs for configuring and constructing changelog documents.

### Installation

Add the library to your program's `Cargo.toml` using `cargo add`:

```bash
$ cargo add gen-changelog
```

Or by configuring the dependencies manually in `Cargo.toml`:

```toml
[dependencies]
gen-changelog = "0.0.8"
```

### Default Configuration

The library provides sensible defaults for conventional commit types:

#### Default Groups

| Group                  | Commit Types         | Published |
| ---------------------- | -------------------- | --------- |
| Added                  | feat                 | ✓         |
| Fixed                  | fix                  | ✓         |
| Changed                | refactor             | ✓         |
| Security               | security, dependency | ✗         |
| Build                  | build                | ✗         |
| Documentation          | doc, docs            | ✗         |
| Chore                  | chore                | ✗         |
| Continuous Integration | ci                   | ✗         |
| Testing                | test                 | ✗         |
| Deprecated             | deprecated           | ✗         |
| Removed                | removed              | ✗         |
| Miscellaneous          | misc                 | ✗         |

By default, only `Added`, `Fixed`, `Changed`, and `Security` groups are published in the changelog.

### Configuration File

The library looks for a `gen-changelog.toml` configuration file. Example structure:

```toml
## Controls the number of changelog sections to display.
display-sections = "all"

## Defines the display order of groups in the changelog.
[headings]
1 = "Added"
2 = "Fixed"
3 = "Changed"
4 = "Security"

## Group tables define the third-level headings used to organize commits.
[groups.Added]
name = "Added"
publish = true
cc-types = ["feat"]

[groups.Fixed]
name = "Fixed"
publish = true
cc-types = ["fix"]

## ... additional groups
```

### Usage Examples

#### Basic Usage

```rust
use gen_changelog::{ChangeLog, ChangeLogConfig};
use git2::Repository;

fn generate_changelog() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    let config = ChangeLogConfig::from_file_or_default()?;
    
    let changelog = ChangeLog::builder()
        .with_config(config)
        .with_header("Changelog", &[
            "All notable changes to this project will be documented in this file.",
            "The format is based on Keep a Changelog."
        ])
        .walk_repository(&repo)?
        .build();
    
    changelog.save()?;
    Ok(())
}
```

#### Custom Configuration

```rust
use gen_changelog::{ChangeLog, ChangeLogConfig};
use git2::Repository;

fn generate_custom_changelog() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    let mut config = ChangeLogConfig::from_file_or_default()?;
    
    // Add custom groups to be published
    config.add_commit_groups(&["Documentation".to_string(), "Testing".to_string()]);
    
    // Limit to last 5 releases
    config.set_display_sections(Some(5));
    
    let changelog = ChangeLog::builder()
        .with_config(config)
        .with_summary_flag(true)
        .walk_repository(&repo)?
        .build();
    
    changelog.save()?;
    Ok(())
}
```

#### Release Preparation

```rust
use gen_changelog::{ChangeLog, ChangeLogConfig};
use git2::Repository;

fn prepare_release(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    let config = ChangeLogConfig::from_file_or_default()?;
    
    let changelog = ChangeLog::builder()
        .with_config(config)
        .walk_repository(&repo)?
        .update_unreleased_to_next_version(Some(&version.to_string()))
        .build();
    
    changelog.save()?;
    Ok(())
}
```

### Requirements

- Git repository with conventional commit messages
- GitHub repository for generating comparison links

The library automatically detects GitHub repositories and generates appropriate comparison and release links in the changelog output.## Gen-changelog CLI

A command-line tool that generates changelogs from git commits using conventional commit messages and keep-a-changelog formatting.

### Installation

Install gen-changelog using Cargo:

```bash
cargo install gen-changelog
```

### Overview

Gen-changelog CLI automatically generates changelogs by analysing your git commit history. It uses conventional commit patterns to categorize changes and outputs them in a format compatible with [Keep a Changelog](https://keepachangelog.com/).

```bash
gen-changelog --help
```

```
Generate a change log based on the git commits compatible
with keep-a-changelog and using conventional commits to categorise commits.

Usage: gen-changelog [OPTIONS] [COMMAND]

Commands:
  generate  Generate changelog from git commits
  config    Manage configuration settings
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

### Commands

#### `generate` - Generate Changelog

Creates a changelog file based on your repository's commit history.

```bash
gen-changelog generate [OPTIONS]
```

##### Options

| Option                         | Description                                        | Default                 |
| ------------------------------ | -------------------------------------------------- | ----------------------- |
| `-n, --next-version <VERSION>` | Version number for unreleased changes              | -                       |
| `-s, --sections <NUMBER>`      | Number of version sections to include in changelog | All                     |
| `-c, --config-file <FILE>`     | Path to configuration file                         | -                       |
| `-r, --repo-dir <PATH>`        | Path to git repository                             | `.` (current directory) |
| `-d, --display-summaries`      | Show commit summaries in output                    | -                       |
| `--add-groups <GROUPS>`        | Include additional commit type groups              | -                       |
| `--remove-groups <GROUPS>`     | Exclude specific commit type groups                | -                       |

##### Examples

Generate a changelog for the current repository:
```bash
gen-changelog generate
```

Generate with a specific next version:
```bash
gen-changelog generate --next-version "2.1.0"
```

Limit to the last 3 releases and show commit summaries:
```bash
gen-changelog generate --sections 3 --display-summaries
```

#### `config` - Configuration Management

Manage configuration settings for gen-changelog.

```bash
gen-changelog config [OPTIONS]
```

##### Options

| Option              | Description                        | Default              |
| ------------------- | ---------------------------------- | -------------------- |
| `-s, --save`        | Save current configuration to file | -                    |
| `-f, --file <FILE>` | Configuration file name            | `gen-changelog.toml` |
| `-p, --show`        | Display current configuration      | -                    |

##### Examples

Show the current configuration:
```bash
gen-changelog config --show
```

Save configuration to the default file:
```bash
gen-changelog config --save
```

Save configuration to a custom file:
```bash
gen-changelog config --save --file my-config.toml
```

### Configuration File

Gen-changelog CLI uses a TOML configuration file to customize its behaviour. The default configuration file is `gen-changelog.toml` in your project root.

To generate a configuration file with default settings and helpful comments:

```bash
gen-changelog config --save
```

### How It Works

1. **Analyses Git History**: Scans your repository's commit messages
2. **Applies Conventional Commits**: Categorizes commits based on conventional commit patterns (feat, fix, chore, etc.)
3. **Groups Changes**: Organizes commits by type and version
4. **Generates Changelog**: Outputs formatted changelog following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) standard

### Conventional Commit Support

gen-changelog recognizes standard conventional commit types:

- **feat**: New features
- **fix**: Bug fixes  
- **docs**: Documentation changes
- **style**: Code style changes
- **refactor**: Code refactoring
- **test**: Test additions or changes
- **chore**: Maintenance tasks

### Logging

Control output verbosity with logging options:

- `-v, --verbose`: Increase verbosity (can be used multiple times: `-vv`, `-vvv`)
- `-q, --quiet`: Decrease verbosity (can be used multiple times: `-qq`, `-qqq`)

### Getting Help

For command-specific help, use:

```bash
gen-changelog <command> --help
```

For general help and available commands:

```bash
gen-changelog --help
```
## License

 Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
 at your option.

## Contribution

 Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
