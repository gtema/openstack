# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.13.8](https://github.com/gtema/openstack/compare/openstack-cli-compute-v0.13.7...openstack-cli-compute-v0.13.8) - 2026-08-19

### Added

- Implement "waitable" interface for rust ([#1929](https://github.com/gtema/openstack/pull/1929))
- Dispatch response schema per negotiated microversion ([#1907](https://github.com/gtema/openstack/pull/1907))
- Start populating max_version for sdk ([#1883](https://github.com/gtema/openstack/pull/1883))
- *(sdk)* Start populating max_ver and min_ver ([#1881](https://github.com/gtema/openstack/pull/1881))
- Changes from adapted codegenerator templates ([#1879](https://github.com/gtema/openstack/pull/1879))
- Adapt codegen to use min ver suffix ([#1870](https://github.com/gtema/openstack/pull/1870))
- *(cli)* Add service command version dispatch ([#1848](https://github.com/gtema/openstack/pull/1848))
- *(compute)* Add `server show-plaintext` ([#1817](https://github.com/gtema/openstack/pull/1817))

### Fixed

- Dispatch response schema by resolved microversion ([#1945](https://github.com/gtema/openstack/pull/1945))
- *(cli)* Group kind-suffix response candidates by microversion ([#1944](https://github.com/gtema/openstack/pull/1944))
- Codegenerator fixes ([#1884](https://github.com/gtema/openstack/pull/1884))

### Other

- Stop use deprecated RawQuery/RawQueryAsync ([#1889](https://github.com/gtema/openstack/pull/1889))
- Bump dependencies ([#1857](https://github.com/gtema/openstack/pull/1857))

## [0.1.0](https://github.com/gtema/openstack/releases/tag/openstack-cli-compute-v0.1.0) - 2026-05-14

### Other

- Combine cli-core and cli-cmd into one folder ([#1761](https://github.com/gtema/openstack/pull/1761))
