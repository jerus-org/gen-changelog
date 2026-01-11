<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-01-11

Summary: Chore[16], Continuous Integration[1], Fixed[32]

### Fixed

 - fix(deps): update rust crate uuid to 1.19.0 (#172)
 - fix(deps): update rust crate uuid to 1.19.0
 - fix(deps): update rust crate tempfile to 3.24.0 (#171)
 - fix(deps): update rust crate tempfile to 3.24.0
 - fix(deps): update rust crate lazy-regex to 3.5.1 (#170)
 - fix(deps): update rust crate lazy-regex to 3.5.1
 - fix(deps): update rust crate criterion to 0.8.1 (#169)
 - fix(deps): update rust crate criterion to 0.8.1
 - fix(deps): update rust crate url to 2.5.8 (#166)
 - fix(deps): update rust crate url to 2.5.8
 - fix(deps): update serde packages (#167)
 - fix(deps): update serde packages
 - fix(deps): update rust crate trycmd to 0.15.11 (#165)
 - fix(deps): update rust crate trycmd to 0.15.11
 - fix(deps): update rust crate toml to 0.9.11 (#164)
 - fix(deps): update rust crate toml to 0.9.11
 - fix(deps): update rust crate thiserror to 2.0.17 (#163)
 - fix(deps): update rust crate thiserror to 2.0.17
 - fix(deps): update rust crate snapbox to 0.6.24 (#162)
 - fix(deps): update rust crate snapbox to 0.6.24
 - fix(deps): update rust crate semver to 1.0.27 (#161)
 - fix(deps): update rust crate semver to 1.0.27
 - fix(deps): update rust crate log to 0.4.29 (#160)
 - fix(deps): update rust crate log to 0.4.29
 - fix(deps): update rust crate git2 to 0.20.3 (#159)
 - fix(deps): update rust crate git2 to 0.20.3
 - fix(deps): update rust crate clap to 4.5.54 (#158)
 - fix(deps): update rust crate clap to 4.5.54
 - fix(deps): update rust crate chrono to 0.4.42 (#157)
 - fix(deps): update rust crate chrono to 0.4.42
 - 🐛 fix(change_log): correct trace log placement in section.rs
 - fix(deps): update dependency toolkit to v4

## [0.1.3] - 2025-09-28

Summary: Added[2], Changed[3], Chore[2]

### Added

 - ✨ feat(lib): export default changelog filename
 - ✨ feat(cli): add name option for changelog file

### Changed

 - ♻️ refactor(changelog): update save method to accept filename
 - ♻️ refactor(generate_cli): use constant for changelog filename
 - ♻️ refactor(change_log): enhance changelog file handling

## [0.1.2] - 2025-09-26

Summary: Changed[1], Chore[2]

### Changed

 - ♻️ refactor(generate_cli): optimize package root determination

## [0.1.1] - 2025-09-26

Summary: Added[10], Build[1], Changed[9], Chore[10], Continuous Integration[4], Documentation[4], Fixed[2], Testing[2]

### Added

 - ✨ feat(generate_cli): add no-save option for changelog
 - ✨ feat(change_log): add package root filtering support
 - ✨ feat(change_log): add filter option to commit retrieval
 - ✨ feat(change_log): add package root configuration
 - ✨ feat(error): add NoPackageFound error variant
 - ✨ feat(cli): add package root support in changelog generation
 - ✨ feat(generate_cli): add package retrieval functionality
 - ✨ feat(dependencies): add new development dependencies
 - ✨ feat(cli): add config root option to generatecli
 - ✨ feat(cli): add package option for changelog generation

### Fixed

 - 🐛 fix(generate_cli): add debug logging for package retrieval
 - 🐛 fix(change_log): improve empty section log message

### Changed

 - ♻️ refactor(cli): rename and add options in GenerateCli
 - ♻️ refactor(section): streamline file filtering logic
 - ♻️ refactor(change_log): enhance changelog path handling
 - ♻️ refactor(generate_cli): streamline package root determination
 - ♻️ refactor(change_log): improve filter handling in ChangeLogBuilder
 - ♻️ refactor(section): simplify commit filtering logic
 - ♻️ refactor(generate_cli): streamline changelog generation
 - ♻️ refactor(generate_cli): improve package retrieval error handling
 - ♻️ refactor(cli): extract config creation into separate function

## [0.1.0] - 2025-09-16

Summary: Added[7], Build[2], Changed[7], Chore[28], Continuous Integration[2], Documentation[14], Testing[4]

### Added

 - ✨ feat(assets): add splash screen image
 - ✨ feat(test): add test logger utility
 - ✨ feat(test): add test utilities module
 - ✨ feat(benchmark): add benchmarks for changelog creation and regex
 - ✨ feat(config_cli): add option to print configuration
 - ✨ feat(change_log_config): add optional file parameter for save function
 - ✨ feat(change_log_config): enhance changelog configuration

### Changed

 - ♻️ refactor(change_log_config): enhance DisplaySections enum
 - ♻️ refactor(change_log_config): restrict enum and method visibility
 - ♻️ refactor(change_log): simplify display section matching
 - ♻️ refactor(lib): update change_log_config imports
 - ♻️ refactor(demo_headings): update save method call
 - ♻️ refactor(tests): remove redundant group mappings in config tests
 - ♻️ refactor(change_log): improve groups mapping usage

## [0.0.8] - 2025-09-12

Summary: Added[17], Build[2], Changed[11], Chore[20], Continuous Integration[1], Fixed[2]

### Added

 - ✨ feat(demo): add default config file for changelog
 - ✨ feat(config_cli): add file name option for configuration saving
 - ✨ feat(change_log): add security heading to changelog configuration
 - ✨ feat(cli): add commit group management options
 - ✨ feat(change_log_config): add group management methods
 - ✨ feat(cli): support commit group management
 - ✨ feat(cli): add display summaries option
 - ✨ feat(change_log): add summary flag parameter to section
 - ✨ feat(generate_cli): add display summary option
 - ✨ feat(cli): add repository path option to generate_cli
 - ✨ feat(change_log_config): enhance display sections setting
 - ✨ feat(cli): add generate_cli command for changelog generation
 - ✨ feat(cli): add configuration CLI for changelog settings
 - ✨ feat(cli): add config file option for changelog
 - ✨ feat(change_log_config): enhance from_file method with generics
 - ✨ feat(change_log): add header accessor to section
 - ✨ feat(cli): add option to specify changelog sections

### Fixed

 - 🐛 fix(change_log): handle section limit overflow
 - 🐛 fix(main): enable changelog saving

### Changed

 - ♻️ refactor(config): enhance save method with file parameter
 - ♻️ refactor(config_cli): update changelog config display sections
 - ♻️ refactor(change_log): move update function to ChangeLogBuilder
 - ♻️ refactor(generate_cli): simplify section handling
 - ♻️ refactor(main): streamline CLI structure
 - ♻️ refactor(cli): extract ConfigCli into separate module
 - ♻️ refactor(main): simplify changelog version update logic
 - ♻️ refactor(change_log): improve version update logic
 - ♻️ refactor(change_log_config): modify custom section type
 - ♻️ refactor(main): rename method for better clarity
 - ♻️ refactor(changelog): remove next_version handling

## [0.0.7] - 2025-09-11

Summary: Added[25], Build[2], Changed[11], Chore[22], Continuous Integration[3], Documentation[4], Fixed[2], Testing[1]

### Added

 - ✨ feat(tag): add new constructor for Tag struct
 - ✨ feat(change_log): enhance set_next_version with logging
 - ✨ feat(change_log): add tagging and version setting functionality
 - ✨ feat(logging): add info log for setting unreleased section title
 - ✨ feat(lib): export ChangeLogBuilder
 - ✨ feat(change_log): add next_version field and method
 - ✨ feat(main): add support for setting next version in changelog
 - ✨ feat(change_log): add section limit handling for changelog display
 - ✨ feat(error): add toml deserializer error handling
 - ✨ feat(config): add file-based config initialization
 - ✨ feat(lib): add error module and export error type
 - ✨ feat(error): add custom error handling with thiserror
 - ✨ feat(main): add custom display sections for changelog
 - ✨ feat(lib): export DisplaySections from change_log_config
 - ✨ feat(change_log): add toml serializer error handling
 - ✨ feat(examples): add demo for ChangeLogConfig usage
 - ✨ feat(change_log_config): add heading serialization/deserialization
 - ✨ feat(change_log_config): enhance group struct with serde support
 - ✨ feat(dependencies): add serde and toml support
 - ✨ feat(config): enhance changelog config with serde support
 - ✨ feat(lib): export ChangeLogError for external use
 - ✨ feat(cli): enhance error handling and command options
 - ✨ feat(cli): add subcommand support for configuration
 - ✨ feat(dependencies): add new dependencies for CLI enhancements
 - ✨ feat(cli): integrate clap for command-line parsing

### Fixed

 - 🐛 fix(change_log): handle tag id unwrap safely
 - 🐛 fix(change_log): handle optional tag id in builder

### Changed

 - ♻️ refactor(lib): consolidate pub use statements
 - ♻️ refactor(tag): support optional tag id
 - ♻️ refactor(logging): use trace level for config logging
 - ♻️ refactor(main): enhance changelog configuration handling
 - ♻️ refactor(main): update error type import
 - ♻️ refactor(main): rename error type alias for clarity
 - ♻️ refactor(changelog): simplify error handling
 - ♻️ refactor(changelog): update error handling
 - ♻️ refactor(changelog): simplify error handling in changelog
 - ♻️ refactor(demo_headings): remove unused variable prefix
 - ♻️ refactor(main): improve command parsing and execution

## [0.0.6] - 2025-09-09

Summary: Build[1], Chore[9], Continuous Integration[1], Documentation[2]

## [0.0.5] - 2025-09-09

Summary: Chore[4], Continuous Integration[2]

## [0.0.4] - 2025-09-09

Summary: Chore[5], Continuous Integration[1]

## [0.0.3] - 2025-09-09

Summary: Chore[5], Continuous Integration[1], Documentation[1]

## [0.0.2] - 2025-09-09

Summary: Chore[9], Continuous Integration[7], Documentation[2]

## [0.0.1] - 

Summary: Added[62], Build[6], Changed[59], Chore[96], Continuous Integration[21], Documentation[13], Fixed[12], Testing[1]

### Added

 - ✨ feat(build): add version update functionality
 - ✨ feat(change_log): add metadata support to Header struct
 - ✨ feat(main): add change_log save functionality
 - ✨ feat(change_log): add save method for changelog
 - ✨ feat(change_log): add section header struct
 - ✨ feat(cc_commit): add title_as_string method
 - ✨ feat(change_log): add scope method to ConvCommit
 - ✨ feat(config): add groups mapping for conventional commits
 - ✨ feat(change_log): add summary report for sections
 - ✨ feat(config): make heading_mgmt module public
 - ✨ feat(main): integrate configurable changelog groups
 - ✨ feat(config): add debug logging for group and heading states
 - ✨ feat(change_log): enhance section initialization
 - ✨ feat(config): add unset_to_publish method to GroupMgmt
 - ✨ feat(config): add unpublish_group method for changelog
 - ✨ feat(config): add remove_heading method to HeadingMgmt
 - ✨ feat(config): add group management functionality
 - ✨ feat(group_mgmt): add set_to_publish method
 - ✨ feat(config): add publish flag management to Group
 - ✨ feat(config): implement heading management feature
 - ✨ feat(group): introduce Group struct in group management
 - ✨ feat(config): add insert_cc_types method
 - ✨ feat(config): add GroupBuilder for flexible group creation
 - ✨ feat(change_log): enhance section display formatting
 - ✨ feat(change_log): implement Display trait for ChangeLogClass
 - ✨ feat(change_log): enhance release-to-release walking
 - ✨ feat(change_log): enable commit retrieval and status logging
 - ✨ feat(change_log): implement version tag processing
 - ✨ feat(change_log): add repository walk functionality
 - ✨ feat(change_log): enhance ChangeLogBuilder with repository handling
 - ✨ feat(header): add new constructor for Header struct
 - ✨ feat(cc_commit): add clone trait to ConvCommit struct
 - ✨ feat(change_log): implement Display trait for Section
 - ✨ feat(link): add clone trait to link struct
 - ✨ feat(tag): introduce tag builder pattern
 - ✨ feat(change_log): enhance tag processing with version and date
 - ✨ feat(config): add release_pattern accessor
 - ✨ feat(config): add release pattern configuration
 - ✨ feat(config): add display sections configuration
 - ✨ feat(lib): add config module
 - ✨ feat(change_log): add config support to ChangeLog
 - ✨ feat(config): add group management trait
 - ✨ feat(config): add group struct for changelog organization
 - ✨ feat(config): add new configuration settings for change log
 - ✨ feat(change_log): add link module for URL handling
 - ✨ feat(changelog): add header struct for changelog format
 - ✨ feat(logging): add logging functionality to main
 - ✨ feat(change_log): enhance ChangeLog with remote details extraction
 - ✨ feat(changelog): add conventional commit parser
 - ✨ feat(change_log): add MarkdownLink struct
 - ✨ feat(change_log): add changelog classification enum
 - ✨ feat(tag): add initial tag struct for semantic versioning
 - ✨ feat(change_log): add section struct for changelog management
 - ✨ feat(main): integrate changelog generation
 - ✨ feat(lib): add change log module
 - ✨ feat(changelog): add initial changelog module
 - ✨ feat(library): add initial library setup
 - ✨ feat(build): enhance README generation process
 - ✨ feat(vscode): add custom dictionary for ltex
 - ✨ feat(build): introduce build script for README assembly
 - ✨ feat(main): add main function with hello world
 - ✨ feat(build): add Cargo.toml for gen-changelog project

### Fixed

 - 🐛 fix(config): correct syntax error in group.rs
 - 🐛 fix(change_log): handle missing commit kind
 - 🐛 fix(change_log): correct changelog display formatting
 - 🐛 fix(change_log): ensure headings are cloned for section creation
 - 🐛 fix(config): correct heading index initialization
 - 🐛 fix(change_log): correct changelog section placement
 - 🐛 fix(tests): correct test function name typo
 - 🐛 fix(change_log): ensure section is added to sections list
 - 🐛 fix(section): improve logging messages for walk setup
 - 🐛 fix(config): correct typo in module import
 - 🐛 fix(link): correct link format in display implementation
 - 🐛 fix(main): handle changelog initialization error

### Changed

 - ♻️ refactor(config): rename config to changelogconfig
 - ♻️ refactor(build): remove backup functionality in README generation
 - ♻️ refactor(build): improve backup file handling
 - ♻️ refactor(main): clean up log and config usage
 - ♻️ refactor(section): separate conventional and non-conventional commit handling
 - ♻️ refactor(section): improve group mapping retrieval
 - ♻️ refactor(section): reorganize imports for clarity
 - ♻️ refactor(change_log): improve error handling and logging
 - ♻️ refactor(change_log): improve section header initialization
 - ♻️ refactor(section_header): change trait implementation for section header
 - ♻️ refactor(section): replace link with header in section
 - ♻️ refactor(section): simplify section struct in changelog
 - ♻️ refactor(change_log): enhance section struct with link
 - ♻️ refactor(change_log): enhance section struct with link support
 - ♻️ refactor(change_log): improve commit formatting in section
 - ♻️ refactor(section): enhance commit grouping logic
 - ♻️ refactor(changelog): update section initialization
 - ♻️ refactor(cc_commit): remove unused title_as_string method
 - ♻️ refactor(section): simplify commit formatting logic
 - ♻️ refactor(change_log): remove print method from ChangeLogBuilder
 - ♻️ refactor(change_log): improve section header handling
 - ♻️ refactor(cc_commit): modify kind_string method signature
 - ♻️ refactor(change_log): simplify commit classification
 - ♻️ refactor(config): simplify headings initialization
 - ♻️ refactor(change_log): replace HashMap with BTreeMap in section
 - ♻️ refactor(section): simplify section markdown generation
 - ♻️ refactor(main): remove unused publish groups
 - ♻️ refactor(change_log): streamline commit reporting logic
 - ♻️ refactor(heading_mgmt): update trait and implementation for clarity
 - ♻️ refactor(section): modify section structure and initialization
 - ♻️ refactor(changelog): enhance section headings management
 - ♻️ refactor(config): rename method for clarity
 - ♻️ refactor(config): rename add_group to add_heading
 - ♻️ refactor(change_log): remove unused constant
 - ♻️ refactor(change_log): optimize module imports
 - ♻️ refactor(config): enhance group and heading management
 - ♻️ refactor(config): update config structure and naming
 - ♻️ refactor(config): update visibility of structs and methods
 - ♻️ refactor(group): implement typed builder pattern
 - ♻️ refactor(config): organize modules in config.rs
 - ♻️ refactor(section): enhance commit grouping by class
 - ♻️ refactor(section): enhance report_status method
 - ♻️ refactor(change_log): improve walk setup and commit retrieval
 - ♻️ refactor(change_log): update walk setup naming
 - ♻️ refactor(change_log): simplify tag handling and section creation
 - ♻️ refactor(main): enhance changelog builder initialization
 - ♻️ refactor(header): derive clone for header struct
 - ♻️ refactor(main): enhance changelog creation and output
 - ♻️ refactor(change_log): restructure ChangeLog and builder pattern
 - ♻️ refactor(tag): simplify semver setting process
 - ♻️ refactor(tag): improve semver extraction logic
 - ♻️ refactor(change_log): update link and footer handling
 - ♻️ refactor(change_log): use Header struct for header management
 - ♻️ refactor(main): improve repository handling in changelog generation
 - ♻️ refactor(change_log): improve ChangeLog struct design
 - ♻️ refactor(change_log): replace println with log macros
 - ♻️ refactor(build): reorder rerun-if-changed println calls
 - ♻️ refactor(section): enhance markdown generation
 - ♻️ refactor(section): simplify tag handling logic

## [0.0.0] - 2025-08-22

Summary: 

[Unreleased]: https://github.com/jerus-org/gen-changelog/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/jerus-org/gen-changelog/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/gen-changelog/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/gen-changelog/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerus-org/gen-changelog/compare/v0.0.8...v0.1.0
[0.0.8]: https://github.com/jerus-org/gen-changelog/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/jerus-org/gen-changelog/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/jerus-org/gen-changelog/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/jerus-org/gen-changelog/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/jerus-org/gen-changelog/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/jerus-org/gen-changelog/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jerus-org/gen-changelog/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jerus-org/gen-changelog/compare/v0.0.0...v0.0.1
[0.0.0]: https://github.com/jerus-org/gen-changelog/releases/tag/v0.0.0

