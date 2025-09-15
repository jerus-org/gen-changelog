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

### Core Components

#### ChangeLog

The main struct for generating and managing changelog documents.

##### Creating a ChangeLog

Use the builder pattern to create a changelog:

```rust
use gen_changelog::{ChangeLog, ChangeLogConfig};
use git2::Repository;

let repo = Repository::open(".")?;
let config = ChangeLogConfig::from_file_or_default()?;

let changelog = ChangeLog::builder()
    .with_config(config)
    .with_header("My Project Changelog", &[
        "All notable changes to this project will be documented in this file."
    ])
    .with_repository(&repo)?
    .build();
```

##### Methods

- `builder()` - Creates a new `ChangeLogBuilder` instance
- `save()` - Writes the changelog to `CHANGELOG.md` in the current directory

#### ChangeLogBuilder

Builder struct for constructing `ChangeLog` instances with custom configurations.

##### Configuration Methods

- `with_config(config: ChangeLogConfig)` - Sets a custom configuration
- `with_header(title: &str, paragraphs: &[&str])` - Sets the changelog header
- `with_summary_flag(value: bool)` - Enables/disables commit summaries
- `with_repository(repository: &Repository)` - Processes Git repository for changes
- `update_unreleased_to_next_version(next_version: Option<&String>)` - Updates unreleased section to specific version
- `build()` - Constructs the final `ChangeLog` instance

#### ChangeLogConfig

Configuration struct that controls how the changelog is generated and formatted.

##### Creating Configuration

```rust
use gen_changelog::ChangeLogConfig;

// Load from default config file or use defaults
let config = ChangeLogConfig::from_file_or_default()?;

// Load from specific file
let config = ChangeLogConfig::from_file("my-config.toml")?;

// Use default configuration
let config = ChangeLogConfig::default();
```

##### Configuration Methods

###### Group Management
- `add_commit_groups(groups: &[String])` - Adds groups to be published
- `remove_commit_groups(groups: &[String])` - Removes groups from publishing
- `publish_group(group_name: &str)` - Adds a specific group to the published headings
- `unpublish_group(group_name: &str)` - Removes a specific group from the published headings
- `add_group(group: Group)` - Adds a new group definition

###### Display Configuration
- `set_display_sections(value: Option<u8>)` - Sets number of changelog sections to display
- `display_sections()` - Gets current display sections setting
- `release_pattern()` - Gets the pattern used to identify release tags

###### Data Access
- `headings()` - Returns ordered list of headings to publish
- `groups_mapping()` - Returns mapping of conventional commit types to groups

###### Persistence
- `save(file: Option<&str>)` - Saves configuration to file or prints to stdout

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

### Error Handling

The library uses a custom `Error` type for error handling. Common error scenarios include:

- Configuration file parsing errors
- Git repository access issues
- File I/O operations
- Invalid remote URL patterns

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
        .with_repository(&repo)?
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
        .with_repository(&repo)?
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
        .with_repository(&repo)?
        .update_unreleased_to_next_version(Some(&version.to_string()))
        .build();
    
    changelog.save()?;
    Ok(())
}
```

### Requirements

- Git repository with conventional commit messages
- GitHub repository for generating comparison links

The library automatically detects GitHub repositories and generates appropriate comparison and release links in the changelog output.