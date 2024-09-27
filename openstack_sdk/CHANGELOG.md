# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.2](https://github.com/gtema/openstack/compare/openstack_sdk-v0.11.1...openstack_sdk-v0.11.2) - 2024-09-27

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
