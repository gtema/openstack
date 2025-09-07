# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.22.1](https://github.com/gtema/openstack/compare/openstack_types-v0.22.0...openstack_types-v0.22.1) - 2025-09-07

### Added

- Workaround BS quota api path issues ([#1404](https://github.com/gtema/openstack/pull/1404))
- Add response key hack to the BS quota resources ([#1397](https://github.com/gtema/openstack/pull/1397))

### Other

- *(deps)* Bump uuid from 1.17.0 to 1.18.0 ([#1372](https://github.com/gtema/openstack/pull/1372))
- Address new linter warning (MISMATCHED_LIFETIME_SYNTAXES) ([#1371](https://github.com/gtema/openstack/pull/1371))

## [0.22.0](https://github.com/gtema/openstack/compare/openstack_types-v0.21.4...openstack_types-v0.22.0) - 2025-07-28

### Added

- Fix octavia operations with empty response body ([#1358](https://github.com/gtema/openstack/pull/1358))
- Set header parameters in the cli ([#1357](https://github.com/gtema/openstack/pull/1357))
- Add header query params to dns list operations ([#1349](https://github.com/gtema/openstack/pull/1349))
- OpenAPI: Fix arrays without `items` ([#1348](https://github.com/gtema/openstack/pull/1348))

### Other

- Add more deserialization tests ([#1359](https://github.com/gtema/openstack/pull/1359))
- [**breaking**] Change signature for adding request headers ([#1356](https://github.com/gtema/openstack/pull/1356))

## [0.21.4](https://github.com/gtema/openstack/compare/openstack_types-v0.21.3...openstack_types-v0.21.4) - 2025-07-11

### Added

- Make image.locations.metadata not required ([#1318](https://github.com/gtema/openstack/pull/1318))
- Improve UX clarity of "limit" query parameter ([#1313](https://github.com/gtema/openstack/pull/1313))
- OpenAPI: Fix nova examples ([#1312](https://github.com/gtema/openstack/pull/1312))
- Ensure schemas are not duplicated ([#1311](https://github.com/gtema/openstack/pull/1311))

### Other

- Init mock test for other services ([#1253](https://github.com/gtema/openstack/pull/1253))

## [0.21.3](https://github.com/gtema/openstack/compare/openstack_types-v0.21.2...openstack_types-v0.21.3) - 2025-06-07

### Added

- Use operation_type when operation_name is empty ([#1284](https://github.com/gtema/openstack/pull/1284))

## [0.21.2](https://github.com/gtema/openstack/compare/openstack_types-v0.21.1...openstack_types-v0.21.2) - 2025-05-28

### Added

- Make openapi specs reproducible ([#1244](https://github.com/gtema/openstack/pull/1244))

### Fixed

- Fix dns qp marker parameter name ([#1268](https://github.com/gtema/openstack/pull/1268))
- Fix type conversion of const bool ([#1258](https://github.com/gtema/openstack/pull/1258))
- Ensure swift bytes are positive integers ([#1255](https://github.com/gtema/openstack/pull/1255))
- Add missed serde macros in types ([#1254](https://github.com/gtema/openstack/pull/1254))

### Other

- Add global clippy config ([#1252](https://github.com/gtema/openstack/pull/1252))
- Fix lint for openstack_types tests ([#1250](https://github.com/gtema/openstack/pull/1250))

## [0.21.1](https://github.com/gtema/openstack/compare/openstack_types-v0.21.0...openstack_types-v0.21.1) - 2025-05-18

### Added

- Hopefully address nested structures renaming race ([#1237](https://github.com/gtema/openstack/pull/1237))
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
