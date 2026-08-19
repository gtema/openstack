# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.22.8](https://github.com/gtema/openstack/compare/openstack-types-compute-v0.22.7...openstack-types-compute-v0.22.8) - 2026-08-19

### Added

- *(rust)* Emit BODY_SCHEMA const on request structs ([#1935](https://github.com/gtema/openstack/pull/1935))
- Implement "waitable" interface for rust ([#1929](https://github.com/gtema/openstack/pull/1929))
- *(tui)* Remove dynamic_item from generated code ([#1919](https://github.com/gtema/openstack/pull/1919))
- Source ColumnSpec from config.yaml ([#1915](https://github.com/gtema/openstack/pull/1915))
- Dispatch response schema per negotiated microversion ([#1907](https://github.com/gtema/openstack/pull/1907))
- Generate mode/action/app wiring per resource ([#1897](https://github.com/gtema/openstack/pull/1897))
- Start populating max_version for sdk ([#1883](https://github.com/gtema/openstack/pull/1883))
- *(sdk)* Start populating max_ver and min_ver ([#1881](https://github.com/gtema/openstack/pull/1881))
- Changes from adapted codegenerator templates ([#1879](https://github.com/gtema/openstack/pull/1879))
- Adapt codegen to use min ver suffix ([#1870](https://github.com/gtema/openstack/pull/1870))

### Fixed

- Codegenerator fixes ([#1884](https://github.com/gtema/openstack/pull/1884))

### Other

- Propagate ConfirmableRequest through enums ([#1943](https://github.com/gtema/openstack/pull/1943))

## [0.22.7](https://github.com/gtema/openstack/compare/openstack-types-compute-v0.1.0...openstack-types-compute-v0.22.7) - 2026-06-01

### Other

- *(ci)* Changes necessary for making initial release ([#1768](https://github.com/gtema/openstack/pull/1768))

## [0.1.0](https://github.com/gtema/openstack/releases/tag/openstack-types-compute-v0.1.0) - 2026-05-14

### Added

- BS.host.show is a "list" operation ([#1759](https://github.com/gtema/openstack/pull/1759))
- Start respecting MV response candidates ([#1744](https://github.com/gtema/openstack/pull/1744))

### Other

- Render types to separate crates ([#1716](https://github.com/gtema/openstack/pull/1716))
- Introduce dedicated types crates ([#1715](https://github.com/gtema/openstack/pull/1715))
