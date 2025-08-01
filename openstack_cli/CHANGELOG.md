# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.13.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.12.4...openstack_cli-v0.13.0) - 2025-07-28

### Added

- Fix octavia operations with empty response body ([#1358](https://github.com/gtema/openstack/pull/1358))
- Set header parameters in the cli ([#1357](https://github.com/gtema/openstack/pull/1357))
- Add header query params to dns list operations ([#1349](https://github.com/gtema/openstack/pull/1349))

### Other

- [**breaking**] Change signature for adding request headers ([#1356](https://github.com/gtema/openstack/pull/1356))

## [0.12.4](https://github.com/gtema/openstack/compare/openstack_cli-v0.12.3...openstack_cli-v0.12.4) - 2025-07-11

### Added

- Add `--auth-helper-cmd` for external AuthHelper ([#1343](https://github.com/gtema/openstack/pull/1343))
- Drop connection_name from the AuthHelper ([#1342](https://github.com/gtema/openstack/pull/1342))
- Allow cli cloud config from env vars ([#1341](https://github.com/gtema/openstack/pull/1341))
- Use the new AuthHelper in cli and tui ([#1330](https://github.com/gtema/openstack/pull/1330))
- Make image.locations.metadata not required ([#1318](https://github.com/gtema/openstack/pull/1318))
- Improve support for array with prefixItems ([#1317](https://github.com/gtema/openstack/pull/1317))
- Improve UX clarity of "limit" query parameter ([#1313](https://github.com/gtema/openstack/pull/1313))
- Ensure schemas are not duplicated ([#1311](https://github.com/gtema/openstack/pull/1311))
- Implement CloudConfig construction from env ([#1339](https://github.com/gtema/openstack/pull/1339))
- Add support for the oidcaccesstoken authentication ([#1332](https://github.com/gtema/openstack/pull/1332))
- Introduce authentication helper ([#1329](https://github.com/gtema/openstack/pull/1329))
- Add future federation login support ([#1319](https://github.com/gtema/openstack/pull/1319))

### Other

- Batch update of the dependencies ([#1331](https://github.com/gtema/openstack/pull/1331))
- Prepare update of identity schemas ([#1308](https://github.com/gtema/openstack/pull/1308))

## [0.12.3](https://github.com/gtema/openstack/compare/openstack_cli-v0.12.2...openstack_cli-v0.12.3) - 2025-06-07

### Added

- Allow starting tui with custom clouds.yaml ([#1290](https://github.com/gtema/openstack/pull/1290))
- Adjust operation_name ([#1288](https://github.com/gtema/openstack/pull/1288))
- Use operation_type when operation_name is empty ([#1284](https://github.com/gtema/openstack/pull/1284))
- Improve service discovery (hopefully) ([#1274](https://github.com/gtema/openstack/pull/1274))

### Fixed

- Make catalog more permissive ([#1291](https://github.com/gtema/openstack/pull/1291))
- Fix crash when osc completion fails ([#1286](https://github.com/gtema/openstack/pull/1286))

### Other

- Add first command hints ([#1285](https://github.com/gtema/openstack/pull/1285))

## [0.12.2](https://github.com/gtema/openstack/compare/openstack_cli-v0.12.1...openstack_cli-v0.12.2) - 2025-05-28

### Added

- Add cli hints support ([#1267](https://github.com/gtema/openstack/pull/1267))
- Implement json pointer extractor for the column ([#1265](https://github.com/gtema/openstack/pull/1265))
- Add output column constraints ([#1259](https://github.com/gtema/openstack/pull/1259))
- Prepare cli config for column magic ([#1246](https://github.com/gtema/openstack/pull/1246))
- Make openapi specs reproducible ([#1244](https://github.com/gtema/openstack/pull/1244))

### Fixed

- Fix dns qp marker parameter name ([#1268](https://github.com/gtema/openstack/pull/1268))

### Other

- Add global clippy config ([#1252](https://github.com/gtema/openstack/pull/1252))

## [0.12.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.12.0...openstack_cli-v0.12.1) - 2025-05-18

### Added

- Hopefully address nested structures renaming race ([#1237](https://github.com/gtema/openstack/pull/1237))
- Add port set and binding commands ([#1231](https://github.com/gtema/openstack/pull/1231))
- Add support setting Null for nullable args ([#1227](https://github.com/gtema/openstack/pull/1227))
- Add table-arrangement output option ([#1225](https://github.com/gtema/openstack/pull/1225))
- Adapt default attributes for network.port listing ([#1217](https://github.com/gtema/openstack/pull/1217))
- Add cascade QP for delete octavia loadbalancer ([#1204](https://github.com/gtema/openstack/pull/1204))

### Fixed

- Fix role_assignments command ([#1238](https://github.com/gtema/openstack/pull/1238))
- Ensure `XXX show` returns all fields ([#1229](https://github.com/gtema/openstack/pull/1229))
- Address explicit nullable types in network ([#1226](https://github.com/gtema/openstack/pull/1226))

### Other

- Add lb.operating_status into default output ([#1230](https://github.com/gtema/openstack/pull/1230))
- Fix typos in parameter names ([#1203](https://github.com/gtema/openstack/pull/1203))
- Vendor token auth endpoint ([#1236](https://github.com/gtema/openstack/pull/1236))
- Initialize mock testing ([#1202](https://github.com/gtema/openstack/pull/1202))

## [0.12.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.11.1...openstack_cli-v0.12.0) - 2025-05-05

### Added

- Update default cli output configuration ([#1184](https://github.com/gtema/openstack/pull/1184))
- Start populating resource_key in the CLI ([#1179](https://github.com/gtema/openstack/pull/1179))
- Introduce CLI configuration ([#1177](https://github.com/gtema/openstack/pull/1177))
- Add network.router.set command ([#1174](https://github.com/gtema/openstack/pull/1174))
- Wrap deserialization error into dedicated error ([#1172](https://github.com/gtema/openstack/pull/1172))
- Introduce macros for skipping tests ([#1170](https://github.com/gtema/openstack/pull/1170))
- Switch tui and cli to openstack_types ([#1148](https://github.com/gtema/openstack/pull/1148))
- Use dedicated deser helpers in response types ([#1142](https://github.com/gtema/openstack/pull/1142))
- Start building openstack_tui crate ([#1132](https://github.com/gtema/openstack/pull/1132))
- Add basic error reporting helper ([#1173](https://github.com/gtema/openstack/pull/1173))
- Switch cli to openstack_types ([#1158](https://github.com/gtema/openstack/pull/1158))

### Fixed

- Fix few nova response schemas ([#1175](https://github.com/gtema/openstack/pull/1175))

### Other

- Address linter comments ([#1186](https://github.com/gtema/openstack/pull/1186))
- Add functional crud tests for compute keypairs ([#1176](https://github.com/gtema/openstack/pull/1176))
- Separate structable into separate repository ([#1145](https://github.com/gtema/openstack/pull/1145))
- Add trace message of config file being used ([#1185](https://github.com/gtema/openstack/pull/1185))
- Cleanup unused code ([#1168](https://github.com/gtema/openstack/pull/1168))

## [0.11.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.11.0...openstack_cli-v0.11.1) - 2025-04-05

### Added

- Import IntString from openstack_sdk ([#1112](https://github.com/gtema/openstack/pull/1112))
- Convert to FakeOpenStackClient in tests ([#1115](https://github.com/gtema/openstack/pull/1115))
- *(test)* Introduce FakeOpenStackClient ([#1114](https://github.com/gtema/openstack/pull/1114))
- Introduce common response type helpers ([#1109](https://github.com/gtema/openstack/pull/1109))

### Other

- Update cli and tui to 2024 edition ([#990](https://github.com/gtema/openstack/pull/990))
- Bundled update of deps ([#1105](https://github.com/gtema/openstack/pull/1105))

## [0.11.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.10.0...openstack_cli-v0.11.0) - 2025-03-29

### Added

- Make sensitive fields be SecretString ([#1093](https://github.com/gtema/openstack/pull/1093))
- Replace wrong simplification of vec parameters ([#1080](https://github.com/gtema/openstack/pull/1080))
- Replace imported protocol schema ([#1079](https://github.com/gtema/openstack/pull/1079))
- Address data sensitivity ([#1083](https://github.com/gtema/openstack/pull/1083))

### Fixed

- Handle missing token in the auth response ([#1078](https://github.com/gtema/openstack/pull/1078))

## [0.10.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.9.5...openstack_cli-v0.10.0) - 2025-03-14

### Added

- Enable majority of magnum commands in cli ([#1060](https://github.com/gtema/openstack/pull/1060))
- Actually start building magnum code ([#1039](https://github.com/gtema/openstack/pull/1039))
- Fix network external_gateway_info schema ([#1038](https://github.com/gtema/openstack/pull/1038))
- Initialize magnum service structure ([#1033](https://github.com/gtema/openstack/pull/1033))
- Add required to compute service responses ([#1011](https://github.com/gtema/openstack/pull/1011))
- BS volume bootable parameter is a string ([#1010](https://github.com/gtema/openstack/pull/1010))
- New generated content ([#1009](https://github.com/gtema/openstack/pull/1009))
- Ensure generated TUI responses are pub ([#1006](https://github.com/gtema/openstack/pull/1006))
- Drop role assignment schema hardcode ([#1005](https://github.com/gtema/openstack/pull/1005))
- Return readable error instead of crashing ([#1002](https://github.com/gtema/openstack/pull/1002))
- Adapt clippy run to include bin targets ([#948](https://github.com/gtema/openstack/pull/948))

### Fixed

- Use '--file' instead of '-f' in the image functest ([#1001](https://github.com/gtema/openstack/pull/1001))

### Other

- Make cli modules and commands public ([#956](https://github.com/gtema/openstack/pull/956))
- Upgrade to bincode2 ([#1063](https://github.com/gtema/openstack/pull/1063))

## [0.9.5](https://github.com/gtema/openstack/compare/openstack_cli-v0.9.4...openstack_cli-v0.9.5) - 2025-02-02

### Added

- Synchronize identity schemas (#945)
- Add get_auth_state method to obtain auth status (#937)

### Other

- Split tui into lib and bin (#947)
- Update dependencies (#940)

## [0.9.4](https://github.com/gtema/openstack/compare/openstack_cli-v0.9.3...openstack_cli-v0.9.4) - 2025-01-17

### Added

- Synchronize code with OpenAPIs (#924)
- Adapt few compute result schema changes (#871)
- Update identity schemas (#870)
- Adapt identity schemas (#868)
- Prepare switch to generated tui code (#880)
- Start building parts of TUI (#876)

### Fixed

- Respect region_name set in the config (#919)

## [0.9.3](https://github.com/gtema/openstack/compare/openstack_cli-v0.9.2...openstack_cli-v0.9.3) - 2024-12-13

### Added

- Fix LB failover schemas (#857)
- Enable identity endpoint-filter commands (#834)
- Enable registered_limit commands (#832)
- Improve `status` field detection (#827)
- Colorize output rows based on the entity status (#824)
- Add support for system scope (#828)

### Other

- Split metadata into independent services (#843)
- Get rid of `to_string` where possible (#844)
- Address another clippy suggestion in sdk (#842)

## [0.9.2](https://github.com/gtema/openstack/compare/openstack_cli-v0.9.1...openstack_cli-v0.9.2) - 2024-12-01

### Added

- Add pagination QP to octavia resources ([#816](https://github.com/gtema/openstack/pull/816))
- Address clippy suggestions ([#813](https://github.com/gtema/openstack/pull/813))
- Add loadbalancer_id QP to octavia pools ([#809](https://github.com/gtema/openstack/pull/809))
- Add octavia query parameters ([#808](https://github.com/gtema/openstack/pull/808))
- Synchronize generated content ([#806](https://github.com/gtema/openstack/pull/806))

### Fixed

- Auth with project_id and no domain info set ([#815](https://github.com/gtema/openstack/pull/815))

### Other

- Use single quotes for single char ([#814](https://github.com/gtema/openstack/pull/814))
- Address clippy complains ([#812](https://github.com/gtema/openstack/pull/812))

## [0.9.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.9.0...openstack_cli-v0.9.1) - 2024-11-22

### Added

- Use Array for serialized param instead of Set ([#774](https://github.com/gtema/openstack/pull/774))
- Make sort_key and sort_dir in neutron array ([#769](https://github.com/gtema/openstack/pull/769))

## [0.9.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.8.2...openstack_cli-v0.9.0) - 2024-11-15

### Added

- Add `--timing` argument ([#731](https://github.com/gtema/openstack/pull/731))
- Update dependencies ([#722](https://github.com/gtema/openstack/pull/722))
- Enable placement commands ([#721](https://github.com/gtema/openstack/pull/721))
- New content for placement ([#720](https://github.com/gtema/openstack/pull/720))
- Add identity.group.user commands ([#719](https://github.com/gtema/openstack/pull/719))
- Restore generating image service code ([#716](https://github.com/gtema/openstack/pull/716))
- Enable bs.service.list command ([#711](https://github.com/gtema/openstack/pull/711))
- Prepare placement skeleton ([#710](https://github.com/gtema/openstack/pull/710))
- Incorporate updated network schemas

### Fixed

- Fix codegeneration for placement esoterics ([#738](https://github.com/gtema/openstack/pull/738))
- Fix identity resource link code ([#698](https://github.com/gtema/openstack/pull/698))

### Other

- Optimize code addressing new clippy suggestions ([#717](https://github.com/gtema/openstack/pull/717))
- New generated content ([#714](https://github.com/gtema/openstack/pull/714))
- *(deps)* update crate-ci/typos action to v1.27.0 ([#704](https://github.com/gtema/openstack/pull/704))

## [0.8.2](https://github.com/gtema/openstack/compare/openstack_cli-v0.8.1...openstack_cli-v0.8.2) - 2024-10-29

### Fixed

- Exclude some errors from 'report issue' ([#689](https://github.com/gtema/openstack/pull/689))

### Other

- Update dependencies ([#691](https://github.com/gtema/openstack/pull/691))

## [0.8.1](https://github.com/gtema/openstack/compare/openstack_cli-v0.8.0...openstack_cli-v0.8.1) - 2024-10-11

### Added

- Fix dns schema mapping names ([#648](https://github.com/gtema/openstack/pull/648))
- Regenerate identity.project ([#646](https://github.com/gtema/openstack/pull/646))
- Enable few more DNS commands ([#645](https://github.com/gtema/openstack/pull/645))
- Extend Designate schemas ([#642](https://github.com/gtema/openstack/pull/642))
- Update identity.credentials schema ([#643](https://github.com/gtema/openstack/pull/643))

### Fixed

- Fix dns recordset schema ([#647](https://github.com/gtema/openstack/pull/647))

## [0.8.0](https://github.com/gtema/openstack/compare/openstack_cli-v0.7.3...openstack_cli-v0.8.0) - 2024-10-04

### Added

- Switch from cli-table to comfy-table ([#633](https://github.com/gtema/openstack/pull/633))
- Enable dns zone/recordset commands ([#632](https://github.com/gtema/openstack/pull/632))
- Add dns zone/recordset query parameters ([#634](https://github.com/gtema/openstack/pull/634))
- Start building DNS bindings ([#628](https://github.com/gtema/openstack/pull/628))
- add the ability to specify the configs to load via a builder ([#619](https://github.com/gtema/openstack/pull/619))
- Initialize the DNS service support ([#620](https://github.com/gtema/openstack/pull/620))
- Add user/project/domain query param resolution ([#618](https://github.com/gtema/openstack/pull/618))
- Add support for --os-client-config-file ([#611](https://github.com/gtema/openstack/pull/611))
- Enable remaining compute commands ([#606](https://github.com/gtema/openstack/pull/606))

### Fixed

- Address find when GET return 400 ([#631](https://github.com/gtema/openstack/pull/631))

### Other

- Show full osc command in the doc ([#609](https://github.com/gtema/openstack/pull/609))
- Drop better/human-panic ([#607](https://github.com/gtema/openstack/pull/607))
- Update locked dependencies ([#605](https://github.com/gtema/openstack/pull/605))
- *(deps)* update crate-ci/typos action to v1.25.0 ([#624](https://github.com/gtema/openstack/pull/624))

## [0.7.3](https://github.com/gtema/openstack/compare/openstack_cli-v0.7.2...openstack_cli-v0.7.3) - 2024-09-28

### Added

- Enable multiple network resources ([#593](https://github.com/gtema/openstack/pull/593))
- Add network.flavor commands ([#585](https://github.com/gtema/openstack/pull/585))
- Enable network.default-security-group-rule commands ([#582](https://github.com/gtema/openstack/pull/582))
- Enable network.auto_allocated_topology command ([#580](https://github.com/gtema/openstack/pull/580))
- Enable network.agent commands ([#579](https://github.com/gtema/openstack/pull/579))
- Rework doc building ([#568](https://github.com/gtema/openstack/pull/568))
- Improve crate (interface) documentation ([#583](https://github.com/gtema/openstack/pull/583))

### Fixed

- Fix network.agent property types ([#586](https://github.com/gtema/openstack/pull/586))

### Other

- Further doc clarifications ([#584](https://github.com/gtema/openstack/pull/584))

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
