## Gen-changelog CLI

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