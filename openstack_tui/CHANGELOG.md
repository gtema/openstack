# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.1](https://github.com/gtema/openstack/compare/openstack_tui-v0.12.0...openstack_tui-v0.12.1) - 2025-05-17

### Added

- Add cascade QP for delete octavia loadbalancer ([#1204](https://github.com/gtema/openstack/pull/1204))

## [0.12.0](https://github.com/gtema/openstack/compare/openstack_tui-v0.11.1...openstack_tui-v0.12.0) - 2025-05-05

### Added

- Add basic error reporting helper ([#1173](https://github.com/gtema/openstack/pull/1173))
- Wrap deserialization error into dedicated error ([#1172](https://github.com/gtema/openstack/pull/1172))
- Add instance_uuid to the instance action schema ([#1160](https://github.com/gtema/openstack/pull/1160))
- Switch cli to openstack_types ([#1158](https://github.com/gtema/openstack/pull/1158))
- Switch tui and cli to openstack_types ([#1148](https://github.com/gtema/openstack/pull/1148))

### Fixed

- Fix few nova response schemas ([#1175](https://github.com/gtema/openstack/pull/1175))

### Other

- Cleanup unused code ([#1168](https://github.com/gtema/openstack/pull/1168))
- Separate structable into separate repository ([#1145](https://github.com/gtema/openstack/pull/1145))
- *(deps)* Bump crossterm from 0.28.1 to 0.29.0 ([#1128](https://github.com/gtema/openstack/pull/1128))

## [0.11.1](https://github.com/gtema/openstack/compare/openstack_tui-v0.11.0...openstack_tui-v0.11.1) - 2025-04-05

### Added

- Add original json data to the error ([#1113](https://github.com/gtema/openstack/pull/1113))
- Import IntString from openstack_sdk ([#1112](https://github.com/gtema/openstack/pull/1112))

### Other

- Update cli and tui to 2024 edition ([#990](https://github.com/gtema/openstack/pull/990))

## [0.11.0](https://github.com/gtema/openstack/compare/openstack_tui-v0.10.0...openstack_tui-v0.11.0) - 2025-03-29

### Added

- Switch to generated response structs ([#1007](https://github.com/gtema/openstack/pull/1007))
- Replace wrong simplification of vec parameters ([#1080](https://github.com/gtema/openstack/pull/1080))

## [0.10.0](https://github.com/gtema/openstack/compare/openstack_tui-v0.9.5...openstack_tui-v0.10.0) - 2025-03-14

### Added

- Add required to compute service responses ([#1011](https://github.com/gtema/openstack/pull/1011))
- BS volume bootable parameter is a string ([#1010](https://github.com/gtema/openstack/pull/1010))
- New generated content ([#1009](https://github.com/gtema/openstack/pull/1009))
- Ensure generated TUI responses are pub ([#1006](https://github.com/gtema/openstack/pull/1006))
- Drop role assignment schema hardcode ([#1005](https://github.com/gtema/openstack/pull/1005))
- Reconnect when token expired ([#989](https://github.com/gtema/openstack/pull/989))
- Send response action also for delete events ([#954](https://github.com/gtema/openstack/pull/954))
- Adapt clippy run to include bin targets ([#948](https://github.com/gtema/openstack/pull/948))

### Fixed

- Discover block-storage endpoint in the TUI ([#992](https://github.com/gtema/openstack/pull/992))

### Other

- *(deps)* Bump strum from 0.26.3 to 0.27.0 ([#965](https://github.com/gtema/openstack/pull/965))
- Implement user deletion ([#952](https://github.com/gtema/openstack/pull/952))

## [0.9.5](https://github.com/gtema/openstack/compare/openstack_tui-v0.9.4...openstack_tui-v0.9.5) - 2025-02-02

### Added

- Generate TUI response structure (#946)
- Synchronize identity schemas (#945)

### Other

- Split tui into lib and bin (#947)
- Remove deny unused in tui (#943)

## [0.9.4](https://github.com/gtema/openstack/compare/openstack_tui-v0.9.3...openstack_tui-v0.9.4) - 2025-01-17

### Added

- Synchronize code with OpenAPIs (#924)
- *(tui)* Implement views configuration (#905)
- Prepare switch to generated tui code (#880)
- Start building parts of TUI (#876)

### Fixed

- Fix few tui generated code issues (#877)

## [0.9.3](https://github.com/gtema/openstack/compare/openstack_tui-v0.9.2...openstack_tui-v0.9.3) - 2024-12-13

### Added

- Colorize output rows based on the entity status (#824)

### Other

- Modularize cloud_worker (#858)

## [0.9.2](https://github.com/gtema/openstack/compare/openstack_tui-v0.9.1...openstack_tui-v0.9.2) - 2024-12-01

### Added

- Add loadbalancer_id QP to octavia pools ([#809](https://github.com/gtema/openstack/pull/809))
- Add octavia query parameters ([#808](https://github.com/gtema/openstack/pull/808))
- Add LB views ([#805](https://github.com/gtema/openstack/pull/805))
- Add few basic DNS views ([#803](https://github.com/gtema/openstack/pull/803))
- Add volume and image deletion ([#798](https://github.com/gtema/openstack/pull/798))

### Other

- Address clippy complains ([#812](https://github.com/gtema/openstack/pull/812))
- Prepare for octavia filters ([#807](https://github.com/gtema/openstack/pull/807))

## [0.9.1](https://github.com/gtema/openstack/compare/openstack_tui-v0.9.0...openstack_tui-v0.9.1) - 2024-11-22

### Added

- Enable listing servers of flavor ([#786](https://github.com/gtema/openstack/pull/786))
- Add server deletion ([#782](https://github.com/gtema/openstack/pull/782))
- Separate keybindings into actions and filters ([#781](https://github.com/gtema/openstack/pull/781))
- Add server instance_actions view ([#777](https://github.com/gtema/openstack/pull/777))
- Add few volume views ([#776](https://github.com/gtema/openstack/pull/776))
- Better sort security group rules ([#775](https://github.com/gtema/openstack/pull/775))
- Add routers view ([#773](https://github.com/gtema/openstack/pull/773))
- Make describe a resource action ([#768](https://github.com/gtema/openstack/pull/768))

### Fixed

- Improve responsiveness ([#790](https://github.com/gtema/openstack/pull/790))
- Ensure esc properly work on deep stacks ([#787](https://github.com/gtema/openstack/pull/787))
- Result in instance action event can be null ([#779](https://github.com/gtema/openstack/pull/779))
- Prevent actions when popup is open ([#778](https://github.com/gtema/openstack/pull/778))
- Set proper default column width ([#770](https://github.com/gtema/openstack/pull/770))
- Fill domain name in the domain scope ([#763](https://github.com/gtema/openstack/pull/763))

### Other

- Add VHS tape file to capture TUI sample ([#785](https://github.com/gtema/openstack/pull/785))
- Normalize certain names ([#783](https://github.com/gtema/openstack/pull/783))
- Update the header component ([#780](https://github.com/gtema/openstack/pull/780))
- Re-enable sorting in network ([#771](https://github.com/gtema/openstack/pull/771))
- Improve action names ([#767](https://github.com/gtema/openstack/pull/767))
- Temporary disable sorting in network resources ([#766](https://github.com/gtema/openstack/pull/766))
- Change components map initialization ([#765](https://github.com/gtema/openstack/pull/765))

## [0.9.0](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.6...openstack_tui-v0.9.0) - 2024-11-15

### Added

- Add security group(rules) mode ([#758](https://github.com/gtema/openstack/pull/758))
- Add application credentials mode ([#757](https://github.com/gtema/openstack/pull/757))
- Implement user enable/disable ([#756](https://github.com/gtema/openstack/pull/756))
- Add group_users mode ([#754](https://github.com/gtema/openstack/pull/754))
- Add identity groups view ([#748](https://github.com/gtema/openstack/pull/748))
- Add users view ([#745](https://github.com/gtema/openstack/pull/745))
- Add nova hypervisors and aggregates views ([#743](https://github.com/gtema/openstack/pull/743))
- Minor improvements ([#742](https://github.com/gtema/openstack/pull/742))
- Add network quota ([#740](https://github.com/gtema/openstack/pull/740))

### Fixed

- Clear the describe on reset ([#759](https://github.com/gtema/openstack/pull/759))
- Do not crash when connected with domain_scope ([#746](https://github.com/gtema/openstack/pull/746))
- Reset input when resource select popup closed

### Other

- Better modularize cloud worker services ([#744](https://github.com/gtema/openstack/pull/744))
- Optimize code addressing new clippy suggestions ([#717](https://github.com/gtema/openstack/pull/717))

## [0.1.6](https://github.com/gtema/openstack/compare/openstack_tui-v0.1.5...openstack_tui-v0.1.6) - 2024-10-29

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
