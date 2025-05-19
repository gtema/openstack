# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.21.0...openstack_sdk-v0.21.1) - 2025-05-18

### Added

- Hopefully address nested structures renaming race ([#1237](https://github.com/gtema/openstack/pull/1237))
- Add cascade QP for delete octavia loadbalancer ([#1204](https://github.com/gtema/openstack/pull/1204))

### Fixed

- Address explicit nullable types in network ([#1226](https://github.com/gtema/openstack/pull/1226))

### Other

- Vendor token auth endpoint ([#1236](https://github.com/gtema/openstack/pull/1236))
- Fix typos in parameter names ([#1203](https://github.com/gtema/openstack/pull/1203))
- Initialize mock testing ([#1202](https://github.com/gtema/openstack/pull/1202))

## [0.21.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.20.1...openstack_sdk-v0.21.0) - 2025-05-05

### Added

- Add basic error reporting helper ([#1173](https://github.com/gtema/openstack/pull/1173))
- Switch cli to openstack_types ([#1158](https://github.com/gtema/openstack/pull/1158))
- Switch tui and cli to openstack_types ([#1148](https://github.com/gtema/openstack/pull/1148))
- Use dedicated deser helpers in response types ([#1142](https://github.com/gtema/openstack/pull/1142))

### Fixed

- Fix few nova response schemas ([#1175](https://github.com/gtema/openstack/pull/1175))

### Other

- Add trace message of config file being used ([#1185](https://github.com/gtema/openstack/pull/1185))
- Cleanup unused code ([#1168](https://github.com/gtema/openstack/pull/1168))

## [0.20.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.20.0...openstack_sdk-v0.20.1) - 2025-04-05

### Added

- Convert to FakeOpenStackClient in tests ([#1115](https://github.com/gtema/openstack/pull/1115))
- *(test)* Introduce FakeOpenStackClient ([#1114](https://github.com/gtema/openstack/pull/1114))
- Import IntString from openstack_sdk ([#1112](https://github.com/gtema/openstack/pull/1112))
- Introduce common response type helpers ([#1109](https://github.com/gtema/openstack/pull/1109))

### Other

- Bundled update of deps ([#1105](https://github.com/gtema/openstack/pull/1105))

## [0.20.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.19.0...openstack_sdk-v0.20.0) - 2025-03-29

### Added

- Make sensitive fields be SecretString ([#1093](https://github.com/gtema/openstack/pull/1093))
- Address data sensitivity ([#1083](https://github.com/gtema/openstack/pull/1083))
- Replace wrong simplification of vec parameters ([#1080](https://github.com/gtema/openstack/pull/1080))
- Replace imported protocol schema ([#1079](https://github.com/gtema/openstack/pull/1079))

### Fixed

- Handle missing token in the auth response ([#1078](https://github.com/gtema/openstack/pull/1078))

## [0.19.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.18.0...openstack_sdk-v0.19.0) - 2025-03-14

### Added

- Actually start building magnum code ([#1039](https://github.com/gtema/openstack/pull/1039))
- Fix network external_gateway_info schema ([#1038](https://github.com/gtema/openstack/pull/1038))
- Initialize magnum service structure ([#1033](https://github.com/gtema/openstack/pull/1033))
- New generated content ([#1009](https://github.com/gtema/openstack/pull/1009))
- Ensure generated TUI responses are pub ([#1006](https://github.com/gtema/openstack/pull/1006))
- Drop role assignment schema hardcode ([#1005](https://github.com/gtema/openstack/pull/1005))

### Other

- Upgrade to bincode2 ([#1063](https://github.com/gtema/openstack/pull/1063))

## [0.18.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.17.0...openstack_sdk-v0.18.0) - 2025-02-02

### Added

- Synchronize identity schemas (#945)
- Add get_auth_state method to obtain auth status (#937)

### Other

- Update dependencies (#940)

## [0.17.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.16.0...openstack_sdk-v0.17.0) - 2025-01-17

### Added

- Synchronize code with OpenAPIs (#924)
- Prepare switch to generated tui code (#880)
- Start building parts of TUI (#876)
- Adapt few compute result schema changes (#871)
- Update identity schemas (#870)
- Adapt identity schemas (#868)

### Fixed

- Respect region_name set in the config (#919)

## [0.16.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.15.0...openstack_sdk-v0.16.0) - 2024-12-13

### Added

- Fix LB failover schemas (#857)
- Add support for system scope (#828)
- Colorize output rows based on the entity status (#824)

### Other

- Get rid of `to_string` where possible (#844)
- Split metadata into independent services (#843)
- Address another clippy suggestion in sdk (#842)

## [0.15.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.14.1...openstack_sdk-v0.15.0) - 2024-12-01

### Added

- Add pagination QP to octavia resources ([#816](https://github.com/gtema/openstack/pull/816))
- Address clippy suggestions ([#813](https://github.com/gtema/openstack/pull/813))
- Add loadbalancer_id QP to octavia pools ([#809](https://github.com/gtema/openstack/pull/809))
- Add octavia query parameters ([#808](https://github.com/gtema/openstack/pull/808))
- Synchronize generated content ([#806](https://github.com/gtema/openstack/pull/806))

### Fixed

- Auth with project_id and no domain info set ([#815](https://github.com/gtema/openstack/pull/815))

### Other

- Address clippy complains ([#812](https://github.com/gtema/openstack/pull/812))

## [0.14.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.14.0...openstack_sdk-v0.14.1) - 2024-11-22

### Added

- Use Array for serialized param instead of Set ([#774](https://github.com/gtema/openstack/pull/774))
- Make sort_key and sort_dir in neutron array ([#769](https://github.com/gtema/openstack/pull/769))

## [0.14.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.13.1...openstack_sdk-v0.14.0) - 2024-11-15

### Added

- Add `--timing` argument ([#731](https://github.com/gtema/openstack/pull/731))
- Update dependencies ([#722](https://github.com/gtema/openstack/pull/722))
- New content for placement ([#720](https://github.com/gtema/openstack/pull/720))
- Restore generating image service code ([#716](https://github.com/gtema/openstack/pull/716))
- Prepare placement skeleton ([#710](https://github.com/gtema/openstack/pull/710))
- Incorporate updated network schemas

### Fixed

- Fix codegeneration for placement esoterics ([#738](https://github.com/gtema/openstack/pull/738))

### Other

- Optimize code addressing new clippy suggestions ([#717](https://github.com/gtema/openstack/pull/717))
- *(deps)* update crate-ci/typos action to v1.27.0 ([#704](https://github.com/gtema/openstack/pull/704))

## [0.13.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.13.0...openstack_sdk-v0.13.1) - 2024-10-29

### Other

- Update dependencies ([#691](https://github.com/gtema/openstack/pull/691))

## [0.13.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.12.0...openstack_sdk-v0.13.0) - 2024-10-11

### Added

- Fix dns schema mapping names ([#648](https://github.com/gtema/openstack/pull/648))
- Regenerate identity.project ([#646](https://github.com/gtema/openstack/pull/646))
- Extend Designate schemas ([#642](https://github.com/gtema/openstack/pull/642))
- Update identity.credentials schema ([#643](https://github.com/gtema/openstack/pull/643))

### Fixed

- Fix dns recordset schema ([#647](https://github.com/gtema/openstack/pull/647))

## [0.12.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.11.2...openstack_sdk-v0.12.0) - 2024-10-04

### Added

- Enable dns zone/recordset commands ([#632](https://github.com/gtema/openstack/pull/632))
- Add dns zone/recordset query parameters ([#634](https://github.com/gtema/openstack/pull/634))
- Start building DNS bindings ([#628](https://github.com/gtema/openstack/pull/628))
- add the ability to specify the configs to load via a builder ([#619](https://github.com/gtema/openstack/pull/619))
- Initialize the DNS service support ([#620](https://github.com/gtema/openstack/pull/620))
- Add support for --os-client-config-file ([#611](https://github.com/gtema/openstack/pull/611))

### Fixed

- Address find when GET return 400 ([#631](https://github.com/gtema/openstack/pull/631))

### Other

- *(deps)* update crate-ci/typos action to v1.25.0 ([#624](https://github.com/gtema/openstack/pull/624))

## [0.11.2](https://github.com/gtema/openstack/compare/openstack_sdk-v0.11.1...openstack_sdk-v0.11.2) - 2024-09-28

### Added

- Improve crate (interface) documentation ([#583](https://github.com/gtema/openstack/pull/583))
- Rework doc building ([#568](https://github.com/gtema/openstack/pull/568))

### Other

- Further doc clarifications ([#584](https://github.com/gtema/openstack/pull/584))

## [0.11.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.11.0...openstack_sdk-v0.11.1) - 2024-09-20

### Added

- Use all resource names for path param naming ([#549](https://github.com/gtema/openstack/pull/549))

### Fixed

- Reset catalog on reauth ([#555](https://github.com/gtema/openstack/pull/555))

## [0.11.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.10.0...openstack_sdk-v0.11.0) - 2024-09-13

### Added

- *(cli)* Interactive cloud connection ([#532](https://github.com/gtema/openstack/pull/532))
- Add resource link in path parameters ([#507](https://github.com/gtema/openstack/pull/507))
- *(sdk)* Add find_by_name method ([#505](https://github.com/gtema/openstack/pull/505))

## [0.10.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.9.0...openstack_sdk-v0.10.0) - 2024-08-23

### Added
- Start Terminal User Interface project ([#470](https://github.com/gtema/openstack/pull/470))
- Add network resource pagination and sorting ([#469](https://github.com/gtema/openstack/pull/469))
- New generated content ([#468](https://github.com/gtema/openstack/pull/468))
- Add list of available connections ([#465](https://github.com/gtema/openstack/pull/465))
- Add support for placement microversion ([#463](https://github.com/gtema/openstack/pull/463))

### Other
- Update compute.floatingip ([#461](https://github.com/gtema/openstack/pull/461))

## [0.9.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.8.0...openstack_sdk-v0.9.0) - 2024-08-16

### Added
- New generated content ([#444](https://github.com/gtema/openstack/pull/444))
- New generated content ([#424](https://github.com/gtema/openstack/pull/424))

### Other
- Add Eq/PartialEq derives in some types ([#446](https://github.com/gtema/openstack/pull/446))
- Remove unused code ([#445](https://github.com/gtema/openstack/pull/445))

## [0.8.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.7.0...openstack_sdk-v0.8.0) - 2024-08-01

### Added
- New generated content ([#401](https://github.com/gtema/openstack/pull/401))
- Enable security_groups command ([#393](https://github.com/gtema/openstack/pull/393))

### Fixed
- Fix certain network.SG properties ([#398](https://github.com/gtema/openstack/pull/398))

### Other
- Regenerate block-storage resources ([#397](https://github.com/gtema/openstack/pull/397))
- Address some warning of newest rust ([#395](https://github.com/gtema/openstack/pull/395))
- Regenerate load-balancer resources ([#388](https://github.com/gtema/openstack/pull/388))
- Regenerate compute resources ([#389](https://github.com/gtema/openstack/pull/389))
- Regenerate identity resource ([#387](https://github.com/gtema/openstack/pull/387))
- Regenerate network resources ([#386](https://github.com/gtema/openstack/pull/386))

## [0.7.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.6.1...openstack_sdk-v0.7.0) - 2024-07-27

### Added
- *(deps)* Drop anyhow dependency ([#368](https://github.com/gtema/openstack/pull/368))
- *(cli)* Start switch to eyre for error handling ([#357](https://github.com/gtema/openstack/pull/357))

### Other
- *(deps)* Update dependencies ([#367](https://github.com/gtema/openstack/pull/367))

## [0.6.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.6.0...openstack_sdk-v0.6.1) - 2024-07-15

### Added
- Add `container prune` command ([#333](https://github.com/gtema/openstack/pull/333))

### Fixed
- *(pagination)* Fix pagination when using iter ([#332](https://github.com/gtema/openstack/pull/332))

## [0.6.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.5.2...openstack_sdk-v0.6.0) - 2024-07-05

### Added
- Allow overriding project in cli ([#313](https://github.com/gtema/openstack/pull/313))
- Allow skipping SSL verification ([#309](https://github.com/gtema/openstack/pull/309))

### Fixed
- *(pagination)* Address new cornercase in pagination ([#312](https://github.com/gtema/openstack/pull/312))
- *(auth)* Deal with InvalidPort in discovery doc ([#311](https://github.com/gtema/openstack/pull/311))

### Other
- Absent state file is not a warning ([#315](https://github.com/gtema/openstack/pull/315))

## [0.5.2](https://github.com/gtema/openstack/compare/openstack_sdk-v0.5.1...openstack_sdk-v0.5.2) - 2024-06-27

### Added
- *(ssl)* Add possibility to add custom root SSL ca ([#293](https://github.com/gtema/openstack/pull/293))

### Fixed
- Repair endpoint url construction for object ([#295](https://github.com/gtema/openstack/pull/295))

## [0.5.1](https://github.com/gtema/openstack/compare/openstack_sdk-v0.5.0...openstack_sdk-v0.5.1) - 2024-06-17

### Added
- *(sdk)* Add api version into RestEndpoint ([#241](https://github.com/gtema/openstack/pull/241))
- Add domain commands implementation ([#233](https://github.com/gtema/openstack/pull/233))
- *(sdk)* Add possibility to skip version discovery ([#227](https://github.com/gtema/openstack/pull/227))
- *(sdk)* Rework service catalog processing ([#225](https://github.com/gtema/openstack/pull/225))
- *(sdk)* Use 'Endpoint' struct for endpoint overrides ([#216](https://github.com/gtema/openstack/pull/216))
- *(sdk)* Start consuming service_authority ([#215](https://github.com/gtema/openstack/pull/215))
- *(sdk)* Deduplicate endpoint construction method ([#214](https://github.com/gtema/openstack/pull/214))
- *(sdk)* Add Endpoint struct ([#213](https://github.com/gtema/openstack/pull/213))
- *(sdk)* Add service authority data ([#212](https://github.com/gtema/openstack/pull/212))

### Other
- *(features)* Renerate code fixing sdk crate features ([#240](https://github.com/gtema/openstack/pull/240))
- *(error)* Improve discovery error ([#237](https://github.com/gtema/openstack/pull/237))
- *(sdk)* Get rid of anyhow in openstack_sdk ([#228](https://github.com/gtema/openstack/pull/228))
- --- ([#211](https://github.com/gtema/openstack/pull/211))

## [0.5.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.4.0...openstack_sdk-v0.5.0) - 2024-05-07

### Added
- Add volume.qos-spec resources ([#196](https://github.com/gtema/openstack/pull/196))
- Add volume.az resource ([#194](https://github.com/gtema/openstack/pull/194))
- Add volume.cluster resource ([#192](https://github.com/gtema/openstack/pull/192))
- Add cinder volume_transfer resources ([#187](https://github.com/gtema/openstack/pull/187))
- Add volume.manageable_XXX resources ([#175](https://github.com/gtema/openstack/pull/175))
- Add volume.default-types ([#173](https://github.com/gtema/openstack/pull/173))
- Add volume.extension ([#167](https://github.com/gtema/openstack/pull/167))
- add volume.host
- add volume.group-snapshots
- add volume.snapshot
- implement volume.attachment
- add volume.limit
- add volume.message
- add volume.resource-filters implementation

### Other
- *(docs)* Add minor specifications into the readme ([#195](https://github.com/gtema/openstack/pull/195))
- *(lint)* Apply fresh clippy suggestions ([#193](https://github.com/gtema/openstack/pull/193))
- *(qa)* Initialize fuzzing ([#191](https://github.com/gtema/openstack/pull/191))
- *(ci)* Update dependencies ([#190](https://github.com/gtema/openstack/pull/190))
- *(test)* Introduce features ([#176](https://github.com/gtema/openstack/pull/176))
- add volume.group-type schemas
- fix code scanning warning
- Merge pull request [#153](https://github.com/gtema/openstack/pull/153) from gtema/gtema-patch-2
- Update documentation
- *(deps)* bump hyper from 1.2.0 to 1.3.0

## [0.4.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.3.0...openstack_sdk-v0.4.0) - 2024-04-05

### Added
- Add LoadBalancer
- regenerate code with latest generator changes
- *(volume)* cover block-storage backup

### Other
- drop serde_yaml dependency
- Merge pull request [#113](https://github.com/gtema/openstack/pull/113) from gtema/deps
- update http/reqwest/hyper lib

## [0.3.0](https://github.com/gtema/openstack/compare/openstack_sdk-v0.2.0...openstack_sdk-v0.3.0) - 2024-03-15

### Added
- add autogenerated functional tests
- add image.metadef implementation
- improve image docstrings
- *(network)* enable volume type enctyption commands
- enable router interface/routes/gateways operations
- implement router {add,remove}_router_interface
- add identity group resources
- *(identity)* extend role commands
- *(identity)* Add role-assignment list command

### Other
- preparation changes for image.metadef
- replace deprecated chrono::Duration::days call
- reorg integration tests
- sort struct fields alphabetically
- *(deps)* bump open from 5.0.1 to 5.1.1
- remove few unnecessary clone invocations
- fix role responses

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
