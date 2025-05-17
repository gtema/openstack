# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.1](https://github.com/gtema/openstack/compare/openstack_types-v0.21.0...openstack_types-v0.21.1) - 2025-05-17

### Added

- Add cascade QP for delete octavia loadbalancer ([#1204](https://github.com/gtema/openstack/pull/1204))
- Include openapi spec into the openstack_types ([#1200](https://github.com/gtema/openstack/pull/1200))

### Fixed

- Address explicit nullable types in network ([#1226](https://github.com/gtema/openstack/pull/1226))
- Adjust fip.port_details schema ([#1211](https://github.com/gtema/openstack/pull/1211))
- Fix server schema ([#1210](https://github.com/gtema/openstack/pull/1210))
- Fix fip.port_details schema ([#1209](https://github.com/gtema/openstack/pull/1209))

### Other

- Fix typos in parameter names ([#1203](https://github.com/gtema/openstack/pull/1203))
- Initialize mock testing ([#1202](https://github.com/gtema/openstack/pull/1202))

## [0.21.0](https://github.com/gtema/openstack/compare/openstack_types-v0.20.1...openstack_types-v0.21.0) - 2025-05-05

### Added

- Add instance_uuid to the instance action schema ([#1160](https://github.com/gtema/openstack/pull/1160))
- Fix BS response schemas ([#1159](https://github.com/gtema/openstack/pull/1159))
- Switch cli to openstack_types ([#1158](https://github.com/gtema/openstack/pull/1158))
- Switch tui and cli to openstack_types ([#1148](https://github.com/gtema/openstack/pull/1148))
- Adapt openstack_types to structable_derive 0.2 ([#1147](https://github.com/gtema/openstack/pull/1147))
- Use dedicated deser helpers in response types ([#1142](https://github.com/gtema/openstack/pull/1142))
- Allow serialization of the Enum based status ([#1141](https://github.com/gtema/openstack/pull/1141))
- Sort types enum kinds alphabetically ([#1140](https://github.com/gtema/openstack/pull/1140))
- Make StructTable serialize the property ([#1135](https://github.com/gtema/openstack/pull/1135))

### Fixed

- Fix few nova response schemas ([#1175](https://github.com/gtema/openstack/pull/1175))
- Fix security-group.rule.port_range_xx type ([#1171](https://github.com/gtema/openstack/pull/1171))
- Add new deser_num_str and deser_bool_str ([#1136](https://github.com/gtema/openstack/pull/1136))

### Other

- Separate structable into separate repository ([#1145](https://github.com/gtema/openstack/pull/1145))
- Fix linter for openstack_types again ([#1139](https://github.com/gtema/openstack/pull/1139))
- Fix linter complains introduced in last commit ([#1138](https://github.com/gtema/openstack/pull/1138))
- Add deser for Option<xxx> ([#1137](https://github.com/gtema/openstack/pull/1137))
