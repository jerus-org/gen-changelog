<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Summary: Added[61], Build[5], Changed[59], Chore[79], Continuous Integration[10], Documentation[10], Fixed[12], Testing[1]

### Added

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
 - ✨ feat(config): add release_pattern accessor
 - ✨ feat(change_log): enhance tag processing with version and date
 - ✨ feat(config): add release pattern configuration
 - ✨ feat(config): add display sections configuration
 - ✨ feat(lib): add config module
 - ✨ feat(config): add group management trait
 - ✨ feat(config): add group struct for changelog organization
 - ✨ feat(config): add new configuration settings for change log
 - ✨ feat(change_log): add config support to ChangeLog
 - ✨ feat(change_log): add link module for URL handling
 - ✨ feat(changelog): add header struct for changelog format
 - ✨ feat(logging): add logging functionality to main
 - ✨ feat(change_log): enhance ChangeLog with remote details extraction
 - ✨ feat(changelog): add initial changelog module
 - ✨ feat(changelog): add conventional commit parser
 - ✨ feat(change_log): add changelog classification enum
 - ✨ feat(change_log): add MarkdownLink struct
 - ✨ feat(lib): add change log module
 - ✨ feat(main): integrate changelog generation
 - ✨ feat(tag): add initial tag struct for semantic versioning
 - ✨ feat(change_log): add section struct for changelog management
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
 - ♻️ refactor(change_log): improve error handling and logging
 - ♻️ refactor(change_log): improve section header initialization
 - ♻️ refactor(section_header): change trait implementation for section header
 - ♻️ refactor(section): replace link with header in section
 - ♻️ refactor(section): simplify section struct in changelog
 - ♻️ refactor(change_log): enhance section struct with link
 - ♻️ refactor(change_log): enhance section struct with link support
 - ♻️ refactor(section): separate conventional and non-conventional commit handling
 - ♻️ refactor(section): improve group mapping retrieval
 - ♻️ refactor(section): reorganize imports for clarity
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
 - ♻️ refactor(main): enhance changelog creation and output
 - ♻️ refactor(change_log): restructure ChangeLog and builder pattern
 - ♻️ refactor(header): derive clone for header struct
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

[Unreleased]: https://github.com/jerus-org/gen-changelog/commits/main/

