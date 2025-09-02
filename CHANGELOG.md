<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Fixed

- 🐛 config: correct typo in module import(pr [#18])
- 🐛 change_log: correct changelog section placement(pr [#41])

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
