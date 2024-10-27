# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.5...openstack_tui-v0.1.6) - 2024-10-27

### Fixed

- Fix "down" button on last entry ([#671](https://github.com/gtema/openstack/pull/671))

### Other

- Update ratatui version ([#670](https://github.com/gtema/openstack/pull/670))

## [0.1.5](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.4...openstack_tui-v0.1.5) - 2024-10-11

### Other

- update Cargo.lock dependencies

## [0.1.4](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.3...openstack_tui-v0.1.4) - 2024-10-04

### Other

- *(deps)* Bump unicode-width from 0.1.14 to 0.2.0 ([#615](https://github.com/gtema/openstack/pull/615))
- Drop better/human-panic ([#607](https://github.com/gtema/openstack/pull/607))

## [0.1.3](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.2...openstack_tui-v0.1.3) - 2024-09-28

### Added

- Enable new fuzzy_select component ([#578](https://github.com/gtema/openstack/pull/578))
- Reorganize cloud_worker better ([#577](https://github.com/gtema/openstack/pull/577))
- Add identity.projects view ([#576](https://github.com/gtema/openstack/pull/576))
- Rework doc building ([#568](https://github.com/gtema/openstack/pull/568))

### Fixed

- Repair go back from console output ([#589](https://github.com/gtema/openstack/pull/589))
- Disable pagination limits ([#587](https://github.com/gtema/openstack/pull/587))

## [0.1.2](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.1...openstack_tui-v0.1.2) - 2024-09-20

### Added

- Add all_tenants filter in servers view ([#566](https://github.com/gtema/openstack/pull/566))
- Add project selection ([#556](https://github.com/gtema/openstack/pull/556))
- Separate service methods into traits ([#554](https://github.com/gtema/openstack/pull/554))
- Add initial tui documentation ([#544](https://github.com/gtema/openstack/pull/544))
- Add styling support ([#543](https://github.com/gtema/openstack/pull/543))
- Support "ESC" button to exit from describe ([#541](https://github.com/gtema/openstack/pull/541))

### Fixed

- Process row only when selected ([#565](https://github.com/gtema/openstack/pull/565))
- Update usage view on auth change ([#564](https://github.com/gtema/openstack/pull/564))

## [0.1.1](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.0...openstack_tui-v0.1.1) - 2024-09-13

### Added

- *(tui)* De-duplicate describe view ([#530](https://github.com/gtema/openstack/pull/530))
- *(tui)* Add action to show server console output ([#529](https://github.com/gtema/openstack/pull/529))
- *(tui)* Add scrolling in the resource describe ([#528](https://github.com/gtema/openstack/pull/528))
- *(tui)* Allow starting tui without selected cloud ([#527](https://github.com/gtema/openstack/pull/527))
- Drop vergen from ostui ([#512](https://github.com/gtema/openstack/pull/512))

### Fixed

- Correct autoreplaced `typos` names ([#485](https://github.com/gtema/openstack/pull/485))

### Other

- Update dependencies ([#499](https://github.com/gtema/openstack/pull/499))

## [0.1.0](https://github.com/gtema/openstack/releases/tag/openstack_tui-v0.1.0) - 2024-08-23

### Added
- Start Terminal User Interface project ([#470](https://github.com/gtema/openstack/pull/470))

### Fixed
- Another attempt to repair release of openstack_tui ([#484](https://github.com/gtema/openstack/pull/484))
- Fix changelog of the openstack_tui ([#481](https://github.com/gtema/openstack/pull/481))
- Fix build for openstack_tui ([#479](https://github.com/gtema/openstack/pull/479))

### Other
- release ([#480](https://github.com/gtema/openstack/pull/480))
- Drop release entry for openstack_tui ([#482](https://github.com/gtema/openstack/pull/482))
- release ([#477](https://github.com/gtema/openstack/pull/477))
- release ([#475](https://github.com/gtema/openstack/pull/475))
- *(ci)* Undo release-plz per package ([#476](https://github.com/gtema/openstack/pull/476))

### Added
- Start Terminal User Interface project ([#470](https://github.com/gtema/openstack/pull/470))

### Fixed
- Fix build for openstack_tui ([#479](https://github.com/gtema/openstack/pull/479))
