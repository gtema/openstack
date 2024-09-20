# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.2](https://github.com/gtema/openstack/compare/openstack_cli-v0.7.1...openstack_cli-v0.7.2) - 2024-09-20

### Added

- Add identity.limit command ([#553](https://github.com/gtema/openstack/pull/553))
- Extend user path param with `--current-user` ([#550](https://github.com/gtema/openstack/pull/550))
- Use all resource names for path param naming ([#549](https://github.com/gtema/openstack/pull/549))

### Fixed

- Reset catalog on reauth ([#555](https://github.com/gtema/openstack/pull/555))

### Other

- Enable user app_creds and rules list ([#557](https://github.com/gtema/openstack/pull/557))

## [0.7.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.7.0...openstack_cli-v0.7.1) - 2024-09-13

### Added

- *(cli)* Interactive cloud connection ([#532](https://github.com/gtema/openstack/pull/532))
- *(cli)* Add identity.credential commands ([#515](https://github.com/gtema/openstack/pull/515))
- *(cli)* Enable identity.auth commands ([#513](https://github.com/gtema/openstack/pull/513))
- Add resource link to domain_id path parameter ([#509](https://github.com/gtema/openstack/pull/509))
- Remove few panics in auth handling ([#508](https://github.com/gtema/openstack/pull/508))
- Add resource link in path parameters ([#507](https://github.com/gtema/openstack/pull/507))
- Add few exceptions for cli ([#506](https://github.com/gtema/openstack/pull/506))
- *(sdk)* Add find_by_name method ([#505](https://github.com/gtema/openstack/pull/505))

## [0.7.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.6.5...openstack_cli-v0.7.0) - 2024-08-23

### Added
- Start Terminal User Interface project ([#470](https://github.com/gtema/openstack/pull/470))
- Add network resource pagination and sorting ([#469](https://github.com/gtema/openstack/pull/469))
- New generated content ([#468](https://github.com/gtema/openstack/pull/468))
- *(cli)* Enable shell autocompletion ([#467](https://github.com/gtema/openstack/pull/467))
- Activate net.subnetpool.XXX_prefixes commands ([#464](https://github.com/gtema/openstack/pull/464))
- Introduce use of human-panic ([#462](https://github.com/gtema/openstack/pull/462))
- Add list of available connections ([#465](https://github.com/gtema/openstack/pull/465))
- Add support for placement microversion ([#463](https://github.com/gtema/openstack/pull/463))

### Other
- Update compute.floatingip ([#461](https://github.com/gtema/openstack/pull/461))

## [0.6.5](https://github.com/gtema/openstack/compare/openstack_cli-v0.6.4...openstack_cli-v0.6.5) - 2024-08-16

### Added
- Enable network.rbac_policy ([#447](https://github.com/gtema/openstack/pull/447))
- New generated content ([#444](https://github.com/gtema/openstack/pull/444))
- New generated content ([#443](https://github.com/gtema/openstack/pull/443))
- New generated content ([#424](https://github.com/gtema/openstack/pull/424))

### Fixed
- Change condition in non-terminal data upload ([#422](https://github.com/gtema/openstack/pull/422))

### Other
- *(deps)* Update dependencies ([#449](https://github.com/gtema/openstack/pull/449))
- Remove unused code ([#445](https://github.com/gtema/openstack/pull/445))
- Add Eq/PartialEq derives in some types ([#446](https://github.com/gtema/openstack/pull/446))

## [0.6.4](https://github.com/gtema/openstack/compare/openstack_cli-v0.6.3...openstack_cli-v0.6.4) - 2024-08-01

### Added
- Enable subnetpool command ([#394](https://github.com/gtema/openstack/pull/394))
- Enable security_groups command ([#393](https://github.com/gtema/openstack/pull/393))
- New generated content ([#401](https://github.com/gtema/openstack/pull/401))

### Fixed
- Fix certain network.SG properties ([#398](https://github.com/gtema/openstack/pull/398))

### Other
- Regenerate block-storage resources ([#397](https://github.com/gtema/openstack/pull/397))
- Address some warning of newest rust ([#395](https://github.com/gtema/openstack/pull/395))
- Regenerate load-balancer resources ([#388](https://github.com/gtema/openstack/pull/388))
- Regenerate compute resources ([#389](https://github.com/gtema/openstack/pull/389))
- Regenerate identity resource ([#387](https://github.com/gtema/openstack/pull/387))
- Regenerate network resources ([#386](https://github.com/gtema/openstack/pull/386))

## [0.6.3](https://github.com/gtema/openstack/compare/openstack_cli-v0.6.2...openstack_cli-v0.6.3) - 2024-07-27

### Other
- update Cargo.lock dependencies

## [0.6.2](https://github.com/gtema/openstack/compare/openstack_cli-v0.6.1...openstack_cli-v0.6.2) - 2024-07-27

### Added
- *(deps)* Drop anyhow dependency ([#368](https://github.com/gtema/openstack/pull/368))
- *(cli)* Start switch to eyre for error handling ([#357](https://github.com/gtema/openstack/pull/357))

### Fixed
- *(deps)* update rust crate color-eyre to ^0.6.0 ([#364](https://github.com/gtema/openstack/pull/364))
- Fix network.fip cli command ([#355](https://github.com/gtema/openstack/pull/355))

### Other
- *(deps)* Update dependencies ([#367](https://github.com/gtema/openstack/pull/367))

## [0.6.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.6.0...openstack_cli-v0.6.1) - 2024-07-15

### Added
- Add `container prune` command ([#333](https://github.com/gtema/openstack/pull/333))

### Fixed
- *(cli)* Fix network.port.dns_assignments schema ([#329](https://github.com/gtema/openstack/pull/329))
- *(pagination)* Fix pagination when using iter ([#332](https://github.com/gtema/openstack/pull/332))

## [0.6.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.5.2...openstack_cli-v0.6.0) - 2024-07-05

### Added
- Allow overriding project in cli ([#313](https://github.com/gtema/openstack/pull/313))
- Allow skipping SSL verification ([#309](https://github.com/gtema/openstack/pull/309))

### Fixed
- *(openstack_cli)* Check `api` content carefully ([#314](https://github.com/gtema/openstack/pull/314))
- *(pagination)* Address new cornercase in pagination ([#312](https://github.com/gtema/openstack/pull/312))
- *(auth)* Deal with InvalidPort in discovery doc ([#311](https://github.com/gtema/openstack/pull/311))

### Other
- Absent state file is not a warning ([#315](https://github.com/gtema/openstack/pull/315))

## [0.5.2](https://github.com/gtema/openstack/compare/openstack_cli-v0.5.1...openstack_cli-v0.5.2) - 2024-06-27

### Added
- *(ssl)* Add possibility to add custom root SSL ca ([#293](https://github.com/gtema/openstack/pull/293))

### Fixed
- Repair endpoint url construction for object ([#295](https://github.com/gtema/openstack/pull/295))

### Other
- *(cli)* Clarify the API command parameters ([#294](https://github.com/gtema/openstack/pull/294))

## [0.5.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.5.0...openstack_cli-v0.5.1) - 2024-06-17

### Added
- *(sdk)* Add api version into RestEndpoint ([#241](https://github.com/gtema/openstack/pull/241))
- Add domain commands implementation ([#233](https://github.com/gtema/openstack/pull/233))
- *(cli)* Add support for patch method in `api` call ([#232](https://github.com/gtema/openstack/pull/232))
- *(sdk)* Rework service catalog processing ([#225](https://github.com/gtema/openstack/pull/225))
- *(sdk)* Add possibility to skip version discovery ([#227](https://github.com/gtema/openstack/pull/227))
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

## [0.5.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.4.0...openstack_cli-v0.5.0) - 2024-05-07

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
- force MV discovery for Compute and Volume

### Fixed
- allow linter warning

### Other
- *(docs)* Add minor specifications into the readme ([#195](https://github.com/gtema/openstack/pull/195))
- *(test)* Introduce features ([#176](https://github.com/gtema/openstack/pull/176))
- add volume.group-type schemas
- Merge pull request [#153](https://github.com/gtema/openstack/pull/153) from gtema/gtema-patch-2
- Update documentation
- *(lint)* Apply fresh clippy suggestions ([#193](https://github.com/gtema/openstack/pull/193))
- *(qa)* Initialize fuzzing ([#191](https://github.com/gtema/openstack/pull/191))
- *(ci)* Update dependencies ([#190](https://github.com/gtema/openstack/pull/190))
- fix code scanning warning
- *(deps)* bump hyper from 1.2.0 to 1.3.0

## [0.4.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.3.0...openstack_cli-v0.4.0) - 2024-04-05

### Added
- Add LoadBalancer
- regenerate code with latest generator changes
- *(volume)* cover block-storage backup
- *(network)* add network address_{group,scope} resources

### Other
- *(deps)* bump tempfile from 3.10.0 to 3.10.1
- drop serde_yaml dependency
- Merge pull request [#113](https://github.com/gtema/openstack/pull/113) from gtema/deps
- update http/reqwest/hyper lib

## [0.3.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.2.0...openstack_cli-v0.3.0) - 2024-03-15

### Added
- add autogenerated functional tests
- add image.metadef implementation
- improve image docstrings
- *(cli)* add image.metadef schema commands
- *(network)* enable volume type enctyption commands
- *(output)* group command args in help message
- visually separate global options in help message
- implement router {add,remove}_router_interface
- activate network.floatingip.port-forwarding cli
- add network.router.{conntrack_helper,l3_agent}
- unify network resource tag command
- exclude `links` from "list" results
- Implement pretty-print support
- use json style in output for complex fields
- add identity group resources
- *(identity)* extend role commands
- *(identity)* Add role-assignment list command
- enable router interface/routes/gateways operations

### Fixed
- remove yaml as supported output format

### Other
- preparation changes for image.metadef
- reorg integration tests
- Merge pull request [#92](https://github.com/gtema/openstack/pull/92) from gtema/output
- remove another unnecessary cloning operation
- sort struct fields alphabetically
- Merge pull request [#88](https://github.com/gtema/openstack/pull/88) from gtema/fix
- Rename mod.rs of the versioned modules
- add clippy exception on common tag module
- remove few unnecessary clone invocations
- fix role responses
- replace deprecated chrono::Duration::days call
- *(deps)* bump open from 5.0.1 to 5.1.1

## [0.2.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.1.1...openstack_cli-v0.2.0) - 2024-02-23

### Fixed
- *(sdk)* flatten _properties of extensible structs
- Respect headers passed into the `api` command

### Other
- release openstack_sdk
- switch to caret requirements syntax
- further improve comments
- better pre-format comments

## [0.1.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.1.0...openstack_cli-v0.1.1) - 2024-02-16

### Added
- *(docs)* Improve documents structure

### Other
- Prepare release of sdk and strucable_derive
- Revert "chore: release"
- release
