<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- ✨ integrate clap for command-line parsing(pr [#88])
- ✨ add subcommand support for configuration(pr [#89])
- ✨ enhance error handling and command options(pr [#91])
- ✨ enhance changelog config with serde support(pr [#93])
- ✨ export DisplaySections from change_log_config(pr [#94])
- ✨ add custom error handling with thiserror(pr [#95])
- ✨ add file-based config initialization(pr [#97])
- ✨ add section limit handling for changelog display(pr [#98])
- ✨ add support for setting next version in changelog(pr [#99])
- ✨ add tagging and version setting functionality(pr [#102])

### Changed

- 📝 docs(readme)-update readme with new splash image and features(pr [#90])
- ♻️ refactor(main)-improve command parsing and execution(pr [#92])
- ♻️ refactor(main)-update error type import(pr [#96])
- ♻️ refactor(logging)-use trace level for config logging(pr [#100])
- ♻️ refactor(tag)-support optional tag id(pr [#101])
- ♻️ refactor(lib)-apply comprehensive rust formatting update(pr [#103])

## [0.0.6] - 2025-09-09

### Changed

- Bump-version-to-0.0.5(pr [#85])
- 🔧 chore(Cargo)-include changelog in package distribution(pr [#86])
- 👷 ci(circleci)-update release command configuration(pr [#87])

## [0.0.5] - 2025-09-09

### Changed

- 👷 ci(circleci)-enhance release command with changelog generation(pr [#83])
- 👷 ci(circleci)-fix command syntax in config.yml(pr [#84])

## [0.0.4] - 2025-09-09

### Changed

- 🔧 chore(scripts)-add release hook script for automation(pr [#82])

## [0.0.3] - 2025-09-09

### Changed

- 👷 ci(circleci)-add gen-changelog installation step(pr [#81])

## [0.0.2] - 2025-09-09

### Changed

- 💄 style(Cargo)-reorder authors in Cargo.toml(pr [#74])
- 👷 ci(circleci)-update cargo release and changelog process(pr [#75])
- 👷 ci(circleci)-add bump version variable to cargo release command(pr [#76])
- 👷 ci(circleci)-simplify pipefail setting in commands(pr [#77])
- 👷 ci(circleci)-fix variable casing in release command(pr [#78])
- 👷 ci(circleci)-enhance cargo release commands with verbosity(pr [#79])
- 🔧 chore(release)-update pre-release replacements in release.toml(pr [#80])

## [0.0.1] - 2025-09-08

### Added

- ✨ add initial changelog module(pr [#8])
- ✨ enhance ChangeLog with remote details extraction(pr [#9])
- ✨ add logging functionality to main(pr [#10])
- ✨ add header struct for changelog format(pr [#14])
- ✨ add link module for URL handling(pr [#15])
- ✨ add new configuration settings for change log(pr [#17])
- ✨ add display sections configuration(pr [#19])
- ✨ add release pattern configuration(pr [#20])
- ✨ add release_pattern accessor(pr [#21])
- ✨ add new constructor for Header struct(pr [#24])
- ✨ enhance ChangeLogBuilder with repository handling(pr [#25])
- ✨ add repository walk functionality(pr [#26])
- ✨ implement version tag processing(pr [#27])
- ✨ enable commit retrieval and status logging(pr [#29])
- ✨ enhance release-to-release walking(pr [#30])
- ✨ enhance section display formatting(pr [#32])
- ✨ add GroupBuilder for flexible group creation(pr [#33])
- ✨ introduce Group struct in group management(pr [#35])
- ✨ implement heading management feature(pr [#36])
- ✨ add group management functionality(pr [#38])
- ✨ add remove_heading method to HeadingMgmt(pr [#39])
- ✨ add unpublish_group method for changelog(pr [#40])
- ✨ add groups mapping for conventional commits(pr [#52])
- ✨ add save method for changelog(pr [#55])

### Changed

- 👷 ci(build)-add rerun-if-changed directives for documentation(pr [#3])
- 📦 build(Cargo)-update dependencies in Cargo.toml(pr [#4])
- 🔧 chore(build)-add build script execution log(pr [#5])
- 🔧 chore(scripts)-add justfile for task automation(pr [#6])
- 📝 docs(CHANGELOG)-add LTex comment to disable spell checking(pr [#7])
- ♻️ refactor(build)-reorder rerun-if-changed println calls(pr [#11])
- ♻️ refactor(change_log)-replace println with log macros(pr [#12])
- ♻️ refactor(change_log)-improve ChangeLog struct design(pr [#13])
- 💄 style(change_log)-reorder imports for readability(pr [#16])
- ♻️ refactor(tag)-improve semver extraction logic(pr [#22])
- ♻️ refactor(change_log)-restructure ChangeLog and builder pattern(pr [#23])
- ♻️ refactor(change_log)-update walk setup naming(pr [#28])
- ♻️ refactor(section)-enhance commit grouping by class(pr [#31])
- ♻️ refactor(group)-implement typed builder pattern(pr [#34])
- ♻️ refactor(config)-enhance group and heading management(pr [#37])
- ♻️ refactor(changelog)-enhance section headings management(pr [#42])
- ♻️ refactor(section)-modify section structure and initialization(pr [#44])
- ♻️ refactor(change_log)-streamline commit reporting logic(pr [#45])
- ♻️ refactor(change_log)-simplify commit classification(pr [#46])
- ♻️ refactor(cc_commit)-change return type of kind_string method(pr [#47])
- Add-update_pcu-parameter-to-config(pr [#48])
- ♻️ refactor(change_log)-improve section header handling(pr [#49])
- ♻️ refactor(cc_commit)-remove unused title_as_string function(pr [#50])
- ♻️ refactor(changelog)-update section initialization(pr [#53])
- ♻️ refactor(change_log)-enhance section struct with link support(pr [#54])
- ♻️ refactor(main)-clean up log and config usage(pr [#56])
- 🔧 chore(config)-add library and binary definitions to Cargo.toml(pr [#57])
- 👷 ci(circleci)-add rust environment and update changelog job(pr [#58])
- 🔧 chore-prepare for release(pr [#59])
- ♻️ refactor(config)-rename config to changelogconfig(pr [#60])
- update library documentation with usage instructions(pr [#61])
- 👷 ci(circleci)-remove redundant SEMVER setting logic(pr [#62])
- 👷 ci(circleci)-update shell options in echo command(pr [#63])
- 👷 ci(config)-update CircleCI config for enhanced release management(pr [#64])
- 👷 ci(circleci)-add branch filter for update_logs job(pr [#65])
- 👷 ci(circleci)-fix case statement for version bump(pr [#66])
- ci-add install smart release to release jobs(pr [#67])
- Update-cargo-release(pr [#68])
- 👷 ci(circleci)-update CircleCI config for improved build process(pr [#69])
- 👷 ci(circleci)-integrate GitHub CLI using official orb(pr [#70])
- 🔧 chore(ci)-remove unused GitHub CLI orb from CircleCI config(pr [#71])
- 👷 ci(circleci)-remove no_push option from cargo release steps(pr [#72])
- Remove-unused-build-script(pr [#73])
- 💄 style(Cargo)-reorder authors in Cargo.toml(pr [#74])
- Add-release-configuration-file-to-switch-to-cargo-release(pr [#75])

### Fixed

- 🐛 config: correct typo in module import(pr [#18])
- 🐛 change_log: correct changelog section placement(pr [#41])
- 🐛 config: correct heading index initialization(pr [#43])

[#3]: https://github.com/jerus-org/gen-changelog/pull/3
[#4]: https://github.com/jerus-org/gen-changelog/pull/4
[#5]: https://github.com/jerus-org/gen-changelog/pull/5
[#6]: https://github.com/jerus-org/gen-changelog/pull/6
[#7]: https://github.com/jerus-org/gen-changelog/pull/7
[#8]: https://github.com/jerus-org/gen-changelog/pull/8
[#9]: https://github.com/jerus-org/gen-changelog/pull/9
[#10]: https://github.com/jerus-org/gen-changelog/pull/10
[#11]: https://github.com/jerus-org/gen-changelog/pull/11
[#12]: https://github.com/jerus-org/gen-changelog/pull/12
[#13]: https://github.com/jerus-org/gen-changelog/pull/13
[#14]: https://github.com/jerus-org/gen-changelog/pull/14
[#15]: https://github.com/jerus-org/gen-changelog/pull/15
[#16]: https://github.com/jerus-org/gen-changelog/pull/16
[#17]: https://github.com/jerus-org/gen-changelog/pull/17
[#18]: https://github.com/jerus-org/gen-changelog/pull/18
[#19]: https://github.com/jerus-org/gen-changelog/pull/19
[#20]: https://github.com/jerus-org/gen-changelog/pull/20
[#21]: https://github.com/jerus-org/gen-changelog/pull/21
[#22]: https://github.com/jerus-org/gen-changelog/pull/22
[#23]: https://github.com/jerus-org/gen-changelog/pull/23
[#24]: https://github.com/jerus-org/gen-changelog/pull/24
[#25]: https://github.com/jerus-org/gen-changelog/pull/25
[#26]: https://github.com/jerus-org/gen-changelog/pull/26
[#27]: https://github.com/jerus-org/gen-changelog/pull/27
[#28]: https://github.com/jerus-org/gen-changelog/pull/28
[#29]: https://github.com/jerus-org/gen-changelog/pull/29
[#30]: https://github.com/jerus-org/gen-changelog/pull/30
[#31]: https://github.com/jerus-org/gen-changelog/pull/31
[#32]: https://github.com/jerus-org/gen-changelog/pull/32
[#33]: https://github.com/jerus-org/gen-changelog/pull/33
[#34]: https://github.com/jerus-org/gen-changelog/pull/34
[#35]: https://github.com/jerus-org/gen-changelog/pull/35
[#36]: https://github.com/jerus-org/gen-changelog/pull/36
[#37]: https://github.com/jerus-org/gen-changelog/pull/37
[#38]: https://github.com/jerus-org/gen-changelog/pull/38
[#39]: https://github.com/jerus-org/gen-changelog/pull/39
[#40]: https://github.com/jerus-org/gen-changelog/pull/40
[#41]: https://github.com/jerus-org/gen-changelog/pull/41
[#42]: https://github.com/jerus-org/gen-changelog/pull/42
[#43]: https://github.com/jerus-org/gen-changelog/pull/43
[#44]: https://github.com/jerus-org/gen-changelog/pull/44
[#45]: https://github.com/jerus-org/gen-changelog/pull/45
[#46]: https://github.com/jerus-org/gen-changelog/pull/46
[#47]: https://github.com/jerus-org/gen-changelog/pull/47
[#48]: https://github.com/jerus-org/gen-changelog/pull/48
[#49]: https://github.com/jerus-org/gen-changelog/pull/49
[#50]: https://github.com/jerus-org/gen-changelog/pull/50
[#52]: https://github.com/jerus-org/gen-changelog/pull/52
[#53]: https://github.com/jerus-org/gen-changelog/pull/53
[#54]: https://github.com/jerus-org/gen-changelog/pull/54
[#55]: https://github.com/jerus-org/gen-changelog/pull/55
[#56]: https://github.com/jerus-org/gen-changelog/pull/56
[#57]: https://github.com/jerus-org/gen-changelog/pull/57
[#58]: https://github.com/jerus-org/gen-changelog/pull/58
[#59]: https://github.com/jerus-org/gen-changelog/pull/59
[#60]: https://github.com/jerus-org/gen-changelog/pull/60
[#61]: https://github.com/jerus-org/gen-changelog/pull/61
[#62]: https://github.com/jerus-org/gen-changelog/pull/62
[#63]: https://github.com/jerus-org/gen-changelog/pull/63
[#64]: https://github.com/jerus-org/gen-changelog/pull/64
[#65]: https://github.com/jerus-org/gen-changelog/pull/65
[#66]: https://github.com/jerus-org/gen-changelog/pull/66
[#67]: https://github.com/jerus-org/gen-changelog/pull/67
[#68]: https://github.com/jerus-org/gen-changelog/pull/68
[#69]: https://github.com/jerus-org/gen-changelog/pull/69
[#70]: https://github.com/jerus-org/gen-changelog/pull/70
[#71]: https://github.com/jerus-org/gen-changelog/pull/71
[#72]: https://github.com/jerus-org/gen-changelog/pull/72
[#73]: https://github.com/jerus-org/gen-changelog/pull/73
[#74]: https://github.com/jerus-org/gen-changelog/pull/74
[#75]: https://github.com/jerus-org/gen-changelog/pull/75
[#75]: https://github.com/jerus-org/gen-changelog/pull/75
[#76]: https://github.com/jerus-org/gen-changelog/pull/76
[#77]: https://github.com/jerus-org/gen-changelog/pull/77
[#78]: https://github.com/jerus-org/gen-changelog/pull/78
[#79]: https://github.com/jerus-org/gen-changelog/pull/79
[#80]: https://github.com/jerus-org/gen-changelog/pull/80
[#81]: https://github.com/jerus-org/gen-changelog/pull/81
[#82]: https://github.com/jerus-org/gen-changelog/pull/82
[#83]: https://github.com/jerus-org/gen-changelog/pull/83
[#84]: https://github.com/jerus-org/gen-changelog/pull/84
[#85]: https://github.com/jerus-org/gen-changelog/pull/85
[#86]: https://github.com/jerus-org/gen-changelog/pull/86
[#87]: https://github.com/jerus-org/gen-changelog/pull/87
[#88]: https://github.com/jerus-org/gen-changelog/pull/88
[#89]: https://github.com/jerus-org/gen-changelog/pull/89
[#90]: https://github.com/jerus-org/gen-changelog/pull/90
[#91]: https://github.com/jerus-org/gen-changelog/pull/91
[#92]: https://github.com/jerus-org/gen-changelog/pull/92
[#93]: https://github.com/jerus-org/gen-changelog/pull/93
[#94]: https://github.com/jerus-org/gen-changelog/pull/94
[#95]: https://github.com/jerus-org/gen-changelog/pull/95
[#96]: https://github.com/jerus-org/gen-changelog/pull/96
[#97]: https://github.com/jerus-org/gen-changelog/pull/97
[#98]: https://github.com/jerus-org/gen-changelog/pull/98
[#99]: https://github.com/jerus-org/gen-changelog/pull/99
[#100]: https://github.com/jerus-org/gen-changelog/pull/100
[#101]: https://github.com/jerus-org/gen-changelog/pull/101
[#102]: https://github.com/jerus-org/gen-changelog/pull/102
[#103]: https://github.com/jerus-org/gen-changelog/pull/103
[Unreleased]: https://github.com/jerus-org/gen-changelog/compare/v0.0.6...HEAD
[0.0.6]: https://github.com/jerus-org/gen-changelog/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/jerus-org/gen-changelog/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/jerus-org/gen-changelog/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/jerus-org/gen-changelog/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jerus-org/gen-changelog/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jerus-org/gen-changelog/releases/tag/v0.0.1
