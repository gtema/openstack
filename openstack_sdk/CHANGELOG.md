# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.1.1...openstack_sdk-v0.2.0) - 2024-02-23

### Added
- split out scope into separate module
- *(auth)* Add support for auth with application credentials
- add func tests verifying connection
- further modularilze sdk
- split OpenStack client module

### Fixed
- *(sdk)* flatten _properties of extensible structs
- respect endpoint_override for missing catalog entry
- Respect server defaults for pagination

### Other
- switch to caret requirements syntax
- further improve comments
- better pre-format comments
- further modularize auth plugins
- Merge pull request [#55](https://github.com/gtema/openstack/pull/55) from gtema/dependabot/cargo/derive_builder-0.20.0

## [0.1.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.1.0...openstack_sdk-v0.1.1) - 2024-02-16

### Added
- *(docs)* Improve documents structure
- *(auth)* Enable WebSSO authentication

### Fixed
- ensure auth cache is enabled by default

### Other
- Revert "chore: release"
- release
- minor reorg in cargo manifests
