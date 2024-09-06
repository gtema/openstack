# Command-Line Help for `osc`

This document contains the help content for the `osc` command-line program.

**Command Overview:**

* [`osc`↴](#osc)
* [`osc api`↴](#osc-api)
* [`osc auth`↴](#osc-auth)
* [`osc auth login`↴](#osc-auth-login)
* [`osc auth show`↴](#osc-auth-show)
* [`osc block-storage`↴](#osc-block-storage)
* [`osc block-storage attachment`↴](#osc-block-storage-attachment)
* [`osc block-storage attachment complete`↴](#osc-block-storage-attachment-complete)
* [`osc block-storage attachment create354`↴](#osc-block-storage-attachment-create354)
* [`osc block-storage attachment create327`↴](#osc-block-storage-attachment-create327)
* [`osc block-storage attachment delete`↴](#osc-block-storage-attachment-delete)
* [`osc block-storage attachment list`↴](#osc-block-storage-attachment-list)
* [`osc block-storage attachment set327`↴](#osc-block-storage-attachment-set327)
* [`osc block-storage attachment show`↴](#osc-block-storage-attachment-show)
* [`osc block-storage availability-zone`↴](#osc-block-storage-availability-zone)
* [`osc block-storage availability-zone list`↴](#osc-block-storage-availability-zone-list)
* [`osc block-storage backup`↴](#osc-block-storage-backup)
* [`osc block-storage backup create351`↴](#osc-block-storage-backup-create351)
* [`osc block-storage backup create343`↴](#osc-block-storage-backup-create343)
* [`osc block-storage backup create30`↴](#osc-block-storage-backup-create30)
* [`osc block-storage backup delete`↴](#osc-block-storage-backup-delete)
* [`osc block-storage backup export`↴](#osc-block-storage-backup-export)
* [`osc block-storage backup force-delete`↴](#osc-block-storage-backup-force-delete)
* [`osc block-storage backup import`↴](#osc-block-storage-backup-import)
* [`osc block-storage backup list`↴](#osc-block-storage-backup-list)
* [`osc block-storage backup reset-status`↴](#osc-block-storage-backup-reset-status)
* [`osc block-storage backup set343`↴](#osc-block-storage-backup-set343)
* [`osc block-storage backup set39`↴](#osc-block-storage-backup-set39)
* [`osc block-storage backup show`↴](#osc-block-storage-backup-show)
* [`osc block-storage cluster`↴](#osc-block-storage-cluster)
* [`osc block-storage cluster list`↴](#osc-block-storage-cluster-list)
* [`osc block-storage cluster set`↴](#osc-block-storage-cluster-set)
* [`osc block-storage cluster show`↴](#osc-block-storage-cluster-show)
* [`osc block-storage default-type`↴](#osc-block-storage-default-type)
* [`osc block-storage default-type delete`↴](#osc-block-storage-default-type-delete)
* [`osc block-storage default-type list`↴](#osc-block-storage-default-type-list)
* [`osc block-storage default-type set362`↴](#osc-block-storage-default-type-set362)
* [`osc block-storage default-type show`↴](#osc-block-storage-default-type-show)
* [`osc block-storage extension`↴](#osc-block-storage-extension)
* [`osc block-storage extension list`↴](#osc-block-storage-extension-list)
* [`osc block-storage group`↴](#osc-block-storage-group)
* [`osc block-storage group create313`↴](#osc-block-storage-group-create313)
* [`osc block-storage group create-from-src314`↴](#osc-block-storage-group-create-from-src314)
* [`osc block-storage group delete313`↴](#osc-block-storage-group-delete313)
* [`osc block-storage group disable-replication338`↴](#osc-block-storage-group-disable-replication338)
* [`osc block-storage group enable-replication338`↴](#osc-block-storage-group-enable-replication338)
* [`osc block-storage group failover-replication338`↴](#osc-block-storage-group-failover-replication338)
* [`osc block-storage group list`↴](#osc-block-storage-group-list)
* [`osc block-storage group list-replication-targets338`↴](#osc-block-storage-group-list-replication-targets338)
* [`osc block-storage group reset-status320`↴](#osc-block-storage-group-reset-status320)
* [`osc block-storage group set313`↴](#osc-block-storage-group-set313)
* [`osc block-storage group show`↴](#osc-block-storage-group-show)
* [`osc block-storage group-snapshot`↴](#osc-block-storage-group-snapshot)
* [`osc block-storage group-snapshot create314`↴](#osc-block-storage-group-snapshot-create314)
* [`osc block-storage group-snapshot delete`↴](#osc-block-storage-group-snapshot-delete)
* [`osc block-storage group-snapshot list`↴](#osc-block-storage-group-snapshot-list)
* [`osc block-storage group-snapshot reset-status319`↴](#osc-block-storage-group-snapshot-reset-status319)
* [`osc block-storage group-snapshot show`↴](#osc-block-storage-group-snapshot-show)
* [`osc block-storage group-type`↴](#osc-block-storage-group-type)
* [`osc block-storage group-type create311`↴](#osc-block-storage-group-type-create311)
* [`osc block-storage group-type delete`↴](#osc-block-storage-group-type-delete)
* [`osc block-storage group-type group-spec`↴](#osc-block-storage-group-type-group-spec)
* [`osc block-storage group-type group-spec create311`↴](#osc-block-storage-group-type-group-spec-create311)
* [`osc block-storage group-type group-spec delete`↴](#osc-block-storage-group-type-group-spec-delete)
* [`osc block-storage group-type group-spec list`↴](#osc-block-storage-group-type-group-spec-list)
* [`osc block-storage group-type group-spec set311`↴](#osc-block-storage-group-type-group-spec-set311)
* [`osc block-storage group-type group-spec show`↴](#osc-block-storage-group-type-group-spec-show)
* [`osc block-storage group-type list`↴](#osc-block-storage-group-type-list)
* [`osc block-storage group-type set311`↴](#osc-block-storage-group-type-set311)
* [`osc block-storage group-type show`↴](#osc-block-storage-group-type-show)
* [`osc block-storage host`↴](#osc-block-storage-host)
* [`osc block-storage host list`↴](#osc-block-storage-host-list)
* [`osc block-storage host show`↴](#osc-block-storage-host-show)
* [`osc block-storage limit`↴](#osc-block-storage-limit)
* [`osc block-storage limit list`↴](#osc-block-storage-limit-list)
* [`osc block-storage message`↴](#osc-block-storage-message)
* [`osc block-storage message delete`↴](#osc-block-storage-message-delete)
* [`osc block-storage message list`↴](#osc-block-storage-message-list)
* [`osc block-storage message show`↴](#osc-block-storage-message-show)
* [`osc block-storage os-volume-transfer`↴](#osc-block-storage-os-volume-transfer)
* [`osc block-storage os-volume-transfer accept`↴](#osc-block-storage-os-volume-transfer-accept)
* [`osc block-storage os-volume-transfer create`↴](#osc-block-storage-os-volume-transfer-create)
* [`osc block-storage os-volume-transfer delete`↴](#osc-block-storage-os-volume-transfer-delete)
* [`osc block-storage os-volume-transfer list`↴](#osc-block-storage-os-volume-transfer-list)
* [`osc block-storage os-volume-transfer show`↴](#osc-block-storage-os-volume-transfer-show)
* [`osc block-storage qos-spec`↴](#osc-block-storage-qos-spec)
* [`osc block-storage qos-spec association`↴](#osc-block-storage-qos-spec-association)
* [`osc block-storage qos-spec association list`↴](#osc-block-storage-qos-spec-association-list)
* [`osc block-storage qos-spec associate`↴](#osc-block-storage-qos-spec-associate)
* [`osc block-storage qos-spec create`↴](#osc-block-storage-qos-spec-create)
* [`osc block-storage qos-spec delete`↴](#osc-block-storage-qos-spec-delete)
* [`osc block-storage qos-spec delete-keys`↴](#osc-block-storage-qos-spec-delete-keys)
* [`osc block-storage qos-spec disassociate`↴](#osc-block-storage-qos-spec-disassociate)
* [`osc block-storage qos-spec disassociate-all`↴](#osc-block-storage-qos-spec-disassociate-all)
* [`osc block-storage qos-spec list`↴](#osc-block-storage-qos-spec-list)
* [`osc block-storage qos-spec set`↴](#osc-block-storage-qos-spec-set)
* [`osc block-storage qos-spec show`↴](#osc-block-storage-qos-spec-show)
* [`osc block-storage snapshot`↴](#osc-block-storage-snapshot)
* [`osc block-storage snapshot create`↴](#osc-block-storage-snapshot-create)
* [`osc block-storage snapshot delete`↴](#osc-block-storage-snapshot-delete)
* [`osc block-storage snapshot force-delete`↴](#osc-block-storage-snapshot-force-delete)
* [`osc block-storage snapshot list`↴](#osc-block-storage-snapshot-list)
* [`osc block-storage snapshot reset-status`↴](#osc-block-storage-snapshot-reset-status)
* [`osc block-storage snapshot set`↴](#osc-block-storage-snapshot-set)
* [`osc block-storage snapshot show`↴](#osc-block-storage-snapshot-show)
* [`osc block-storage snapshot unmanage`↴](#osc-block-storage-snapshot-unmanage)
* [`osc block-storage snapshot update-status`↴](#osc-block-storage-snapshot-update-status)
* [`osc block-storage snapshot-manage`↴](#osc-block-storage-snapshot-manage)
* [`osc block-storage snapshot-manage create`↴](#osc-block-storage-snapshot-manage-create)
* [`osc block-storage snapshot-manage list`↴](#osc-block-storage-snapshot-manage-list)
* [`osc block-storage resource-filter`↴](#osc-block-storage-resource-filter)
* [`osc block-storage resource-filter list`↴](#osc-block-storage-resource-filter-list)
* [`osc block-storage type`↴](#osc-block-storage-type)
* [`osc block-storage type add-project-access`↴](#osc-block-storage-type-add-project-access)
* [`osc block-storage type create`↴](#osc-block-storage-type-create)
* [`osc block-storage type delete`↴](#osc-block-storage-type-delete)
* [`osc block-storage type encryption`↴](#osc-block-storage-type-encryption)
* [`osc block-storage type encryption create`↴](#osc-block-storage-type-encryption-create)
* [`osc block-storage type encryption delete`↴](#osc-block-storage-type-encryption-delete)
* [`osc block-storage type encryption list`↴](#osc-block-storage-type-encryption-list)
* [`osc block-storage type encryption set`↴](#osc-block-storage-type-encryption-set)
* [`osc block-storage type encryption show`↴](#osc-block-storage-type-encryption-show)
* [`osc block-storage type extraspecs`↴](#osc-block-storage-type-extraspecs)
* [`osc block-storage type extraspecs create`↴](#osc-block-storage-type-extraspecs-create)
* [`osc block-storage type extraspecs delete`↴](#osc-block-storage-type-extraspecs-delete)
* [`osc block-storage type extraspecs list`↴](#osc-block-storage-type-extraspecs-list)
* [`osc block-storage type extraspecs show`↴](#osc-block-storage-type-extraspecs-show)
* [`osc block-storage type extraspecs set`↴](#osc-block-storage-type-extraspecs-set)
* [`osc block-storage type list`↴](#osc-block-storage-type-list)
* [`osc block-storage type remove-project-access`↴](#osc-block-storage-type-remove-project-access)
* [`osc block-storage type set`↴](#osc-block-storage-type-set)
* [`osc block-storage type show`↴](#osc-block-storage-type-show)
* [`osc block-storage volume`↴](#osc-block-storage-volume)
* [`osc block-storage volume create353`↴](#osc-block-storage-volume-create353)
* [`osc block-storage volume create347`↴](#osc-block-storage-volume-create347)
* [`osc block-storage volume create313`↴](#osc-block-storage-volume-create313)
* [`osc block-storage volume create30`↴](#osc-block-storage-volume-create30)
* [`osc block-storage volume delete`↴](#osc-block-storage-volume-delete)
* [`osc block-storage volume extend`↴](#osc-block-storage-volume-extend)
* [`osc block-storage volume list`↴](#osc-block-storage-volume-list)
* [`osc block-storage volume metadata`↴](#osc-block-storage-volume-metadata)
* [`osc block-storage volume metadata create`↴](#osc-block-storage-volume-metadata-create)
* [`osc block-storage volume metadata delete`↴](#osc-block-storage-volume-metadata-delete)
* [`osc block-storage volume metadata list`↴](#osc-block-storage-volume-metadata-list)
* [`osc block-storage volume metadata replace`↴](#osc-block-storage-volume-metadata-replace)
* [`osc block-storage volume metadata set`↴](#osc-block-storage-volume-metadata-set)
* [`osc block-storage volume metadata show`↴](#osc-block-storage-volume-metadata-show)
* [`osc block-storage volume set353`↴](#osc-block-storage-volume-set353)
* [`osc block-storage volume set30`↴](#osc-block-storage-volume-set30)
* [`osc block-storage volume show`↴](#osc-block-storage-volume-show)
* [`osc block-storage volume-manage`↴](#osc-block-storage-volume-manage)
* [`osc block-storage volume-manage create316`↴](#osc-block-storage-volume-manage-create316)
* [`osc block-storage volume-manage create30`↴](#osc-block-storage-volume-manage-create30)
* [`osc block-storage volume-manage list`↴](#osc-block-storage-volume-manage-list)
* [`osc block-storage volume-transfer`↴](#osc-block-storage-volume-transfer)
* [`osc block-storage volume-transfer accept`↴](#osc-block-storage-volume-transfer-accept)
* [`osc block-storage volume-transfer create355`↴](#osc-block-storage-volume-transfer-create355)
* [`osc block-storage volume-transfer delete`↴](#osc-block-storage-volume-transfer-delete)
* [`osc block-storage volume-transfer list`↴](#osc-block-storage-volume-transfer-list)
* [`osc block-storage volume-transfer show`↴](#osc-block-storage-volume-transfer-show)
* [`osc catalog`↴](#osc-catalog)
* [`osc catalog list`↴](#osc-catalog-list)
* [`osc compute`↴](#osc-compute)
* [`osc compute aggregate`↴](#osc-compute-aggregate)
* [`osc compute aggregate add-host`↴](#osc-compute-aggregate-add-host)
* [`osc compute aggregate create21`↴](#osc-compute-aggregate-create21)
* [`osc compute aggregate cache-image`↴](#osc-compute-aggregate-cache-image)
* [`osc compute aggregate delete`↴](#osc-compute-aggregate-delete)
* [`osc compute aggregate list`↴](#osc-compute-aggregate-list)
* [`osc compute aggregate remove-host`↴](#osc-compute-aggregate-remove-host)
* [`osc compute aggregate show`↴](#osc-compute-aggregate-show)
* [`osc compute aggregate set21`↴](#osc-compute-aggregate-set21)
* [`osc compute aggregate set-metadata`↴](#osc-compute-aggregate-set-metadata)
* [`osc compute availability-zone`↴](#osc-compute-availability-zone)
* [`osc compute availability-zone list`↴](#osc-compute-availability-zone-list)
* [`osc compute availability-zone list-detail`↴](#osc-compute-availability-zone-list-detail)
* [`osc compute extension`↴](#osc-compute-extension)
* [`osc compute extension list`↴](#osc-compute-extension-list)
* [`osc compute extension show`↴](#osc-compute-extension-show)
* [`osc compute flavor`↴](#osc-compute-flavor)
* [`osc compute flavor access`↴](#osc-compute-flavor-access)
* [`osc compute flavor access add`↴](#osc-compute-flavor-access-add)
* [`osc compute flavor access list`↴](#osc-compute-flavor-access-list)
* [`osc compute flavor access remove`↴](#osc-compute-flavor-access-remove)
* [`osc compute flavor create255`↴](#osc-compute-flavor-create255)
* [`osc compute flavor create21`↴](#osc-compute-flavor-create21)
* [`osc compute flavor create20`↴](#osc-compute-flavor-create20)
* [`osc compute flavor delete`↴](#osc-compute-flavor-delete)
* [`osc compute flavor extraspecs`↴](#osc-compute-flavor-extraspecs)
* [`osc compute flavor extraspecs create`↴](#osc-compute-flavor-extraspecs-create)
* [`osc compute flavor extraspecs delete`↴](#osc-compute-flavor-extraspecs-delete)
* [`osc compute flavor extraspecs list`↴](#osc-compute-flavor-extraspecs-list)
* [`osc compute flavor extraspecs show`↴](#osc-compute-flavor-extraspecs-show)
* [`osc compute flavor extraspecs set`↴](#osc-compute-flavor-extraspecs-set)
* [`osc compute flavor list`↴](#osc-compute-flavor-list)
* [`osc compute flavor set255`↴](#osc-compute-flavor-set255)
* [`osc compute flavor show`↴](#osc-compute-flavor-show)
* [`osc compute hypervisor`↴](#osc-compute-hypervisor)
* [`osc compute hypervisor list`↴](#osc-compute-hypervisor-list)
* [`osc compute hypervisor show`↴](#osc-compute-hypervisor-show)
* [`osc compute keypair`↴](#osc-compute-keypair)
* [`osc compute keypair create292`↴](#osc-compute-keypair-create292)
* [`osc compute keypair create210`↴](#osc-compute-keypair-create210)
* [`osc compute keypair create22`↴](#osc-compute-keypair-create22)
* [`osc compute keypair create21`↴](#osc-compute-keypair-create21)
* [`osc compute keypair create20`↴](#osc-compute-keypair-create20)
* [`osc compute keypair delete`↴](#osc-compute-keypair-delete)
* [`osc compute keypair list`↴](#osc-compute-keypair-list)
* [`osc compute keypair show`↴](#osc-compute-keypair-show)
* [`osc compute server`↴](#osc-compute-server)
* [`osc compute server add-fixed-ip21`↴](#osc-compute-server-add-fixed-ip21)
* [`osc compute server add-floating-ip21`↴](#osc-compute-server-add-floating-ip21)
* [`osc compute server add-security-group`↴](#osc-compute-server-add-security-group)
* [`osc compute server change-password`↴](#osc-compute-server-change-password)
* [`osc compute server confirm-resize`↴](#osc-compute-server-confirm-resize)
* [`osc compute server create294`↴](#osc-compute-server-create294)
* [`osc compute server create290`↴](#osc-compute-server-create290)
* [`osc compute server create274`↴](#osc-compute-server-create274)
* [`osc compute server create267`↴](#osc-compute-server-create267)
* [`osc compute server create263`↴](#osc-compute-server-create263)
* [`osc compute server create257`↴](#osc-compute-server-create257)
* [`osc compute server create252`↴](#osc-compute-server-create252)
* [`osc compute server create242`↴](#osc-compute-server-create242)
* [`osc compute server create237`↴](#osc-compute-server-create237)
* [`osc compute server create233`↴](#osc-compute-server-create233)
* [`osc compute server create232`↴](#osc-compute-server-create232)
* [`osc compute server create219`↴](#osc-compute-server-create219)
* [`osc compute server create21`↴](#osc-compute-server-create21)
* [`osc compute server create-backup21`↴](#osc-compute-server-create-backup21)
* [`osc compute server create-image21`↴](#osc-compute-server-create-image21)
* [`osc compute server delete`↴](#osc-compute-server-delete)
* [`osc compute server diagnostic`↴](#osc-compute-server-diagnostic)
* [`osc compute server evacuate214`↴](#osc-compute-server-evacuate214)
* [`osc compute server evacuate229`↴](#osc-compute-server-evacuate229)
* [`osc compute server evacuate268`↴](#osc-compute-server-evacuate268)
* [`osc compute server evacuate295`↴](#osc-compute-server-evacuate295)
* [`osc compute server force-delete`↴](#osc-compute-server-force-delete)
* [`osc compute server get-console-output`↴](#osc-compute-server-get-console-output)
* [`osc compute server instance-action`↴](#osc-compute-server-instance-action)
* [`osc compute server instance-action list`↴](#osc-compute-server-instance-action-list)
* [`osc compute server instance-action show`↴](#osc-compute-server-instance-action-show)
* [`osc compute server interface`↴](#osc-compute-server-interface)
* [`osc compute server interface create249`↴](#osc-compute-server-interface-create249)
* [`osc compute server interface delete`↴](#osc-compute-server-interface-delete)
* [`osc compute server interface list`↴](#osc-compute-server-interface-list)
* [`osc compute server interface show`↴](#osc-compute-server-interface-show)
* [`osc compute server inject-network-info`↴](#osc-compute-server-inject-network-info)
* [`osc compute server ip`↴](#osc-compute-server-ip)
* [`osc compute server ip list`↴](#osc-compute-server-ip-list)
* [`osc compute server ip show`↴](#osc-compute-server-ip-show)
* [`osc compute server list`↴](#osc-compute-server-list)
* [`osc compute server live-migrate20`↴](#osc-compute-server-live-migrate20)
* [`osc compute server live-migrate225`↴](#osc-compute-server-live-migrate225)
* [`osc compute server live-migrate230`↴](#osc-compute-server-live-migrate230)
* [`osc compute server live-migrate268`↴](#osc-compute-server-live-migrate268)
* [`osc compute server lock273`↴](#osc-compute-server-lock273)
* [`osc compute server metadata`↴](#osc-compute-server-metadata)
* [`osc compute server metadata create`↴](#osc-compute-server-metadata-create)
* [`osc compute server metadata delete`↴](#osc-compute-server-metadata-delete)
* [`osc compute server metadata list`↴](#osc-compute-server-metadata-list)
* [`osc compute server metadata replace`↴](#osc-compute-server-metadata-replace)
* [`osc compute server metadata set`↴](#osc-compute-server-metadata-set)
* [`osc compute server metadata show`↴](#osc-compute-server-metadata-show)
* [`osc compute server migrate256`↴](#osc-compute-server-migrate256)
* [`osc compute server migration`↴](#osc-compute-server-migration)
* [`osc compute server migration delete`↴](#osc-compute-server-migration-delete)
* [`osc compute server migration force-complete222`↴](#osc-compute-server-migration-force-complete222)
* [`osc compute server migration list`↴](#osc-compute-server-migration-list)
* [`osc compute server migration show`↴](#osc-compute-server-migration-show)
* [`osc compute server password`↴](#osc-compute-server-password)
* [`osc compute server password delete`↴](#osc-compute-server-password-delete)
* [`osc compute server password show`↴](#osc-compute-server-password-show)
* [`osc compute server pause`↴](#osc-compute-server-pause)
* [`osc compute server reset-state`↴](#osc-compute-server-reset-state)
* [`osc compute server reboot`↴](#osc-compute-server-reboot)
* [`osc compute server rebuild21`↴](#osc-compute-server-rebuild21)
* [`osc compute server rebuild219`↴](#osc-compute-server-rebuild219)
* [`osc compute server rebuild254`↴](#osc-compute-server-rebuild254)
* [`osc compute server rebuild257`↴](#osc-compute-server-rebuild257)
* [`osc compute server rebuild263`↴](#osc-compute-server-rebuild263)
* [`osc compute server rebuild290`↴](#osc-compute-server-rebuild290)
* [`osc compute server rebuild294`↴](#osc-compute-server-rebuild294)
* [`osc compute server remote-console`↴](#osc-compute-server-remote-console)
* [`osc compute server remote-console create26`↴](#osc-compute-server-remote-console-create26)
* [`osc compute server remote-console create28`↴](#osc-compute-server-remote-console-create28)
* [`osc compute server remove-fixed-ip21`↴](#osc-compute-server-remove-fixed-ip21)
* [`osc compute server remove-floating-ip21`↴](#osc-compute-server-remove-floating-ip21)
* [`osc compute server remove-security-group`↴](#osc-compute-server-remove-security-group)
* [`osc compute server rescue`↴](#osc-compute-server-rescue)
* [`osc compute server reset-network`↴](#osc-compute-server-reset-network)
* [`osc compute server resize`↴](#osc-compute-server-resize)
* [`osc compute server restore`↴](#osc-compute-server-restore)
* [`osc compute server resume`↴](#osc-compute-server-resume)
* [`osc compute server revert-resize`↴](#osc-compute-server-revert-resize)
* [`osc compute server security-groups`↴](#osc-compute-server-security-groups)
* [`osc compute server set21`↴](#osc-compute-server-set21)
* [`osc compute server set219`↴](#osc-compute-server-set219)
* [`osc compute server set290`↴](#osc-compute-server-set290)
* [`osc compute server set294`↴](#osc-compute-server-set294)
* [`osc compute server shelve`↴](#osc-compute-server-shelve)
* [`osc compute server shelve-offload`↴](#osc-compute-server-shelve-offload)
* [`osc compute server show`↴](#osc-compute-server-show)
* [`osc compute server start`↴](#osc-compute-server-start)
* [`osc compute server stop`↴](#osc-compute-server-stop)
* [`osc compute server suspend`↴](#osc-compute-server-suspend)
* [`osc compute server tag`↴](#osc-compute-server-tag)
* [`osc compute server tag add`↴](#osc-compute-server-tag-add)
* [`osc compute server tag check`↴](#osc-compute-server-tag-check)
* [`osc compute server tag delete`↴](#osc-compute-server-tag-delete)
* [`osc compute server tag list`↴](#osc-compute-server-tag-list)
* [`osc compute server tag purge`↴](#osc-compute-server-tag-purge)
* [`osc compute server tag replace226`↴](#osc-compute-server-tag-replace226)
* [`osc compute server topology`↴](#osc-compute-server-topology)
* [`osc compute server trigger-crash-dump217`↴](#osc-compute-server-trigger-crash-dump217)
* [`osc compute server unlock21`↴](#osc-compute-server-unlock21)
* [`osc compute server unpause`↴](#osc-compute-server-unpause)
* [`osc compute server unrescue`↴](#osc-compute-server-unrescue)
* [`osc compute server unshelve277`↴](#osc-compute-server-unshelve277)
* [`osc compute server unshelve291`↴](#osc-compute-server-unshelve291)
* [`osc compute server volume-attachment`↴](#osc-compute-server-volume-attachment)
* [`osc compute server volume-attachment create20`↴](#osc-compute-server-volume-attachment-create20)
* [`osc compute server volume-attachment create249`↴](#osc-compute-server-volume-attachment-create249)
* [`osc compute server volume-attachment create279`↴](#osc-compute-server-volume-attachment-create279)
* [`osc compute server volume-attachment delete`↴](#osc-compute-server-volume-attachment-delete)
* [`osc compute server volume-attachment list`↴](#osc-compute-server-volume-attachment-list)
* [`osc compute server volume-attachment set20`↴](#osc-compute-server-volume-attachment-set20)
* [`osc compute server volume-attachment set285`↴](#osc-compute-server-volume-attachment-set285)
* [`osc compute server volume-attachment show`↴](#osc-compute-server-volume-attachment-show)
* [`osc identity`↴](#osc-identity)
* [`osc identity auth`↴](#osc-identity-auth)
* [`osc identity auth catalog`↴](#osc-identity-auth-catalog)
* [`osc identity auth catalog list`↴](#osc-identity-auth-catalog-list)
* [`osc identity auth domain`↴](#osc-identity-auth-domain)
* [`osc identity auth domain list`↴](#osc-identity-auth-domain-list)
* [`osc identity auth federation`↴](#osc-identity-auth-federation)
* [`osc identity auth federation identity-provider`↴](#osc-identity-auth-federation-identity-provider)
* [`osc identity auth federation identity-provider protocol`↴](#osc-identity-auth-federation-identity-provider-protocol)
* [`osc identity auth federation identity-provider protocol websso`↴](#osc-identity-auth-federation-identity-provider-protocol-websso)
* [`osc identity auth federation identity-provider protocol websso create`↴](#osc-identity-auth-federation-identity-provider-protocol-websso-create)
* [`osc identity auth federation identity-provider protocol websso get`↴](#osc-identity-auth-federation-identity-provider-protocol-websso-get)
* [`osc identity auth federation websso`↴](#osc-identity-auth-federation-websso)
* [`osc identity auth federation websso create`↴](#osc-identity-auth-federation-websso-create)
* [`osc identity auth federation websso show`↴](#osc-identity-auth-federation-websso-show)
* [`osc identity auth project`↴](#osc-identity-auth-project)
* [`osc identity auth project list`↴](#osc-identity-auth-project-list)
* [`osc identity auth system`↴](#osc-identity-auth-system)
* [`osc identity auth system list`↴](#osc-identity-auth-system-list)
* [`osc identity auth token`↴](#osc-identity-auth-token)
* [`osc identity auth token delete`↴](#osc-identity-auth-token-delete)
* [`osc identity auth token get`↴](#osc-identity-auth-token-get)
* [`osc identity access-rule`↴](#osc-identity-access-rule)
* [`osc identity access-rule delete`↴](#osc-identity-access-rule-delete)
* [`osc identity access-rule list`↴](#osc-identity-access-rule-list)
* [`osc identity access-rule show`↴](#osc-identity-access-rule-show)
* [`osc identity application-credential`↴](#osc-identity-application-credential)
* [`osc identity application-credential create`↴](#osc-identity-application-credential-create)
* [`osc identity application-credential delete`↴](#osc-identity-application-credential-delete)
* [`osc identity application-credential list`↴](#osc-identity-application-credential-list)
* [`osc identity application-credential show`↴](#osc-identity-application-credential-show)
* [`osc identity credential`↴](#osc-identity-credential)
* [`osc identity credential create`↴](#osc-identity-credential-create)
* [`osc identity credential delete`↴](#osc-identity-credential-delete)
* [`osc identity credential list`↴](#osc-identity-credential-list)
* [`osc identity credential set`↴](#osc-identity-credential-set)
* [`osc identity credential show`↴](#osc-identity-credential-show)
* [`osc identity domain`↴](#osc-identity-domain)
* [`osc identity domain create`↴](#osc-identity-domain-create)
* [`osc identity domain config`↴](#osc-identity-domain-config)
* [`osc identity domain config default`↴](#osc-identity-domain-config-default)
* [`osc identity domain config group`↴](#osc-identity-domain-config-group)
* [`osc identity domain config group default`↴](#osc-identity-domain-config-group-default)
* [`osc identity domain config group delete`↴](#osc-identity-domain-config-group-delete)
* [`osc identity domain config group option`↴](#osc-identity-domain-config-group-option)
* [`osc identity domain config group option default`↴](#osc-identity-domain-config-group-option-default)
* [`osc identity domain config group option delete`↴](#osc-identity-domain-config-group-option-delete)
* [`osc identity domain config group option set`↴](#osc-identity-domain-config-group-option-set)
* [`osc identity domain config group option show`↴](#osc-identity-domain-config-group-option-show)
* [`osc identity domain config group set`↴](#osc-identity-domain-config-group-set)
* [`osc identity domain config group show`↴](#osc-identity-domain-config-group-show)
* [`osc identity domain config purge`↴](#osc-identity-domain-config-purge)
* [`osc identity domain config list`↴](#osc-identity-domain-config-list)
* [`osc identity domain config replace`↴](#osc-identity-domain-config-replace)
* [`osc identity domain config set`↴](#osc-identity-domain-config-set)
* [`osc identity domain delete`↴](#osc-identity-domain-delete)
* [`osc identity domain group`↴](#osc-identity-domain-group)
* [`osc identity domain group role`↴](#osc-identity-domain-group-role)
* [`osc identity domain group role delete`↴](#osc-identity-domain-group-role-delete)
* [`osc identity domain group role list`↴](#osc-identity-domain-group-role-list)
* [`osc identity domain group role set`↴](#osc-identity-domain-group-role-set)
* [`osc identity domain group role show`↴](#osc-identity-domain-group-role-show)
* [`osc identity domain list`↴](#osc-identity-domain-list)
* [`osc identity domain set`↴](#osc-identity-domain-set)
* [`osc identity domain show`↴](#osc-identity-domain-show)
* [`osc identity domain user`↴](#osc-identity-domain-user)
* [`osc identity domain user role`↴](#osc-identity-domain-user-role)
* [`osc identity domain user role delete`↴](#osc-identity-domain-user-role-delete)
* [`osc identity domain user role list`↴](#osc-identity-domain-user-role-list)
* [`osc identity domain user role set`↴](#osc-identity-domain-user-role-set)
* [`osc identity domain user role show`↴](#osc-identity-domain-user-role-show)
* [`osc identity endpoint`↴](#osc-identity-endpoint)
* [`osc identity endpoint create`↴](#osc-identity-endpoint-create)
* [`osc identity endpoint delete`↴](#osc-identity-endpoint-delete)
* [`osc identity endpoint list`↴](#osc-identity-endpoint-list)
* [`osc identity endpoint set`↴](#osc-identity-endpoint-set)
* [`osc identity endpoint show`↴](#osc-identity-endpoint-show)
* [`osc identity federation`↴](#osc-identity-federation)
* [`osc identity federation identity-provider`↴](#osc-identity-federation-identity-provider)
* [`osc identity federation identity-provider create`↴](#osc-identity-federation-identity-provider-create)
* [`osc identity federation identity-provider delete`↴](#osc-identity-federation-identity-provider-delete)
* [`osc identity federation identity-provider list`↴](#osc-identity-federation-identity-provider-list)
* [`osc identity federation identity-provider protocol`↴](#osc-identity-federation-identity-provider-protocol)
* [`osc identity federation identity-provider protocol create`↴](#osc-identity-federation-identity-provider-protocol-create)
* [`osc identity federation identity-provider protocol delete`↴](#osc-identity-federation-identity-provider-protocol-delete)
* [`osc identity federation identity-provider protocol list`↴](#osc-identity-federation-identity-provider-protocol-list)
* [`osc identity federation identity-provider protocol set`↴](#osc-identity-federation-identity-provider-protocol-set)
* [`osc identity federation identity-provider protocol show`↴](#osc-identity-federation-identity-provider-protocol-show)
* [`osc identity federation identity-provider set`↴](#osc-identity-federation-identity-provider-set)
* [`osc identity federation identity-provider show`↴](#osc-identity-federation-identity-provider-show)
* [`osc identity federation mapping`↴](#osc-identity-federation-mapping)
* [`osc identity federation mapping create`↴](#osc-identity-federation-mapping-create)
* [`osc identity federation mapping delete`↴](#osc-identity-federation-mapping-delete)
* [`osc identity federation mapping list`↴](#osc-identity-federation-mapping-list)
* [`osc identity federation mapping set`↴](#osc-identity-federation-mapping-set)
* [`osc identity federation mapping show`↴](#osc-identity-federation-mapping-show)
* [`osc identity federation service-provider`↴](#osc-identity-federation-service-provider)
* [`osc identity federation service-provider create`↴](#osc-identity-federation-service-provider-create)
* [`osc identity federation service-provider delete`↴](#osc-identity-federation-service-provider-delete)
* [`osc identity federation service-provider list`↴](#osc-identity-federation-service-provider-list)
* [`osc identity federation service-provider set`↴](#osc-identity-federation-service-provider-set)
* [`osc identity federation service-provider show`↴](#osc-identity-federation-service-provider-show)
* [`osc identity federation saml2-metadata`↴](#osc-identity-federation-saml2-metadata)
* [`osc identity federation saml2-metadata show`↴](#osc-identity-federation-saml2-metadata-show)
* [`osc identity group`↴](#osc-identity-group)
* [`osc identity group create`↴](#osc-identity-group-create)
* [`osc identity group delete`↴](#osc-identity-group-delete)
* [`osc identity group list`↴](#osc-identity-group-list)
* [`osc identity group set`↴](#osc-identity-group-set)
* [`osc identity group show`↴](#osc-identity-group-show)
* [`osc identity project`↴](#osc-identity-project)
* [`osc identity project create`↴](#osc-identity-project-create)
* [`osc identity project delete`↴](#osc-identity-project-delete)
* [`osc identity project group`↴](#osc-identity-project-group)
* [`osc identity project group role`↴](#osc-identity-project-group-role)
* [`osc identity project group role delete`↴](#osc-identity-project-group-role-delete)
* [`osc identity project group role list`↴](#osc-identity-project-group-role-list)
* [`osc identity project group role set`↴](#osc-identity-project-group-role-set)
* [`osc identity project group role show`↴](#osc-identity-project-group-role-show)
* [`osc identity project list`↴](#osc-identity-project-list)
* [`osc identity project set`↴](#osc-identity-project-set)
* [`osc identity project show`↴](#osc-identity-project-show)
* [`osc identity project user`↴](#osc-identity-project-user)
* [`osc identity project user role`↴](#osc-identity-project-user-role)
* [`osc identity project user role delete`↴](#osc-identity-project-user-role-delete)
* [`osc identity project user role list`↴](#osc-identity-project-user-role-list)
* [`osc identity project user role set`↴](#osc-identity-project-user-role-set)
* [`osc identity project user role show`↴](#osc-identity-project-user-role-show)
* [`osc identity region`↴](#osc-identity-region)
* [`osc identity region create`↴](#osc-identity-region-create)
* [`osc identity region delete`↴](#osc-identity-region-delete)
* [`osc identity region list`↴](#osc-identity-region-list)
* [`osc identity region set`↴](#osc-identity-region-set)
* [`osc identity region show`↴](#osc-identity-region-show)
* [`osc identity role`↴](#osc-identity-role)
* [`osc identity role assignment`↴](#osc-identity-role-assignment)
* [`osc identity role assignment list`↴](#osc-identity-role-assignment-list)
* [`osc identity role create`↴](#osc-identity-role-create)
* [`osc identity role delete`↴](#osc-identity-role-delete)
* [`osc identity role imply`↴](#osc-identity-role-imply)
* [`osc identity role imply delete`↴](#osc-identity-role-imply-delete)
* [`osc identity role imply list`↴](#osc-identity-role-imply-list)
* [`osc identity role imply set`↴](#osc-identity-role-imply-set)
* [`osc identity role imply show`↴](#osc-identity-role-imply-show)
* [`osc identity role inference`↴](#osc-identity-role-inference)
* [`osc identity role inference list`↴](#osc-identity-role-inference-list)
* [`osc identity role list`↴](#osc-identity-role-list)
* [`osc identity role set`↴](#osc-identity-role-set)
* [`osc identity role show`↴](#osc-identity-role-show)
* [`osc identity role-assignment`↴](#osc-identity-role-assignment)
* [`osc identity role-assignment list`↴](#osc-identity-role-assignment-list)
* [`osc identity role-inference`↴](#osc-identity-role-inference)
* [`osc identity role-inference list`↴](#osc-identity-role-inference-list)
* [`osc identity service`↴](#osc-identity-service)
* [`osc identity service create`↴](#osc-identity-service-create)
* [`osc identity service delete`↴](#osc-identity-service-delete)
* [`osc identity service list`↴](#osc-identity-service-list)
* [`osc identity service set`↴](#osc-identity-service-set)
* [`osc identity service show`↴](#osc-identity-service-show)
* [`osc identity user`↴](#osc-identity-user)
* [`osc identity user create`↴](#osc-identity-user-create)
* [`osc identity user delete`↴](#osc-identity-user-delete)
* [`osc identity user groups`↴](#osc-identity-user-groups)
* [`osc identity user list`↴](#osc-identity-user-list)
* [`osc identity user password`↴](#osc-identity-user-password)
* [`osc identity user password set`↴](#osc-identity-user-password-set)
* [`osc identity user projects`↴](#osc-identity-user-projects)
* [`osc identity user set`↴](#osc-identity-user-set)
* [`osc identity user show`↴](#osc-identity-user-show)
* [`osc image`↴](#osc-image)
* [`osc image image`↴](#osc-image-image)
* [`osc image image create`↴](#osc-image-image-create)
* [`osc image image deactivate`↴](#osc-image-image-deactivate)
* [`osc image image delete`↴](#osc-image-image-delete)
* [`osc image image download`↴](#osc-image-image-download)
* [`osc image image list`↴](#osc-image-image-list)
* [`osc image image reactivate`↴](#osc-image-image-reactivate)
* [`osc image image set`↴](#osc-image-image-set)
* [`osc image image show`↴](#osc-image-image-show)
* [`osc image image upload`↴](#osc-image-image-upload)
* [`osc image metadef`↴](#osc-image-metadef)
* [`osc image metadef namespace`↴](#osc-image-metadef-namespace)
* [`osc image metadef namespace create`↴](#osc-image-metadef-namespace-create)
* [`osc image metadef namespace delete`↴](#osc-image-metadef-namespace-delete)
* [`osc image metadef namespace list`↴](#osc-image-metadef-namespace-list)
* [`osc image metadef namespace object`↴](#osc-image-metadef-namespace-object)
* [`osc image metadef namespace object create`↴](#osc-image-metadef-namespace-object-create)
* [`osc image metadef namespace object delete`↴](#osc-image-metadef-namespace-object-delete)
* [`osc image metadef namespace object list`↴](#osc-image-metadef-namespace-object-list)
* [`osc image metadef namespace object purge`↴](#osc-image-metadef-namespace-object-purge)
* [`osc image metadef namespace object set`↴](#osc-image-metadef-namespace-object-set)
* [`osc image metadef namespace object show`↴](#osc-image-metadef-namespace-object-show)
* [`osc image metadef namespace property`↴](#osc-image-metadef-namespace-property)
* [`osc image metadef namespace property create`↴](#osc-image-metadef-namespace-property-create)
* [`osc image metadef namespace property delete`↴](#osc-image-metadef-namespace-property-delete)
* [`osc image metadef namespace property list`↴](#osc-image-metadef-namespace-property-list)
* [`osc image metadef namespace property purge`↴](#osc-image-metadef-namespace-property-purge)
* [`osc image metadef namespace property set`↴](#osc-image-metadef-namespace-property-set)
* [`osc image metadef namespace property show`↴](#osc-image-metadef-namespace-property-show)
* [`osc image metadef namespace resource-type-association`↴](#osc-image-metadef-namespace-resource-type-association)
* [`osc image metadef namespace resource-type-association create`↴](#osc-image-metadef-namespace-resource-type-association-create)
* [`osc image metadef namespace resource-type-association delete`↴](#osc-image-metadef-namespace-resource-type-association-delete)
* [`osc image metadef namespace resource-type-association list`↴](#osc-image-metadef-namespace-resource-type-association-list)
* [`osc image metadef namespace set`↴](#osc-image-metadef-namespace-set)
* [`osc image metadef namespace show`↴](#osc-image-metadef-namespace-show)
* [`osc image metadef namespace tag`↴](#osc-image-metadef-namespace-tag)
* [`osc image metadef namespace tag create`↴](#osc-image-metadef-namespace-tag-create)
* [`osc image metadef namespace tag delete`↴](#osc-image-metadef-namespace-tag-delete)
* [`osc image metadef namespace tag list`↴](#osc-image-metadef-namespace-tag-list)
* [`osc image metadef namespace tag purge`↴](#osc-image-metadef-namespace-tag-purge)
* [`osc image metadef namespace tag set`↴](#osc-image-metadef-namespace-tag-set)
* [`osc image metadef namespace tag show`↴](#osc-image-metadef-namespace-tag-show)
* [`osc image metadef resource-type`↴](#osc-image-metadef-resource-type)
* [`osc image metadef resource-type list`↴](#osc-image-metadef-resource-type-list)
* [`osc image schema`↴](#osc-image-schema)
* [`osc image schema image`↴](#osc-image-schema-image)
* [`osc image schema image show`↴](#osc-image-schema-image-show)
* [`osc image schema images`↴](#osc-image-schema-images)
* [`osc image schema images show`↴](#osc-image-schema-images-show)
* [`osc image schema member`↴](#osc-image-schema-member)
* [`osc image schema member show`↴](#osc-image-schema-member-show)
* [`osc image schema members`↴](#osc-image-schema-members)
* [`osc image schema members show`↴](#osc-image-schema-members-show)
* [`osc image schema metadef`↴](#osc-image-schema-metadef)
* [`osc image schema metadef namespace`↴](#osc-image-schema-metadef-namespace)
* [`osc image schema metadef namespace show`↴](#osc-image-schema-metadef-namespace-show)
* [`osc image schema metadef namespaces`↴](#osc-image-schema-metadef-namespaces)
* [`osc image schema metadef namespaces show`↴](#osc-image-schema-metadef-namespaces-show)
* [`osc image schema metadef object`↴](#osc-image-schema-metadef-object)
* [`osc image schema metadef object show`↴](#osc-image-schema-metadef-object-show)
* [`osc image schema metadef objects`↴](#osc-image-schema-metadef-objects)
* [`osc image schema metadef objects show`↴](#osc-image-schema-metadef-objects-show)
* [`osc image schema metadef properties`↴](#osc-image-schema-metadef-properties)
* [`osc image schema metadef properties show`↴](#osc-image-schema-metadef-properties-show)
* [`osc image schema metadef property`↴](#osc-image-schema-metadef-property)
* [`osc image schema metadef property show`↴](#osc-image-schema-metadef-property-show)
* [`osc image schema metadef resource-type`↴](#osc-image-schema-metadef-resource-type)
* [`osc image schema metadef resource-type show`↴](#osc-image-schema-metadef-resource-type-show)
* [`osc image schema metadef resource-types`↴](#osc-image-schema-metadef-resource-types)
* [`osc image schema metadef resource-types show`↴](#osc-image-schema-metadef-resource-types-show)
* [`osc image schema metadef tag`↴](#osc-image-schema-metadef-tag)
* [`osc image schema metadef tag show`↴](#osc-image-schema-metadef-tag-show)
* [`osc image schema metadef tags`↴](#osc-image-schema-metadef-tags)
* [`osc image schema metadef tags show`↴](#osc-image-schema-metadef-tags-show)
* [`osc load-balancer`↴](#osc-load-balancer)
* [`osc load-balancer amphorae`↴](#osc-load-balancer-amphorae)
* [`osc load-balancer amphorae config`↴](#osc-load-balancer-amphorae-config)
* [`osc load-balancer amphorae delete`↴](#osc-load-balancer-amphorae-delete)
* [`osc load-balancer amphorae failover`↴](#osc-load-balancer-amphorae-failover)
* [`osc load-balancer amphorae list`↴](#osc-load-balancer-amphorae-list)
* [`osc load-balancer amphorae show`↴](#osc-load-balancer-amphorae-show)
* [`osc load-balancer amphorae stats`↴](#osc-load-balancer-amphorae-stats)
* [`osc load-balancer availability-zone`↴](#osc-load-balancer-availability-zone)
* [`osc load-balancer availability-zone create`↴](#osc-load-balancer-availability-zone-create)
* [`osc load-balancer availability-zone delete`↴](#osc-load-balancer-availability-zone-delete)
* [`osc load-balancer availability-zone list`↴](#osc-load-balancer-availability-zone-list)
* [`osc load-balancer availability-zone set`↴](#osc-load-balancer-availability-zone-set)
* [`osc load-balancer availability-zone show`↴](#osc-load-balancer-availability-zone-show)
* [`osc load-balancer availability-zone-profile`↴](#osc-load-balancer-availability-zone-profile)
* [`osc load-balancer availability-zone-profile create`↴](#osc-load-balancer-availability-zone-profile-create)
* [`osc load-balancer availability-zone-profile delete`↴](#osc-load-balancer-availability-zone-profile-delete)
* [`osc load-balancer availability-zone-profile list`↴](#osc-load-balancer-availability-zone-profile-list)
* [`osc load-balancer availability-zone-profile set`↴](#osc-load-balancer-availability-zone-profile-set)
* [`osc load-balancer availability-zone-profile show`↴](#osc-load-balancer-availability-zone-profile-show)
* [`osc load-balancer flavor`↴](#osc-load-balancer-flavor)
* [`osc load-balancer flavor create`↴](#osc-load-balancer-flavor-create)
* [`osc load-balancer flavor delete`↴](#osc-load-balancer-flavor-delete)
* [`osc load-balancer flavor list`↴](#osc-load-balancer-flavor-list)
* [`osc load-balancer flavor set`↴](#osc-load-balancer-flavor-set)
* [`osc load-balancer flavor show`↴](#osc-load-balancer-flavor-show)
* [`osc load-balancer flavor-profile`↴](#osc-load-balancer-flavor-profile)
* [`osc load-balancer flavor-profile create`↴](#osc-load-balancer-flavor-profile-create)
* [`osc load-balancer flavor-profile delete`↴](#osc-load-balancer-flavor-profile-delete)
* [`osc load-balancer flavor-profile list`↴](#osc-load-balancer-flavor-profile-list)
* [`osc load-balancer flavor-profile set`↴](#osc-load-balancer-flavor-profile-set)
* [`osc load-balancer flavor-profile show`↴](#osc-load-balancer-flavor-profile-show)
* [`osc load-balancer healthmonitor`↴](#osc-load-balancer-healthmonitor)
* [`osc load-balancer healthmonitor create`↴](#osc-load-balancer-healthmonitor-create)
* [`osc load-balancer healthmonitor delete`↴](#osc-load-balancer-healthmonitor-delete)
* [`osc load-balancer healthmonitor list`↴](#osc-load-balancer-healthmonitor-list)
* [`osc load-balancer healthmonitor set`↴](#osc-load-balancer-healthmonitor-set)
* [`osc load-balancer healthmonitor show`↴](#osc-load-balancer-healthmonitor-show)
* [`osc load-balancer l7policy`↴](#osc-load-balancer-l7policy)
* [`osc load-balancer l7policy create`↴](#osc-load-balancer-l7policy-create)
* [`osc load-balancer l7policy delete`↴](#osc-load-balancer-l7policy-delete)
* [`osc load-balancer l7policy list`↴](#osc-load-balancer-l7policy-list)
* [`osc load-balancer l7policy rule`↴](#osc-load-balancer-l7policy-rule)
* [`osc load-balancer l7policy rule create`↴](#osc-load-balancer-l7policy-rule-create)
* [`osc load-balancer l7policy rule delete`↴](#osc-load-balancer-l7policy-rule-delete)
* [`osc load-balancer l7policy rule list`↴](#osc-load-balancer-l7policy-rule-list)
* [`osc load-balancer l7policy rule set`↴](#osc-load-balancer-l7policy-rule-set)
* [`osc load-balancer l7policy rule show`↴](#osc-load-balancer-l7policy-rule-show)
* [`osc load-balancer l7policy set`↴](#osc-load-balancer-l7policy-set)
* [`osc load-balancer l7policy show`↴](#osc-load-balancer-l7policy-show)
* [`osc load-balancer listener`↴](#osc-load-balancer-listener)
* [`osc load-balancer listener create`↴](#osc-load-balancer-listener-create)
* [`osc load-balancer listener delete`↴](#osc-load-balancer-listener-delete)
* [`osc load-balancer listener list`↴](#osc-load-balancer-listener-list)
* [`osc load-balancer listener set`↴](#osc-load-balancer-listener-set)
* [`osc load-balancer listener show`↴](#osc-load-balancer-listener-show)
* [`osc load-balancer listener stats`↴](#osc-load-balancer-listener-stats)
* [`osc load-balancer loadbalancer`↴](#osc-load-balancer-loadbalancer)
* [`osc load-balancer loadbalancer create`↴](#osc-load-balancer-loadbalancer-create)
* [`osc load-balancer loadbalancer delete`↴](#osc-load-balancer-loadbalancer-delete)
* [`osc load-balancer loadbalancer failover`↴](#osc-load-balancer-loadbalancer-failover)
* [`osc load-balancer loadbalancer list`↴](#osc-load-balancer-loadbalancer-list)
* [`osc load-balancer loadbalancer set`↴](#osc-load-balancer-loadbalancer-set)
* [`osc load-balancer loadbalancer show`↴](#osc-load-balancer-loadbalancer-show)
* [`osc load-balancer loadbalancer stats`↴](#osc-load-balancer-loadbalancer-stats)
* [`osc load-balancer loadbalancer status`↴](#osc-load-balancer-loadbalancer-status)
* [`osc load-balancer pool`↴](#osc-load-balancer-pool)
* [`osc load-balancer pool create`↴](#osc-load-balancer-pool-create)
* [`osc load-balancer pool delete`↴](#osc-load-balancer-pool-delete)
* [`osc load-balancer pool list`↴](#osc-load-balancer-pool-list)
* [`osc load-balancer pool member`↴](#osc-load-balancer-pool-member)
* [`osc load-balancer pool member create`↴](#osc-load-balancer-pool-member-create)
* [`osc load-balancer pool member delete`↴](#osc-load-balancer-pool-member-delete)
* [`osc load-balancer pool member list`↴](#osc-load-balancer-pool-member-list)
* [`osc load-balancer pool member set`↴](#osc-load-balancer-pool-member-set)
* [`osc load-balancer pool member show`↴](#osc-load-balancer-pool-member-show)
* [`osc load-balancer pool set`↴](#osc-load-balancer-pool-set)
* [`osc load-balancer pool show`↴](#osc-load-balancer-pool-show)
* [`osc load-balancer provider`↴](#osc-load-balancer-provider)
* [`osc load-balancer provider availability-zone-capability`↴](#osc-load-balancer-provider-availability-zone-capability)
* [`osc load-balancer provider availability-zone-capability list`↴](#osc-load-balancer-provider-availability-zone-capability-list)
* [`osc load-balancer provider flavor-capability`↴](#osc-load-balancer-provider-flavor-capability)
* [`osc load-balancer provider flavor-capability list`↴](#osc-load-balancer-provider-flavor-capability-list)
* [`osc load-balancer provider list`↴](#osc-load-balancer-provider-list)
* [`osc load-balancer quota`↴](#osc-load-balancer-quota)
* [`osc load-balancer quota delete`↴](#osc-load-balancer-quota-delete)
* [`osc load-balancer quota list`↴](#osc-load-balancer-quota-list)
* [`osc load-balancer quota set`↴](#osc-load-balancer-quota-set)
* [`osc load-balancer quota show`↴](#osc-load-balancer-quota-show)
* [`osc load-balancer version`↴](#osc-load-balancer-version)
* [`osc load-balancer version get`↴](#osc-load-balancer-version-get)
* [`osc network`↴](#osc-network)
* [`osc network address-group`↴](#osc-network-address-group)
* [`osc network address-group add-address`↴](#osc-network-address-group-add-address)
* [`osc network address-group create`↴](#osc-network-address-group-create)
* [`osc network address-group delete`↴](#osc-network-address-group-delete)
* [`osc network address-group list`↴](#osc-network-address-group-list)
* [`osc network address-group remove-address`↴](#osc-network-address-group-remove-address)
* [`osc network address-group set`↴](#osc-network-address-group-set)
* [`osc network address-group show`↴](#osc-network-address-group-show)
* [`osc network address-scope`↴](#osc-network-address-scope)
* [`osc network address-scope create`↴](#osc-network-address-scope-create)
* [`osc network address-scope delete`↴](#osc-network-address-scope-delete)
* [`osc network address-scope list`↴](#osc-network-address-scope-list)
* [`osc network address-scope set`↴](#osc-network-address-scope-set)
* [`osc network address-scope show`↴](#osc-network-address-scope-show)
* [`osc network availability-zone`↴](#osc-network-availability-zone)
* [`osc network availability-zone list`↴](#osc-network-availability-zone-list)
* [`osc network extension`↴](#osc-network-extension)
* [`osc network extension list`↴](#osc-network-extension-list)
* [`osc network extension show`↴](#osc-network-extension-show)
* [`osc network floating-ip`↴](#osc-network-floating-ip)
* [`osc network floating-ip create`↴](#osc-network-floating-ip-create)
* [`osc network floating-ip delete`↴](#osc-network-floating-ip-delete)
* [`osc network floating-ip list`↴](#osc-network-floating-ip-list)
* [`osc network floating-ip port-forwarding`↴](#osc-network-floating-ip-port-forwarding)
* [`osc network floating-ip port-forwarding create`↴](#osc-network-floating-ip-port-forwarding-create)
* [`osc network floating-ip port-forwarding delete`↴](#osc-network-floating-ip-port-forwarding-delete)
* [`osc network floating-ip port-forwarding list`↴](#osc-network-floating-ip-port-forwarding-list)
* [`osc network floating-ip port-forwarding set`↴](#osc-network-floating-ip-port-forwarding-set)
* [`osc network floating-ip port-forwarding show`↴](#osc-network-floating-ip-port-forwarding-show)
* [`osc network floating-ip set`↴](#osc-network-floating-ip-set)
* [`osc network floating-ip show`↴](#osc-network-floating-ip-show)
* [`osc network floating-ip tag`↴](#osc-network-floating-ip-tag)
* [`osc network floating-ip tag add`↴](#osc-network-floating-ip-tag-add)
* [`osc network floating-ip tag check`↴](#osc-network-floating-ip-tag-check)
* [`osc network floating-ip tag delete`↴](#osc-network-floating-ip-tag-delete)
* [`osc network floating-ip tag list`↴](#osc-network-floating-ip-tag-list)
* [`osc network floating-ip tag purge`↴](#osc-network-floating-ip-tag-purge)
* [`osc network network`↴](#osc-network-network)
* [`osc network network create`↴](#osc-network-network-create)
* [`osc network network delete`↴](#osc-network-network-delete)
* [`osc network network dhcp-agent`↴](#osc-network-network-dhcp-agent)
* [`osc network network dhcp-agent list`↴](#osc-network-network-dhcp-agent-list)
* [`osc network network list`↴](#osc-network-network-list)
* [`osc network network show`↴](#osc-network-network-show)
* [`osc network network tag`↴](#osc-network-network-tag)
* [`osc network network tag add`↴](#osc-network-network-tag-add)
* [`osc network network tag check`↴](#osc-network-network-tag-check)
* [`osc network network tag delete`↴](#osc-network-network-tag-delete)
* [`osc network network tag list`↴](#osc-network-network-tag-list)
* [`osc network network tag purge`↴](#osc-network-network-tag-purge)
* [`osc network port`↴](#osc-network-port)
* [`osc network port create`↴](#osc-network-port-create)
* [`osc network port delete`↴](#osc-network-port-delete)
* [`osc network port list`↴](#osc-network-port-list)
* [`osc network port show`↴](#osc-network-port-show)
* [`osc network port tag`↴](#osc-network-port-tag)
* [`osc network port tag add`↴](#osc-network-port-tag-add)
* [`osc network port tag check`↴](#osc-network-port-tag-check)
* [`osc network port tag delete`↴](#osc-network-port-tag-delete)
* [`osc network port tag list`↴](#osc-network-port-tag-list)
* [`osc network port tag purge`↴](#osc-network-port-tag-purge)
* [`osc network rbac-policy`↴](#osc-network-rbac-policy)
* [`osc network rbac-policy create`↴](#osc-network-rbac-policy-create)
* [`osc network rbac-policy delete`↴](#osc-network-rbac-policy-delete)
* [`osc network rbac-policy list`↴](#osc-network-rbac-policy-list)
* [`osc network rbac-policy set`↴](#osc-network-rbac-policy-set)
* [`osc network rbac-policy show`↴](#osc-network-rbac-policy-show)
* [`osc network router`↴](#osc-network-router)
* [`osc network router add-external-gateway`↴](#osc-network-router-add-external-gateway)
* [`osc network router add-extraroute`↴](#osc-network-router-add-extraroute)
* [`osc network router add-router-interface`↴](#osc-network-router-add-router-interface)
* [`osc network router conntrack-helper`↴](#osc-network-router-conntrack-helper)
* [`osc network router conntrack-helper create`↴](#osc-network-router-conntrack-helper-create)
* [`osc network router conntrack-helper delete`↴](#osc-network-router-conntrack-helper-delete)
* [`osc network router conntrack-helper list`↴](#osc-network-router-conntrack-helper-list)
* [`osc network router conntrack-helper set`↴](#osc-network-router-conntrack-helper-set)
* [`osc network router conntrack-helper show`↴](#osc-network-router-conntrack-helper-show)
* [`osc network router create`↴](#osc-network-router-create)
* [`osc network router delete`↴](#osc-network-router-delete)
* [`osc network router l3-agent`↴](#osc-network-router-l3-agent)
* [`osc network router l3-agent list`↴](#osc-network-router-l3-agent-list)
* [`osc network router list`↴](#osc-network-router-list)
* [`osc network router remove-external-gateway`↴](#osc-network-router-remove-external-gateway)
* [`osc network router remove-extraroute`↴](#osc-network-router-remove-extraroute)
* [`osc network router remove-router-interface`↴](#osc-network-router-remove-router-interface)
* [`osc network router show`↴](#osc-network-router-show)
* [`osc network router tag`↴](#osc-network-router-tag)
* [`osc network router tag add`↴](#osc-network-router-tag-add)
* [`osc network router tag check`↴](#osc-network-router-tag-check)
* [`osc network router tag delete`↴](#osc-network-router-tag-delete)
* [`osc network router tag list`↴](#osc-network-router-tag-list)
* [`osc network router tag purge`↴](#osc-network-router-tag-purge)
* [`osc network security-group`↴](#osc-network-security-group)
* [`osc network security-group create`↴](#osc-network-security-group-create)
* [`osc network security-group delete`↴](#osc-network-security-group-delete)
* [`osc network security-group list`↴](#osc-network-security-group-list)
* [`osc network security-group set`↴](#osc-network-security-group-set)
* [`osc network security-group show`↴](#osc-network-security-group-show)
* [`osc network security-group tag`↴](#osc-network-security-group-tag)
* [`osc network security-group tag add`↴](#osc-network-security-group-tag-add)
* [`osc network security-group tag check`↴](#osc-network-security-group-tag-check)
* [`osc network security-group tag delete`↴](#osc-network-security-group-tag-delete)
* [`osc network security-group tag list`↴](#osc-network-security-group-tag-list)
* [`osc network security-group tag purge`↴](#osc-network-security-group-tag-purge)
* [`osc network security-group-rule`↴](#osc-network-security-group-rule)
* [`osc network security-group-rule create`↴](#osc-network-security-group-rule-create)
* [`osc network security-group-rule delete`↴](#osc-network-security-group-rule-delete)
* [`osc network security-group-rule list`↴](#osc-network-security-group-rule-list)
* [`osc network security-group-rule set`↴](#osc-network-security-group-rule-set)
* [`osc network security-group-rule show`↴](#osc-network-security-group-rule-show)
* [`osc network subnet`↴](#osc-network-subnet)
* [`osc network subnet create`↴](#osc-network-subnet-create)
* [`osc network subnet delete`↴](#osc-network-subnet-delete)
* [`osc network subnet list`↴](#osc-network-subnet-list)
* [`osc network subnet set`↴](#osc-network-subnet-set)
* [`osc network subnet show`↴](#osc-network-subnet-show)
* [`osc network subnet tag`↴](#osc-network-subnet-tag)
* [`osc network subnet tag add`↴](#osc-network-subnet-tag-add)
* [`osc network subnet tag check`↴](#osc-network-subnet-tag-check)
* [`osc network subnet tag delete`↴](#osc-network-subnet-tag-delete)
* [`osc network subnet tag list`↴](#osc-network-subnet-tag-list)
* [`osc network subnet tag purge`↴](#osc-network-subnet-tag-purge)
* [`osc network subnetpool`↴](#osc-network-subnetpool)
* [`osc network subnetpool add-prefixes`↴](#osc-network-subnetpool-add-prefixes)
* [`osc network subnetpool create`↴](#osc-network-subnetpool-create)
* [`osc network subnetpool delete`↴](#osc-network-subnetpool-delete)
* [`osc network subnetpool list`↴](#osc-network-subnetpool-list)
* [`osc network subnetpool remove-prefixes`↴](#osc-network-subnetpool-remove-prefixes)
* [`osc network subnetpool set`↴](#osc-network-subnetpool-set)
* [`osc network subnetpool show`↴](#osc-network-subnetpool-show)
* [`osc network subnetpool tag`↴](#osc-network-subnetpool-tag)
* [`osc network subnetpool tag add`↴](#osc-network-subnetpool-tag-add)
* [`osc network subnetpool tag check`↴](#osc-network-subnetpool-tag-check)
* [`osc network subnetpool tag delete`↴](#osc-network-subnetpool-tag-delete)
* [`osc network subnetpool tag list`↴](#osc-network-subnetpool-tag-list)
* [`osc network subnetpool tag purge`↴](#osc-network-subnetpool-tag-purge)
* [`osc object-store`↴](#osc-object-store)
* [`osc object-store account`↴](#osc-object-store-account)
* [`osc object-store account show`↴](#osc-object-store-account-show)
* [`osc object-store account set`↴](#osc-object-store-account-set)
* [`osc object-store container`↴](#osc-object-store-container)
* [`osc object-store container create`↴](#osc-object-store-container-create)
* [`osc object-store container delete`↴](#osc-object-store-container-delete)
* [`osc object-store container list`↴](#osc-object-store-container-list)
* [`osc object-store container prune`↴](#osc-object-store-container-prune)
* [`osc object-store container set`↴](#osc-object-store-container-set)
* [`osc object-store container show`↴](#osc-object-store-container-show)
* [`osc object-store object`↴](#osc-object-store-object)
* [`osc object-store object delete`↴](#osc-object-store-object-delete)
* [`osc object-store object download`↴](#osc-object-store-object-download)
* [`osc object-store object list`↴](#osc-object-store-object-list)
* [`osc object-store object show`↴](#osc-object-store-object-show)
* [`osc object-store object upload`↴](#osc-object-store-object-upload)
* [`osc completion`↴](#osc-completion)

## `osc`

OpenStack client rewritten in Rust

**Usage:** `osc [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `api` — Perform direct REST API requests with authorization
* `auth` — Cloud Authentication operations
* `block-storage` — Block Storage (Volume) service (Cinder) commands
* `catalog` — Catalog commands args
* `compute` — Compute service (Nova) operations
* `identity` — Identity (Keystone) commands
* `image` — Image service operations
* `load-balancer` — Load Balancer service operations
* `network` — Network (Neutron) commands
* `object-store` — Object Store service (Swift) commands
* `completion` — Output shell completion code for the specified shell (bash, zsh, fish, or powershell). The shell code must be evaluated to provide interactive completion of `osc` commands.  This can be done by sourcing it from the .bash_profile

###### **Options:**

* `--os-cloud <OS_CLOUD>` — Name reference to the clouds.yaml entry for the cloud configuration
* `--os-project-id <OS_PROJECT_ID>` — Project ID to use instead of the one in connection profile
* `--os-project-name <OS_PROJECT_NAME>` — Project Name to use instead of the one in the connection profile
* `-o`, `--output <OUTPUT>` — Output format

  Possible values:
  - `json`:
    Json output
  - `wide`:
    Wide (Human readable table with extra attributes). Note: this has effect only in list operations

* `-f`, `--fields <FIELDS>` — Fields to return in the output (only in normal and wide mode)
* `-p`, `--pretty` — Pretty print the output
* `-v`, `--verbose` — Verbosity level. Repeat to increase level



## `osc api`

Perform direct REST API requests with authorization

This command enables direct REST API call with the authorization and version discovery handled transparently. This may be used when required operation is not implemented by the `osc` or some of the parameters require special handling.

Example:

```console osc --os-cloud devstack api compute flavors/detail | jq ```

**Usage:** `osc api [OPTIONS] <SERVICE_TYPE> <URL>`

###### **Arguments:**

* `<SERVICE_TYPE>` — Service type as used in the service catalog
* `<URL>` — Rest URL (relative to the endpoint information from the service catalog). Do not start URL with the "/" to respect endpoint version information

###### **Options:**

* `-m`, `--method <METHOD>` — HTTP Method

  Default value: `get`

  Possible values:
  - `head`:
    HEAD
  - `get`:
    GET
  - `patch`:
    PATCH
  - `put`:
    PUT
  - `post`:
    POST
  - `delete`:
    DELETE

* `-H`, `--header <key=value>` — Additional headers
* `--body <BODY>` — Request body to be used



## `osc auth`

Cloud Authentication operations

This command provides various authorization operations (login, show, status, etc)

**Usage:** `osc auth <COMMAND>`

###### **Subcommands:**

* `login` — Login to the cloud and get a valid authorization token
* `show` — Show current authorization information for the cloud



## `osc auth login`

Fetch a new valid authorization token for the cloud.

This command writes token to the stdout

**Usage:** `osc auth login [OPTIONS]`

###### **Options:**

* `--renew` — Require token renewal



## `osc auth show`

Show current authorization information for the cloud

This command returns authentication and authorization information for the currently active connection. It includes issue and expiration information, user data, list of granted roles and project/domain information.

**NOTE**: The command does not support selecting individual fields in the output, but it supports `-o json` command and returns full available information in json format what allows further processing with `jq`

**Usage:** `osc auth show`



## `osc block-storage`

Block Storage (Volume) service (Cinder) commands

**Usage:** `osc block-storage <COMMAND>`

###### **Subcommands:**

* `attachment` — Attachments (attachments)
* `availability-zone` — Availability zones
* `backup` — Backups
* `cluster` — Clusters (clusters)
* `default-type` — Default Volume Types (default-types)
* `extension` — API extensions (extensions)
* `group` — Generic volume groups (groups)
* `group-snapshot` — GroupSnapshot snapshots (group_snapshots)
* `group-type` — Group types (group_types)
* `host` — Hosts extension (os-hosts)
* `limit` — Limits (limits)
* `message` — Messages (messages)
* `os-volume-transfer` — Volume transfers
* `qos-spec` — Quality of service (QoS) specifications (qos-specs)
* `snapshot` — Volume snapshots (snapshots)
* `snapshot-manage` — SnapshotManage manage extension (manageable_snapshots)
* `resource-filter` — Resource filters
* `type` — Block Storage VolumeType type commands
* `volume` — Block Storage Volume commands
* `volume-manage` — Volume manage extension (manageable_volumes)
* `volume-transfer` — Volume transfers (volume-transfers) (3.55 or later)



## `osc block-storage attachment`

Attachments (attachments)

Lists all, lists all with details, shows details for, creates, and deletes attachment.

Note

Everything except for Complete attachment is new as of the 3.27 microversion. Complete attachment is new as of the 3.44 microversion.

When you create, list, update, or delete attachment, the possible status values are:

- attached: A volume is attached for the attachment.

- attaching: A volume is attaching for the attachment.

- detached: A volume is detached for the attachment.

- reserved: A volume is reserved for the attachment.

- error_attaching: A volume is error attaching for the attachment.

- error_detaching: A volume is error detaching for the attachment.

- deleted: The attachment is deleted.

**Usage:** `osc block-storage attachment <COMMAND>`

###### **Subcommands:**

* `complete` — Empty body for os-complete action
* `create354` — Create an attachment
* `create327` — Create an attachment
* `delete` — Delete an attachment
* `list` — Return a detailed list of attachments
* `set327` — Update an attachment record
* `show` — Return data about the given attachment



## `osc block-storage attachment complete`

Empty body for os-complete action

**Usage:** `osc block-storage attachment complete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/attachments/{id}/action API



## `osc block-storage attachment create354`

Create an attachment.

This method can be used to create an empty attachment (reserve) or to create and initialize a volume attachment based on the provided input parameters.

If the caller does not yet have the connector information but needs to reserve an attachment for the volume (ie Nova BootFromVolume) the create can be called with just the volume-uuid and the server identifier. This will reserve an attachment, mark the volume as reserved and prevent any new attachment_create calls from being made until the attachment is updated (completed).

The alternative is that the connection can be reserved and initialized all at once with a single call if the caller has all of the required information (connector data) at the time of the call.

NOTE: In Nova terms server == instance, the server_id parameter referenced below is the UUID of the Instance, for non-nova consumers this can be a server UUID or some other arbitrary unique identifier.

Starting from microversion 3.54, we can pass the attach mode as argument in the request body.

Expected format of the input parameter 'body':

```text

{ "attachment": { "volume_uuid": "volume-uuid", "instance_uuid": "null|nova-server-uuid", "connector": "null|<connector-object>", "mode": "null|rw|ro" } }

```

Example connector:

```text

{ "connector": { "initiator": "iqn.1993-08.org.debian:01:cad181614cec", "ip": "192.168.1.20", "platform": "x86_64", "host": "tempest-1", "os_type": "linux2", "multipath": false, "mountpoint": "/dev/vdb", "mode": "null|rw|ro" } }

```

NOTE all that's required for a reserve is volume_uuid and an instance_uuid.

returns: A summary view of the attachment object

**Usage:** `osc block-storage attachment create354 [OPTIONS] --volume-uuid <VOLUME_UUID>`

###### **Options:**

* `--connector <key=value>` — The `connector` object
* `--instance-uuid <INSTANCE_UUID>` — The UUID of the volume which the attachment belongs to
* `--mode <MODE>` — The attach mode of attachment, acceptable values are read-only (‘ro’) and read-and-write (‘rw’).

   **New in version 3.54**

  Possible values: `ro`, `rw`

* `--volume-uuid <VOLUME_UUID>` — The UUID of the volume which the attachment belongs to



## `osc block-storage attachment create327`

Create an attachment.

This method can be used to create an empty attachment (reserve) or to create and initialize a volume attachment based on the provided input parameters.

If the caller does not yet have the connector information but needs to reserve an attachment for the volume (ie Nova BootFromVolume) the create can be called with just the volume-uuid and the server identifier. This will reserve an attachment, mark the volume as reserved and prevent any new attachment_create calls from being made until the attachment is updated (completed).

The alternative is that the connection can be reserved and initialized all at once with a single call if the caller has all of the required information (connector data) at the time of the call.

NOTE: In Nova terms server == instance, the server_id parameter referenced below is the UUID of the Instance, for non-nova consumers this can be a server UUID or some other arbitrary unique identifier.

Starting from microversion 3.54, we can pass the attach mode as argument in the request body.

Expected format of the input parameter 'body':

```text

{ "attachment": { "volume_uuid": "volume-uuid", "instance_uuid": "null|nova-server-uuid", "connector": "null|<connector-object>", "mode": "null|rw|ro" } }

```

Example connector:

```text

{ "connector": { "initiator": "iqn.1993-08.org.debian:01:cad181614cec", "ip": "192.168.1.20", "platform": "x86_64", "host": "tempest-1", "os_type": "linux2", "multipath": false, "mountpoint": "/dev/vdb", "mode": "null|rw|ro" } }

```

NOTE all that's required for a reserve is volume_uuid and an instance_uuid.

returns: A summary view of the attachment object

**Usage:** `osc block-storage attachment create327 [OPTIONS] --volume-uuid <VOLUME_UUID>`

###### **Options:**

* `--connector <key=value>` — The `connector` object
* `--instance-uuid <INSTANCE_UUID>` — The UUID of the volume which the attachment belongs to
* `--volume-uuid <VOLUME_UUID>` — The UUID of the volume which the attachment belongs to



## `osc block-storage attachment delete`

Delete an attachment.

Disconnects/Deletes the specified attachment, returns a list of any known shared attachment-id's for the effected backend device.

returns: A summary list of any attachments sharing this connection

**Usage:** `osc block-storage attachment delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/attachments/{id} API



## `osc block-storage attachment list`

Return a detailed list of attachments

**Usage:** `osc block-storage attachment list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage attachment set327`

Update an attachment record.

Update a reserved attachment record with connector information and set up the appropriate connection_info from the driver.

Expected format of the input parameter 'body':

```text

{ "attachment": { "connector": { "initiator": "iqn.1993-08.org.debian:01:cad181614cec", "ip": "192.168.1.20", "platform": "x86_64", "host": "tempest-1", "os_type": "linux2", "multipath": false, "mountpoint": "/dev/vdb", "mode": "None|rw|ro" } } }

```

**Usage:** `osc block-storage attachment set327 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/attachments/{id} API

###### **Options:**

* `--connector <key=value>`



## `osc block-storage attachment show`

Return data about the given attachment

**Usage:** `osc block-storage attachment show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/attachments/{id} API



## `osc block-storage availability-zone`

Availability zones

Lists and gets detailed availability zone information.

**Usage:** `osc block-storage availability-zone <COMMAND>`

###### **Subcommands:**

* `list` — Describe all known availability zones



## `osc block-storage availability-zone list`

Describe all known availability zones

**Usage:** `osc block-storage availability-zone list`



## `osc block-storage backup`

Backups

A backup is a full copy of a volume stored in an external service. The service can be configured. The only supported service is Object Storage. A backup can subsequently be restored from the external service to either the same volume that the backup was originally taken from or to a new volume.

**Usage:** `osc block-storage backup <COMMAND>`

###### **Subcommands:**

* `create351` — Create a new backup
* `create343` — Create a new backup
* `create30` — Create a new backup
* `delete` — Delete a backup
* `export` — Export a backup
* `force-delete` — Empty body for os-force_delete action
* `import` — Import a backup
* `list` — Returns a detailed list of backups
* `reset-status` — Command without description in OpenAPI
* `set343` — Update a backup
* `set39` — Update a backup
* `show` — Return data about the given backup



## `osc block-storage backup create351`

Create a new backup

**Usage:** `osc block-storage backup create351 [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>` — The backup availability zone key value pair.

   **New in version 3.51**
* `--container <CONTAINER>` — The container name or null
* `--description <DESCRIPTION>` — The backup description or null
* `--force <FORCE>` — Indicates whether to backup, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--incremental <INCREMENTAL>` — Indicates whether to backup, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--metadata <key=value>` — The backup metadata key value pairs.

   **New in version 3.43**
* `--name <NAME>` — The name of the Volume Backup
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the source snapshot that you want to back up
* `--volume-id <VOLUME_ID>` — The UUID of the volume that you want to back up



## `osc block-storage backup create343`

Create a new backup

**Usage:** `osc block-storage backup create343 [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--container <CONTAINER>` — The container name or null
* `--description <DESCRIPTION>` — The backup description or null
* `--force <FORCE>` — Indicates whether to backup, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--incremental <INCREMENTAL>` — Indicates whether to backup, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--metadata <key=value>` — The backup metadata key value pairs.

   **New in version 3.43**
* `--name <NAME>` — The name of the Volume Backup
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the source snapshot that you want to back up
* `--volume-id <VOLUME_ID>` — The UUID of the volume that you want to back up



## `osc block-storage backup create30`

Create a new backup

**Usage:** `osc block-storage backup create30 [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--container <CONTAINER>` — The container name or null
* `--description <DESCRIPTION>` — The backup description or null
* `--force <FORCE>` — Indicates whether to backup, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--incremental <INCREMENTAL>` — Indicates whether to backup, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--name <NAME>` — The name of the Volume Backup
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the source snapshot that you want to back up
* `--volume-id <VOLUME_ID>` — The UUID of the volume that you want to back up



## `osc block-storage backup delete`

Delete a backup

**Usage:** `osc block-storage backup delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id} API



## `osc block-storage backup export`

Export a backup

**Usage:** `osc block-storage backup export <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id}/export_record API



## `osc block-storage backup force-delete`

Empty body for os-force_delete action

**Usage:** `osc block-storage backup force-delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id}/action API



## `osc block-storage backup import`

Import a backup

**Usage:** `osc block-storage backup import --backup-service <BACKUP_SERVICE> --backup-url <BACKUP_URL>`

###### **Options:**

* `--backup-service <BACKUP_SERVICE>` — The service used to perform the backup
* `--backup-url <BACKUP_URL>` — An identifier string to locate the backup



## `osc block-storage backup list`

Returns a detailed list of backups

**Usage:** `osc block-storage backup list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--with-count <WITH_COUNT>` — Whether to show count in API response or not, default is False

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage backup reset-status`

Command without description in OpenAPI

**Usage:** `osc block-storage backup reset-status --status <STATUS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id}/action API

###### **Options:**

* `--status <STATUS>`



## `osc block-storage backup set343`

Update a backup

**Usage:** `osc block-storage backup set343 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--metadata <key=value>`
* `--name <NAME>`



## `osc block-storage backup set39`

Update a backup

**Usage:** `osc block-storage backup set39 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--name <NAME>`



## `osc block-storage backup show`

Return data about the given backup

**Usage:** `osc block-storage backup show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/backups/{id} API



## `osc block-storage cluster`

Clusters (clusters)

Administrator only. Lists all Cinder clusters, show cluster detail, enable or disable a cluster.

Each cinder service runs on a host computer (possibly multiple services on the same host; it depends how you decide to deploy cinder). In order to support High Availability scenarios, services can be grouped into clusters where the same type of service (for example, cinder-volume) can run on different hosts so that if one host goes down the service is still available on a different host. Since there’s no point having these services sitting around doing nothing while waiting for some other host to go down (which is also known as Active/Passive mode), grouping services into clusters also allows cinder to support Active/Active mode in which all services in a cluster are doing work all the time.

**Note**: Currently the only service that can be grouped into clusters is cinder-volume.

Clusters are determined by the deployment configuration; that’s why there is no ‘create-cluster’ API call listed below. Once your services are up and running, however, you can use the following API requests to get information about your clusters and to update their status.

**Usage:** `osc block-storage cluster <COMMAND>`

###### **Subcommands:**

* `list` — Return a detailed list of all existing clusters
* `set` — Enable/Disable scheduling for a cluster
* `show` — Return data for a given cluster name with optional binary



## `osc block-storage cluster list`

Return a detailed list of all existing clusters.

Filter by is_up, disabled, num_hosts, and num_down_hosts.

**Usage:** `osc block-storage cluster list [OPTIONS]`

###### **Options:**

* `--active-backend-id <ACTIVE_BACKEND_ID>` — The ID of active storage backend. Only in cinder-volume service
* `--binary <BINARY>` — Filter the cluster list result by binary name of the clustered services. One of cinder-api, cinder-scheduler, cinder-volume or cinder-backup

  Possible values: `cinder-api`, `cinder-backup`, `cinder-scheduler`, `cinder-volume`

* `--disabled <DISABLED>` — Filter the cluster list result by status

  Possible values: `true`, `false`

* `--frozen <FROZEN>` — Whether the cluster is frozen or not

  Possible values: `true`, `false`

* `--is-up <IS_UP>` — Filter the cluster list result by state

  Possible values: `true`, `false`

* `--name <NAME>` — Filter the cluster list result by cluster name
* `--num-down-hosts <NUM_DOWN_HOSTS>` — Filter the cluster list result by number of down hosts
* `--num-hosts <NUM_HOSTS>` — Filter the cluster list result by number of hosts
* `--replication-stats <REPLICATION_STATS>` — Filter the cluster list result by replication status

  Possible values: `disabled`, `enabled`




## `osc block-storage cluster set`

Enable/Disable scheduling for a cluster

**Usage:** `osc block-storage cluster set [OPTIONS] --name <NAME> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/clusters/{id} API

###### **Options:**

* `--binary <BINARY>` — The binary name of the services in the cluster
* `--disabled-reason <DISABLED_REASON>` — The reason for disabling a resource
* `--name <NAME>` — The name to identify the service cluster



## `osc block-storage cluster show`

Return data for a given cluster name with optional binary

**Usage:** `osc block-storage cluster show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/clusters/{id} API



## `osc block-storage default-type`

Default Volume Types (default-types)

Manage a default volume type for individual projects.

By default, a volume-create request that does not specify a volume-type will assign the configured system default volume type to the volume. You can override this behavior on a per-project basis by setting a different default volume type for any project.

Available in microversion 3.62 or higher.

NOTE: The default policy for list API is system admin so you would require a system scoped token to access it.

**Usage:** `osc block-storage default-type <COMMAND>`

###### **Subcommands:**

* `delete` — Unset a default volume type for a project
* `list` — Return a list of default types
* `set362` — Set a default volume type for the specified project
* `show` — Return detail of a default type



## `osc block-storage default-type delete`

Unset a default volume type for a project

**Usage:** `osc block-storage default-type delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/default-types/{id} API



## `osc block-storage default-type list`

Return a list of default types

**Usage:** `osc block-storage default-type list`



## `osc block-storage default-type set362`

Set a default volume type for the specified project

**Usage:** `osc block-storage default-type set362 --volume-type <VOLUME_TYPE> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/default-types/{id} API

###### **Options:**

* `--volume-type <VOLUME_TYPE>`



## `osc block-storage default-type show`

Return detail of a default type

**Usage:** `osc block-storage default-type show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/default-types/{id} API



## `osc block-storage extension`

API extensions (extensions)

**Usage:** `osc block-storage extension <COMMAND>`

###### **Subcommands:**

* `list` — Command without description in OpenAPI



## `osc block-storage extension list`

Command without description in OpenAPI

**Usage:** `osc block-storage extension list`



## `osc block-storage group`

Generic volume groups (groups)

Generic volume groups enable you to create a group of volumes and manage them together.

How is generic volume groups different from consistency groups? Currently consistency groups in cinder only support consistent group snapshot. It cannot be extended easily to serve other purposes. A project may want to put volumes used in the same application together in a group so that it is easier to manage them together, and this group of volumes may or may not support consistent group snapshot. Generic volume group is introduced to solve this problem. By decoupling the tight relationship between the group construct and the consistency concept, generic volume groups can be extended to support other features in the future.

**Usage:** `osc block-storage group <COMMAND>`

###### **Subcommands:**

* `create313` — Create a new group
* `create-from-src314` — Command without description in OpenAPI
* `delete313` — Command without description in OpenAPI
* `disable-replication338` — Command without description in OpenAPI
* `enable-replication338` — Command without description in OpenAPI
* `failover-replication338` — Command without description in OpenAPI
* `list` — Returns a detailed list of groups
* `list-replication-targets338` — Command without description in OpenAPI
* `reset-status320` — Command without description in OpenAPI
* `set313` — Update the group
* `show` — Return data about the given group



## `osc block-storage group create313`

Create a new group

**Usage:** `osc block-storage group create313 [OPTIONS] --group-type <GROUP_TYPE>`

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--description <DESCRIPTION>` — The group description
* `--group-type <GROUP_TYPE>` — The group type ID
* `--name <NAME>` — The group name
* `--volume-types <VOLUME_TYPES>` — The list of volume types. In an environment with multiple-storage back ends, the scheduler determines where to send the volume based on the volume type. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html)



## `osc block-storage group create-from-src314`

Command without description in OpenAPI

**Usage:** `osc block-storage group create-from-src314 [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>`
* `--group-snapshot-id <GROUP_SNAPSHOT_ID>`
* `--name <NAME>`
* `--source-group-id <SOURCE_GROUP_ID>`



## `osc block-storage group delete313`

Command without description in OpenAPI

**Usage:** `osc block-storage group delete313 [OPTIONS]`

###### **Options:**

* `--delete-volumes <DELETE_VOLUMES>`

  Possible values: `true`, `false`




## `osc block-storage group disable-replication338`

Command without description in OpenAPI

**Usage:** `osc block-storage group disable-replication338 [OPTIONS]`

###### **Options:**

* `--disable-replication <key=value>`



## `osc block-storage group enable-replication338`

Command without description in OpenAPI

**Usage:** `osc block-storage group enable-replication338 [OPTIONS]`

###### **Options:**

* `--enable-replication <key=value>`



## `osc block-storage group failover-replication338`

Command without description in OpenAPI

**Usage:** `osc block-storage group failover-replication338 [OPTIONS]`

###### **Options:**

* `--allow-attached-volume <ALLOW_ATTACHED_VOLUME>`

  Possible values: `true`, `false`

* `--secondary-backend-id <SECONDARY_BACKEND_ID>`



## `osc block-storage group list`

Returns a detailed list of groups

**Usage:** `osc block-storage group list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage group list-replication-targets338`

Command without description in OpenAPI

**Usage:** `osc block-storage group list-replication-targets338 [OPTIONS]`

###### **Options:**

* `--list-replication-targets <key=value>`



## `osc block-storage group reset-status320`

Command without description in OpenAPI

**Usage:** `osc block-storage group reset-status320 --status <STATUS>`

###### **Options:**

* `--status <STATUS>`



## `osc block-storage group set313`

Update the group.

Expected format of the input parameter 'body':

```text

{ "group": { "name": "my_group", "description": "My group", "add_volumes": "volume-uuid-1,volume-uuid-2,...", "remove_volumes": "volume-uuid-8,volume-uuid-9,..." } }

```

**Usage:** `osc block-storage group set313 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/groups/{id} API

###### **Options:**

* `--add-volumes <ADD_VOLUMES>`
* `--description <DESCRIPTION>`
* `--name <NAME>`
* `--remove-volumes <REMOVE_VOLUMES>`



## `osc block-storage group show`

Return data about the given group

**Usage:** `osc block-storage group show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/groups/{id} API



## `osc block-storage group-snapshot`

GroupSnapshot snapshots (group_snapshots)

Lists all, lists all with details, shows details for, creates, and deletes group snapshots.

**Usage:** `osc block-storage group-snapshot <COMMAND>`

###### **Subcommands:**

* `create314` — Create a new group_snapshot
* `delete` — Delete a group_snapshot
* `list` — Returns a detailed list of group_snapshots
* `reset-status319` — Command without description in OpenAPI
* `show` — Return data about the given group_snapshot



## `osc block-storage group-snapshot create314`

Create a new group_snapshot

**Usage:** `osc block-storage group-snapshot create314 [OPTIONS] --group-id <GROUP_ID>`

###### **Options:**

* `--description <DESCRIPTION>` — The group snapshot description
* `--group-id <GROUP_ID>` — The ID of the group
* `--name <NAME>` — The group snapshot name



## `osc block-storage group-snapshot delete`

Delete a group_snapshot

**Usage:** `osc block-storage group-snapshot delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/group_snapshots/{id} API



## `osc block-storage group-snapshot list`

Returns a detailed list of group_snapshots

**Usage:** `osc block-storage group-snapshot list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage group-snapshot reset-status319`

Command without description in OpenAPI

**Usage:** `osc block-storage group-snapshot reset-status319 --status <STATUS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/group_snapshots/{id}/action API

###### **Options:**

* `--status <STATUS>`



## `osc block-storage group-snapshot show`

Return data about the given group_snapshot

**Usage:** `osc block-storage group-snapshot show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/group_snapshots/{id} API



## `osc block-storage group-type`

Group types (group_types)

To create a generic volume group, you must specify a group type.

**Usage:** `osc block-storage group-type <COMMAND>`

###### **Subcommands:**

* `create311` — Creates a new group type
* `delete` — Deletes an existing group type
* `group-spec` — Server metadata
* `list` — Returns the list of group types
* `set311` — Command without description in OpenAPI
* `show` — Return a single group type item



## `osc block-storage group-type create311`

Creates a new group type

**Usage:** `osc block-storage group-type create311 [OPTIONS] --name <NAME>`

###### **Options:**

* `--description <DESCRIPTION>` — The group type description
* `--group-specs <key=value>` — A set of key and value pairs that contains the specifications for a group type
* `--is-public <IS_PUBLIC>` — Whether the group type is publicly visible. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--name <NAME>` — The group type name



## `osc block-storage group-type delete`

Deletes an existing group type

**Usage:** `osc block-storage group-type delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/group_types/{id} API



## `osc block-storage group-type group-spec`

Server metadata

**Usage:** `osc block-storage group-type group-spec <COMMAND>`

###### **Subcommands:**

* `create311` — Command without description in OpenAPI
* `delete` — Deletes an existing group spec
* `list` — Returns the list of group specs for a given group type
* `set311` — Command without description in OpenAPI
* `show` — Return a single extra spec item



## `osc block-storage group-type group-spec create311`

Command without description in OpenAPI

**Usage:** `osc block-storage group-type group-spec create311 [OPTIONS] <GROUP_TYPE_ID>`

###### **Arguments:**

* `<GROUP_TYPE_ID>` — group_type_id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API

###### **Options:**

* `--group-specs <key=value>` — A set of key and value pairs that contains the specifications for a group type



## `osc block-storage group-type group-spec delete`

Deletes an existing group spec

**Usage:** `osc block-storage group-type group-spec delete <GROUP_TYPE_ID> <ID>`

###### **Arguments:**

* `<GROUP_TYPE_ID>` — group_type_id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API
* `<ID>` — id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API



## `osc block-storage group-type group-spec list`

Returns the list of group specs for a given group type

**Usage:** `osc block-storage group-type group-spec list <GROUP_TYPE_ID>`

###### **Arguments:**

* `<GROUP_TYPE_ID>` — group_type_id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API



## `osc block-storage group-type group-spec set311`

Command without description in OpenAPI

**Usage:** `osc block-storage group-type group-spec set311 [OPTIONS] <GROUP_TYPE_ID> <ID>`

###### **Arguments:**

* `<GROUP_TYPE_ID>` — group_type_id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API
* `<ID>` — id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API

###### **Options:**

* `--property <key=value>`



## `osc block-storage group-type group-spec show`

Return a single extra spec item

**Usage:** `osc block-storage group-type group-spec show <GROUP_TYPE_ID> <ID>`

###### **Arguments:**

* `<GROUP_TYPE_ID>` — group_type_id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API
* `<ID>` — id parameter for /v3/group_types/{group_type_id}/group_specs/{id} API



## `osc block-storage group-type list`

Returns the list of group types

**Usage:** `osc block-storage group-type list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage group-type set311`

Command without description in OpenAPI

**Usage:** `osc block-storage group-type set311 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/group_types/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--is-public <IS_PUBLIC>`

  Possible values: `true`, `false`

* `--name <NAME>`



## `osc block-storage group-type show`

Return a single group type item

**Usage:** `osc block-storage group-type show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/group_types/{id} API



## `osc block-storage host`

Hosts extension (os-hosts)

Administrators only, depending on policy settings.

Lists, shows hosts.

**Usage:** `osc block-storage host <COMMAND>`

###### **Subcommands:**

* `list` — Command without description in OpenAPI
* `show` — Shows the volume usage info given by hosts



## `osc block-storage host list`

Command without description in OpenAPI

**Usage:** `osc block-storage host list`



## `osc block-storage host show`

Shows the volume usage info given by hosts.

| param req: | security context | | --- | --- | | param id: | hostname | | returns: | dict -- the host resources dictionary. ex.: `  {'host': [{'resource': D},..]} D: {'host': 'hostname','project': 'admin',     'volume_count': 1, 'total_volume_gb': 2048}  ` |

**Usage:** `osc block-storage host show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/os-hosts/{id} API



## `osc block-storage limit`

Limits (limits)

An absolute limit value of -1 indicates that the absolute limit for the item is infinite.

**Usage:** `osc block-storage limit <COMMAND>`

###### **Subcommands:**

* `list` — Return all global and rate limit information



## `osc block-storage limit list`

Return all global and rate limit information

**Usage:** `osc block-storage limit list`



## `osc block-storage message`

Messages (messages)

Lists all, shows, and deletes messages. These are error messages generated by failed operations as a way to find out what happened when an asynchronous operation failed.

**Usage:** `osc block-storage message <COMMAND>`

###### **Subcommands:**

* `delete` — Delete a message
* `list` — Returns a list of messages, transformed through view builder
* `show` — Return the given message



## `osc block-storage message delete`

Delete a message

**Usage:** `osc block-storage message delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/messages/{id} API



## `osc block-storage message list`

Returns a list of messages, transformed through view builder

**Usage:** `osc block-storage message list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage message show`

Return the given message

**Usage:** `osc block-storage message show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/messages/{id} API



## `osc block-storage os-volume-transfer`

Volume transfers

Transfers a volume from one user to another user.

**Usage:** `osc block-storage os-volume-transfer <COMMAND>`

###### **Subcommands:**

* `accept` — Accept a new volume transfer
* `create` — Create a new volume transfer
* `delete` — Delete a transfer
* `list` — Returns a detailed list of transfers
* `show` — Return data about active transfers



## `osc block-storage os-volume-transfer accept`

Accept a new volume transfer

**Usage:** `osc block-storage os-volume-transfer accept --auth-key <AUTH_KEY> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/os-volume-transfer/{id}/accept API

###### **Options:**

* `--auth-key <AUTH_KEY>`



## `osc block-storage os-volume-transfer create`

Create a new volume transfer

**Usage:** `osc block-storage os-volume-transfer create [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--name <NAME>` — The name of the object
* `--volume-id <VOLUME_ID>` — The UUID of the volume



## `osc block-storage os-volume-transfer delete`

Delete a transfer

**Usage:** `osc block-storage os-volume-transfer delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/os-volume-transfer/{id} API



## `osc block-storage os-volume-transfer list`

Returns a detailed list of transfers

**Usage:** `osc block-storage os-volume-transfer list`



## `osc block-storage os-volume-transfer show`

Return data about active transfers

**Usage:** `osc block-storage os-volume-transfer show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/os-volume-transfer/{id} API



## `osc block-storage qos-spec`

Quality of service (QoS) specifications (qos-specs)

Administrators only, depending on policy settings.

Creates, lists, shows details for, associates, disassociates, sets keys, unsets keys, and deletes quality of service (QoS) specifications.

**Usage:** `osc block-storage qos-spec <COMMAND>`

###### **Subcommands:**

* `association` — Supported subcommands
* `associate` — Associate a qos specs with a volume type
* `create` — Command without description in OpenAPI
* `delete` — Deletes an existing qos specs
* `delete-keys` — Deletes specified keys in qos specs
* `disassociate` — Disassociate a qos specs from a volume type
* `disassociate-all` — Disassociate a qos specs from all volume types
* `list` — Returns the list of qos_specs
* `set` — Command without description in OpenAPI
* `show` — Return a single qos spec item



## `osc block-storage qos-spec association`

Supported subcommands

**Usage:** `osc block-storage qos-spec association <COMMAND>`

###### **Subcommands:**

* `list` — List all associations of given qos specs



## `osc block-storage qos-spec association list`

List all associations of given qos specs

**Usage:** `osc block-storage qos-spec association list <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id}/associations API



## `osc block-storage qos-spec associate`

Associate a qos specs with a volume type

**Usage:** `osc block-storage qos-spec associate <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id}/associate API



## `osc block-storage qos-spec create`

Command without description in OpenAPI

**Usage:** `osc block-storage qos-spec create --name <NAME>`

###### **Options:**

* `--name <NAME>` — The name of the QoS specification



## `osc block-storage qos-spec delete`

Deletes an existing qos specs

**Usage:** `osc block-storage qos-spec delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id} API



## `osc block-storage qos-spec delete-keys`

Deletes specified keys in qos specs

**Usage:** `osc block-storage qos-spec delete-keys [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id}/delete_keys API

###### **Options:**

* `--keys <KEYS>`



## `osc block-storage qos-spec disassociate`

Disassociate a qos specs from a volume type

**Usage:** `osc block-storage qos-spec disassociate <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id}/disassociate API



## `osc block-storage qos-spec disassociate-all`

Disassociate a qos specs from all volume types

**Usage:** `osc block-storage qos-spec disassociate-all <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id}/disassociate_all API



## `osc block-storage qos-spec list`

Returns the list of qos_specs

**Usage:** `osc block-storage qos-spec list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage qos-spec set`

Command without description in OpenAPI

**Usage:** `osc block-storage qos-spec set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id} API

###### **Options:**

* `--qos-specs <key=value>`



## `osc block-storage qos-spec show`

Return a single qos spec item

**Usage:** `osc block-storage qos-spec show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/qos-specs/{id} API



## `osc block-storage snapshot`

Volume snapshots (snapshots)

A snapshot is a point-in-time copy of the data that a volume contains.

When you create, list, or delete snapshots, these status values are possible:

- creating: The snapshot is being created.

- available: The snapshot is ready to use.

- backing-up: The snapshot is being backed up.

- deleting: The snapshot is being deleted.

- error: A snapshot creation error occurred.

- deleted: The snapshot has been deleted.

- unmanaging: The snapshot is being unmanaged.

- restoring: The snapshot is being restored to a volume.

- error_deleting: A snapshot deletion error occurred.

**Usage:** `osc block-storage snapshot <COMMAND>`

###### **Subcommands:**

* `create` — Creates a new snapshot
* `delete` — Delete a snapshot
* `force-delete` — Empty body for os-force_delete action
* `list` — Returns a detailed list of snapshots
* `reset-status` — Empty body for os-reset_status action
* `set` — Update a snapshot
* `show` — Return data about the given snapshot
* `unmanage` — Empty body for os-unmanage action
* `update-status` — Command without description in OpenAPI



## `osc block-storage snapshot create`

Creates a new snapshot

**Usage:** `osc block-storage snapshot create [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--description <DESCRIPTION>` — A description for the snapshot. Default is `None`
* `--display-name <DISPLAY_NAME>` — The name of the snapshot
* `--force <FORCE>` — Indicates whether to snapshot, even if the volume is attached. Default is `false`. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--metadata <key=value>` — One or more metadata key and value pairs for the snapshot
* `--name <NAME>` — The name of the snapshot
* `--volume-id <VOLUME_ID>` — The UUID of the volume



## `osc block-storage snapshot delete`

Delete a snapshot

**Usage:** `osc block-storage snapshot delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id} API



## `osc block-storage snapshot force-delete`

Empty body for os-force_delete action

**Usage:** `osc block-storage snapshot force-delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id}/action API



## `osc block-storage snapshot list`

Returns a detailed list of snapshots

**Usage:** `osc block-storage snapshot list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--consumes-quota <CONSUMES_QUOTA>` — Filters results by consumes_quota field. Resources that don’t use quotas are usually temporary internal resources created to perform an operation. Default is to not filter by it. Filtering by this option may not be always possible in a cloud, see List Resource Filters to determine whether this filter is available in your cloud

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--with-count <WITH_COUNT>` — Whether to show count in API response or not, default is False

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage snapshot reset-status`

Empty body for os-reset_status action

**Usage:** `osc block-storage snapshot reset-status <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id}/action API



## `osc block-storage snapshot set`

Update a snapshot

**Usage:** `osc block-storage snapshot set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--name <NAME>`



## `osc block-storage snapshot show`

Return data about the given snapshot

**Usage:** `osc block-storage snapshot show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id} API



## `osc block-storage snapshot unmanage`

Empty body for os-unmanage action

**Usage:** `osc block-storage snapshot unmanage <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id}/action API



## `osc block-storage snapshot update-status`

Command without description in OpenAPI

**Usage:** `osc block-storage snapshot update-status [OPTIONS] --status <STATUS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/snapshots/{id}/action API

###### **Options:**

* `--progress <PROGRESS>`
* `--status <STATUS>`



## `osc block-storage snapshot-manage`

SnapshotManage manage extension (manageable_snapshots)

Creates or lists snapshots by using existing storage instead of allocating new storage.

**Usage:** `osc block-storage snapshot-manage <COMMAND>`

###### **Subcommands:**

* `create` — Instruct Cinder to manage a storage snapshot object
* `list` — Returns a detailed list of snapshots available to manage



## `osc block-storage snapshot-manage create`

Instruct Cinder to manage a storage snapshot object.

Manages an existing backend storage snapshot object (e.g. a Linux logical volume or a SAN disk) by creating the Cinder objects required to manage it, and possibly renaming the backend storage snapshot object (driver dependent).

From an API perspective, this operation behaves very much like a snapshot creation operation.

Required HTTP Body:

```text

{ "snapshot": { "volume_id": "<Cinder volume already exists in volume backend>", "ref": "<Driver-specific reference to the existing storage object>" } }

```

See the appropriate Cinder drivers' implementations of the manage_snapshot method to find out the accepted format of 'ref'. For example,in LVM driver, it will be the logic volume name of snapshot which you want to manage.

This API call will return with an error if any of the above elements are missing from the request, or if the 'volume_id' element refers to a cinder volume that could not be found.

The snapshot will later enter the error state if it is discovered that 'ref' is bad.

Optional elements to 'snapshot' are:

```text

name           A name for the new snapshot. description    A description for the new snapshot. metadata       Key/value pairs to be associated with the new snapshot.

```

**Usage:** `osc block-storage snapshot-manage create [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--description <DESCRIPTION>`
* `--metadata <key=value>`
* `--name <NAME>`
* `--ref <JSON>`
* `--volume-id <VOLUME_ID>`



## `osc block-storage snapshot-manage list`

Returns a detailed list of snapshots available to manage

**Usage:** `osc block-storage snapshot-manage list`



## `osc block-storage resource-filter`

Resource filters

Lists all resource filters, available since microversion 3.33.

**Usage:** `osc block-storage resource-filter <COMMAND>`

###### **Subcommands:**

* `list` — Return a list of resource filters



## `osc block-storage resource-filter list`

Return a list of resource filters

**Usage:** `osc block-storage resource-filter list`



## `osc block-storage type`

Block Storage VolumeType type commands

To create an environment with multiple-storage back ends, you must specify a volume type. The API spawns Block Storage volume back ends as children to cinder-volume, and keys them from a unique queue. The API names the back ends cinder-volume.HOST.BACKEND. For example, cinder-volume.ubuntu.lvmdriver. When you create a volume, the scheduler chooses an appropriate back end for the volume type to handle the request.

For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html).

**Usage:** `osc block-storage type <COMMAND>`

###### **Subcommands:**

* `add-project-access` — Command without description in OpenAPI
* `create` — Command without description in OpenAPI
* `delete` — Deletes an existing volume type
* `encryption` — Volume Type Encryption commands
* `extraspecs` — Type extra specs
* `list` — Returns the list of volume types
* `remove-project-access` — Command without description in OpenAPI
* `set` — Command without description in OpenAPI
* `show` — Return a single volume type item



## `osc block-storage type add-project-access`

Command without description in OpenAPI

**Usage:** `osc block-storage type add-project-access --project <PROJECT> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/types/{id}/action API

###### **Options:**

* `--project <PROJECT>`



## `osc block-storage type create`

Command without description in OpenAPI

**Usage:** `osc block-storage type create [OPTIONS] --name <NAME> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/types/{id}/action API

###### **Options:**

* `--description <DESCRIPTION>`
* `--extra-specs <key=value>`
* `--name <NAME>`
* `--os-volume-type-access-is-public <OS_VOLUME_TYPE_ACCESS_IS_PUBLIC>`

  Possible values: `true`, `false`




## `osc block-storage type delete`

Deletes an existing volume type

**Usage:** `osc block-storage type delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/types/{id} API



## `osc block-storage type encryption`

Volume Type Encryption commands

Block Storage volume type assignment provides scheduling to a specific back-end, and can be used to specify actionable information for a back-end storage device.

**Usage:** `osc block-storage type encryption <COMMAND>`

###### **Subcommands:**

* `create` — Create encryption specs for an existing volume type
* `delete` — Delete encryption specs for a given volume type
* `list` — Returns the encryption specs for a given volume type
* `set` — Update encryption specs for a given volume type
* `show` — Return a single encryption item



## `osc block-storage type encryption create`

Create encryption specs for an existing volume type

**Usage:** `osc block-storage type encryption create [OPTIONS] --control-location <CONTROL_LOCATION> --provider <PROVIDER> <TYPE_ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/encryption/{id} API

###### **Options:**

* `--cipher <CIPHER>`
* `--control-location <CONTROL_LOCATION>`

  Possible values: `back-end`, `front-end`

* `--key-size <KEY_SIZE>`
* `--provider <PROVIDER>`



## `osc block-storage type encryption delete`

Delete encryption specs for a given volume type

**Usage:** `osc block-storage type encryption delete <TYPE_ID> <ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/encryption/{id} API
* `<ID>` — id parameter for /v3/types/{type_id}/encryption/{id} API



## `osc block-storage type encryption list`

Returns the encryption specs for a given volume type

**Usage:** `osc block-storage type encryption list <TYPE_ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/encryption/{id} API



## `osc block-storage type encryption set`

Update encryption specs for a given volume type

**Usage:** `osc block-storage type encryption set [OPTIONS] <TYPE_ID> <ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/encryption/{id} API
* `<ID>` — id parameter for /v3/types/{type_id}/encryption/{id} API

###### **Options:**

* `--cipher <CIPHER>`
* `--control-location <CONTROL_LOCATION>`

  Possible values: `back-end`, `front-end`

* `--key-size <KEY_SIZE>`
* `--provider <PROVIDER>`



## `osc block-storage type encryption show`

Return a single encryption item

**Usage:** `osc block-storage type encryption show <TYPE_ID> <ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/encryption/{id} API
* `<ID>` — id parameter for /v3/types/{type_id}/encryption/{id} API



## `osc block-storage type extraspecs`

Type extra specs

**Usage:** `osc block-storage type extraspecs <COMMAND>`

###### **Subcommands:**

* `create` — Command without description in OpenAPI
* `delete` — Deletes an existing extra spec
* `list` — Returns the list of extra specs for a given volume type
* `show` — Return a single extra spec item
* `set` — Command without description in OpenAPI



## `osc block-storage type extraspecs create`

Command without description in OpenAPI

**Usage:** `osc block-storage type extraspecs create [OPTIONS] <TYPE_ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/extra_specs/{id} API

###### **Options:**

* `--extra-specs <key=value>`



## `osc block-storage type extraspecs delete`

Deletes an existing extra spec

**Usage:** `osc block-storage type extraspecs delete <TYPE_ID> <ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/extra_specs/{id} API
* `<ID>` — id parameter for /v3/types/{type_id}/extra_specs/{id} API



## `osc block-storage type extraspecs list`

Returns the list of extra specs for a given volume type

**Usage:** `osc block-storage type extraspecs list <TYPE_ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/extra_specs/{id} API



## `osc block-storage type extraspecs show`

Return a single extra spec item

**Usage:** `osc block-storage type extraspecs show <TYPE_ID> <ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/extra_specs/{id} API
* `<ID>` — id parameter for /v3/types/{type_id}/extra_specs/{id} API



## `osc block-storage type extraspecs set`

Command without description in OpenAPI

**Usage:** `osc block-storage type extraspecs set [OPTIONS] <TYPE_ID> <ID>`

###### **Arguments:**

* `<TYPE_ID>` — type_id parameter for /v3/types/{type_id}/extra_specs/{id} API
* `<ID>` — id parameter for /v3/types/{type_id}/extra_specs/{id} API

###### **Options:**

* `--property <key=value>`



## `osc block-storage type list`

Returns the list of volume types

**Usage:** `osc block-storage type list`



## `osc block-storage type remove-project-access`

Command without description in OpenAPI

**Usage:** `osc block-storage type remove-project-access --project <PROJECT> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/types/{id}/action API

###### **Options:**

* `--project <PROJECT>`



## `osc block-storage type set`

Command without description in OpenAPI

**Usage:** `osc block-storage type set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/types/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--is-public <IS_PUBLIC>`

  Possible values: `true`, `false`

* `--name <NAME>`



## `osc block-storage type show`

Return a single volume type item

**Usage:** `osc block-storage type show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/types/{id} API



## `osc block-storage volume`

Block Storage Volume commands

**Usage:** `osc block-storage volume <COMMAND>`

###### **Subcommands:**

* `create353` — Creates a new volume
* `create347` — Creates a new volume
* `create313` — Creates a new volume
* `create30` — Creates a new volume
* `delete` — Delete a volume
* `extend` — Command without description in OpenAPI
* `list` — Returns a detailed list of volumes
* `metadata` — Volume metadata
* `set353` — Update a volume
* `set30` — Update a volume
* `show` — Return data about the given volume



## `osc block-storage volume create353`

Creates a new volume.

| param req: | the request | | --- | --- | | param body: | the request body | | returns: | dict -- the new volume dictionary | | raises HTTPNotFound, HTTPBadRequest: | | | | |

**Usage:** `osc block-storage volume create353 [OPTIONS]`

###### **Options:**

* `--os-sch-hnt-scheduler-hints <key=value>` — The dictionary of data to send to the scheduler
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--backup-id <BACKUP_ID>` — The UUID of the backup.

   **New in version 3.47**
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--description <DESCRIPTION>` — The volume description
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--group-id <GROUP_ID>`
* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--multiattach <MULTIATTACH>`

  Possible values: `true`, `false`

* `--name <NAME>` — The volume name
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html)



## `osc block-storage volume create347`

Creates a new volume.

| param req: | the request | | --- | --- | | param body: | the request body | | returns: | dict -- the new volume dictionary | | raises HTTPNotFound, HTTPBadRequest: | | | | |

**Usage:** `osc block-storage volume create347 [OPTIONS]`

###### **Options:**

* `--os-sch-hnt-scheduler-hints <key=value>` — The dictionary of data to send to the scheduler
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--backup-id <BACKUP_ID>` — The UUID of the backup.

   **New in version 3.47**
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--description <DESCRIPTION>` — The volume description
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--group-id <GROUP_ID>`
* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--multiattach <MULTIATTACH>`

  Possible values: `true`, `false`

* `--name <NAME>` — The volume name
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html)



## `osc block-storage volume create313`

Creates a new volume.

| param req: | the request | | --- | --- | | param body: | the request body | | returns: | dict -- the new volume dictionary | | raises HTTPNotFound, HTTPBadRequest: | | | | |

**Usage:** `osc block-storage volume create313 [OPTIONS]`

###### **Options:**

* `--os-sch-hnt-scheduler-hints <key=value>` — The dictionary of data to send to the scheduler
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--description <DESCRIPTION>` — The volume description
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--group-id <GROUP_ID>`
* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--multiattach <MULTIATTACH>`

  Possible values: `true`, `false`

* `--name <NAME>` — The volume name
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html)



## `osc block-storage volume create30`

Creates a new volume.

| param req: | the request | | --- | --- | | param body: | the request body | | returns: | dict -- the new volume dictionary | | raises HTTPNotFound, HTTPBadRequest: | | | | |

**Usage:** `osc block-storage volume create30 [OPTIONS]`

###### **Options:**

* `--os-sch-hnt-scheduler-hints <key=value>` — The dictionary of data to send to the scheduler
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--description <DESCRIPTION>` — The volume description
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--multiattach <MULTIATTACH>`

  Possible values: `true`, `false`

* `--name <NAME>` — The volume name
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html)



## `osc block-storage volume delete`

Delete a volume

**Usage:** `osc block-storage volume delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API



## `osc block-storage volume extend`

Command without description in OpenAPI

**Usage:** `osc block-storage volume extend --new-size <NEW_SIZE> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id}/action API

###### **Options:**

* `--new-size <NEW_SIZE>`



## `osc block-storage volume list`

Returns a detailed list of volumes

**Usage:** `osc block-storage volume list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--consumes-quota <CONSUMES_QUOTA>` — Filters results by consumes_quota field. Resources that don’t use quotas are usually temporary internal resources created to perform an operation. Default is to not filter by it. Filtering by this option may not be always possible in a cloud, see List Resource Filters to determine whether this filter is available in your cloud

  Possible values: `true`, `false`

* `--created-at <CREATED_AT>` — Filters reuslts by a time that resources are created at with time comparison operators: gt/gte/eq/neq/lt/lte
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--updated-at <UPDATED_AT>` — Filters reuslts by a time that resources are updated at with time comparison operators: gt/gte/eq/neq/lt/lte
* `--with-count <WITH_COUNT>` — Whether to show count in API response or not, default is False

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage volume metadata`

Volume metadata

Lists metadata, creates or replaces one or more metadata items, and updates one or more metadata items for a volume.

**Usage:** `osc block-storage volume metadata <COMMAND>`

###### **Subcommands:**

* `create` — Command without description in OpenAPI
* `delete` — Deletes an existing metadata
* `list` — Returns the list of metadata for a given volume
* `replace` — Command without description in OpenAPI
* `set` — Command without description in OpenAPI
* `show` — Return a single metadata item



## `osc block-storage volume metadata create`

Command without description in OpenAPI

**Usage:** `osc block-storage volume metadata create [OPTIONS] <VOLUME_ID>`

###### **Arguments:**

* `<VOLUME_ID>` — volume_id parameter for /v3/volumes/{volume_id}/metadata API

###### **Options:**

* `--metadata <key=value>` — One or more metadata key and value pairs that are associated with the volume



## `osc block-storage volume metadata delete`

Deletes an existing metadata

**Usage:** `osc block-storage volume metadata delete <VOLUME_ID> <ID>`

###### **Arguments:**

* `<VOLUME_ID>` — volume_id parameter for /v3/volumes/{volume_id}/metadata API
* `<ID>` — id parameter for /v3/volumes/{volume_id}/metadata/{id} API



## `osc block-storage volume metadata list`

Returns the list of metadata for a given volume

**Usage:** `osc block-storage volume metadata list <VOLUME_ID>`

###### **Arguments:**

* `<VOLUME_ID>` — volume_id parameter for /v3/volumes/{volume_id}/metadata API



## `osc block-storage volume metadata replace`

Command without description in OpenAPI

**Usage:** `osc block-storage volume metadata replace [OPTIONS] <VOLUME_ID>`

###### **Arguments:**

* `<VOLUME_ID>` — volume_id parameter for /v3/volumes/{volume_id}/metadata API

###### **Options:**

* `--metadata <key=value>` — One or more metadata key and value pairs that are associated with the volume



## `osc block-storage volume metadata set`

Command without description in OpenAPI

**Usage:** `osc block-storage volume metadata set [OPTIONS] <VOLUME_ID> <ID>`

###### **Arguments:**

* `<VOLUME_ID>` — volume_id parameter for /v3/volumes/{volume_id}/metadata API
* `<ID>` — id parameter for /v3/volumes/{volume_id}/metadata/{id} API

###### **Options:**

* `--meta <key=value>`



## `osc block-storage volume metadata show`

Return a single metadata item

**Usage:** `osc block-storage volume metadata show <VOLUME_ID> <ID>`

###### **Arguments:**

* `<VOLUME_ID>` — volume_id parameter for /v3/volumes/{volume_id}/metadata API
* `<ID>` — id parameter for /v3/volumes/{volume_id}/metadata/{id} API



## `osc block-storage volume set353`

Update a volume

**Usage:** `osc block-storage volume set353 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--metadata <key=value>`
* `--name <NAME>`



## `osc block-storage volume set30`

Update a volume

**Usage:** `osc block-storage volume set30 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--metadata <key=value>`
* `--name <NAME>`



## `osc block-storage volume show`

Return data about the given volume

**Usage:** `osc block-storage volume show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API



## `osc block-storage volume-manage`

Volume manage extension (manageable_volumes)

Creates or lists volumes by using existing storage instead of allocating new storage.

**Usage:** `osc block-storage volume-manage <COMMAND>`

###### **Subcommands:**

* `create316` — Instruct Cinder to manage a storage object
* `create30` — Instruct Cinder to manage a storage object
* `list` — Returns a detailed list of volumes available to manage



## `osc block-storage volume-manage create316`

Instruct Cinder to manage a storage object.

Manages an existing backend storage object (e.g. a Linux logical volume or a SAN disk) by creating the Cinder objects required to manage it, and possibly renaming the backend storage object (driver dependent)

From an API perspective, this operation behaves very much like a volume creation operation, except that properties such as image, snapshot and volume references don't make sense, because we are taking an existing storage object into Cinder management.

Required HTTP Body:

```text

{ "volume": { "host": "<Cinder host on which the existing storage resides>", "cluster": "<Cinder cluster on which the storage resides>", "ref": "<Driver-specific reference to existing storage object>" } }

```

See the appropriate Cinder drivers' implementations of the manage_volume method to find out the accepted format of 'ref'.

This API call will return with an error if any of the above elements are missing from the request, or if the 'host' element refers to a cinder host that is not registered.

The volume will later enter the error state if it is discovered that 'ref' is bad.

Optional elements to 'volume' are:

```text

name               A name for the new volume. description        A description for the new volume. volume_type        ID or name of a volume type to associate with the new Cinder volume. Does not necessarily guarantee that the managed volume will have the properties described in the volume_type. The driver may choose to fail if it identifies that the specified volume_type is not compatible with the backend storage object. metadata           Key/value pairs to be associated with the new volume. availability_zone  The availability zone to associate with the new volume. bootable           If set to True, marks the volume as bootable.

```

**Usage:** `osc block-storage volume-manage create316 [OPTIONS] --ref <JSON>`

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>`
* `--bootable <BOOTABLE>`

  Possible values: `true`, `false`

* `--cluster <CLUSTER>`
* `--description <DESCRIPTION>`
* `--host <HOST>`
* `--metadata <key=value>`
* `--name <NAME>`
* `--ref <JSON>`
* `--volume-type <VOLUME_TYPE>`



## `osc block-storage volume-manage create30`

Instruct Cinder to manage a storage object.

Manages an existing backend storage object (e.g. a Linux logical volume or a SAN disk) by creating the Cinder objects required to manage it, and possibly renaming the backend storage object (driver dependent)

From an API perspective, this operation behaves very much like a volume creation operation, except that properties such as image, snapshot and volume references don't make sense, because we are taking an existing storage object into Cinder management.

Required HTTP Body:

```text

{ "volume": { "host": "<Cinder host on which the existing storage resides>", "cluster": "<Cinder cluster on which the storage resides>", "ref": "<Driver-specific reference to existing storage object>" } }

```

See the appropriate Cinder drivers' implementations of the manage_volume method to find out the accepted format of 'ref'.

This API call will return with an error if any of the above elements are missing from the request, or if the 'host' element refers to a cinder host that is not registered.

The volume will later enter the error state if it is discovered that 'ref' is bad.

Optional elements to 'volume' are:

```text

name               A name for the new volume. description        A description for the new volume. volume_type        ID or name of a volume type to associate with the new Cinder volume. Does not necessarily guarantee that the managed volume will have the properties described in the volume_type. The driver may choose to fail if it identifies that the specified volume_type is not compatible with the backend storage object. metadata           Key/value pairs to be associated with the new volume. availability_zone  The availability zone to associate with the new volume. bootable           If set to True, marks the volume as bootable.

```

**Usage:** `osc block-storage volume-manage create30 [OPTIONS] --ref <JSON>`

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>`
* `--bootable <BOOTABLE>`

  Possible values: `true`, `false`

* `--description <DESCRIPTION>`
* `--host <HOST>`
* `--metadata <key=value>`
* `--name <NAME>`
* `--ref <JSON>`
* `--volume-type <VOLUME_TYPE>`



## `osc block-storage volume-manage list`

Returns a detailed list of volumes available to manage

**Usage:** `osc block-storage volume-manage list`



## `osc block-storage volume-transfer`

Volume transfers (volume-transfers) (3.55 or later)

Transfers a volume from one user to another user. This is the new transfer APIs with microversion 3.55.

**Usage:** `osc block-storage volume-transfer <COMMAND>`

###### **Subcommands:**

* `accept` — Accept a new volume transfer
* `create355` — Create a new volume transfer
* `delete` — Delete a transfer
* `list` — Returns a detailed list of transfers
* `show` — Return data about active transfers



## `osc block-storage volume-transfer accept`

Accept a new volume transfer

**Usage:** `osc block-storage volume-transfer accept --auth-key <AUTH_KEY> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volume-transfers/{id}/accept API

###### **Options:**

* `--auth-key <AUTH_KEY>`



## `osc block-storage volume-transfer create355`

Create a new volume transfer

**Usage:** `osc block-storage volume-transfer create355 [OPTIONS] --volume-id <VOLUME_ID>`

###### **Options:**

* `--name <NAME>` — The name of the object
* `--no-snapshots <NO_SNAPSHOTS>` — Transfer volume without snapshots. Defaults to False if not specified.

   **New in version 3.55**

  Possible values: `true`, `false`

* `--volume-id <VOLUME_ID>` — The UUID of the volume



## `osc block-storage volume-transfer delete`

Delete a transfer

**Usage:** `osc block-storage volume-transfer delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volume-transfers/{id} API



## `osc block-storage volume-transfer list`

Returns a detailed list of transfers

**Usage:** `osc block-storage volume-transfer list [OPTIONS]`

###### **Options:**

* `--all-tenants <ALL_TENANTS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--is-public <IS_PUBLIC>` — Filter the volume transfer by public visibility

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of \< key > \[: \< direction > \]. A valid direction is asc (ascending) or desc (descending)
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage volume-transfer show`

Return data about active transfers

**Usage:** `osc block-storage volume-transfer show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volume-transfers/{id} API



## `osc catalog`

Catalog commands args

**Usage:** `osc catalog <COMMAND>`

###### **Subcommands:**

* `list` — Shows current catalog information



## `osc catalog list`

Shows current catalog information

**Usage:** `osc catalog list`



## `osc compute`

Compute service (Nova) operations

**Usage:** `osc compute <COMMAND>`

###### **Subcommands:**

* `aggregate` — Host Aggregates
* `availability-zone` — Availability zones
* `extension` — Extension commands
* `flavor` — Flavor commands
* `hypervisor` — Hypervisors
* `keypair` — Keypairs commands
* `server` — Servers



## `osc compute aggregate`

Creates and manages host aggregates. An aggregate assigns metadata to groups of compute nodes.

Policy defaults enable only users with the administrative role to perform operations with aggregates. Cloud providers can change these permissions through policy file configuration.

**Usage:** `osc compute aggregate <COMMAND>`

###### **Subcommands:**

* `add-host` — Add Host
* `create21` — Create Aggregate (microversion = 2.1)
* `cache-image` — Request Image Pre-caching for Aggregate (microversion = 2.81)
* `delete` — Delete Aggregate
* `list` — List Aggregates
* `remove-host` — Remove Host
* `show` — Show Aggregate Details
* `set21` — Update Aggregate (microversion = 2.1)
* `set-metadata` — Create Or Update Aggregate Metadata



## `osc compute aggregate add-host`

Add Host

**Usage:** `osc compute aggregate add-host --host <HOST> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id}/action API

###### **Options:**

* `--host <HOST>`



## `osc compute aggregate create21`

Creates an aggregate. If specifying an option availability_zone, the aggregate is created as an availability zone and the availability zone is visible to normal users.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute aggregate create21 [OPTIONS] --name <NAME>`

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>` — The availability zone of the host aggregate. You should use a custom availability zone rather than the default returned by the os-availability-zone API. The availability zone must not include ‘:’ in its name
* `--name <NAME>` — The name of the host aggregate



## `osc compute aggregate cache-image`

Requests that a set of images be pre-cached on compute nodes within the referenced aggregate.

This API is available starting with microversion 2.81.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute aggregate cache-image [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id}/images API

###### **Options:**

* `--cache <CACHE>` — A list of image objects to cache



## `osc compute aggregate delete`

Deletes an aggregate.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute aggregate delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id} API



## `osc compute aggregate list`

Lists all aggregates. Includes the ID, name, and availability zone for each aggregate.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403)

**Usage:** `osc compute aggregate list`



## `osc compute aggregate remove-host`

Remove Host

**Usage:** `osc compute aggregate remove-host --host <HOST> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id}/action API

###### **Options:**

* `--host <HOST>`



## `osc compute aggregate show`

Shows details for an aggregate. Details include hosts and metadata.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute aggregate show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id} API



## `osc compute aggregate set21`

Updates either or both the name and availability zone for an aggregate. If the aggregate to be updated has host that already in the given availability zone, the request will fail with 400 error.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute aggregate set21 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id} API

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>` — The availability zone of the host aggregate. You should use a custom availability zone rather than the default returned by the os-availability-zone API. The availability zone must not include ‘:’ in its name.

   Warning

   You should not change or unset the availability zone of an aggregate when that aggregate has hosts which contain servers in it since that may impact the ability for those servers to move to another host.
* `--name <NAME>` — The name of the host aggregate



## `osc compute aggregate set-metadata`

Create Or Update Aggregate Metadata

**Usage:** `osc compute aggregate set-metadata [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-aggregates/{id}/action API

###### **Options:**

* `--metadata <key=value>`



## `osc compute availability-zone`

Lists and gets detailed availability zone information.

An availability zone is created or updated by setting the availability_zone parameter in the create, update, or create or update methods of the Host Aggregates API. See Host Aggregates for more details.

**Usage:** `osc compute availability-zone <COMMAND>`

###### **Subcommands:**

* `list` — Get Availability Zone Information
* `list-detail` — Get Detailed Availability Zone Information



## `osc compute availability-zone list`

Lists availability zone information.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403)

**Usage:** `osc compute availability-zone list`



## `osc compute availability-zone list-detail`

Gets detailed availability zone information. Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403)

**Usage:** `osc compute availability-zone list-detail`



## `osc compute extension`

Extension commands

**Usage:** `osc compute extension <COMMAND>`

###### **Subcommands:**

* `list` — List Extensions
* `show` — Show Extension Details



## `osc compute extension list`

Lists all extensions to the API.

Normal response codes: 200

Error response codes: unauthorized(401)

**Usage:** `osc compute extension list`



## `osc compute extension show`

Shows details for an extension, by alias.

Normal response codes: 200

Error response codes: unauthorized(401), itemNotFound(404)

**Usage:** `osc compute extension show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/extensions/{id} API



## `osc compute flavor`

Flavor commands

Flavors are a way to describe the basic dimensions of a server to be created including how much cpu, ram, and disk space are allocated to a server built with this flavor.

**Usage:** `osc compute flavor <COMMAND>`

###### **Subcommands:**

* `access` — Flavor access command
* `create255` — Create Flavor (microversion = 2.55)
* `create21` — Create Flavor (microversion = 2.1)
* `create20` — Create Flavor (microversion = 2.0)
* `delete` — Delete Flavor
* `extraspecs` — Flavor extra specs
* `list` — List Flavors With Details
* `set255` — Update Flavor Description (microversion = 2.55)
* `show` — Show Flavor Details



## `osc compute flavor access`

Flavor access command

**Usage:** `osc compute flavor access <COMMAND>`

###### **Subcommands:**

* `add` — Add Flavor Access To Tenant (addTenantAccess Action)
* `list` — List Flavor Access Information For Given Flavor
* `remove` — Remove Flavor Access From Tenant (removeTenantAccess Action)



## `osc compute flavor access add`

Add Flavor Access To Tenant (addTenantAccess Action)

**Usage:** `osc compute flavor access add --tenant <TENANT> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/flavors/{id}/action API

###### **Options:**

* `--tenant <TENANT>` — The UUID of the tenant in a multi-tenancy cloud



## `osc compute flavor access list`

List Flavor Access Information For Given Flavor

**Usage:** `osc compute flavor access list <FLAVOR_ID>`

###### **Arguments:**

* `<FLAVOR_ID>` — flavor_id parameter for /v2.1/flavors/{flavor_id}/os-flavor-access API



## `osc compute flavor access remove`

Remove Flavor Access From Tenant (removeTenantAccess Action)

**Usage:** `osc compute flavor access remove --tenant <TENANT> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/flavors/{id}/action API

###### **Options:**

* `--tenant <TENANT>` — The UUID of the tenant in a multi-tenancy cloud



## `osc compute flavor create255`

Creates a flavor.

Creating a flavor is typically only available to administrators of a cloud because this has implications for scheduling efficiently in the cloud.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute flavor create255 [OPTIONS] --disk <DISK> --name <NAME> --ram <RAM> --vcpus <VCPUS>`

###### **Options:**

* `--description <DESCRIPTION>` — A free form description of the flavor. Limited to 65535 characters in length. Only printable characters are allowed.

   **New in version 2.55**
* `--disk <DISK>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--id <ID>` — Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces and dots ‘.’ are permitted. If an ID is not provided, then a default UUID will be assigned
* `--name <NAME>` — The display name of a flavor
* `--os-flavor-access-is-public <OS_FLAVOR_ACCESS_IS_PUBLIC>` — Whether the flavor is public (available to all projects) or scoped to a set of projects. Default is True if not specified

  Possible values: `true`, `false`

* `--os-flv-ext-data-ephemeral <OS_FLV_EXT_DATA_EPHEMERAL>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--ram <RAM>` — The number of virtual CPUs that will be allocated to the server
* `--rxtx-factor <RXTX_FACTOR>` — The receive / transmit factor (as a float) that will be set on ports if the network backend supports the QOS extension. Otherwise it will be ignored. It defaults to 1.0
* `--swap <SWAP>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--vcpus <VCPUS>` — The number of virtual CPUs that will be allocated to the server



## `osc compute flavor create21`

Creates a flavor.

Creating a flavor is typically only available to administrators of a cloud because this has implications for scheduling efficiently in the cloud.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute flavor create21 [OPTIONS] --disk <DISK> --name <NAME> --ram <RAM> --vcpus <VCPUS>`

###### **Options:**

* `--disk <DISK>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--id <ID>` — Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces and dots ‘.’ are permitted. If an ID is not provided, then a default UUID will be assigned
* `--name <NAME>` — The display name of a flavor
* `--os-flavor-access-is-public <OS_FLAVOR_ACCESS_IS_PUBLIC>` — Whether the flavor is public (available to all projects) or scoped to a set of projects. Default is True if not specified

  Possible values: `true`, `false`

* `--os-flv-ext-data-ephemeral <OS_FLV_EXT_DATA_EPHEMERAL>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--ram <RAM>` — The number of virtual CPUs that will be allocated to the server
* `--rxtx-factor <RXTX_FACTOR>` — The receive / transmit factor (as a float) that will be set on ports if the network backend supports the QOS extension. Otherwise it will be ignored. It defaults to 1.0
* `--swap <SWAP>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--vcpus <VCPUS>` — The number of virtual CPUs that will be allocated to the server



## `osc compute flavor create20`

Creates a flavor.

Creating a flavor is typically only available to administrators of a cloud because this has implications for scheduling efficiently in the cloud.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute flavor create20 [OPTIONS] --disk <DISK> --name <NAME> --ram <RAM> --vcpus <VCPUS>`

###### **Options:**

* `--disk <DISK>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--id <ID>` — Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces and dots ‘.’ are permitted. If an ID is not provided, then a default UUID will be assigned
* `--name <NAME>` — The display name of a flavor
* `--os-flavor-access-is-public <OS_FLAVOR_ACCESS_IS_PUBLIC>` — Whether the flavor is public (available to all projects) or scoped to a set of projects. Default is True if not specified

  Possible values: `true`, `false`

* `--os-flv-ext-data-ephemeral <OS_FLV_EXT_DATA_EPHEMERAL>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--ram <RAM>` — The number of virtual CPUs that will be allocated to the server
* `--rxtx-factor <RXTX_FACTOR>` — The receive / transmit factor (as a float) that will be set on ports if the network backend supports the QOS extension. Otherwise it will be ignored. It defaults to 1.0
* `--swap <SWAP>` — The size of a dedicated swap disk that will be allocated, in MiB. If 0 (the default), no dedicated swap disk will be created
* `--vcpus <VCPUS>` — The number of virtual CPUs that will be allocated to the server



## `osc compute flavor delete`

Deletes a flavor.

This is typically an admin only action. Deleting a flavor that is in use by existing servers is not recommended as it can cause incorrect data to be returned to the user under some operations.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute flavor delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/flavors/{id} API



## `osc compute flavor extraspecs`

Flavor extra specs

**Usage:** `osc compute flavor extraspecs <COMMAND>`

###### **Subcommands:**

* `create` — Create Extra Specs For A Flavor
* `delete` — Delete An Extra Spec For A Flavor
* `list` — List Extra Specs For A Flavor
* `show` — Show An Extra Spec For A Flavor
* `set` — Update An Extra Spec For A Flavor




## `osc compute flavor extraspecs create`

Create Extra Specs For A Flavor

**Usage:** `osc compute flavor extraspecs create [OPTIONS] <FLAVOR_ID>`

###### **Arguments:**

* `<FLAVOR_ID>` — flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API

###### **Options:**

* `--extra-specs <key=value>` — A dictionary of the flavor’s extra-specs key-and-value pairs. It appears in the os-extra-specs’ “create” REQUEST body, as well as the os-extra-specs’ “create” and “list” RESPONSE body



## `osc compute flavor extraspecs delete`

Delete An Extra Spec For A Flavor

**Usage:** `osc compute flavor extraspecs delete <FLAVOR_ID> <ID>`

###### **Arguments:**

* `<FLAVOR_ID>` — flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API
* `<ID>` — id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API



## `osc compute flavor extraspecs list`

List Extra Specs For A Flavor

**Usage:** `osc compute flavor extraspecs list <FLAVOR_ID>`

###### **Arguments:**

* `<FLAVOR_ID>` — flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API



## `osc compute flavor extraspecs show`

Show An Extra Spec For A Flavor

**Usage:** `osc compute flavor extraspecs show <FLAVOR_ID> <ID>`

###### **Arguments:**

* `<FLAVOR_ID>` — flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API
* `<ID>` — id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API



## `osc compute flavor extraspecs set`

Update An Extra Spec For A Flavor


**Usage:** `osc compute flavor extraspecs set [OPTIONS] <FLAVOR_ID> <ID>`

###### **Arguments:**

* `<FLAVOR_ID>` — flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API
* `<ID>` — id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id} API

###### **Options:**

* `--property <key=value>`



## `osc compute flavor list`

Lists flavors with details.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403)

**Usage:** `osc compute flavor list [OPTIONS]`

###### **Options:**

* `--is-public <IS_PUBLIC>`
* `--limit <LIMIT>`
* `--marker <MARKER>`
* `--min-disk <MIN_DISK>`
* `--min-ram <MIN_RAM>`
* `--sort-dir <SORT_DIR>`

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>`

  Possible values: `created_at`, `description`, `disabled`, `ephemeral_gb`, `flavorid`, `id`, `is_public`, `memory_mb`, `name`, `root_gb`, `rxtx_factor`, `swap`, `updated_at`, `vcpu_weight`, `vcpus`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute flavor set255`

Updates a flavor description.

This API is available starting with microversion 2.55.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute flavor set255 --description <DESCRIPTION> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/flavors/{id} API

###### **Options:**

* `--description <DESCRIPTION>` — A free form description of the flavor. Limited to 65535 characters in length. Only printable characters are allowed



## `osc compute flavor show`

Shows details for a flavor.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute flavor show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/flavors/{id} API



## `osc compute hypervisor`

Hypervisors

**Usage:** `osc compute hypervisor <COMMAND>`

###### **Subcommands:**

* `list` — List Hypervisors Details
* `show` — Show Hypervisor Details



## `osc compute hypervisor list`

Lists hypervisors details.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403)

**Usage:** `osc compute hypervisor list [OPTIONS]`

###### **Options:**

* `--hypervisor-hostname-pattern <HYPERVISOR_HOSTNAME_PATTERN>`
* `--limit <LIMIT>`
* `--marker <MARKER>`
* `--with-servers <WITH_SERVERS>`

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute hypervisor show`

Shows details for a given hypervisor.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute hypervisor show [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-hypervisors/{id} API

###### **Options:**

* `--with-servers <WITH_SERVERS>`

  Possible values: `true`, `false`




## `osc compute keypair`

Keypairs commands

Generates, imports, and deletes SSH keys.

**Usage:** `osc compute keypair <COMMAND>`

###### **Subcommands:**

* `create292` — Import (or create) Keypair (microversion = 2.92)
* `create210` — Import (or create) Keypair (microversion = 2.10)
* `create22` — Import (or create) Keypair (microversion = 2.2)
* `create21` — Import (or create) Keypair (microversion = 2.1)
* `create20` — Import (or create) Keypair (microversion = 2.0)
* `delete` — Delete Keypair
* `list` — List Keypairs
* `show` — Show Keypair Details



## `osc compute keypair create292`

Imports (or generates) a keypair.

Normal response codes: 200, 201

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute keypair create292 [OPTIONS] --name <NAME> --public-key <PUBLIC_KEY>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later.

   Note

   Since microversion 2.92, allowed characters are ASCII letters `[a-zA-Z]`, digits `[0-9]` and the following special characters: `[@._- ]`.
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`.

   **New in version 2.2**

  Possible values: `ssh`, `x509`

* `--user-id <USER_ID>` — The user_id for a keypair. This allows administrative users to upload keys for other users than themselves.

   **New in version 2.10**



## `osc compute keypair create210`

Imports (or generates) a keypair.

Normal response codes: 200, 201

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute keypair create210 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later.

   Note

   Since microversion 2.92, allowed characters are ASCII letters `[a-zA-Z]`, digits `[0-9]` and the following special characters: `[@._- ]`.
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`.

   **New in version 2.2**

  Possible values: `ssh`, `x509`

* `--user-id <USER_ID>` — The user_id for a keypair. This allows administrative users to upload keys for other users than themselves.

   **New in version 2.10**



## `osc compute keypair create22`

Imports (or generates) a keypair.

Normal response codes: 200, 201

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute keypair create22 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later.

   Note

   Since microversion 2.92, allowed characters are ASCII letters `[a-zA-Z]`, digits `[0-9]` and the following special characters: `[@._- ]`.
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`.

   **New in version 2.2**

  Possible values: `ssh`, `x509`




## `osc compute keypair create21`

Imports (or generates) a keypair.

Normal response codes: 200, 201

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute keypair create21 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later.

   Note

   Since microversion 2.92, allowed characters are ASCII letters `[a-zA-Z]`, digits `[0-9]` and the following special characters: `[@._- ]`.
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you



## `osc compute keypair create20`

Imports (or generates) a keypair.

Normal response codes: 200, 201

Error response codes: badRequest(400), unauthorized(401), forbidden(403), conflict(409)

**Usage:** `osc compute keypair create20 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later.

   Note

   Since microversion 2.92, allowed characters are ASCII letters `[a-zA-Z]`, digits `[0-9]` and the following special characters: `[@._- ]`.
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you



## `osc compute keypair delete`

Deletes a keypair.

Normal response codes: 202, 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute keypair delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-keypairs/{id} API

###### **Options:**

* `--user-id <USER_ID>`



## `osc compute keypair list`

Lists keypairs that are associated with the account.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403)

**Usage:** `osc compute keypair list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>`
* `--marker <MARKER>`
* `--user-id <USER_ID>`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute keypair show`

Shows details for a keypair that is associated with the account.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute keypair show [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-keypairs/{id} API

###### **Options:**

* `--user-id <USER_ID>`



## `osc compute server`

**Servers (servers)**

Lists, creates, shows details for, updates, and deletes servers.

**Passwords**

When you create a server, you can specify a password through the optional adminPass attribute. The password must meet the complexity requirements set by your OpenStack Compute provider. The server might enter an ERROR state if the complexity requirements are not met. In this case, a client might issue a change password action to reset the server password.

If you do not specify a password, the API generates and assigns a random password that it returns in the response object. This password meets the security requirements set by the compute provider. For security reasons, subsequent GET calls do not require this password.

**Server metadata**

You can specify custom server metadata at server launch time. The maximum size for each metadata key-value pair is 255 bytes. The compute provider determines the maximum number of key-value pairs for each server. You can query this value through the maxServerMeta absolute limit.

**Usage:** `osc compute server <COMMAND>`

###### **Subcommands:**

* `add-fixed-ip21` — Add (Associate) Fixed Ip (addFixedIp Action) (DEPRECATED) (microversion = 2.1)
* `add-floating-ip21` — Add (Associate) Floating Ip (addFloatingIp Action) (DEPRECATED) (microversion = 2.1)
* `add-security-group` — Add Security Group To A Server (addSecurityGroup Action)
* `change-password` — Change Administrative Password (changePassword Action)
* `confirm-resize` — Confirm Resized Server (confirmResize Action)
* `create294` — Create Server (microversion = 2.94)
* `create290` — Create Server (microversion = 2.90)
* `create274` — Create Server (microversion = 2.74)
* `create267` — Create Server (microversion = 2.67)
* `create263` — Create Server (microversion = 2.63)
* `create257` — Create Server (microversion = 2.57)
* `create252` — Create Server (microversion = 2.52)
* `create242` — Create Server (microversion = 2.42)
* `create237` — Create Server (microversion = 2.37)
* `create233` — Create Server (microversion = 2.33)
* `create232` — Create Server (microversion = 2.32)
* `create219` — Create Server (microversion = 2.19)
* `create21` — Create Server (microversion = 2.1)
* `create-backup21` — Create Server Back Up (createBackup Action) (microversion = 2.1)
* `create-image21` — Create Image (createImage Action) (microversion = 2.1)
* `delete` — Delete Server
* `diagnostic` — Show Server Diagnostics
* `evacuate214` — Evacuate Server (evacuate Action) (microversion = 2.14)
* `evacuate229` — Evacuate Server (evacuate Action) (microversion = 2.29)
* `evacuate268` — Evacuate Server (evacuate Action) (microversion = 2.68)
* `evacuate295` — Evacuate Server (evacuate Action) (microversion = 2.95)
* `force-delete` — Force-Delete Server (forceDelete Action)
* `get-console-output` — Show Console Output (os-getConsoleOutput Action)
* `instance-action` — Servers actions
* `interface` — Port interfaces (servers, os-interface)
* `inject-network-info` — Inject Network Information (injectNetworkInfo Action)
* `ip` — Servers IPs (servers, ips)
* `list` — List Servers Detailed
* `live-migrate20` — Live-Migrate Server (os-migrateLive Action) (microversion = 2.0)
* `live-migrate225` — Live-Migrate Server (os-migrateLive Action) (microversion = 2.25)
* `live-migrate230` — Live-Migrate Server (os-migrateLive Action) (microversion = 2.30)
* `live-migrate268` — Live-Migrate Server (os-migrateLive Action) (microversion = 2.68)
* `lock273` — Lock Server (lock Action) (microversion = 2.73)
* `metadata` — Server metadata
* `migrate256` — Migrate Server (migrate Action) (microversion = 2.56)
* `migration` — Server migrations (servers, migrations)
* `password` — Servers password
* `pause` — Pause Server (pause Action)
* `reset-state` — Reset Server State (os-resetState Action)
* `reboot` — Reboot Server (reboot Action)
* `rebuild21` — Rebuild Server (rebuild Action) (microversion = 2.1)
* `rebuild219` — Rebuild Server (rebuild Action) (microversion = 2.19)
* `rebuild254` — Rebuild Server (rebuild Action) (microversion = 2.54)
* `rebuild257` — Rebuild Server (rebuild Action) (microversion = 2.57)
* `rebuild263` — Rebuild Server (rebuild Action) (microversion = 2.63)
* `rebuild290` — Rebuild Server (rebuild Action) (microversion = 2.90)
* `rebuild294` — Rebuild Server (rebuild Action) (microversion = 2.94)
* `remote-console` — Server Consoles
* `remove-fixed-ip21` — Remove (Disassociate) Fixed Ip (removeFixedIp Action) (DEPRECATED) (microversion = 2.1)
* `remove-floating-ip21` — Remove (Disassociate) Floating Ip (removeFloatingIp Action) (DEPRECATED) (microversion = 2.1)
* `remove-security-group` — Remove Security Group From A Server (removeSecurityGroup Action)
* `rescue` — Rescue Server (rescue Action)
* `reset-network` — Reset Networking On A Server (resetNetwork Action) (DEPRECATED)
* `resize` — Resize Server (resize Action)
* `restore` — Restore Soft-Deleted Instance (restore Action)
* `resume` — Resume Suspended Server (resume Action)
* `revert-resize` — Revert Resized Server (revertResize Action)
* `security-groups` — List Security Groups By Server
* `set21` — Update Server (microversion = 2.1)
* `set219` — Update Server (microversion = 2.19)
* `set290` — Update Server (microversion = 2.90)
* `set294` — Update Server (microversion = 2.94)
* `shelve` — Shelve Server (shelve Action)
* `shelve-offload` — Shelf-Offload (Remove) Server (shelveOffload Action)
* `show` — Show Server Details
* `start` — Start Server (os-start Action)
* `stop` — Stop Server (os-stop Action)
* `suspend` — Suspend Server (suspend Action)
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a server, checks the existence of a tag for a server
* `topology` — Show Server Topology
* `trigger-crash-dump217` — Command without description in OpenAPI
* `unlock21` — Unlock Server (unlock Action) (microversion = 2.1)
* `unpause` — Unpause Server (unpause Action)
* `unrescue` — Unrescue Server (unrescue Action)
* `unshelve277` — Unshelve (Restore) Shelved Server (unshelve Action) (microversion = 2.77)
* `unshelve291` — Unshelve (Restore) Shelved Server (unshelve Action) (microversion = 2.91)
* `volume-attachment` — Servers with volume attachments



## `osc compute server add-fixed-ip21`

Adds a fixed IP address to a server instance, which associates that address with the server. The fixed IP address is retrieved from the network that you specify in the request.

Specify the `addFixedIp` action and the network ID in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server add-fixed-ip21 --network-id <NETWORK_ID> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--network-id <NETWORK_ID>` — The network ID



## `osc compute server add-floating-ip21`

Adds a floating IP address to a server, which associates that address with the server.

A pool of floating IP addresses, configured by the cloud administrator, is available in OpenStack Compute. The project quota defines the maximum number of floating IP addresses that you can allocate to the project. After you [create (allocate) a floating IPaddress](https://docs.openstack.org/api-ref/compute/#create-allocate-floating-ip-address) for a project, you can associate that address with the server. Specify the `addFloatingIp` action in the request body.

If an instance is connected to multiple networks, you can associate a floating IP address with a specific fixed IP address by using the optional `fixed_address` parameter.

**Preconditions**

The server must exist.

You can only add a floating IP address to the server when its status is `ACTIVE` or `STOPPED`

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server add-floating-ip21 [OPTIONS] --address <ADDRESS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--address <ADDRESS>` — The fixed IP address with which you want to associate the floating IP address
* `--fixed-address <FIXED_ADDRESS>` — The fixed IP address with which you want to associate the floating IP address



## `osc compute server add-security-group`

Adds a security group to a server.

Specify the `addSecurityGroup` action in the request body.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server add-security-group [OPTIONS] --name <NAME> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--name <NAME>` — The security group name
* `--property <key=value>` — Additional properties to be sent with the request



## `osc compute server change-password`

Changes the administrative password for a server.

Specify the `changePassword` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server change-password --admin-pass <ADMIN_PASS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--admin-pass <ADMIN_PASS>` — The administrative password for the server



## `osc compute server confirm-resize`

Confirms a pending resize action for a server.

Specify the `confirmResize` action in the request body.

After you make this request, you typically must keep polling the server status to determine whether the request succeeded. A successfully confirming resize operation shows a status of `ACTIVE` or `SHUTOFF` and a migration status of `confirmed`. You can also see the resized server in the compute node that OpenStack Compute manages.

**Preconditions**

You can only confirm the resized server where the status is `VERIFY_RESIZE`.

If the server is locked, you must have administrator privileges to confirm the server.

**Troubleshooting**

If the server status remains `VERIFY_RESIZE`, the request failed. Ensure you meet the preconditions and run the request again. If the request fails again, the server status should be `ERROR` and a migration status of `error`. Investigate the compute back end or ask your cloud provider. There are some options for trying to correct the server status:

Note that the cloud provider may still need to cleanup any orphaned resources on the source hypervisor.

Normal response codes: 204

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server confirm-resize --confirm-resize <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--confirm-resize <JSON>`



## `osc compute server create294`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create294 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--host <HOST>` — The hostname of the hypervisor on which the server is to be created. The API will return 400 if no hypervisors are found with the given hostname. By default, it can be specified by administrators only.

   **New in version 2.74**
* `--hostname <HOSTNAME>` — The hostname to configure for the instance in the metadata service.

   Starting with microversion 2.94, this can be a Fully Qualified Domain Name (FQDN) of up to 255 characters in length.

   Note

   This information is published via the metadata service and requires application such as `cloud-init` to propagate it through to the instance.

   **New in version 2.90**
* `--hypervisor-hostname <HYPERVISOR_HOSTNAME>` — The hostname of the hypervisor on which the server is to be created. The API will return 400 if no hypervisors are found with the given hostname. By default, it can be specified by administrators only.

   **New in version 2.74**
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server create requests if allowed by policy, and is not supported for volume-backed instances.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create290`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create290 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--host <HOST>` — The hostname of the hypervisor on which the server is to be created. The API will return 400 if no hypervisors are found with the given hostname. By default, it can be specified by administrators only.

   **New in version 2.74**
* `--hostname <HOSTNAME>` — The hostname to configure for the instance in the metadata service.

   Starting with microversion 2.94, this can be a Fully Qualified Domain Name (FQDN) of up to 255 characters in length.

   Note

   This information is published via the metadata service and requires application such as `cloud-init` to propagate it through to the instance.

   **New in version 2.90**
* `--hypervisor-hostname <HYPERVISOR_HOSTNAME>` — The hostname of the hypervisor on which the server is to be created. The API will return 400 if no hypervisors are found with the given hostname. By default, it can be specified by administrators only.

   **New in version 2.74**
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server create requests if allowed by policy, and is not supported for volume-backed instances.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create274`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create274 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--host <HOST>` — The hostname of the hypervisor on which the server is to be created. The API will return 400 if no hypervisors are found with the given hostname. By default, it can be specified by administrators only.

   **New in version 2.74**
* `--hypervisor-hostname <HYPERVISOR_HOSTNAME>` — The hostname of the hypervisor on which the server is to be created. The API will return 400 if no hypervisors are found with the given hostname. By default, it can be specified by administrators only.

   **New in version 2.74**
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server create requests if allowed by policy, and is not supported for volume-backed instances.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create267`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create267 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server create requests if allowed by policy, and is not supported for volume-backed instances.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create263`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create263 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server create requests if allowed by policy, and is not supported for volume-backed instances.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create257`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create257 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create252`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create252 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--tags <TAGS>` — A list of tags. Tags have the following restrictions:

   - Tag is a Unicode bytestring no longer than 60 characters. - Tag is a non-empty string. - ‘/’ is not allowed to be in a tag name - Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags - All other characters are allowed to be in a tag name - Each server can have up to 50 tags.

   **New in version 2.52**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create242`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create242 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create237`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create237 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME> <--auto-networks|--networks <JSON>|--none-networks>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--auto-networks`
* `--networks <JSON>`
* `--none-networks`
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create233`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create233 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--networks <JSON>` — A list of `network` object. Required parameter when there are multiple networks defined for the tenant. When you do not specify the networks parameter, the server attaches to the only network created for the current tenant. Optionally, you can create one or more NICs on the server. To provision the server instance with a NIC for a network, specify the UUID of the network in the `uuid` attribute in a `networks` object. To provision the server instance with a NIC for an already existing port, specify the port-id in the `port` attribute in a `networks` object.

   If multiple networks are defined, the order in which they appear in the guest operating system will not necessarily reflect the order in which they are given in the server boot request. Guests should therefore not depend on device order to deduce any information about their network devices. Instead, device role tags should be used: introduced in 2.32, broken in 2.37, and re-introduced and fixed in 2.42, the `tag` is an optional, string attribute that can be used to assign a tag to a virtual network interface. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that network interface, such as bus (ex: PCI), bus address (ex: 0000:00:02.0), and MAC address.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.37. Therefore, network interfaces could only be tagged in versions 2.32 to 2.36 inclusively. Version 2.42 has restored the `tag` attribute.

   Starting with microversion 2.37, this field is required and the special string values *auto* and *none* can be specified for networks. *auto* tells the Compute service to use a network that is available to the project, if one exists. If one does not exist, the Compute service will attempt to automatically allocate a network for the project (if possible). *none* tells the Compute service to not allocate a network for the instance. The *auto* and *none* values cannot be used with any other network values, including other network uuids, ports, fixed IPs or device tags. These are requested as strings for the networks value, not in a list. See the associated example.
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create232`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create232 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--networks <JSON>` — A list of `network` object. Required parameter when there are multiple networks defined for the tenant. When you do not specify the networks parameter, the server attaches to the only network created for the current tenant. Optionally, you can create one or more NICs on the server. To provision the server instance with a NIC for a network, specify the UUID of the network in the `uuid` attribute in a `networks` object. To provision the server instance with a NIC for an already existing port, specify the port-id in the `port` attribute in a `networks` object.

   If multiple networks are defined, the order in which they appear in the guest operating system will not necessarily reflect the order in which they are given in the server boot request. Guests should therefore not depend on device order to deduce any information about their network devices. Instead, device role tags should be used: introduced in 2.32, broken in 2.37, and re-introduced and fixed in 2.42, the `tag` is an optional, string attribute that can be used to assign a tag to a virtual network interface. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that network interface, such as bus (ex: PCI), bus address (ex: 0000:00:02.0), and MAC address.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.37. Therefore, network interfaces could only be tagged in versions 2.32 to 2.36 inclusively. Version 2.42 has restored the `tag` attribute.

   Starting with microversion 2.37, this field is required and the special string values *auto* and *none* can be specified for networks. *auto* tells the Compute service to use a network that is available to the project, if one exists. If one does not exist, the Compute service will attempt to automatically allocate a network for the project (if possible). *none* tells the Compute service to not allocate a network for the instance. The *auto* and *none* values cannot be used with any other network values, including other network uuids, ports, fixed IPs or device tags. These are requested as strings for the networks value, not in a list. See the associated example.
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create219`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create219 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--networks <JSON>` — A list of `network` object. Required parameter when there are multiple networks defined for the tenant. When you do not specify the networks parameter, the server attaches to the only network created for the current tenant. Optionally, you can create one or more NICs on the server. To provision the server instance with a NIC for a network, specify the UUID of the network in the `uuid` attribute in a `networks` object. To provision the server instance with a NIC for an already existing port, specify the port-id in the `port` attribute in a `networks` object.

   If multiple networks are defined, the order in which they appear in the guest operating system will not necessarily reflect the order in which they are given in the server boot request. Guests should therefore not depend on device order to deduce any information about their network devices. Instead, device role tags should be used: introduced in 2.32, broken in 2.37, and re-introduced and fixed in 2.42, the `tag` is an optional, string attribute that can be used to assign a tag to a virtual network interface. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that network interface, such as bus (ex: PCI), bus address (ex: 0000:00:02.0), and MAC address.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.37. Therefore, network interfaces could only be tagged in versions 2.32 to 2.36 inclusively. Version 2.42 has restored the `tag` attribute.

   Starting with microversion 2.37, this field is required and the special string values *auto* and *none* can be specified for networks. *auto* tells the Compute service to use a network that is available to the project, if one exists. If one does not exist, the Compute service will attempt to automatically allocate a network for the project (if possible). *none* tells the Compute service to not allocate a network for the instance. The *auto* and *none* values cannot be used with any other network values, including other network uuids, ports, fixed IPs or device tags. These are requested as strings for the networks value, not in a list. See the associated example.
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create21`

Creates a server.

The progress of this operation depends on the location of the requested image, network I/O, host load, selected flavor, and other factors.

To check the progress of the request, make a `GET /servers/{id}` request. This call returns a progress attribute, which is a percentage value from 0 to 100.

The `Location` header returns the full URL to the newly created server and is available as a `self` and `bookmark` link in the server representation.

When you create a server, the response shows only the server ID, its links, and the admin password. You can get additional attributes through subsequent `GET` requests on the server.

Include the `block_device_mapping_v2` parameter in the create request body to boot a server from a volume.

Include the `key_name` parameter in the create request body to add a keypair to the server when you create it. To create a keypair, make a [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair) request.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server create21 [OPTIONS] --flavor-ref <FLAVOR_REF> --name <NAME>`

###### **Options:**

* `--build-near-host-ip <BUILD_NEAR_HOST_IP>` — Schedule the server on a host in the network specified with this parameter and a cidr (`os:scheduler_hints.cidr`). It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--cidr <CIDR>` — Schedule the server on a host in the network specified with an IP address (`os:scheduler_hints:build_near_host_ip`) and this parameter. If `os:scheduler_hints:build_near_host_ip` is specified and this parameter is omitted, `/24` is used. It is available when `SimpleCIDRAffinityFilter` is available on cloud side
* `--different-cell <DIFFERENT_CELL>` — A list of cell routes or a cell route (string). Schedule the server in a cell that is not specified. It is available when `DifferentCellFilter` is available on cloud side that is cell v1 environment
* `--different-host <DIFFERENT_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on a different host from a set of servers. It is available when `DifferentHostFilter` is available on cloud side
* `--group <GROUP>` — The server group UUID. Schedule the server according to a policy of the server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or `soft-affinity`). It is available when `ServerGroupAffinityFilter`, `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`, `ServerGroupSoftAffinityWeigher` are available on cloud side
* `--query <JSON>` — Schedule the server by using a custom filter in JSON format. For example:

   ```text "query": "[\">=\",\"$free_ram_mb\",1024]"

   ```

   It is available when `JsonFilter` is available on cloud side.
* `--same-host <SAME_HOST>` — A list of server UUIDs or a server UUID. Schedule the server on the same host as another server in a set of servers. It is available when `SameHostFilter` is available on cloud side
* `--target-cell <TARGET_CELL>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--availability-zone <AVAILABILITY_ZONE>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--block-device-mapping <JSON>`
* `--block-device-mapping-v2 <JSON>` — Enables fine grained control of the block device mapping for an instance. This is typically used for booting servers from volumes. An example format would look as follows:

   > ```text > "block_device_mapping_v2": [{ >     "boot_index": "0", >     "uuid": "ac408821-c95a-448f-9292-73986c790911", >     "source_type": "image", >     "volume_size": "25", >     "destination_type": "volume", >     "delete_on_termination": true, >     "tag": "disk1", >     "disk_bus": "scsi"}] > > ```

   In microversion 2.32, `tag` is an optional string attribute that can be used to assign a tag to the block device. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that block device, such as bus (ex: SCSI), bus address (ex: 1:0:2:0), and serial.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.33. It has been restored in version 2.42.
* `--config-drive <CONFIG_DRIVE>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--flavor-ref <FLAVOR_REF>` — The flavor reference, as an ID (including a UUID) or full URL, for the flavor for your server instance
* `--image-ref <IMAGE_REF>` — The UUID of the image to use for your server instance. This is not required in case of boot from volume. In all other cases it is required and must be a valid UUID otherwise API will return 400
* `--key-name <KEY_NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--max-count <MAX_COUNT>`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--min-count <MIN_COUNT>`
* `--name <NAME>` — A target cell name. Schedule the server in a host in the cell specified. It is available when `TargetCellFilter` is available on cloud side that is cell v1 environment
* `--networks <JSON>` — A list of `network` object. Required parameter when there are multiple networks defined for the tenant. When you do not specify the networks parameter, the server attaches to the only network created for the current tenant. Optionally, you can create one or more NICs on the server. To provision the server instance with a NIC for a network, specify the UUID of the network in the `uuid` attribute in a `networks` object. To provision the server instance with a NIC for an already existing port, specify the port-id in the `port` attribute in a `networks` object.

   If multiple networks are defined, the order in which they appear in the guest operating system will not necessarily reflect the order in which they are given in the server boot request. Guests should therefore not depend on device order to deduce any information about their network devices. Instead, device role tags should be used: introduced in 2.32, broken in 2.37, and re-introduced and fixed in 2.42, the `tag` is an optional, string attribute that can be used to assign a tag to a virtual network interface. This tag is then exposed to the guest in the metadata API and the config drive and is associated to hardware metadata for that network interface, such as bus (ex: PCI), bus address (ex: 0000:00:02.0), and MAC address.

   A bug has caused the `tag` attribute to no longer be accepted starting with version 2.37. Therefore, network interfaces could only be tagged in versions 2.32 to 2.36 inclusively. Version 2.42 has restored the `tag` attribute.

   Starting with microversion 2.37, this field is required and the special string values *auto* and *none* can be specified for networks. *auto* tells the Compute service to use a network that is available to the project, if one exists. If one does not exist, the Compute service will attempt to automatically allocate a network for the project (if possible). *none* tells the Compute service to not allocate a network for the instance. The *auto* and *none* values cannot be used with any other network values, including other network uuids, ports, fixed IPs or device tags. These are requested as strings for the networks value, not in a list. See the associated example.
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--return-reservation-id <RETURN_RESERVATION_ID>` — Indicates whether a config drive enables metadata injection. The config_drive setting provides information about a drive that the instance can mount at boot time. The instance reads files from the drive to get information that is normally available through the metadata service. This metadata is different from the user data. Not all cloud providers enable the `config_drive`. Read more in the [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html)

  Possible values: `true`, `false`

* `--security-groups <SECURITY_GROUPS>` — One or more security groups. Specify the name of the security group in the `name` attribute. If you omit this attribute, the API creates the server in the `default` security group. Requested security groups are not applied to pre-existing ports
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon launch. Must be Base64 encoded. Restricted to 65535 bytes.

   Note

   The `null` value allowed in Nova legacy v2 API, but due to the strict input validation, it isn’t allowed in Nova v2.1 API.



## `osc compute server create-backup21`

Create Server Back Up (createBackup Action) (microversion = 2.1)

**Usage:** `osc compute server create-backup21 [OPTIONS] --backup-type <BACKUP_TYPE> --name <NAME> --rotation <ROTATION> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--backup-type <BACKUP_TYPE>` — The type of the backup, for example, `daily`
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The name of the image to be backed up
* `--rotation <ROTATION>` — The rotation of the back up image, the oldest image will be removed when image count exceed the rotation count



## `osc compute server create-image21`

Create Image (createImage Action) (microversion = 2.1)

**Usage:** `osc compute server create-image21 [OPTIONS] --name <NAME> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--metadata <key=value>` — Metadata key and value pairs for the image. The maximum size for each metadata key and value pair is 255 bytes
* `--name <NAME>` — The display name of an Image



## `osc compute server delete`

Deletes a server.

By default, the instance is going to be (hard) deleted immediately from the system, but you can set `reclaim_instance_interval` > 0 to make the API soft delete the instance, so that the instance won’t be deleted until the `reclaim_instance_interval` has expired since the instance was soft deleted. The instance marked as `SOFT_DELETED` can be recovered via `restore` action before it’s really deleted from the system.

**Preconditions**

**Asynchronous postconditions**

**Troubleshooting**

Normal response codes: 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id} API



## `osc compute server diagnostic`

Shows basic usage data for a server.

Policy defaults enable only users with the administrative role. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), notfound(404), conflict(409), notimplemented(501)

**Usage:** `osc compute server diagnostic <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/diagnostics API



## `osc compute server evacuate214`

Evacuate Server (evacuate Action) (microversion = 2.14)

**Usage:** `osc compute server evacuate214 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--admin-pass <ADMIN_PASS>` — An administrative password to access the evacuated server. If you omit this parameter, the operation generates a new password. Up to API version 2.13, if `onSharedStorage` is set to `True` and this parameter is specified, an error is raised
* `--host <HOST>` — The name or ID of the host to which the server is evacuated. If you omit this parameter, the scheduler chooses a host.

   Warning

   Prior to microversion 2.29, specifying a host will bypass validation by the scheduler, which could result in failures to actually evacuate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.29 and without `force=True` set.



## `osc compute server evacuate229`

Evacuate Server (evacuate Action) (microversion = 2.29)

**Usage:** `osc compute server evacuate229 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--admin-pass <ADMIN_PASS>` — An administrative password to access the evacuated server. If you omit this parameter, the operation generates a new password. Up to API version 2.13, if `onSharedStorage` is set to `True` and this parameter is specified, an error is raised
* `--force <FORCE>` — Force an evacuation by not verifying the provided destination host by the scheduler.

   Warning

   This could result in failures to actually evacuate the instance to the specified host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host without `force=True` set.

   Furthermore, this should not be specified when evacuating instances managed by a clustered hypervisor driver like ironic since you cannot specify a node, so the compute service will pick a node randomly which may not be able to accommodate the instance.

   **New in version 2.29**

   **Available until version 2.67**

  Possible values: `true`, `false`

* `--host <HOST>` — The name or ID of the host to which the server is evacuated. If you omit this parameter, the scheduler chooses a host.

   Warning

   Prior to microversion 2.29, specifying a host will bypass validation by the scheduler, which could result in failures to actually evacuate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.29 and without `force=True` set.



## `osc compute server evacuate268`

Evacuate Server (evacuate Action) (microversion = 2.68)

**Usage:** `osc compute server evacuate268 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--admin-pass <ADMIN_PASS>` — An administrative password to access the evacuated server. If you omit this parameter, the operation generates a new password. Up to API version 2.13, if `onSharedStorage` is set to `True` and this parameter is specified, an error is raised
* `--host <HOST>` — The name or ID of the host to which the server is evacuated. If you omit this parameter, the scheduler chooses a host.

   Warning

   Prior to microversion 2.29, specifying a host will bypass validation by the scheduler, which could result in failures to actually evacuate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.29 and without `force=True` set.



## `osc compute server evacuate295`

Evacuate Server (evacuate Action) (microversion = 2.95)

**Usage:** `osc compute server evacuate295 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--admin-pass <ADMIN_PASS>` — An administrative password to access the evacuated server. If you omit this parameter, the operation generates a new password. Up to API version 2.13, if `onSharedStorage` is set to `True` and this parameter is specified, an error is raised
* `--host <HOST>` — The name or ID of the host to which the server is evacuated. If you omit this parameter, the scheduler chooses a host.

   Warning

   Prior to microversion 2.29, specifying a host will bypass validation by the scheduler, which could result in failures to actually evacuate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.29 and without `force=True` set.



## `osc compute server force-delete`

Force-deletes a server before deferred cleanup.

Specify the `forceDelete` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server force-delete --force-delete <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--force-delete <JSON>`



## `osc compute server get-console-output`

Shows console output for a server.

This API returns the text of the console since boot. The content returned may be large. Limit the lines of console text, beginning at the tail of the content, by setting the optional `length` parameter in the request body.

The server to get console log from should set `export LC_ALL=en_US.UTF-8` in order to avoid incorrect unicode error.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), notFound(404), conflict(409), methodNotImplemented(501)

**Usage:** `osc compute server get-console-output [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--length <LENGTH>` — The number of lines to fetch from the end of console log. All lines will be returned if this is not specified.

   Note

   This parameter can be specified as not only ‘integer’ but also ‘string’.



## `osc compute server instance-action`

Servers actions

List actions and action details for a server.

**Usage:** `osc compute server instance-action <COMMAND>`

###### **Subcommands:**

* `list` — List Actions For Server
* `show` — Show Server Action Details



## `osc compute server instance-action list`

Lists actions for a server.

Action information of deleted instances can be returned for requests starting with microversion 2.21.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server instance-action list [OPTIONS] <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-instance-actions/{id} API

###### **Options:**

* `--changes-before <CHANGES_BEFORE>`
* `--changes-since <CHANGES_SINCE>`
* `--limit <LIMIT>`
* `--marker <MARKER>`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute server instance-action show`

Shows details for a server action.

Action details of deleted instances can be returned for requests later than microversion 2.21.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server instance-action show <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-instance-actions/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-instance-actions/{id} API



## `osc compute server interface`

Port interfaces (servers, os-interface)

List port interfaces, show port interface details of the given server. Create a port interface and uses it to attach a port to the given server, detach a port interface from the given server.

**Usage:** `osc compute server interface <COMMAND>`

###### **Subcommands:**

* `create249` — Create Interface (microversion = 2.49)
* `delete` — Detach Interface
* `list` — List Port Interfaces
* `show` — Show Port Interface Details



## `osc compute server interface create249`

Creates a port interface and uses it to attach a port to a server.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), computeFault(500), NotImplemented(501)

**Usage:** `osc compute server interface create249 [OPTIONS] <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-interface/{id} API

###### **Options:**

* `--fixed-ips <FIXED_IPS>` — Fixed IP addresses. If you request a specific fixed IP address without a `net_id`, the request returns a `Bad Request (400)` response code
* `--net-id <NET_ID>` — The ID of the network for which you want to create a port interface. The `net_id` and `port_id` parameters are mutually exclusive. If you do not specify the `net_id` parameter, the OpenStack Networking API v2.0 uses the network information cache that is associated with the instance
* `--port-id <PORT_ID>` — The ID of the port for which you want to create an interface. The `net_id` and `port_id` parameters are mutually exclusive. If you do not specify the `port_id` parameter, the OpenStack Networking API v2.0 allocates a port and creates an interface for it on the network
* `--tag <TAG>` — A device role tag that can be applied to a network interface when attaching it to the VM. The guest OS of a server that has devices tagged in this manner can access hardware metadata about the tagged devices from the metadata API and on the config drive, if enabled.

   **New in version 2.49**



## `osc compute server interface delete`

Detaches a port interface from a server.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), NotImplemented(501)

**Usage:** `osc compute server interface delete <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-interface/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-interface/{id} API



## `osc compute server interface list`

Lists port interfaces that are attached to a server.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), NotImplemented(501)

**Usage:** `osc compute server interface list <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-interface/{id} API



## `osc compute server interface show`

Shows details for a port interface that is attached to a server.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server interface show <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-interface/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-interface/{id} API



## `osc compute server inject-network-info`

Injects network information into a server.

Specify the `injectNetworkInfo` action in the request body.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server inject-network-info --inject-network-info <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--inject-network-info <JSON>`



## `osc compute server ip`

Servers IPs (servers, ips)

Lists the IP addresses for an instance and shows details for an IP address.

**Usage:** `osc compute server ip <COMMAND>`

###### **Subcommands:**

* `list` — List Ips
* `show` — Show Ip Details



## `osc compute server ip list`

Lists IP addresses that are assigned to an instance.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server ip list <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/ips/{id} API



## `osc compute server ip show`

Shows IP addresses details for a network label of a server instance.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server ip show <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/ips/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/ips/{id} API



## `osc compute server list`

For each server, shows server details including config drive, extended status, and server usage information.

The extended status information appears in the OS-EXT-STS:vm_state, OS-EXT-STS:power_state, and OS-EXT-STS:task_state attributes.

The server usage information appears in the OS-SRV-USG:launched_at and OS-SRV-USG:terminated_at attributes.

HostId is unique per account and is not globally unique.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403)

**Usage:** `osc compute server list [OPTIONS]`

###### **Options:**

* `--access-ip-v4 <ACCESS_IP_V4>`
* `--access-ip-v6 <ACCESS_IP_V6>`
* `--all-tenants <ALL_TENANTS>`
* `--auto-disk-config <AUTO_DISK_CONFIG>`
* `--availability-zone <AVAILABILITY_ZONE>`
* `--block-device-mapping <BLOCK_DEVICE_MAPPING>`
* `--changes-before <CHANGES_BEFORE>`
* `--changes-since <CHANGES_SINCE>`
* `--config-drive <CONFIG_DRIVE>`
* `--created-at <CREATED_AT>`
* `--deleted <DELETED>`
* `--description <DESCRIPTION>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--flavor <FLAVOR>`
* `--host <HOST>`
* `--hostname <HOSTNAME>`
* `--image <IMAGE>`
* `--image-ref <IMAGE_REF>`
* `--info-cache <INFO_CACHE>`
* `--ip <IP>`
* `--ip6 <IP6>`
* `--kernel-id <KERNEL_ID>`
* `--key-name <KEY_NAME>`
* `--launch-index <LAUNCH_INDEX>`
* `--launched-at <LAUNCHED_AT>`
* `--limit <LIMIT>`
* `--locked <LOCKED>`
* `--locked-by <LOCKED_BY>`
* `--marker <MARKER>`
* `--metadata <METADATA>`
* `--name <NAME>`
* `--node <NODE>`
* `--not-tags <NOT_TAGS>`
* `--not-tags-any <NOT_TAGS_ANY>`
* `--pci-devices <PCI_DEVICES>`
* `--power-state <POWER_STATE>`
* `--progress <PROGRESS>`
* `--project-id <PROJECT_ID>`
* `--ramdisk-id <RAMDISK_ID>`
* `--reservation-id <RESERVATION_ID>`
* `--root-device-name <ROOT_DEVICE_NAME>`
* `--security-groups <SECURITY_GROUPS>`
* `--services <SERVICES>`
* `--soft-deleted <SOFT_DELETED>`
* `--sort-dir <SORT_DIR>`
* `--sort-key <SORT_KEY>`

  Possible values: `access_ip_v4`, `access_ip_v6`, `auto_disk_config`, `availability_zone`, `config_drive`, `created_at`, `display_description`, `display_name`, `host`, `hostname`, `image_ref`, `instance_type_id`, `kernel_id`, `key_name`, `launch_index`, `launched_at`, `locked`, `locked_by`, `node`, `power_state`, `progress`, `project_id`, `ramdisk_id`, `root_device_name`, `task_state`, `terminated_at`, `updated_at`, `user_id`, `uuid`, `vm_state`

* `--status <STATUS>`
* `--system-metadata <SYSTEM_METADATA>`
* `--tags <TAGS>`
* `--tags-any <TAGS_ANY>`
* `--task-state <TASK_STATE>`
* `--tenant-id <TENANT_ID>`
* `--terminated-at <TERMINATED_AT>`
* `--user-id <USER_ID>`
* `--uuid <UUID>`
* `--vm-state <VM_STATE>`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute server live-migrate20`

Live-Migrate Server (os-migrateLive Action) (microversion = 2.0)

**Usage:** `osc compute server live-migrate20 --block-migration <BLOCK_MIGRATION> --disk-over-commit <DISK_OVER_COMMIT> --host <HOST> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--block-migration <BLOCK_MIGRATION>` — Set to `True` to enable over commit when the destination host is checked for available disk space. Set to `False` to disable over commit. This setting affects only the libvirt virt driver.

   **Available until version 2.25**

  Possible values: `true`, `false`

* `--disk-over-commit <DISK_OVER_COMMIT>` — Set to `True` to enable over commit when the destination host is checked for available disk space. Set to `False` to disable over commit. This setting affects only the libvirt virt driver.

   **Available until version 2.25**

  Possible values: `true`, `false`

* `--host <HOST>` — The host to which to migrate the server. If this parameter is `None`, the scheduler chooses a host.

   Warning

   Prior to microversion 2.30, specifying a host will bypass validation by the scheduler, which could result in failures to actually migrate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.30 and without `force=True` set.



## `osc compute server live-migrate225`

Live-Migrate Server (os-migrateLive Action) (microversion = 2.25)

**Usage:** `osc compute server live-migrate225 --block-migration <BLOCK_MIGRATION> --host <HOST> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--block-migration <BLOCK_MIGRATION>` — Migrates local disks by using block migration. Set to `auto` which means nova will detect whether source and destination hosts on shared storage. if they are on shared storage, the live-migration won’t be block migration. Otherwise the block migration will be executed. Set to `True`, means the request will fail when the source or destination host uses shared storage. Set to `False` means the request will fail when the source and destination hosts are not on the shared storage.

   **New in version 2.25**

  Possible values: `true`, `false`

* `--host <HOST>` — The host to which to migrate the server. If this parameter is `None`, the scheduler chooses a host.

   Warning

   Prior to microversion 2.30, specifying a host will bypass validation by the scheduler, which could result in failures to actually migrate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.30 and without `force=True` set.



## `osc compute server live-migrate230`

Live-Migrate Server (os-migrateLive Action) (microversion = 2.30)

**Usage:** `osc compute server live-migrate230 [OPTIONS] --block-migration <BLOCK_MIGRATION> --host <HOST> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--block-migration <BLOCK_MIGRATION>` — Migrates local disks by using block migration. Set to `auto` which means nova will detect whether source and destination hosts on shared storage. if they are on shared storage, the live-migration won’t be block migration. Otherwise the block migration will be executed. Set to `True`, means the request will fail when the source or destination host uses shared storage. Set to `False` means the request will fail when the source and destination hosts are not on the shared storage.

   **New in version 2.25**

  Possible values: `true`, `false`

* `--force <FORCE>` — Force a live-migration by not verifying the provided destination host by the scheduler.

   Warning

   This could result in failures to actually live migrate the instance to the specified host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host without `force=True` set.

   **New in version 2.30**

   **Available until version 2.67**

  Possible values: `true`, `false`

* `--host <HOST>` — The host to which to migrate the server. If this parameter is `None`, the scheduler chooses a host.

   Warning

   Prior to microversion 2.30, specifying a host will bypass validation by the scheduler, which could result in failures to actually migrate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.30 and without `force=True` set.



## `osc compute server live-migrate268`

Live-Migrate Server (os-migrateLive Action) (microversion = 2.68)

**Usage:** `osc compute server live-migrate268 --block-migration <BLOCK_MIGRATION> --host <HOST> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--block-migration <BLOCK_MIGRATION>` — Migrates local disks by using block migration. Set to `auto` which means nova will detect whether source and destination hosts on shared storage. if they are on shared storage, the live-migration won’t be block migration. Otherwise the block migration will be executed. Set to `True`, means the request will fail when the source or destination host uses shared storage. Set to `False` means the request will fail when the source and destination hosts are not on the shared storage.

   **New in version 2.25**

  Possible values: `true`, `false`

* `--host <HOST>` — The host to which to migrate the server. If this parameter is `None`, the scheduler chooses a host.

   Warning

   Prior to microversion 2.30, specifying a host will bypass validation by the scheduler, which could result in failures to actually migrate the instance to the specified host, or over-subscription of the host. It is recommended to either not specify a host so that the scheduler will pick one, or specify a host with microversion >= 2.30 and without `force=True` set.



## `osc compute server lock273`

Lock Server (lock Action) (microversion = 2.73)

**Usage:** `osc compute server lock273 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--locked-reason <LOCKED_REASON>`



## `osc compute server metadata`

Lists metadata, creates or replaces one or more metadata items, and updates one or more metadata items for a server.

Shows details for, creates or replaces, and updates a metadata item, by key, for a server.

**Usage:** `osc compute server metadata <COMMAND>`

###### **Subcommands:**

* `create` — Create or Update Metadata Items
* `delete` — Delete Metadata Item
* `list` — List All Metadata
* `replace` — Replace Metadata Items
* `set` — Create Or Update Metadata Item
* `show` — Show Metadata Item Details



## `osc compute server metadata create`

Create or update one or more metadata items for a server.

Creates any metadata items that do not already exist in the server, replaces exists metadata items that match keys. Does not modify items that are not in the request.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server metadata create [OPTIONS] <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API

###### **Options:**

* `--metadata <key=value>` — Metadata key and value pairs. The maximum size for each metadata key and value pair is 255 bytes



## `osc compute server metadata delete`

Deletes a metadata item, by key, from a server.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server metadata delete <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/metadata/{id} API



## `osc compute server metadata list`

Lists all metadata for a server.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server metadata list <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API



## `osc compute server metadata replace`

Replaces one or more metadata items for a server.

Creates any metadata items that do not already exist in the server. Removes and completely replaces any metadata items that already exist in the server with the metadata items in the request.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server metadata replace [OPTIONS] <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API

###### **Options:**

* `--metadata <key=value>` — Metadata key and value pairs. The maximum size for each metadata key and value pair is 255 bytes



## `osc compute server metadata set`

Creates or replaces a metadata item, by key, for a server.

Creates a metadata item that does not already exist in the server. Replaces existing metadata items that match keys with the metadata item in the request.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server metadata set [OPTIONS] <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/metadata/{id} API

###### **Options:**

* `--meta <key=value>`



## `osc compute server metadata show`

Shows details for a metadata item, by key, for a server.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server metadata show <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/metadata/{id} API



## `osc compute server migrate256`

Migrates a server to a host.

Specify the `migrate` action in the request body.

Up to microversion 2.55, the scheduler chooses the host. Starting from microversion 2.56, the `host` parameter is available to specify the destination host. If you specify `null` or don’t specify this parameter, the scheduler chooses a host.

**Asynchronous Postconditions**

A successfully migrated server shows a `VERIFY_RESIZE` status and `finished` migration status. If the cloud has configured the [resize_confirm_window](https://docs.openstack.org/nova/latest/configuration/config.html#DEFAULT.resize_confirm_window) option of the Compute service to a positive value, the Compute service automatically confirms the migrate operation after the configured interval.

There are two different policies for this action, depending on whether the host parameter is set. Both defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403) itemNotFound(404), conflict(409)

**Usage:** `osc compute server migrate256 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--host <HOST>`



## `osc compute server migration`

Server migrations (servers, migrations)

List, show, perform actions on and delete server migrations.

**Usage:** `osc compute server migration <COMMAND>`

###### **Subcommands:**

* `delete` — Delete (Abort) Migration
* `force-complete222` — Force Migration Complete Action (force_complete Action) (microversion = 2.22)
* `list` — List Migrations
* `show` — Show Migration Details



## `osc compute server migration delete`

Abort an in-progress live migration.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

**Preconditions**

The server OS-EXT-STS:task_state value must be `migrating`.

If the server is locked, you must have administrator privileges to force the completion of the server migration.

For microversions from 2.24 to 2.64 the migration status must be `running`, for microversion 2.65 and greater, the migration status can also be `queued` and `preparing`.

**Asynchronous Postconditions**

After you make this request, you typically must keep polling the server status to determine whether the request succeeded. You may also monitor the migration using:

**Troubleshooting**

If the server status remains `MIGRATING` for an inordinate amount of time, the request may have failed. Ensure you meet the preconditions and run the request again. If the request fails again, investigate the compute back end.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server migration delete <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/migrations/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/migrations/{id} API



## `osc compute server migration force-complete222`

Force an in-progress live migration for a given server to complete.

Specify the `force_complete` action in the request body.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

**Preconditions**

The server OS-EXT-STS:vm_state value must be `active` and the server OS-EXT-STS:task_state value must be `migrating`.

If the server is locked, you must have administrator privileges to force the completion of the server migration.

The migration status must be `running`.

**Asynchronous Postconditions**

After you make this request, you typically must keep polling the server status to determine whether the request succeeded.

**Troubleshooting**

If the server status remains `MIGRATING` for an inordinate amount of time, the request may have failed. Ensure you meet the preconditions and run the request again. If the request fails again, investigate the compute back end. More details can be found in the [admin guide](https://docs.openstack.org/nova/latest/admin/live-migration-usage.html#what-to-do-when-the-migration-times-out).

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server migration force-complete222 <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/migrations/{id}/action API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/migrations/{id}/action API



## `osc compute server migration list`

Lists in-progress live migrations for a given server.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server migration list <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/migrations/{id} API



## `osc compute server migration show`

Show details for an in-progress live migration for a given server.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server migration show <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/migrations/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/migrations/{id} API



## `osc compute server password`

Servers password

Shows the encrypted administrative password. Also, clears the encrypted administrative password for a server, which removes it from the metadata server.

**Usage:** `osc compute server password <COMMAND>`

###### **Subcommands:**

* `delete` — Clear Admin Password
* `show` — Show Server Password



## `osc compute server password delete`

Clears the encrypted administrative password for a server, which removes it from the database.

This action does not actually change the instance server password.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server password delete <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-server-password API



## `osc compute server password show`

Shows the administrative password for a server.

This operation calls the metadata service to query metadata information and does not read password information from the server itself.

The password saved in the metadata service is typically encrypted using the public SSH key injected into this server, so the SSH private key is needed to read the password.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server password show <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-server-password API



## `osc compute server pause`

Pauses a server. Changes its status to `PAUSED`.

Specify the `pause` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server pause --pause <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--pause <JSON>`



## `osc compute server reset-state`

Resets the state of a server.

Specify the `os-resetState` action and the `state` in the request body.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server reset-state --state <STATE> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--state <STATE>` — The state of the server to be set, `active` or `error` are valid

  Possible values: `active`, `error`




## `osc compute server reboot`

Reboots a server.

Specify the `reboot` action in the request body.

**Preconditions**

The preconditions for rebooting a server depend on the type of reboot.

You can only *SOFT* reboot a server when its status is `ACTIVE`.

You can only *HARD* reboot a server when its status is one of:

If the server is locked, you must have administrator privileges to reboot the server.

**Asynchronous Postconditions**

After you successfully reboot a server, its status changes to `ACTIVE`.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server reboot --type <TYPE> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--type <TYPE>` — The type of the reboot action. The valid values are `HARD` and `SOFT`. A `SOFT` reboot attempts a graceful shutdown and restart of the server. A `HARD` reboot attempts a forced shutdown and restart of the server. The `HARD` reboot corresponds to the power cycles of the server

  Possible values: `hard`, `soft`




## `osc compute server rebuild21`

Rebuild Server (rebuild Action) (microversion = 2.1)

**Usage:** `osc compute server rebuild21 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`




## `osc compute server rebuild219`

Rebuild Server (rebuild Action) (microversion = 2.19)

**Usage:** `osc compute server rebuild219 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`




## `osc compute server rebuild254`

Rebuild Server (rebuild Action) (microversion = 2.54)

**Usage:** `osc compute server rebuild254 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--key-name <KEY_NAME>` — Key pair name for rebuild API. If `null` is specified, the existing keypair is unset.

   Note

   Users within the same project are able to rebuild other user’s instances in that project with a new keypair. Keys are owned by users (which is the only resource that’s true of). Servers are owned by projects. Because of this a rebuild with a key_name is looking up the keypair by the user calling rebuild.

   **New in version 2.54**
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--personality <JSON>` — The file path and contents, text only, to inject into the server at launch. The maximum size of the file path data is 255 bytes. The maximum limit is the number of allowed bytes in the decoded, rather than encoded, data.

   **Available until version 2.56**
* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`




## `osc compute server rebuild257`

Rebuild Server (rebuild Action) (microversion = 2.57)

**Usage:** `osc compute server rebuild257 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--key-name <KEY_NAME>` — Key pair name for rebuild API. If `null` is specified, the existing keypair is unset.

   Note

   Users within the same project are able to rebuild other user’s instances in that project with a new keypair. Keys are owned by users (which is the only resource that’s true of). Servers are owned by projects. Because of this a rebuild with a key_name is looking up the keypair by the user calling rebuild.

   **New in version 2.54**
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`

* `--user-data <USER_DATA>` — Configuration information or scripts to use upon rebuild. Must be Base64 encoded. Restricted to 65535 bytes. If `null` is specified, the existing user_data is unset.

   **New in version 2.57**



## `osc compute server rebuild263`

Rebuild Server (rebuild Action) (microversion = 2.63)

**Usage:** `osc compute server rebuild263 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--key-name <KEY_NAME>` — Key pair name for rebuild API. If `null` is specified, the existing keypair is unset.

   Note

   Users within the same project are able to rebuild other user’s instances in that project with a new keypair. Keys are owned by users (which is the only resource that’s true of). Servers are owned by projects. Because of this a rebuild with a key_name is looking up the keypair by the user calling rebuild.

   **New in version 2.54**
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`

* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server rebuild requests if allowed by policy, and is not supported for volume-backed instances.

   If `null` is specified, the existing trusted certificate IDs are either unset or reset to the configured defaults.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon rebuild. Must be Base64 encoded. Restricted to 65535 bytes. If `null` is specified, the existing user_data is unset.

   **New in version 2.57**



## `osc compute server rebuild290`

Rebuild Server (rebuild Action) (microversion = 2.90)

**Usage:** `osc compute server rebuild290 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--hostname <HOSTNAME>` — The hostname to configure for the instance in the metadata service.

   Starting with microversion 2.94, this can be a Fully Qualified Domain Name (FQDN) of up to 255 characters in length.

   Note

   This information is published via the metadata service and requires application such as `cloud-init` to propagate it through to the instance.

   **New in version 2.90**
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--key-name <KEY_NAME>` — Key pair name for rebuild API. If `null` is specified, the existing keypair is unset.

   Note

   Users within the same project are able to rebuild other user’s instances in that project with a new keypair. Keys are owned by users (which is the only resource that’s true of). Servers are owned by projects. Because of this a rebuild with a key_name is looking up the keypair by the user calling rebuild.

   **New in version 2.54**
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`

* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server rebuild requests if allowed by policy, and is not supported for volume-backed instances.

   If `null` is specified, the existing trusted certificate IDs are either unset or reset to the configured defaults.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon rebuild. Must be Base64 encoded. Restricted to 65535 bytes. If `null` is specified, the existing user_data is unset.

   **New in version 2.57**



## `osc compute server rebuild294`

Rebuild Server (rebuild Action) (microversion = 2.94)

**Usage:** `osc compute server rebuild294 [OPTIONS] --image-ref <IMAGE_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--admin-pass <ADMIN_PASS>` — The administrative password of the server. If you omit this parameter, the operation generates a new password
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--hostname <HOSTNAME>` — The hostname to configure for the instance in the metadata service.

   Starting with microversion 2.94, this can be a Fully Qualified Domain Name (FQDN) of up to 255 characters in length.

   Note

   This information is published via the metadata service and requires application such as `cloud-init` to propagate it through to the instance.

   **New in version 2.90**
* `--image-ref <IMAGE_REF>` — The UUID of the image to rebuild for your server instance. It must be a valid UUID otherwise API will return 400. To rebuild a volume-backed server with a new image, at least microversion 2.93 needs to be provided in the request else the request will fall back to old behaviour i.e. the API will return 400 (for an image different from the image used when creating the volume). For non-volume-backed servers, specifying a new image will result in validating that the image is acceptable for the current compute host on which the server exists. If the new image is not valid, the server will go into `ERROR` status
* `--key-name <KEY_NAME>` — Key pair name for rebuild API. If `null` is specified, the existing keypair is unset.

   Note

   Users within the same project are able to rebuild other user’s instances in that project with a new keypair. Keys are owned by users (which is the only resource that’s true of). Servers are owned by projects. Because of this a rebuild with a key_name is looking up the keypair by the user calling rebuild.

   **New in version 2.54**
* `--metadata <key=value>` — Metadata key and value pairs. The maximum size of the metadata key and value is 255 bytes each
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`

* `--preserve-ephemeral <PRESERVE_EPHEMERAL>` — Indicates whether the server is rebuilt with the preservation of the ephemeral partition (`true`).

   Note

   This only works with baremetal servers provided by Ironic. Passing it to any other server instance results in a fault and will prevent the rebuild from happening.

  Possible values: `true`, `false`

* `--trusted-image-certificates <TRUSTED_IMAGE_CERTIFICATES>` — A list of trusted certificate IDs, which are used during image signature verification to verify the signing certificate. The list is restricted to a maximum of 50 IDs. This parameter is optional in server rebuild requests if allowed by policy, and is not supported for volume-backed instances.

   If `null` is specified, the existing trusted certificate IDs are either unset or reset to the configured defaults.

   **New in version 2.63**
* `--user-data <USER_DATA>` — Configuration information or scripts to use upon rebuild. Must be Base64 encoded. Restricted to 65535 bytes. If `null` is specified, the existing user_data is unset.

   **New in version 2.57**



## `osc compute server remote-console`

Server Consoles

Manage server consoles.

**Usage:** `osc compute server remote-console <COMMAND>`

###### **Subcommands:**

* `create26` — Create Console (microversion = 2.6)
* `create28` — Create Console (microversion = 2.8)



## `osc compute server remote-console create26`

The API provides a unified request for creating a remote console. The user can get a URL to connect the console from this API. The URL includes the token which is used to get permission to access the console. Servers may support different console protocols. To return a remote console using a specific protocol, such as VNC, set the `protocol` parameter to `vnc`.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server remote-console create26 --protocol <PROTOCOL> --type <TYPE> <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/remote-consoles API

###### **Options:**

* `--protocol <PROTOCOL>` — The protocol of remote console. The valid values are `vnc`, `spice`, `serial` and `mks`. The protocol `mks` is added since Microversion `2.8`

  Possible values: `serial`, `spice`, `vnc`

* `--type <TYPE>` — The type of remote console. The valid values are `novnc`, `spice-html5`, `serial`, and `webmks`. The type `webmks` is added since Microversion `2.8`

  Possible values: `novnc`, `serial`, `spice-html5`, `xvpvnc`




## `osc compute server remote-console create28`

The API provides a unified request for creating a remote console. The user can get a URL to connect the console from this API. The URL includes the token which is used to get permission to access the console. Servers may support different console protocols. To return a remote console using a specific protocol, such as VNC, set the `protocol` parameter to `vnc`.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server remote-console create28 --protocol <PROTOCOL> --type <TYPE> <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/remote-consoles API

###### **Options:**

* `--protocol <PROTOCOL>` — The protocol of remote console. The valid values are `vnc`, `spice`, `serial` and `mks`. The protocol `mks` is added since Microversion `2.8`

  Possible values: `mks`, `serial`, `spice`, `vnc`

* `--type <TYPE>` — The type of remote console. The valid values are `novnc`, `spice-html5`, `serial`, and `webmks`. The type `webmks` is added since Microversion `2.8`

  Possible values: `novnc`, `serial`, `spice-html5`, `webmks`, `xvpvnc`




## `osc compute server remove-fixed-ip21`

Removes, or disassociates, a fixed IP address from a server.

Specify the `removeFixedIp` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server remove-fixed-ip21 --address <ADDRESS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--address <ADDRESS>` — The IP address



## `osc compute server remove-floating-ip21`

Removes, or disassociates, a floating IP address from a server.

The IP address is returned to the pool of IP addresses that is available for all projects. When you remove a floating IP address and that IP address is still associated with a running instance, it is automatically disassociated from that instance.

Specify the `removeFloatingIp` action in the request body.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server remove-floating-ip21 --address <ADDRESS> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--address <ADDRESS>` — The floating IP address



## `osc compute server remove-security-group`

Removes a security group from a server.

Specify the `removeSecurityGroup` action in the request body.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server remove-security-group [OPTIONS] --name <NAME> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--name <NAME>` — The security group name
* `--property <key=value>` — Additional properties to be sent with the request



## `osc compute server rescue`

Puts a server in rescue mode and changes its status to `RESCUE`.

Specify the `rescue` action in the request body.

If you specify the `rescue_image_ref` extended attribute, the image is used to rescue the instance. If you omit an image reference, the base image reference is used by default.

**Asynchronous Postconditions**

After you successfully rescue a server and make a `GET /servers/​{server_id}​` request, its status changes to `RESCUE`.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server rescue [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--admin-pass <ADMIN_PASS>`
* `--rescue-image-ref <RESCUE_IMAGE_REF>`



## `osc compute server reset-network`

Resets networking on a server.

Specify the `resetNetwork` action in the request body.

Policy defaults enable only users with the administrative role to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), gone(410)

**Usage:** `osc compute server reset-network <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API



## `osc compute server resize`

Resizes a server.

Specify the `resize` action in the request body.

**Preconditions**

You can only resize a server when its status is `ACTIVE` or `SHUTOFF`.

If the server is locked, you must have administrator privileges to resize the server.

**Asynchronous Postconditions**

A successfully resized server shows a `VERIFY_RESIZE` status and `finished` migration status. If the cloud has configured the [resize_confirm_window](https://docs.openstack.org/nova/latest/configuration/config.html#DEFAULT.resize_confirm_window) option of the Compute service to a positive value, the Compute service automatically confirms the resize operation after the configured interval.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server resize [OPTIONS] --flavor-ref <FLAVOR_REF> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--flavor-ref <FLAVOR_REF>` — The flavor ID for resizing the server. The size of the disk in the flavor being resized to must be greater than or equal to the size of the disk in the current flavor.

   If a specified flavor ID is the same as the current one of the server, the request returns a `Bad Request (400)` response code.
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`




## `osc compute server restore`

Restores a previously soft-deleted server instance. You cannot use this method to restore deleted instances.

Specify the `restore` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server restore --restore <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--restore <JSON>`



## `osc compute server resume`

Resumes a suspended server and changes its status to `ACTIVE`.

Specify the `resume` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server resume --resume <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--resume <JSON>`



## `osc compute server revert-resize`

Cancels and reverts a pending resize action for a server.

Specify the `revertResize` action in the request body.

**Preconditions**

You can only revert the resized server where the status is `VERIFY_RESIZE` and the OS-EXT-STS:vm_state is `resized`.

If the server is locked, you must have administrator privileges to revert the resizing.

**Asynchronous Postconditions**

After you make this request, you typically must keep polling the server status to determine whether the request succeeded. A reverting resize operation shows a status of `REVERT_RESIZE` and a task_state of `resize_reverting`. If successful, the status will return to `ACTIVE` or `SHUTOFF`. You can also see the reverted server in the compute node that OpenStack Compute manages.

**Troubleshooting**

If the server status remains `VERIFY_RESIZE`, the request failed. Ensure you meet the preconditions and run the request again. If the request fails again, investigate the compute back end.

The server is not reverted in the compute node that OpenStack Compute manages.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server revert-resize --revert-resize <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--revert-resize <JSON>`



## `osc compute server security-groups`

Lists security groups for a server.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server security-groups <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-security-groups API



## `osc compute server set21`

Updates the editable attributes of an existing server.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server set21 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id} API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`




## `osc compute server set219`

Updates the editable attributes of an existing server.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server set219 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id} API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`




## `osc compute server set290`

Updates the editable attributes of an existing server.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server set290 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id} API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--hostname <HOSTNAME>` — The hostname to configure for the instance in the metadata service.

   Starting with microversion 2.94, this can be a Fully Qualified Domain Name (FQDN) of up to 255 characters in length.

   Note

   This information is published via the metadata service and requires application such as `cloud-init` to propagate it through to the instance.

   **New in version 2.90**
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`




## `osc compute server set294`

Updates the editable attributes of an existing server.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server set294 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id} API

###### **Options:**

* `--access-ipv4 <ACCESS_IPV4>` — IPv4 address that should be used to access this server
* `--access-ipv6 <ACCESS_IPV6>` — IPv6 address that should be used to access this server
* `--description <DESCRIPTION>` — A free form description of the server. Limited to 255 characters in length. Before microversion 2.19 this was set to the server name.

   **New in version 2.19**
* `--hostname <HOSTNAME>` — The hostname to configure for the instance in the metadata service.

   Starting with microversion 2.94, this can be a Fully Qualified Domain Name (FQDN) of up to 255 characters in length.

   Note

   This information is published via the metadata service and requires application such as `cloud-init` to propagate it through to the instance.

   **New in version 2.90**
* `--name <NAME>` — The server name
* `--os-dcf-disk-config <OS_DCF_DISK_CONFIG>` — Controls how the API partitions the disk when you create, rebuild, or resize servers. A server inherits the `OS-DCF:diskConfig` value from the image from which it was created, and an image inherits the `OS-DCF:diskConfig` value from the server from which it was created. To override the inherited setting, you can include this attribute in the request body of a server create, rebuild, or resize request. If the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a server from that image and set its `OS-DCF:diskConfig` value to `AUTO`. A valid value is:

   - `AUTO`. The API builds the server with a single partition the size of the target flavor disk. The API automatically adjusts the file system to fit the entire partition. - `MANUAL`. The API builds the server by using whatever partition scheme and file system is in the source image. If the target flavor disk is larger, the API does not partition the remaining disk space.

  Possible values: `auto`, `manual`




## `osc compute server shelve`

Shelves a server.

Specify the `shelve` action in the request body.

All associated data and resources are kept but anything still in memory is not retained. To restore a shelved instance, use the `unshelve` action. To remove a shelved instance, use the `shelveOffload` action.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

**Preconditions**

The server status must be `ACTIVE`, `SHUTOFF`, `PAUSED`, or `SUSPENDED`.

If the server is locked, you must have administrator privileges to shelve the server.

**Asynchronous Postconditions**

After you successfully shelve a server, its status changes to `SHELVED` and the image status is `ACTIVE`. The server instance data appears on the compute node that the Compute service manages.

If you boot the server from volumes or set the `shelved_offload_time` option to 0, the Compute service automatically deletes the instance on compute nodes and changes the server status to `SHELVED_OFFLOADED`.

**Troubleshooting**

If the server status does not change to `SHELVED` or `SHELVED_OFFLOADED`, the shelve operation failed. Ensure that you meet the preconditions and run the request again. If the request fails again, investigate whether another operation is running that causes a race condition.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server shelve --shelve <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--shelve <JSON>`



## `osc compute server shelve-offload`

Shelf-offloads, or removes, a shelved server.

Specify the `shelveOffload` action in the request body.

Data and resource associations are deleted. If an instance is no longer needed, you can remove that instance from the hypervisor to minimize resource usage.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

**Preconditions**

The server status must be `SHELVED`.

If the server is locked, you must have administrator privileges to shelve-offload the server.

**Asynchronous Postconditions**

After you successfully shelve-offload a server, its status changes to `SHELVED_OFFLOADED`. The server instance data appears on the compute node.

**Troubleshooting**

If the server status does not change to `SHELVED_OFFLOADED`, the shelve-offload operation failed. Ensure that you meet the preconditions and run the request again. If the request fails again, investigate whether another operation is running that causes a race condition.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server shelve-offload --shelve-offload <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--shelve-offload <JSON>`



## `osc compute server show`

Shows details for a server.

Includes server details including configuration drive, extended status, and server usage information.

The extended status information appears in the `OS-EXT-STS:vm_state`, `OS-EXT-STS:power_state`, and `OS-EXT-STS:task_state` attributes.

The server usage information appears in the `OS-SRV-USG:launched_at` and `OS-SRV-USG:terminated_at` attributes.

HostId is unique per account and is not globally unique.

**Preconditions**

The server must exist.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id} API



## `osc compute server start`

Starts a stopped server and changes its status to `ACTIVE`.

Specify the `os-start` action in the request body.

**Preconditions**

The server status must be `SHUTOFF`.

If the server is locked, you must have administrator privileges to start the server.

**Asynchronous Postconditions**

After you successfully start a server, its status changes to `ACTIVE`.

**Troubleshooting**

If the server status does not change to `ACTIVE`, the start operation failed. Ensure that you meet the preconditions and run the request again. If the request fails again, investigate whether another operation is running that causes a race condition.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server start --os-start <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--os-start <JSON>`



## `osc compute server stop`

Stops a running server and changes its status to `SHUTOFF`.

Specify the `os-stop` action in the request body.

**Preconditions**

The server status must be `ACTIVE` or `ERROR`.

If the server is locked, you must have administrator privileges to stop the server.

**Asynchronous Postconditions**

After you successfully stop a server, its status changes to `SHUTOFF`. This API operation does not delete the server instance data and the data will be available again after `os-start` action.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server stop --os-stop <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--os-stop <JSON>`



## `osc compute server suspend`

Suspends a server and changes its status to `SUSPENDED`.

Specify the `suspend` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server suspend --suspend <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--suspend <JSON>`



## `osc compute server tag`

Lists tags, creates, replaces or deletes one or more tags for a server, checks the existence of a tag for a server.

Available since version 2.26

Tags have the following restrictions:

- Tag is a Unicode bytestring no longer than 60 characters.

- Tag is a non-empty string.

- ‘/’ is not allowed to be in a tag name

- Comma is not allowed to be in a tag name in order to simplify requests that specify lists of tags

- All other characters are allowed to be in a tag name

- Each server can have up to 50 tags.

**Usage:** `osc compute server tag <COMMAND>`

###### **Subcommands:**

* `add` — Add a Single Tag
* `check` — Check Tag Existence
* `delete` — Delete a Single Tag
* `list` — List Tags
* `purge` — Delete All Tags
* `replace226` — Replace Tags (microversion = 2.26)



## `osc compute server tag add`

Adds a single tag to the server if server has no specified tag. Response code in this case is 201.

If the server has specified tag just returns 204.

Normal response codes: 201, 204

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server tag add <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/tags/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/tags/{id} API



## `osc compute server tag check`

Checks tag existence on the server. If tag exists response with 204 status code will be returned. Otherwise returns 404.

Normal response codes: 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server tag check <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/tags/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/tags/{id} API



## `osc compute server tag delete`

Deletes a single tag from the specified server.

Normal response codes: 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server tag delete <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/tags/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/tags/{id} API



## `osc compute server tag list`

Lists all tags for a server.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server tag list <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/tags/{id} API



## `osc compute server tag purge`

Deletes all tags from the specified server.

Normal response codes: 204

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server tag purge <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/tags/{id} API



## `osc compute server tag replace226`

Replaces all tags on specified server with the new set of tags.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server tag replace226 [OPTIONS] <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/tags/{id} API

###### **Options:**

* `--tags <TAGS>` — A list of tags. The maximum count of tags in this list is 50



## `osc compute server topology`

Shows NUMA topology information for a server.

Policy defaults enable only users with the administrative role or the owners of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 200

Error response codes: unauthorized(401), notfound(404), forbidden(403)

**Usage:** `osc compute server topology <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/topology API



## `osc compute server trigger-crash-dump217`

Command without description in OpenAPI

**Usage:** `osc compute server trigger-crash-dump217 <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API



## `osc compute server unlock21`

Unlocks a locked server.

Specify the `unlock` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server unlock21 --unlock <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--unlock <JSON>`



## `osc compute server unpause`

Unpauses a paused server and changes its status to `ACTIVE`.

Specify the `unpause` action in the request body.

Policy defaults enable only users with the administrative role or the owner of the server to perform this operation. Cloud providers can change these permissions through the `policy.json` file.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server unpause --unpause <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--unpause <JSON>`



## `osc compute server unrescue`

Unrescues a server. Changes status to `ACTIVE`.

Specify the `unrescue` action in the request body.

**Preconditions**

The server must exist.

You can only unrescue a server when its status is `RESCUE`.

**Asynchronous Postconditions**

After you successfully unrescue a server and make a `GET /servers/​{server_id}​` request, its status changes to `ACTIVE`.

Normal response codes: 202

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404), conflict(409), notImplemented(501)

**Usage:** `osc compute server unrescue --unrescue <JSON> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--unrescue <JSON>`



## `osc compute server unshelve277`

Unshelve (Restore) Shelved Server (unshelve Action) (microversion = 2.77)

**Usage:** `osc compute server unshelve277 --availability-zone <AVAILABILITY_ZONE> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>`



## `osc compute server unshelve291`

Unshelve (Restore) Shelved Server (unshelve Action) (microversion = 2.91)

**Usage:** `osc compute server unshelve291 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/servers/{id}/action API

###### **Options:**

* `--availability-zone <AVAILABILITY_ZONE>`
* `--host <HOST>`



## `osc compute server volume-attachment`

Servers with volume attachments

Attaches volumes that are created through the volume API to server instances. Also, lists volume attachments for a server, shows details for a volume attachment, and detaches a volume.

**Usage:** `osc compute server volume-attachment <COMMAND>`

###### **Subcommands:**

* `create20` — Attach a volume to an instance (microversion = 2.0)
* `create249` — Attach a volume to an instance (microversion = 2.49)
* `create279` — Attach a volume to an instance (microversion = 2.79)
* `delete` — Detach a volume from an instance
* `list` — List volume attachments for an instance
* `set20` — Update a volume attachment (microversion = 2.0)
* `set285` — Update a volume attachment (microversion = 2.85)
* `show` — Show a detail of a volume attachment



## `osc compute server volume-attachment create20`

Attach a volume to an instance.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server volume-attachment create20 [OPTIONS] --volume-id <VOLUME_ID> <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API

###### **Options:**

* `--device <DEVICE>` — Name of the device such as, `/dev/vdb`. Omit or set this parameter to null for auto-assignment, if supported. If you specify this parameter, the device must not exist in the guest operating system. Note that as of the 12.0.0 Liberty release, the Nova libvirt driver no longer honors a user-supplied device name. This is the same behavior as if the device name parameter is not supplied on the request
* `--volume-id <VOLUME_ID>` — The UUID of the volume to attach



## `osc compute server volume-attachment create249`

Attach a volume to an instance.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server volume-attachment create249 [OPTIONS] --volume-id <VOLUME_ID> <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API

###### **Options:**

* `--device <DEVICE>` — Name of the device such as, `/dev/vdb`. Omit or set this parameter to null for auto-assignment, if supported. If you specify this parameter, the device must not exist in the guest operating system. Note that as of the 12.0.0 Liberty release, the Nova libvirt driver no longer honors a user-supplied device name. This is the same behavior as if the device name parameter is not supplied on the request
* `--tag <TAG>` — A device role tag that can be applied to a volume when attaching it to the VM. The guest OS of a server that has devices tagged in this manner can access hardware metadata about the tagged devices from the metadata API and on the config drive, if enabled.

   Note

   Tagged volume attachment is not supported for shelved-offloaded instances.

   **New in version 2.49**
* `--volume-id <VOLUME_ID>` — The UUID of the volume to attach



## `osc compute server volume-attachment create279`

Attach a volume to an instance.

Normal response codes: 200

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server volume-attachment create279 [OPTIONS] --volume-id <VOLUME_ID> <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API

###### **Options:**

* `--delete-on-termination <DELETE_ON_TERMINATION>` — To delete the attached volume when the server is destroyed, specify `true`. Otherwise, specify `false`. Default: `false`

   **New in version 2.79**

  Possible values: `true`, `false`

* `--device <DEVICE>` — Name of the device such as, `/dev/vdb`. Omit or set this parameter to null for auto-assignment, if supported. If you specify this parameter, the device must not exist in the guest operating system. Note that as of the 12.0.0 Liberty release, the Nova libvirt driver no longer honors a user-supplied device name. This is the same behavior as if the device name parameter is not supplied on the request
* `--tag <TAG>` — A device role tag that can be applied to a volume when attaching it to the VM. The guest OS of a server that has devices tagged in this manner can access hardware metadata about the tagged devices from the metadata API and on the config drive, if enabled.

   Note

   Tagged volume attachment is not supported for shelved-offloaded instances.

   **New in version 2.49**
* `--volume-id <VOLUME_ID>` — The UUID of the volume to attach



## `osc compute server volume-attachment delete`

Detach a volume from an instance.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server volume-attachment delete <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API



## `osc compute server volume-attachment list`

List volume attachments for an instance.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server volume-attachment list [OPTIONS] <SERVER_ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API

###### **Options:**

* `--limit <LIMIT>`
* `--offset <OFFSET>`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute server volume-attachment set20`

Update a volume attachment.

Policy default role is ‘rule:system_admin_or_owner’, its scope is \[system, project\], which allow project members or system admins to change the fields of an attached volume of a server. Policy defaults enable only users with the administrative role to change `volumeId` via this operation. Cloud providers can change these permissions through the `policy.json` file.

Updating, or what is commonly referred to as “swapping”, volume attachments with volumes that have more than one read/write attachment, is not supported.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server volume-attachment set20 --volume-id <VOLUME_ID> <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API

###### **Options:**

* `--volume-id <VOLUME_ID>` — The UUID of the volume to attach instead of the attached volume



## `osc compute server volume-attachment set285`

Update a volume attachment.

Policy default role is ‘rule:system_admin_or_owner’, its scope is \[system, project\], which allow project members or system admins to change the fields of an attached volume of a server. Policy defaults enable only users with the administrative role to change `volumeId` via this operation. Cloud providers can change these permissions through the `policy.json` file.

Updating, or what is commonly referred to as “swapping”, volume attachments with volumes that have more than one read/write attachment, is not supported.

Normal response codes: 202

Error response codes: badRequest(400), unauthorized(401), forbidden(403), itemNotFound(404), conflict(409)

**Usage:** `osc compute server volume-attachment set285 [OPTIONS] --volume-id <VOLUME_ID> <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API

###### **Options:**

* `--delete-on-termination <DELETE_ON_TERMINATION>` — A flag indicating if the attached volume will be deleted when the server is deleted.

   **New in version 2.85**

  Possible values: `true`, `false`

* `--device <DEVICE>` — Name of the device in the attachment object, such as, `/dev/vdb`.

   **New in version 2.85**
* `--id <ID>` — The UUID of the attachment.

   **New in version 2.85**
* `--server-id <SERVER_ID>` — The UUID of the server.

   **New in version 2.85**
* `--tag <TAG>` — The device tag applied to the volume block device or `null`.

   **New in version 2.85**
* `--volume-id <VOLUME_ID>` — The UUID of the volume to attach instead of the attached volume



## `osc compute server volume-attachment show`

Show a detail of a volume attachment.

Normal response codes: 200

Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)

**Usage:** `osc compute server volume-attachment show <SERVER_ID> <ID>`

###### **Arguments:**

* `<SERVER_ID>` — server_id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API
* `<ID>` — id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id} API



## `osc identity`

Identity (Keystone) commands

**Usage:** `osc identity <COMMAND>`

###### **Subcommands:**

* `auth` — Identity Auth commands
* `access-rule` — **Application Credentials - Access Rules**
* `application-credential` — **Application Credentials**
* `credential` — Identity Credential commands
* `domain` — Identity Domain commands
* `endpoint` — Endpoint commands
* `federation` — OS-Federation
* `group` — Identity Group commands
* `project` — Identity Project commands
* `region` — Region commands
* `role` — Identity Role commands
* `role-assignment` — Role Assignments commands
* `role-inference` — Role Inferences commands
* `service` — Service commands
* `user` — User commands



## `osc identity auth`

Identity Auth commands

The Identity service generates tokens in exchange for authentication credentials. A token represents the authenticated identity of a user and, optionally, grants authorization on a specific project, domain, or the deployment system.

The body of an authentication request must include a payload that specifies the authentication methods, which are normally just password or token, the credentials, and, optionally, the authorization scope. You can scope a token to a project, domain, the deployment system, or the token can be unscoped. You cannot scope a token to multiple scope targets.

Tokens have IDs, which the Identity API returns in the X-Subject-Token response header.

In the case of multi-factor authentication (MFA) more than one authentication method needs to be supplied to authenticate. As of v3.12 a failure due to MFA rules only partially being met will result in an auth receipt ID being returned in the response header Openstack-Auth-Receipt, and a response body that details the receipt itself and the missing authentication methods. Supplying the auth receipt ID in the Openstack-Auth-Receipt header in a follow-up authentication request, with the missing authentication methods, will result in a valid token by reusing the successful methods from the first request. This allows MFA authentication to be a multi-step process.

After you obtain an authentication token, you can:

- Make REST API requests to other OpenStack services. You supply the ID of your authentication token in the X-Auth-Token request header.

- Validate your authentication token and list the domains, projects, roles, and endpoints that your token gives you access to.

- Use your token to request another token scoped for a different domain and project.

- Force the immediate revocation of a token.

- List revoked public key infrastructure (PKI) tokens.

**Usage:** `osc identity auth <COMMAND>`

###### **Subcommands:**

* `catalog` — Identity Catalog commands
* `domain` — Identity Domain scope commands
* `federation` — Identity Federated auch commands
* `project` — Identity project scope commands
* `system` — Identity system scope commands
* `token` — Identity token commands



## `osc identity auth catalog`

Identity Catalog commands

A service is an OpenStack web service that you can access through a URL, i.e. an endpoint. A service catalog lists the services that are available to the caller based upon the current authorization.

**Usage:** `osc identity auth catalog <COMMAND>`

###### **Subcommands:**

* `list` — Get service catalog



## `osc identity auth catalog list`

New in version 3.3

This call returns a service catalog for the X-Auth-Token provided in the request, even if the token does not contain a catalog itself (for example, if it was generated using ?nocatalog).

The structure of the catalog object is identical to that contained in a token.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/auth_catalog`

**Usage:** `osc identity auth catalog list`



## `osc identity auth domain`

Identity Domain scope commands

authorization.

**Usage:** `osc identity auth domain <COMMAND>`

###### **Subcommands:**

* `list` — Get available domain scopes



## `osc identity auth domain list`

New in version 3.3

This call returns the list of domains that are available to be scoped to based on the X-Auth-Token provided in the request.

The structure is the same as listing domains.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/auth_domains`

**Usage:** `osc identity auth domain list`



## `osc identity auth federation`

Identity Federated auch commands

authorization.

**Usage:** `osc identity auth federation <COMMAND>`

###### **Subcommands:**

* `identity-provider` — Identity WebSSO auth commands
* `websso` — Identity WebSSO auth commands



## `osc identity auth federation identity-provider`

Identity WebSSO auth commands

**Usage:** `osc identity auth federation identity-provider <COMMAND>`

###### **Subcommands:**

* `protocol` — Identity WebSSO auth commands



## `osc identity auth federation identity-provider protocol`

Identity WebSSO auth commands

**Usage:** `osc identity auth federation identity-provider protocol <COMMAND>`

###### **Subcommands:**

* `websso` — Identity WebSSO auth commands



## `osc identity auth federation identity-provider protocol websso`

Identity WebSSO auth commands

**Usage:** `osc identity auth federation identity-provider protocol websso <COMMAND>`

###### **Subcommands:**

* `create` — POST operation on /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso
* `get` — GET operation on /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso



## `osc identity auth federation identity-provider protocol websso create`

POST operation on /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso

**Usage:** `osc identity auth federation identity-provider protocol websso create <IDP_ID> <PROTOCOL_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso API
* `<PROTOCOL_ID>` — protocol_id parameter for /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso API



## `osc identity auth federation identity-provider protocol websso get`

GET operation on /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso

**Usage:** `osc identity auth federation identity-provider protocol websso get <IDP_ID> <PROTOCOL_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso API
* `<PROTOCOL_ID>` — protocol_id parameter for /v3/auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso API



## `osc identity auth federation websso`

Identity WebSSO auth commands

**Usage:** `osc identity auth federation websso <COMMAND>`

###### **Subcommands:**

* `create` — POST operation on /v3/auth/OS-FEDERATION/websso/{protocol_id}
* `show` — GET operation on /v3/auth/OS-FEDERATION/websso/{protocol_id}



## `osc identity auth federation websso create`

POST operation on /v3/auth/OS-FEDERATION/websso/{protocol_id}

**Usage:** `osc identity auth federation websso create <PROTOCOL_ID>`

###### **Arguments:**

* `<PROTOCOL_ID>` — protocol_id parameter for /v3/auth/OS-FEDERATION/websso/{protocol_id} API



## `osc identity auth federation websso show`

GET operation on /v3/auth/OS-FEDERATION/websso/{protocol_id}

**Usage:** `osc identity auth federation websso show <PROTOCOL_ID>`

###### **Arguments:**

* `<PROTOCOL_ID>` — protocol_id parameter for /v3/auth/OS-FEDERATION/websso/{protocol_id} API



## `osc identity auth project`

Identity project scope commands

authorization.

**Usage:** `osc identity auth project <COMMAND>`

###### **Subcommands:**

* `list` — Get available project scopes



## `osc identity auth project list`

New in version 3.3

This call returns the list of projects that are available to be scoped to based on the X-Auth-Token provided in the request.

The structure of the response is exactly the same as listing projects for a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/auth_projects`

**Usage:** `osc identity auth project list`



## `osc identity auth system`

Identity system scope commands

authorization.

**Usage:** `osc identity auth system <COMMAND>`

###### **Subcommands:**

* `list` — Get available system scopes



## `osc identity auth system list`

New in version 3.10

This call returns the list of systems that are available to be scoped to based on the X-Auth-Token provided in the request.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/auth_system`

**Usage:** `osc identity auth system list`



## `osc identity auth token`

Identity token commands

authorization.

**Usage:** `osc identity auth token <COMMAND>`

###### **Subcommands:**

* `delete` — Revoke token
* `get` — Validate and show information for token



## `osc identity auth token delete`

Revokes a token.

This call is similar to the HEAD `/auth/tokens` call except that the `X-Subject-Token` token is immediately not valid, regardless of the `expires_at` attribute value. An additional `X-Auth-Token` is not required.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/auth_tokens`

**Usage:** `osc identity auth token delete`



## `osc identity auth token get`

Validates and shows information for a token, including its expiration date and authorization scope.

Pass your own token in the `X-Auth-Token` request header.

Pass the token that you want to validate in the `X-Subject-Token` request header.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/auth_tokens`

**Usage:** `osc identity auth token get`



## `osc identity access-rule`

**Application Credentials - Access Rules**

Users also have the option of delegating more fine-grained access control to their application credentials by using access rules. For example, to create an application credential that is constricted to creating servers in nova, the user can add the following access rules:

```json { "access_rules": [{ "path": "/v2.1/servers", "method": "POST", "service": "compute" }] } ```

The "path" attribute of application credential access rules uses a wildcard syntax to make it more flexible. For example, to create an application credential that is constricted to listing server IP addresses, you could use either of the following access rules:

```json { "access_rules": [ { "path": "/v2.1/servers/*/ips", "method": "GET", "service": "compute" } ] } ```

or equivalently:

```json { "access_rules": [ { "path": "/v2.1/servers/{server_id}/ips", "method": "GET", "service": "compute" } ] } ```

In both cases, a request path containing any server ID will match the access rule. For even more flexibility, the recursive wildcard ** indicates that request paths containing any number of / will be matched. For example:

```json { "access_rules": [ { "path": "/v2.1/**", "method": "GET", "service": "compute" } ] } ```

will match any nova API for version 2.1.

An access rule created for one application credential can be re-used by providing its ID to another application credential, for example:

```json { "access_rules": [ { "id": "abcdef" } ] } ```

**Usage:** `osc identity access-rule <COMMAND>`

###### **Subcommands:**

* `delete` — Delete access rule
* `list` — List access rules
* `show` — Show access rule details



## `osc identity access-rule delete`

Delete an access rule. An access rule that is still in use by an application credential cannot be deleted.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/access_rules`

**Usage:** `osc identity access-rule delete <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — access_rule_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity access-rule list`

List all access rules for a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/access_rules`

**Usage:** `osc identity access-rule list <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity access-rule show`

Show details of an access rule.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/access_rules`

**Usage:** `osc identity access-rule show <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — access_rule_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity application-credential`

**Application Credentials**

Application credentials provide a way to delegate a user’s authorization to an application without sharing the user’s password authentication. This is a useful security measure, especially for situations where the user’s identification is provided by an external source, such as LDAP or a single-sign-on service. Instead of storing user passwords in config files, a user creates an application credential for a specific project, with all or a subset of the role assignments they have on that project, and then stores the application credential identifier and secret in the config file.

Multiple application credentials may be active at once, so you can easily rotate application credentials by creating a second one, converting your applications to use it one by one, and finally deleting the first one.

Application credentials are limited by the lifespan of the user that created them. If the user is deleted, disabled, or loses a role assignment on a project, the application credential is deleted.

Application credentials can have their privileges limited in two ways. First, the owner may specify a subset of their own roles that the application credential may assume when getting a token for a project. For example, if a user has the member role on a project, they also have the implied role reader and can grant the application credential only the reader role for the project:

"roles": [ {"name": "reader"} ]

Users also have the option of delegating more fine-grained access control to their application credentials by using access rules. For example, to create an application credential that is constricted to creating servers in nova, the user can add the following access rules:

"access_rules": [ { "path": "/v2.1/servers", "method": "POST", "service": "compute" } ]

The "path" attribute of application credential access rules uses a wildcard syntax to make it more flexible. For example, to create an application credential that is constricted to listing server IP addresses, you could use either of the following access rules:

"access_rules": [ { "path": "/v2.1/servers/*/ips", "method": "GET", "service": "compute" } ]

or equivalently:

"access_rules": [ { "path": "/v2.1/servers/{server_id}/ips", "method": "GET", "service": "compute" } ]

In both cases, a request path containing any server ID will match the access rule. For even more flexibility, the recursive wildcard ** indicates that request paths containing any number of / will be matched. For example:

"access_rules": [ { "path": "/v2.1/**", "method": "GET", "service": "compute" } ]

will match any nova API for version 2.1.

An access rule created for one application credential can be re-used by providing its ID to another application credential, for example:

"access_rules": [ { "id": "abcdef" } ]

**Usage:** `osc identity application-credential <COMMAND>`

###### **Subcommands:**

* `create` — Create application credential
* `delete` — Delete application credential
* `list` — List application credentials
* `show` — Show application credential details



## `osc identity application-credential create`

Creates an application credential for a user on the project to which the current token is scoped.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`

**Usage:** `osc identity application-credential create [OPTIONS] --name <NAME> <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--access-rules <JSON>` — A list of `access_rules` objects
* `--description <DESCRIPTION>` — A description of the application credential’s purpose
* `--expires-at <EXPIRES_AT>` — An optional expiry time for the application credential. If unset, the application credential does not expire
* `--name <NAME>` — The name of the application credential. Must be unique to a user
* `--roles <JSON>` — An optional list of role objects, identified by ID or name. The list may only contain roles that the user has assigned on the project. If not provided, the roles assigned to the application credential will be the same as the roles in the current token
* `--secret <SECRET>` — The secret that the application credential will be created with. If not provided, one will be generated
* `--unrestricted <UNRESTRICTED>` — An optional flag to restrict whether the application credential may be used for the creation or destruction of other application credentials or trusts. Defaults to false

  Possible values: `true`, `false`




## `osc identity application-credential delete`

Delete an application credential.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`

**Usage:** `osc identity application-credential delete <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — application_credential_id parameter for /v3/users/{user_id}/application_credentials/{application_credential_id} API



## `osc identity application-credential list`

List all application credentials for a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`

**Usage:** `osc identity application-credential list [OPTIONS] <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--name <NAME>` — The name of the application credential. Must be unique to a user



## `osc identity application-credential show`

Show details of an application credential.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`

**Usage:** `osc identity application-credential show <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — application_credential_id parameter for /v3/users/{user_id}/application_credentials/{application_credential_id} API



## `osc identity credential`

Identity Credential commands

In exchange for a set of authentication credentials that the user submits, the Identity service generates and returns a token. A token represents the authenticated identity of a user and, optionally, grants authorization on a specific project or domain.

You can list all credentials, and create, show details for, update, and delete a credential.

**Usage:** `osc identity credential <COMMAND>`

###### **Subcommands:**

* `create` — Create credential
* `delete` — Delete credential
* `list` — List credentials
* `set` — Update credential
* `show` — Show credential details



## `osc identity credential create`

Creates a credential.

The following example shows how to create an EC2-style credential. The credential blob is a string that contains a JSON-serialized dictionary with the `access` and `secret` keys. This format is required when you specify the `ec2` type. To specify other credentials, such as `access_key`, change the type and contents of the data blob.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/credentials`

**Usage:** `osc identity credential create [OPTIONS]`

###### **Options:**

* `--property <key=value>`



## `osc identity credential delete`

Deletes a credential.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/credential`

**Usage:** `osc identity credential delete <ID>`

###### **Arguments:**

* `<ID>` — credential_id parameter for /v3/credentials/{credential_id} API



## `osc identity credential list`

Lists all credentials.

Optionally, you can include the `user_id` or `type` query parameter in the URI to filter the response by a user or credential type.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/credentials`

**Usage:** `osc identity credential list`



## `osc identity credential set`

Updates a credential.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/credential`

**Usage:** `osc identity credential set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — credential_id parameter for /v3/credentials/{credential_id} API

###### **Options:**

* `--property <key=value>`



## `osc identity credential show`

Shows details for a credential.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/credential`

**Usage:** `osc identity credential show <ID>`

###### **Arguments:**

* `<ID>` — credential_id parameter for /v3/credentials/{credential_id} API



## `osc identity domain`

Identity Domain commands

A domain is a collection of users, groups, and projects. Each group and project is owned by exactly one domain.

Each domain defines a namespace where certain API-visible name attributes exist, which affects whether those names must be globally unique or unique within that domain. In the Identity API, the uniqueness of these attributes is as follows:

- Domain name. Globally unique across all domains.

- Role name. Unique within the owning domain.

- User name. Unique within the owning domain.

- Project name. Unique within the owning domain.

- Group name. Unique within the owning domain.

**Usage:** `osc identity domain <COMMAND>`

###### **Subcommands:**

* `create` — Create domain
* `config` — Config configuration
* `delete` — Delete domain
* `group` — Domain groups
* `list` — List domains
* `set` — Update domain
* `show` — Show domain details
* `user` — Domain groups



## `osc identity domain create`

Creates a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domains`

**Usage:** `osc identity domain create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — The description of the domain
* `--enabled <ENABLED>` — If set to `true`, domain is enabled. If set to `false`, domain is disabled

  Possible values: `true`, `false`

* `--name <NAME>` — The name of the domain
* `--options <key=value>` — The resource options for the role. Available resource options are `immutable`
* `--tags <TAGS>`



## `osc identity domain config`

Config configuration

You can manage domain-specific configuration options.

Config-specific configuration options are structured within their group objects. The API supports only the identity and ldap groups. These groups override the default configuration settings for the storage of users and groups by the Identity server.

You can create, update, and delete domain-specific configuration options by using the HTTP PUT , PATCH , and DELETE methods. When updating, it is only necessary to include those options that are being updated.

To create an option, use the PUT method. The Identity API does not return options that are considered sensitive, although you can create and update these options. The only option currently considered sensitive is the password option within the ldap group.

The API enables you to include sensitive options as part of non- sensitive options. For example, you can include the password as part of the url option.

If you try to create or update configuration options for groups other than whitelisted on the server side, the Forbidden (403) response code is returned.

For information about how to integrate the Identity service with LDAP, see Integrate Identity with LDAP.

A domain is a collection of users, groups, and projects. Each group and project is owned by exactly one domain.

**Usage:** `osc identity domain config <COMMAND>`

###### **Subcommands:**

* `default` — Show default configuration settings
* `group` — Domain Group group
* `purge` — Delete domain configuration
* `list` — Show domain configuration
* `replace` — Create domain configuration
* `set` — Update domain configuration



## `osc identity domain config default`

The default configuration settings for the options that can be overridden can be retrieved.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config default`



## `osc identity domain config group`

Domain Group group

Group group is a driver specific configuration for the domain. It contains individual configuration options.

**Usage:** `osc identity domain config group <COMMAND>`

###### **Subcommands:**

* `default` — Show default configuration for a group
* `delete` — Delete domain group configuration
* `option` — Domain Group Option
* `set` — Update domain group configuration
* `show` — Show domain group configuration



## `osc identity domain config group default`

Reads the default configuration settings for a specific group.

The API supports only the `identity` and `ldap` groups.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group default <GROUP>`

###### **Arguments:**

* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API



## `osc identity domain config group delete`

Deletes a domain group configuration.

The API supports only the `identity` and `ldap` groups.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group delete <DOMAIN_ID> <GROUP>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API



## `osc identity domain config group option`

Domain Group Option

A domain group option is a configuration option of the domain specific driver.

**Usage:** `osc identity domain config group option <COMMAND>`

###### **Subcommands:**

* `default` — Show default option for a group
* `delete` — Delete domain group option configuration
* `set` — Update domain group option configuration
* `show` — Show domain group option configuration



## `osc identity domain config group option default`

Reads the default configuration setting for an option within a group.

The API supports only the `identity` and `ldap` groups. For the `ldap` group, a valid value is `url` or `user_tree_dn`. For the `identity` group, a valid value is `driver`.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group option default <GROUP> <OPTION>`

###### **Arguments:**

* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API
* `<OPTION>` — option parameter for /v3/domains/config/{group}/{option}/default API



## `osc identity domain config group option delete`

Deletes a domain group option configuration.

The API supports only the `identity` and `ldap` groups. For the `ldap` group, a valid value is `url` or `user_tree_dn`. For the `identity` group, a valid value is `driver`.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group option delete <DOMAIN_ID> <GROUP> <OPTION>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API
* `<OPTION>` — option parameter for /v3/domains/config/{group}/{option}/default API



## `osc identity domain config group option set`

Updates a domain group option configuration.

The API supports only the `identity` and `ldap` groups. For the `ldap` group, a valid value is `url` or `user_tree_dn`. For the `identity` group, a valid value is `driver`.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group option set [OPTIONS] <DOMAIN_ID> <GROUP> <OPTION>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API
* `<OPTION>` — option parameter for /v3/domains/config/{group}/{option}/default API

###### **Options:**

* `--config <key=value>` — A `config` object



## `osc identity domain config group option show`

Shows details for a domain group option configuration.

The API supports only the `identity` and `ldap` groups. For the `ldap` group, a valid value is `url` or `user_tree_dn`. For the `identity` group, a valid value is `driver`.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group option show <DOMAIN_ID> <GROUP> <OPTION>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API
* `<OPTION>` — option parameter for /v3/domains/config/{group}/{option}/default API



## `osc identity domain config group set`

Updates a domain group configuration.

The API supports only the `identity` and `ldap` groups. If you try to set configuration options for other groups, this call fails with the `Forbidden (403)` response code.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group set --config <JSON> <DOMAIN_ID> <GROUP>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API

###### **Options:**

* `--config <JSON>` — A `config` object



## `osc identity domain config group show`

Shows details for a domain group configuration.

The API supports only the `identity` and `ldap` groups.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config_default`

**Usage:** `osc identity domain config group show <DOMAIN_ID> <GROUP>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP>` — group parameter for /v3/domains/config/{group}/{option}/default API



## `osc identity domain config purge`

Deletes a domain configuration.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config`

**Usage:** `osc identity domain config purge <DOMAIN_ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain config list`

Shows details for a domain configuration.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config`

**Usage:** `osc identity domain config list <DOMAIN_ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain config replace`

Creates a domain configuration.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config`

**Usage:** `osc identity domain config replace --config <JSON> <DOMAIN_ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API

###### **Options:**

* `--config <JSON>` — A `config` object



## `osc identity domain config set`

Updates a domain configuration.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_config`

**Usage:** `osc identity domain config set --config <JSON> <DOMAIN_ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API

###### **Options:**

* `--config <JSON>` — A `config` object



## `osc identity domain delete`

Deletes a domain. To minimize the risk of accidentally deleting a domain, you must first disable the domain by using the update domain method.

When you delete a domain, this call also deletes all entities owned by it, such as users, groups, and projects, and any credentials and granted roles that relate to those entities.

If you try to delete an enabled domain, this call returns the `Forbidden (403)` response code.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain`

**Usage:** `osc identity domain delete <ID>`

###### **Arguments:**

* `<ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain group`

Domain groups

**Usage:** `osc identity domain group <COMMAND>`

###### **Subcommands:**

* `role` — Domain group roles



## `osc identity domain group role`

Domain group roles

OpenStack services typically determine whether a user’s API request should be allowed using Role Based Access Control (RBAC). For OpenStack this means the service compares the roles that user has on the project (as indicated by the roles in the token), against the roles required for the API in question (as defined in the service’s policy file). A user obtains roles on a project by having these assigned to them via the Identity service API.

Roles must initially be created as entities via the Identity services API and, once created, can then be assigned. You can assign roles to a user or group on a project, including projects owned by other domains. You can also assign roles to a user or group on a domain, although this is only currently relevant for using a domain scoped token to execute domain-level Identity service API requests.

**Usage:** `osc identity domain group role <COMMAND>`

###### **Subcommands:**

* `delete` — Unassign role from group on domain
* `list` — List role assignments for group on domain
* `set` — Assign role to group on domain
* `show` — Check if a group has a specific role on a domain



## `osc identity domain group role delete`

Unassigns a role from a group on a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_group_role`

**Usage:** `osc identity domain group role delete <DOMAIN_ID> <GROUP_ID> <ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP_ID>` — group_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<ID>` — role_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain group role list`

Lists role assignments for a group on a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_group_roles`

**Usage:** `osc identity domain group role list <DOMAIN_ID> <GROUP_ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP_ID>` — group_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain group role set`

Assigns a role to a group on a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_group_role`

**Usage:** `osc identity domain group role set <DOMAIN_ID> <GROUP_ID> <ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP_ID>` — group_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<ID>` — role_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain group role show`

Check if a group has a specific role on a domain.

GET/HEAD /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id}

**Usage:** `osc identity domain group role show <DOMAIN_ID> <GROUP_ID> <ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<GROUP_ID>` — group_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<ID>` — role_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain list`

Lists all domains.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domains`

**Usage:** `osc identity domain list [OPTIONS]`

###### **Options:**

* `--enabled <ENABLED>` — If set to true, then only domains that are enabled will be returned, if set to false only that are disabled will be returned. Any value other than 0, including no value, will be interpreted as true

  Possible values: `true`, `false`

* `--name <NAME>` — Filters the response by a domain name



## `osc identity domain set`

Updates a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain`

**Usage:** `osc identity domain set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API

###### **Options:**

* `--description <DESCRIPTION>` — The description of the domain
* `--enabled <ENABLED>` — If set to `true`, domain is enabled. If set to `false`, domain is disabled

  Possible values: `true`, `false`

* `--name <NAME>` — The name of the domain
* `--options <key=value>` — The resource options for the role. Available resource options are `immutable`
* `--tags <TAGS>`



## `osc identity domain show`

Shows details for a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domains`

**Usage:** `osc identity domain show <ID>`

###### **Arguments:**

* `<ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API



## `osc identity domain user`

Domain groups

**Usage:** `osc identity domain user <COMMAND>`

###### **Subcommands:**

* `role` — Domain user roles



## `osc identity domain user role`

Domain user roles

OpenStack services typically determine whether a user’s API request should be allowed using Role Based Access Control (RBAC). For OpenStack this means the service compares the roles that user has on the project (as indicated by the roles in the token), against the roles required for the API in question (as defined in the service’s policy file). A user obtains roles on a project by having these assigned to them via the Identity service API.

Roles must initially be created as entities via the Identity services API and, once created, can then be assigned. You can assign roles to a user or group on a project, including projects owned by other domains. You can also assign roles to a user or group on a domain, although this is only currently relevant for using a domain scoped token to execute domain-level Identity service API requests.

**Usage:** `osc identity domain user role <COMMAND>`

###### **Subcommands:**

* `delete` — Unassigns role from user on domain
* `list` — List role assignments for user on domain
* `set` — Assign role to user on domain
* `show` — Check if a user has a specific role on the domain



## `osc identity domain user role delete`

Unassigns a role from a user on a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_user_role`

**Usage:** `osc identity domain user role delete <--user-name <USER_NAME>|--user-id <USER_ID>> <DOMAIN_ID> <ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<ID>` — role_id parameter for /v3/domains/{domain_id}/users/{user_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity domain user role list`

Lists role assignments for a user on a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/domain_user_roles`

**Usage:** `osc identity domain user role list <--user-name <USER_NAME>|--user-id <USER_ID>> <DOMAIN_ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity domain user role set`

Assigns a role to a user on a domain.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/index.html#assign-role-to-user-on-domain`

**Usage:** `osc identity domain user role set <--user-name <USER_NAME>|--user-id <USER_ID>> <DOMAIN_ID> <ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<ID>` — role_id parameter for /v3/domains/{domain_id}/users/{user_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity domain user role show`

Check if a user has a specific role on the domain.

GET/HEAD /v3/domains/{domain_id}/users/{user_id}/roles/{role_id}

**Usage:** `osc identity domain user role show <--user-name <USER_NAME>|--user-id <USER_ID>> <DOMAIN_ID> <ID>`

###### **Arguments:**

* `<DOMAIN_ID>` — domain_id parameter for /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
* `<ID>` — role_id parameter for /v3/domains/{domain_id}/users/{user_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity endpoint`

Endpoint commands

**Usage:** `osc identity endpoint <COMMAND>`

###### **Subcommands:**

* `create` — Create endpoint
* `delete` — Delete endpoint
* `list` — List endpoints
* `set` — Update endpoint
* `show` — Show endpoint details



## `osc identity endpoint create`

Creates an endpoint.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/endpoints`

**Usage:** `osc identity endpoint create [OPTIONS]`

###### **Options:**

* `--enabled <ENABLED>` — Indicates whether the endpoint appears in the service catalog: - `false`. The endpoint does not appear in the service catalog. - `true`. The endpoint appears in the service catalog

  Possible values: `true`, `false`

* `--interface <INTERFACE>` — The interface type, which describes the visibility of the endpoint. Value is: - `public`. Visible by end users on a publicly available network interface. - `internal`. Visible by end users on an unmetered internal network interface. - `admin`. Visible by administrative users on a secure network interface

  Possible values: `admin`, `internal`, `public`

* `--region <REGION>` — (Deprecated in v3.2) The geographic location of the service endpoint
* `--region-id <REGION_ID>` — (Since v3.2) The ID of the region that contains the service endpoint
* `--service-id <SERVICE_ID>` — The UUID of the service to which the endpoint belongs
* `--url <URL>` — The endpoint URL



## `osc identity endpoint delete`

Deletes an endpoint.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/endpoint`

**Usage:** `osc identity endpoint delete <ID>`

###### **Arguments:**

* `<ID>` — endpoint_id parameter for /v3/endpoints/{endpoint_id}/OS-ENDPOINT-POLICY/policy API



## `osc identity endpoint list`

Lists all available endpoints.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/endpoints`

**Usage:** `osc identity endpoint list [OPTIONS]`

###### **Options:**

* `--interface <INTERFACE>` — Filters the response by an interface

  Possible values: `admin`, `internal`, `public`

* `--region <REGION>` — Filters the response by a region ID
* `--service-id <SERVICE_ID>` — Filters the response by a service ID



## `osc identity endpoint set`

Updates an endpoint.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/endpoint`

**Usage:** `osc identity endpoint set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — endpoint_id parameter for /v3/endpoints/{endpoint_id}/OS-ENDPOINT-POLICY/policy API

###### **Options:**

* `--property <key=value>`



## `osc identity endpoint show`

Shows details for an endpoint.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/endpoints`

**Usage:** `osc identity endpoint show <ID>`

###### **Arguments:**

* `<ID>` — endpoint_id parameter for /v3/endpoints/{endpoint_id}/OS-ENDPOINT-POLICY/policy API



## `osc identity federation`

OS-Federation

Provide the ability for users to manage Identity Providers (IdPs) and establish a set of rules to map federation protocol attributes to Identity API attributes.

**Usage:** `osc identity federation <COMMAND>`

###### **Subcommands:**

* `identity-provider` — Identity Providers
* `mapping` — Mappings
* `service-provider` — Service Providers
* `saml2-metadata` — A user may retrieve Metadata about an Identity Service acting as an Identity Provider



## `osc identity federation identity-provider`

Identity Providers

An Identity Provider (IdP) is a third party service that is trusted by the Identity API to authenticate identities.

**Usage:** `osc identity federation identity-provider <COMMAND>`

###### **Subcommands:**

* `create` — Create an idp resource for federated authentication
* `delete` — DELETE operation on /v3/OS-FEDERATION/identity_providers/{idp_id}
* `list` — GET operation on /v3/OS-FEDERATION/identity_providers
* `protocol` — Identity provider protocols
* `set` — PATCH operation on /v3/OS-FEDERATION/identity_providers/{idp_id}
* `show` — GET operation on /v3/OS-FEDERATION/identity_providers/{idp_id}



## `osc identity federation identity-provider create`

Create an idp resource for federated authentication.

PUT /OS-FEDERATION/identity_providers/{idp_id}

**Usage:** `osc identity federation identity-provider create [OPTIONS] <IDP_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API

###### **Options:**

* `--authorization-ttl <AUTHORIZATION_TTL>`
* `--description <DESCRIPTION>`
* `--domain-id <DOMAIN_ID>`
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--remote-ids <REMOTE_IDS>`



## `osc identity federation identity-provider delete`

DELETE operation on /v3/OS-FEDERATION/identity_providers/{idp_id}

**Usage:** `osc identity federation identity-provider delete <IDP_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API



## `osc identity federation identity-provider list`

GET operation on /v3/OS-FEDERATION/identity_providers

**Usage:** `osc identity federation identity-provider list [OPTIONS]`

###### **Options:**

* `--enabled <ENABLED>` — Filter for Identity Providers’ enabled attribute

  Possible values: `true`, `false`

* `--id <ID>` — Filter for Identity Providers’ ID attribute



## `osc identity federation identity-provider protocol`

Identity provider protocols

A protocol entry contains information that dictates which mapping rules to use for a given incoming request. An IdP may have multiple supported protocols.

Required attributes:

- mapping_id (string): Indicates which mapping should be used to process federated authentication requests.

Optional attributes:

- remote_id_attribute (string): Key to obtain the entity ID of the Identity Provider from the HTTPD environment. For mod_shib, this would be Shib-Identity-Provider. For mod_auth_openidc, this could be HTTP_OIDC_ISS. For mod_auth_mellon, this could be MELLON_IDP. This overrides the default value provided in keystone.conf.

**Usage:** `osc identity federation identity-provider protocol <COMMAND>`

###### **Subcommands:**

* `create` — Create protocol for an IDP
* `delete` — Delete a protocol from an IDP
* `list` — List protocols for an IDP
* `set` — Update protocol for an IDP
* `show` — Get protocols for an IDP



## `osc identity federation identity-provider protocol create`

Create protocol for an IDP.

PUT /OS-Federation/identity_providers/{idp_id}/protocols/{protocol_id}

**Usage:** `osc identity federation identity-provider protocol create [OPTIONS] --mapping-id <MAPPING_ID> <IDP_ID> <ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API
* `<ID>` — protocol_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id} API

###### **Options:**

* `--mapping-id <MAPPING_ID>`
* `--remote-id-attribute <REMOTE_ID_ATTRIBUTE>`



## `osc identity federation identity-provider protocol delete`

Delete a protocol from an IDP.

DELETE /OS-FEDERATION/identity_providers/ {idp_id}/protocols/{protocol_id}

**Usage:** `osc identity federation identity-provider protocol delete <IDP_ID> <ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API
* `<ID>` — protocol_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id} API



## `osc identity federation identity-provider protocol list`

List protocols for an IDP.

HEAD/GET /OS-FEDERATION/identity_providers/{idp_id}/protocols

**Usage:** `osc identity federation identity-provider protocol list <IDP_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API



## `osc identity federation identity-provider protocol set`

Update protocol for an IDP.

PATCH /OS-FEDERATION/identity_providers/ {idp_id}/protocols/{protocol_id}

**Usage:** `osc identity federation identity-provider protocol set [OPTIONS] <IDP_ID> <ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API
* `<ID>` — protocol_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id} API

###### **Options:**

* `--mapping-id <MAPPING_ID>`
* `--remote-id-attribute <REMOTE_ID_ATTRIBUTE>`



## `osc identity federation identity-provider protocol show`

Get protocols for an IDP.

HEAD/GET /OS-FEDERATION/identity_providers/ {idp_id}/protocols/{protocol_id}

**Usage:** `osc identity federation identity-provider protocol show <IDP_ID> <ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API
* `<ID>` — protocol_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id} API



## `osc identity federation identity-provider set`

PATCH operation on /v3/OS-FEDERATION/identity_providers/{idp_id}

**Usage:** `osc identity federation identity-provider set [OPTIONS] <IDP_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API

###### **Options:**

* `--authorization-ttl <AUTHORIZATION_TTL>`
* `--description <DESCRIPTION>`
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--remote-ids <REMOTE_IDS>`



## `osc identity federation identity-provider show`

GET operation on /v3/OS-FEDERATION/identity_providers/{idp_id}

**Usage:** `osc identity federation identity-provider show <IDP_ID>`

###### **Arguments:**

* `<IDP_ID>` — idp_id parameter for /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API



## `osc identity federation mapping`

Mappings

A mapping is a set of rules to map federation protocol attributes to Identity API objects. An Identity Provider can have a single mapping specified per protocol. A mapping is simply a list of rules.

**Usage:** `osc identity federation mapping <COMMAND>`

###### **Subcommands:**

* `create` — Create a mapping
* `delete` — Delete a mapping
* `list` — GET operation on /v3/OS-FEDERATION/mappings
* `set` — Update an attribute mapping for identity federation
* `show` — GET operation on /v3/OS-FEDERATION/mappings/{mapping_id}



## `osc identity federation mapping create`

Create a mapping.

PUT /OS-FEDERATION/mappings/{mapping_id}

**Usage:** `osc identity federation mapping create [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — mapping_id parameter for /v3/OS-FEDERATION/mappings/{mapping_id} API

###### **Options:**

* `--rules <JSON>`
* `--schema-version <SCHEMA_VERSION>` — Mapping schema version



## `osc identity federation mapping delete`

Delete a mapping.

DELETE /OS-FEDERATION/mappings/{mapping_id}

**Usage:** `osc identity federation mapping delete <ID>`

###### **Arguments:**

* `<ID>` — mapping_id parameter for /v3/OS-FEDERATION/mappings/{mapping_id} API



## `osc identity federation mapping list`

GET operation on /v3/OS-FEDERATION/mappings

**Usage:** `osc identity federation mapping list`



## `osc identity federation mapping set`

Update an attribute mapping for identity federation.

PATCH /OS-FEDERATION/mappings/{mapping_id}

**Usage:** `osc identity federation mapping set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — mapping_id parameter for /v3/OS-FEDERATION/mappings/{mapping_id} API

###### **Options:**

* `--rules <JSON>`
* `--schema-version <SCHEMA_VERSION>` — Mapping schema version



## `osc identity federation mapping show`

GET operation on /v3/OS-FEDERATION/mappings/{mapping_id}

**Usage:** `osc identity federation mapping show <ID>`

###### **Arguments:**

* `<ID>` — mapping_id parameter for /v3/OS-FEDERATION/mappings/{mapping_id} API



## `osc identity federation service-provider`

Service Providers

A service provider is a third party service that is trusted by the Identity Service.

**Usage:** `osc identity federation service-provider <COMMAND>`

###### **Subcommands:**

* `create` — Create a service provider
* `delete` — Delete a service provider
* `list` — GET operation on /v3/OS-FEDERATION/service_providers
* `set` — Update a service provider
* `show` — GET operation on /v3/OS-FEDERATION/service_providers/{sp_id}



## `osc identity federation service-provider create`

Create a service provider.

PUT /OS-FEDERATION/service_providers/{sp_id}

**Usage:** `osc identity federation service-provider create [OPTIONS] --auth-url <AUTH_URL> --sp-url <SP_URL> <SP_ID>`

###### **Arguments:**

* `<SP_ID>` — sp_id parameter for /v3/OS-FEDERATION/service_providers/{sp_id} API

###### **Options:**

* `--auth-url <AUTH_URL>`
* `--description <DESCRIPTION>`
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--relay-state-prefix <RELAY_STATE_PREFIX>`
* `--sp-url <SP_URL>`



## `osc identity federation service-provider delete`

Delete a service provider.

DELETE /OS-FEDERATION/service_providers/{sp_id}

**Usage:** `osc identity federation service-provider delete <SP_ID>`

###### **Arguments:**

* `<SP_ID>` — sp_id parameter for /v3/OS-FEDERATION/service_providers/{sp_id} API



## `osc identity federation service-provider list`

GET operation on /v3/OS-FEDERATION/service_providers

**Usage:** `osc identity federation service-provider list`



## `osc identity federation service-provider set`

Update a service provider.

PATCH /OS-FEDERATION/service_providers/{sp_id}

**Usage:** `osc identity federation service-provider set [OPTIONS] <SP_ID>`

###### **Arguments:**

* `<SP_ID>` — sp_id parameter for /v3/OS-FEDERATION/service_providers/{sp_id} API

###### **Options:**

* `--auth-url <AUTH_URL>`
* `--description <DESCRIPTION>`
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--relay-state-prefix <RELAY_STATE_PREFIX>`
* `--sp-url <SP_URL>`



## `osc identity federation service-provider show`

GET operation on /v3/OS-FEDERATION/service_providers/{sp_id}

**Usage:** `osc identity federation service-provider show <SP_ID>`

###### **Arguments:**

* `<SP_ID>` — sp_id parameter for /v3/OS-FEDERATION/service_providers/{sp_id} API



## `osc identity federation saml2-metadata`

A user may retrieve Metadata about an Identity Service acting as an Identity Provider.

The response will be a full document with Metadata properties. Note that for readability, this example certificate has been truncated.

**Usage:** `osc identity federation saml2-metadata <COMMAND>`

###### **Subcommands:**

* `show` — Get SAML2 metadata



## `osc identity federation saml2-metadata show`

Get SAML2 metadata.

GET/HEAD /OS-FEDERATION/saml2/metadata

**Usage:** `osc identity federation saml2-metadata show`



## `osc identity group`

Identity Group commands

A group is a collection of users. Each group is owned by a domain.

You can use groups to ease the task of managing role assignments for users. Assigning a role to a group on a project or domain is equivalent to assigning the role to each group member on that project or domain.

When you unassign a role from a group, that role is automatically unassigned from any user that is a member of the group. Any tokens that authenticates those users to the relevant project or domain are revoked.

As with users, a group without any role assignments is useless from the perspective of an OpenStack service and has no access to resources. However, a group without role assignments is permitted as a way of acquiring or loading users and groups from external sources before mapping them to projects and domains.

**Usage:** `osc identity group <COMMAND>`

###### **Subcommands:**

* `create` — Create group
* `delete` — Delete group
* `list` — List groups
* `set` — Update group
* `show` — Show group details



## `osc identity group create`

Creates a group.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/groups`

**Usage:** `osc identity group create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — The description of the group
* `--domain-id <DOMAIN_ID>` — The ID of the domain
* `--name <NAME>` — The user name. Must be unique within the owning domain



## `osc identity group delete`

Deletes a group.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/group`

**Usage:** `osc identity group delete <ID>`

###### **Arguments:**

* `<ID>` — group_id parameter for /v3/groups/{group_id}/users/{user_id} API



## `osc identity group list`

Lists groups.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/groups`

**Usage:** `osc identity group list [OPTIONS]`

###### **Options:**

* `--domain-id <DOMAIN_ID>` — Filters the response by a domain ID



## `osc identity group set`

Updates a group.

If the back-end driver does not support this functionality, the call returns the `Not Implemented (501)` response code.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/group`

**Usage:** `osc identity group set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — group_id parameter for /v3/groups/{group_id}/users/{user_id} API

###### **Options:**

* `--description <DESCRIPTION>` — The description of the group
* `--domain-id <DOMAIN_ID>` — The ID of the domain
* `--name <NAME>` — The user name. Must be unique within the owning domain



## `osc identity group show`

Shows details for a group.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/group`

**Usage:** `osc identity group show <ID>`

###### **Arguments:**

* `<ID>` — group_id parameter for /v3/groups/{group_id}/users/{user_id} API



## `osc identity project`

Identity Project commands

**Usage:** `osc identity project <COMMAND>`

###### **Subcommands:**

* `create` — Create project
* `delete` — Delete project
* `group` — Project Group commands
* `list` — List projects
* `set` — Update project
* `show` — Show project details
* `user` — Project User commands



## `osc identity project create`

Creates a project, where the project may act as a domain.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/projects`

**Usage:** `osc identity project create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — The description of the project
* `--domain-id <DOMAIN_ID>` — The ID of the domain for the project
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--is-domain <IS_DOMAIN>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--name <NAME>` — The name of the project
* `--immutable <IMMUTABLE>`

  Possible values: `true`, `false`

* `--parent-id <PARENT_ID>` — The ID of the parent for the project.

   **New in version 3.4**
* `--tags <TAGS>` — A list of simple strings assigned to a project



## `osc identity project delete`

Deletes a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project`

**Usage:** `osc identity project delete <ID>`

###### **Arguments:**

* `<ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API



## `osc identity project group`

Project Group commands

This command allows managing of the user group roles on the `project`

**Usage:** `osc identity project group <COMMAND>`

###### **Subcommands:**

* `role` — Identity Project User Group Role commands



## `osc identity project group role`

Identity Project User Group Role commands

This command allows managing of the user roles on the `project`

**Usage:** `osc identity project group role <COMMAND>`

###### **Subcommands:**

* `delete` — Unassign role from group on project
* `list` — List role assignments for group on project
* `set` — Assign role to group on project
* `show` — Check grant for project, group, role



## `osc identity project group role delete`

Unassigns a role from a group on a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project_group_role`

**Usage:** `osc identity project group role delete <PROJECT_ID> <GROUP_ID> <ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<GROUP_ID>` — group_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<ID>` — role_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles/{role_id} API



## `osc identity project group role list`

Lists role assignments for a group on a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project_user_role`

**Usage:** `osc identity project group role list <PROJECT_ID> <GROUP_ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<GROUP_ID>` — group_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API



## `osc identity project group role set`

Assigns a role to a group on a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project_group_role`

**Usage:** `osc identity project group role set <PROJECT_ID> <GROUP_ID> <ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<GROUP_ID>` — group_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<ID>` — role_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles/{role_id} API



## `osc identity project group role show`

Check grant for project, group, role.

GET/HEAD /v3/projects/{project_id/groups/{group_id}/roles/{role_id}

**Usage:** `osc identity project group role show <PROJECT_ID> <GROUP_ID> <ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<GROUP_ID>` — group_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<ID>` — role_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles/{role_id} API



## `osc identity project list`

Lists projects.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/projects`

**Usage:** `osc identity project list [OPTIONS]`

###### **Options:**

* `--domain-id <DOMAIN_ID>` — Filters the response by a domain ID
* `--enabled <ENABLED>` — If set to true, then only enabled projects will be returned. Any value other than 0 (including no value) will be interpreted as true

  Possible values: `true`, `false`

* `--is-domain <IS_DOMAIN>` — If this is specified as true, then only projects acting as a domain are included. Otherwise, only projects that are not acting as a domain are included

  Possible values: `true`, `false`

* `--name <NAME>` — Filters the response by a resource name
* `--parent-id <PARENT_ID>` — Filters the response by a parent ID



## `osc identity project set`

Updates a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project`

**Usage:** `osc identity project set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API

###### **Options:**

* `--description <DESCRIPTION>` — The description of the project
* `--domain-id <DOMAIN_ID>` — The ID of the domain for the project
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--is-domain <IS_DOMAIN>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--name <NAME>` — The name of the project
* `--immutable <IMMUTABLE>`

  Possible values: `true`, `false`

* `--parent-id <PARENT_ID>` — The ID of the parent for the project.

   **New in version 3.4**
* `--tags <TAGS>` — A list of simple strings assigned to a project



## `osc identity project show`

Shows details for a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project`

**Usage:** `osc identity project show <ID>`

###### **Arguments:**

* `<ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API



## `osc identity project user`

Project User commands

This command allows managing of the user roles on the `project`

**Usage:** `osc identity project user <COMMAND>`

###### **Subcommands:**

* `role` — Identity Project User Role commands



## `osc identity project user role`

Identity Project User Role commands

This command allows managing of the user roles on the `project`

**Usage:** `osc identity project user role <COMMAND>`

###### **Subcommands:**

* `delete` — Unassign role from user on project
* `list` — List role assignments for user on project
* `set` — Assign role to user on project
* `show` — Check grant for project, user, role



## `osc identity project user role delete`

Unassigns a role from a user on a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project_user_role`

**Usage:** `osc identity project user role delete <--user-name <USER_NAME>|--user-id <USER_ID>> <PROJECT_ID> <ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<ID>` — role_id parameter for /v3/projects/{project_id}/users/{user_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity project user role list`

Lists role assignments for a user on a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project_user_role`

**Usage:** `osc identity project user role list <--user-name <USER_NAME>|--user-id <USER_ID>> <PROJECT_ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity project user role set`

Assigns a role to a user on a project.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/project_user_role`

**Usage:** `osc identity project user role set <--user-name <USER_NAME>|--user-id <USER_ID>> <PROJECT_ID> <ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<ID>` — role_id parameter for /v3/projects/{project_id}/users/{user_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity project user role show`

Check grant for project, user, role.

GET/HEAD /v3/projects/{project_id/users/{user_id}/roles/{role_id}

**Usage:** `osc identity project user role show <--user-name <USER_NAME>|--user-id <USER_ID>> <PROJECT_ID> <ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API
* `<ID>` — role_id parameter for /v3/projects/{project_id}/users/{user_id}/roles/{role_id} API

###### **Options:**

* `--user-name <USER_NAME>` — User Name
* `--user-id <USER_ID>` — User ID



## `osc identity region`

Region commands

A region is a general division of an OpenStack deployment. You can associate zero or more sub-regions with a region to create a tree- like structured hierarchy.

Although a region does not have a geographical connotation, a deployment can use a geographical name for a region ID, such as us- east.

You can list, create, update, show details for, and delete regions.

**Usage:** `osc identity region <COMMAND>`

###### **Subcommands:**

* `create` — Create region
* `delete` — Delete region
* `list` — List regions
* `set` — Update region
* `show` — Show region details



## `osc identity region create`

Creates a region.

When you create the region, you can optionally specify a region ID. If you include characters in the region ID that are not allowed in a URI, you must URL-encode the ID. If you omit an ID, the API assigns an ID to the region.

The following errors might occur:

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/regions`

**Usage:** `osc identity region create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — The region description
* `--parent-id <PARENT_ID>` — To make this region a child of another region, set this parameter to the ID of the parent region



## `osc identity region delete`

Deletes a region.

The following error might occur:

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/region`

**Usage:** `osc identity region delete <ID>`

###### **Arguments:**

* `<ID>` — region_id parameter for /v3/regions/{region_id} API



## `osc identity region list`

Lists regions.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/regions`

**Usage:** `osc identity region list [OPTIONS]`

###### **Options:**

* `--parent-region-id <PARENT_REGION_ID>` — Filters the response by a parent region, by ID



## `osc identity region set`

Updates a region.

You can update the description or parent region ID for a region. You cannot update the region ID.

The following error might occur:

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/region`

**Usage:** `osc identity region set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — region_id parameter for /v3/regions/{region_id} API

###### **Options:**

* `--description <DESCRIPTION>` — The region description
* `--parent-id <PARENT_ID>` — To make this region a child of another region, set this parameter to the ID of the parent region



## `osc identity region show`

Shows details for a region, by ID.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/regions`

**Usage:** `osc identity region show <ID>`

###### **Arguments:**

* `<ID>` — region_id parameter for /v3/regions/{region_id} API



## `osc identity role`

Identity Role commands

OpenStack services typically determine whether a user’s API request should be allowed using Role Based Access Control (RBAC). For OpenStack this means the service compares the roles that user has on the project (as indicated by the roles in the token), against the roles required for the API in question (as defined in the service’s policy file). A user obtains roles on a project by having these assigned to them via the Identity service API.

Roles must initially be created as entities via the Identity services API and, once created, can then be assigned. You can assign roles to a user or group on a project, including projects owned by other domains. You can also assign roles to a user or group on a domain, although this is only currently relevant for using a domain scoped token to execute domain-level Identity service API requests.

**Usage:** `osc identity role <COMMAND>`

###### **Subcommands:**

* `assignment` — Role Assignments commands
* `create` — Create role
* `delete` — Delete role
* `imply` — Identity Implied Imply commands
* `inference` — Role Inferences commands
* `list` — List roles
* `set` — Update role
* `show` — Show role details



## `osc identity role assignment`

Role Assignments commands

**Usage:** `osc identity role assignment <COMMAND>`

###### **Subcommands:**

* `list` — List role assignments



## `osc identity role assignment list`

Get a list of role assignments.

If no query parameters are specified, then this API will return a list of all role assignments.

Since this list is likely to be very long, this API would typically always be used with one of more of the filter queries. Some typical examples are:

`GET /v3/role_assignments?user.id={user_id}` would list all role assignments involving the specified user.

`GET /v3/role_assignments?scope.project.id={project_id}` would list all role assignments involving the specified project.

It is also possible to list all role assignments within a tree of projects: `GET /v3/role_assignments?scope.project.id={project_id}&include_subtree=true` would list all role assignments involving the specified project and all sub-projects. `include_subtree=true` can only be specified in conjunction with `scope.project.id`, specifying it without this will result in an HTTP 400 Bad Request being returned.

Each role assignment entity in the collection contains a link to the assignment that gave rise to this entity.

The scope section in the list response is extended to allow the representation of role assignments that are inherited to projects.

The query filter `scope.OS-INHERIT:inherited_to` can be used to filter based on role assignments that are inherited. The only value of `scope.OS-INHERIT:inherited_to` that is currently supported is `projects`, indicating that this role is inherited to all projects of the owning domain or parent project.

If the query parameter `effective` is specified, rather than simply returning a list of role assignments that have been made, the API returns a list of effective assignments at the user, project and domain level, having allowed for the effects of group membership, role inference rules as well as inheritance from the parent domain or project. Since the effects of group membership have already been allowed for, the group role assignment entities themselves will not be returned in the collection. Likewise, since the effects of inheritance have already been allowed for, the role assignment entities themselves that specify the inheritance will also not be returned in the collection. This represents the effective role assignments that would be included in a scoped token. The same set of query parameters can also be used in combination with the `effective` parameter.

For example:

`GET /v3/role_assignments?user.id={user_id}&effective` would, in other words, answer the question “what can this user actually do?”.

`GET /v3/role_assignments?user.id={user_id}&scope.project.id={project_id}&effective` would return the equivalent set of role assignments that would be included in the token response of a project scoped token.

An example response for an API call with the query parameter `effective` specified is given below:

The entity `links` section of a response using the `effective` query parameter also contains, for entities that are included by virtue of group membership, a url that can be used to access the membership of the group.

If the query parameter `include_names` is specified, rather than simply returning the entity IDs in the role assignments, the collection will additionally include the names of the entities. For example:

`GET /v3/role_assignments?user.id={user_id}&effective&include_names=true` would return:

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/role_assignments`

**Usage:** `osc identity role assignment list [OPTIONS]`

###### **Options:**

* `--effective` — Returns the effective assignments, including any assignments gained by virtue of group membership
* `--group-id <GROUP_ID>` — Filters the response by a group ID
* `--include-names` — If set, then the names of any entities returned will be include as well as their IDs. Any value other than 0 (including no value) will be interpreted as true
* `--include-subtree` — If set, then relevant assignments in the project hierarchy below the project specified in the scope.project_id query parameter are also included in the response. Any value other than 0 (including no value) for include_subtree will be interpreted as true
* `--role-id <ROLE_ID>` — Filters the response by a role ID
* `--scope-domain-id <SCOPE_DOMAIN_ID>` — Filters the response by a domain ID
* `--scope-os-inherit-inherited-to <SCOPE_OS_INHERIT_INHERITED_TO>` — Filters based on role assignments that are inherited. The only value of inherited_to that is currently supported is projects
* `--scope-project-id <SCOPE_PROJECT_ID>` — Filters the response by a project ID
* `--user-id <USER_ID>` — Filters the response by a user ID



## `osc identity role create`

Creates a role.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/roles`

**Usage:** `osc identity role create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — The role description
* `--name <NAME>` — The role name
* `--immutable <IMMUTABLE>`

  Possible values: `true`, `false`




## `osc identity role delete`

Deletes a role.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/role`

**Usage:** `osc identity role delete <ID>`

###### **Arguments:**

* `<ID>` — role_id parameter for /v3/roles/{role_id} API



## `osc identity role imply`

Identity Implied Imply commands

**Usage:** `osc identity role imply <COMMAND>`

###### **Subcommands:**

* `delete` — Delete role inference rule
* `list` — List implied (inference) roles for role
* `set` — Create role inference rule
* `show` — Get role inference rule



## `osc identity role imply delete`

Deletes a role inference rule.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/#delete-role-inference-rule`

**Usage:** `osc identity role imply delete <PRIOR_ROLE_ID> <IMPLIED_ROLE_ID>`

###### **Arguments:**

* `<PRIOR_ROLE_ID>` — prior_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API
* `<IMPLIED_ROLE_ID>` — implied_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API



## `osc identity role imply list`

Lists implied (inference) roles for a role.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/#list-implied-roles-for-role`

**Usage:** `osc identity role imply list <PRIOR_ROLE_ID>`

###### **Arguments:**

* `<PRIOR_ROLE_ID>` — prior_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API



## `osc identity role imply set`

Creates a role inference rule.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/#create-role-inference-rule`

**Usage:** `osc identity role imply set [OPTIONS] <PRIOR_ROLE_ID> <IMPLIED_ROLE_ID>`

###### **Arguments:**

* `<PRIOR_ROLE_ID>` — prior_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API
* `<IMPLIED_ROLE_ID>` — implied_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API

###### **Options:**

* `--property <key=value>`



## `osc identity role imply show`

Gets a role inference rule.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/#get-role-inference-rule`

**Usage:** `osc identity role imply show <PRIOR_ROLE_ID> <IMPLIED_ROLE_ID>`

###### **Arguments:**

* `<PRIOR_ROLE_ID>` — prior_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API
* `<IMPLIED_ROLE_ID>` — implied_role_id parameter for /v3/roles/{prior_role_id}/implies/{implied_role_id} API



## `osc identity role inference`

Role Inferences commands

Operating the role inferences (implied roles)

**Usage:** `osc identity role inference <COMMAND>`

###### **Subcommands:**

* `list` — List all role inference rules



## `osc identity role inference list`

Lists all role inference rules.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/#list-all-role-inference-rules`

**Usage:** `osc identity role inference list`



## `osc identity role list`

Lists roles.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/roles`

**Usage:** `osc identity role list [OPTIONS]`

###### **Options:**

* `--domain-id <DOMAIN_ID>` — Filters the response by a domain ID



## `osc identity role set`

Updates a role.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/role`

**Usage:** `osc identity role set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — role_id parameter for /v3/roles/{role_id} API

###### **Options:**

* `--description <DESCRIPTION>` — The role description
* `--name <NAME>` — The role name
* `--immutable <IMMUTABLE>`

  Possible values: `true`, `false`




## `osc identity role show`

Shows details for a role.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/role`

**Usage:** `osc identity role show <ID>`

###### **Arguments:**

* `<ID>` — role_id parameter for /v3/roles/{role_id} API



## `osc identity role-assignment`

Role Assignments commands

**Usage:** `osc identity role-assignment <COMMAND>`

###### **Subcommands:**

* `list` — List role assignments



## `osc identity role-assignment list`

Get a list of role assignments.

If no query parameters are specified, then this API will return a list of all role assignments.

Since this list is likely to be very long, this API would typically always be used with one of more of the filter queries. Some typical examples are:

`GET /v3/role_assignments?user.id={user_id}` would list all role assignments involving the specified user.

`GET /v3/role_assignments?scope.project.id={project_id}` would list all role assignments involving the specified project.

It is also possible to list all role assignments within a tree of projects: `GET /v3/role_assignments?scope.project.id={project_id}&include_subtree=true` would list all role assignments involving the specified project and all sub-projects. `include_subtree=true` can only be specified in conjunction with `scope.project.id`, specifying it without this will result in an HTTP 400 Bad Request being returned.

Each role assignment entity in the collection contains a link to the assignment that gave rise to this entity.

The scope section in the list response is extended to allow the representation of role assignments that are inherited to projects.

The query filter `scope.OS-INHERIT:inherited_to` can be used to filter based on role assignments that are inherited. The only value of `scope.OS-INHERIT:inherited_to` that is currently supported is `projects`, indicating that this role is inherited to all projects of the owning domain or parent project.

If the query parameter `effective` is specified, rather than simply returning a list of role assignments that have been made, the API returns a list of effective assignments at the user, project and domain level, having allowed for the effects of group membership, role inference rules as well as inheritance from the parent domain or project. Since the effects of group membership have already been allowed for, the group role assignment entities themselves will not be returned in the collection. Likewise, since the effects of inheritance have already been allowed for, the role assignment entities themselves that specify the inheritance will also not be returned in the collection. This represents the effective role assignments that would be included in a scoped token. The same set of query parameters can also be used in combination with the `effective` parameter.

For example:

`GET /v3/role_assignments?user.id={user_id}&effective` would, in other words, answer the question “what can this user actually do?”.

`GET /v3/role_assignments?user.id={user_id}&scope.project.id={project_id}&effective` would return the equivalent set of role assignments that would be included in the token response of a project scoped token.

An example response for an API call with the query parameter `effective` specified is given below:

The entity `links` section of a response using the `effective` query parameter also contains, for entities that are included by virtue of group membership, a url that can be used to access the membership of the group.

If the query parameter `include_names` is specified, rather than simply returning the entity IDs in the role assignments, the collection will additionally include the names of the entities. For example:

`GET /v3/role_assignments?user.id={user_id}&effective&include_names=true` would return:

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/role_assignments`

**Usage:** `osc identity role-assignment list [OPTIONS]`

###### **Options:**

* `--effective` — Returns the effective assignments, including any assignments gained by virtue of group membership
* `--group-id <GROUP_ID>` — Filters the response by a group ID
* `--include-names` — If set, then the names of any entities returned will be include as well as their IDs. Any value other than 0 (including no value) will be interpreted as true
* `--include-subtree` — If set, then relevant assignments in the project hierarchy below the project specified in the scope.project_id query parameter are also included in the response. Any value other than 0 (including no value) for include_subtree will be interpreted as true
* `--role-id <ROLE_ID>` — Filters the response by a role ID
* `--scope-domain-id <SCOPE_DOMAIN_ID>` — Filters the response by a domain ID
* `--scope-os-inherit-inherited-to <SCOPE_OS_INHERIT_INHERITED_TO>` — Filters based on role assignments that are inherited. The only value of inherited_to that is currently supported is projects
* `--scope-project-id <SCOPE_PROJECT_ID>` — Filters the response by a project ID
* `--user-id <USER_ID>` — Filters the response by a user ID



## `osc identity role-inference`

Role Inferences commands

Operating the role inferences (implied roles)

**Usage:** `osc identity role-inference <COMMAND>`

###### **Subcommands:**

* `list` — List all role inference rules



## `osc identity role-inference list`

Lists all role inference rules.

Relationship: `https://developer.openstack.org/api-ref/identity/v3/#list-all-role-inference-rules`

**Usage:** `osc identity role-inference list`



## `osc identity service`

Service commands

A service is an OpenStack web service that you can access through a URL, i.e. an endpoint.

**Usage:** `osc identity service <COMMAND>`

###### **Subcommands:**

* `create` — Create service
* `delete` — Delete service
* `list` — List services
* `set` — Update service
* `show` — Show service details



## `osc identity service create`

Creates a service.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/services`

**Usage:** `osc identity service create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — The service description
* `--enabled <ENABLED>` — Defines whether the service and its endpoints appear in the service catalog: - `false`. The service and its endpoints do not appear in the service catalog. - `true`. The service and its endpoints appear in the service catalog. Default is `true`

  Possible values: `true`, `false`

* `--name <NAME>` — The service name
* `--type <TYPE>` — The service type, which describes the API implemented by the service. Value is `compute`, `ec2`, `identity`, `image`, `network`, or `volume`



## `osc identity service delete`

Deletes a service.

If you try to delete a service that still has associated endpoints, this call either deletes all associated endpoints or fails until all endpoints are deleted.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/service`

**Usage:** `osc identity service delete <ID>`

###### **Arguments:**

* `<ID>` — service_id parameter for /v3/services/{service_id} API



## `osc identity service list`

Lists all services.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/services`

**Usage:** `osc identity service list [OPTIONS]`

###### **Options:**

* `--service <SERVICE>` — Filters the response by a domain ID



## `osc identity service set`

Updates a service.

The request body is the same as the create service request body, except that you include only those attributes that you want to update.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/services`

**Usage:** `osc identity service set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — service_id parameter for /v3/services/{service_id} API

###### **Options:**

* `--description <DESCRIPTION>` — The service description
* `--enabled <ENABLED>` — Defines whether the service and its endpoints appear in the service catalog: - `false`. The service and its endpoints do not appear in the service catalog. - `true`. The service and its endpoints appear in the service catalog. Default is `true`

  Possible values: `true`, `false`

* `--name <NAME>` — The service name
* `--type <TYPE>` — The service type, which describes the API implemented by the service. Value is `compute`, `ec2`, `identity`, `image`, `network`, or `volume`



## `osc identity service show`

Shows details for a service.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/service`

**Usage:** `osc identity service show <ID>`

###### **Arguments:**

* `<ID>` — service_id parameter for /v3/services/{service_id} API



## `osc identity user`

User commands

A user is an individual API consumer that is owned by a domain. A role explicitly associates a user with projects or domains. A user with no assigned roles has no access to OpenStack resources.

You can list, create, show details for, update, delete, and change the password for users.

You can also list groups, projects, and role assignments for a specified user.

**Usage:** `osc identity user <COMMAND>`

###### **Subcommands:**

* `create` — Create user
* `delete` — Delete user
* `groups` — List groups to which a user belongs
* `list` — List users
* `password` — User password commands
* `projects` — List projects for user
* `set` — Update user
* `show` — Show user details



## `osc identity user create`

Creates a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/users`

**Usage:** `osc identity user create [OPTIONS] --name <NAME>`

###### **Options:**

* `--default-project-id <DEFAULT_PROJECT_ID>` — The ID of the default project for the user
* `--description <DESCRIPTION>`
* `--domain-id <DOMAIN_ID>` — The ID of the domain
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--federated <JSON>` — List of federated objects associated with a user. Each object in the list contains the `idp_id` and `protocols`. `protocols` is a list of objects, each of which contains `protocol_id` and `unique_id` of the protocol and user respectively. For example:

   ```text "federated": [ { "idp_id": "efbab5a6acad4d108fec6c63d9609d83", "protocols": [ {"protocol_id": "mapped", "unique_id": "test@example.com"} ] } ]

   ```
* `--name <NAME>` — The user name. Must be unique within the owning domain
* `--ignore-change-password-upon-first-use <IGNORE_CHANGE_PASSWORD_UPON_FIRST_USE>`

  Possible values: `true`, `false`

* `--ignore-lockout-failure-attempts <IGNORE_LOCKOUT_FAILURE_ATTEMPTS>`

  Possible values: `true`, `false`

* `--ignore-password-expiry <IGNORE_PASSWORD_EXPIRY>`

  Possible values: `true`, `false`

* `--ignore-user-inactivity <IGNORE_USER_INACTIVITY>`

  Possible values: `true`, `false`

* `--lock-password <LOCK_PASSWORD>`

  Possible values: `true`, `false`

* `--multi-factor-auth-enabled <MULTI_FACTOR_AUTH_ENABLED>`

  Possible values: `true`, `false`

* `--multi-factor-auth-rules <MULTI_FACTOR_AUTH_RULES>`
* `--password <PASSWORD>` — The new password for the user



## `osc identity user delete`

Deletes a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/user`

**Usage:** `osc identity user delete <ID>`

###### **Arguments:**

* `<ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity user groups`

Lists groups to which a user belongs.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/user_groups`

**Usage:** `osc identity user groups <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity user list`

Lists users.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/users`

**Usage:** `osc identity user list [OPTIONS]`

###### **Options:**

* `--domain-id <DOMAIN_ID>` — Filters the response by a domain ID
* `--enabled <ENABLED>` — If set to true, then only enabled projects will be returned. Any value other than 0 (including no value) will be interpreted as true

  Possible values: `true`, `false`

* `--id <ID>` — Filter for Identity Providers’ ID attribute
* `--name <NAME>` — Filters the response by a resource name
* `--password-expires-at <PASSWORD_EXPIRES_AT>` — Filter results based on which user passwords have expired. The query should include an operator and a timestamp with a colon (:) separating the two, for example: `password_expires_at={operator}:{timestamp}`. Valid operators are: `lt`, `lte`, `gt`, `gte`, `eq`, and `neq`. Valid timestamps are of the form: YYYY-MM-DDTHH:mm:ssZ
* `--protocol-id <PROTOCOL_ID>` — Filters the response by a protocol ID
* `--unique-id <UNIQUE_ID>` — Filters the response by a unique ID



## `osc identity user password`

User password commands

This subcommand allows user to change the password

**Usage:** `osc identity user password <COMMAND>`

###### **Subcommands:**

* `set` — Change password for user



## `osc identity user password set`

Changes the password for a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/user_change_password`

**Usage:** `osc identity user password set [OPTIONS] <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--original-password <ORIGINAL_PASSWORD>` — The original password for the user
* `--password <PASSWORD>` — The new password for the user



## `osc identity user projects`

List projects to which the user has authorization to access.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/user_projects`

**Usage:** `osc identity user projects <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity user set`

Updates a user.

If the back-end driver does not support this functionality, this call might return the HTTP `Not Implemented (501)` response code.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/user`

**Usage:** `osc identity user set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--default-project-id <DEFAULT_PROJECT_ID>` — The ID of the default project for the user
* `--description <DESCRIPTION>`
* `--domain-id <DOMAIN_ID>` — The ID of the domain
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--federated <JSON>` — List of federated objects associated with a user. Each object in the list contains the `idp_id` and `protocols`. `protocols` is a list of objects, each of which contains `protocol_id` and `unique_id` of the protocol and user respectively. For example:

   ```text "federated": [ { "idp_id": "efbab5a6acad4d108fec6c63d9609d83", "protocols": [ {"protocol_id": "mapped", "unique_id": "test@example.com"} ] } ]

   ```
* `--name <NAME>` — The user name. Must be unique within the owning domain
* `--ignore-change-password-upon-first-use <IGNORE_CHANGE_PASSWORD_UPON_FIRST_USE>`

  Possible values: `true`, `false`

* `--ignore-lockout-failure-attempts <IGNORE_LOCKOUT_FAILURE_ATTEMPTS>`

  Possible values: `true`, `false`

* `--ignore-password-expiry <IGNORE_PASSWORD_EXPIRY>`

  Possible values: `true`, `false`

* `--ignore-user-inactivity <IGNORE_USER_INACTIVITY>`

  Possible values: `true`, `false`

* `--lock-password <LOCK_PASSWORD>`

  Possible values: `true`, `false`

* `--multi-factor-auth-enabled <MULTI_FACTOR_AUTH_ENABLED>`

  Possible values: `true`, `false`

* `--multi-factor-auth-rules <MULTI_FACTOR_AUTH_RULES>`
* `--password <PASSWORD>` — The new password for the user



## `osc identity user show`

Shows details for a user.

Relationship: `https://docs.openstack.org/api/openstack-identity/3/rel/user`

**Usage:** `osc identity user show <ID>`

###### **Arguments:**

* `<ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc image`

Image service operations

**Usage:** `osc image <COMMAND>`

###### **Subcommands:**

* `image` — Image commands
* `metadef` — Metadef commands
* `schema` — Schema commands



## `osc image image`

Image commands

**Usage:** `osc image image <COMMAND>`

###### **Subcommands:**

* `create` — Create image
* `deactivate` — Deactivate image
* `delete` — Delete image
* `download` — Download binary image data
* `list` — List images
* `reactivate` — Reactivate image
* `set` — Update image
* `show` — Show image
* `upload` — Upload binary image data



## `osc image image create`

Creates a catalog record for an operating system disk image. *(Since Image API v2.0)*

The `Location` response header contains the URI for the image.

A multiple store backend support is introduced in the Rocky release as a part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new header `OpenStack-image-store-ids` which contains the list of available stores will be included in response. This header is only included if multiple backend stores are supported.

The response body contains the new image entity.

Synchronous Postconditions

Normal response codes: 201

Error response codes: 400, 401, 403, 409, 413, 415

**Usage:** `osc image image create [OPTIONS]`

###### **Options:**

* `--container-format <CONTAINER_FORMAT>` — Format of the image container.

   Values may vary based on the configuration available in a particular OpenStack cloud. See the [Image Schema](#image-schema) response from the cloud itself for the valid values available.

   Example formats are: `ami`, `ari`, `aki`, `bare`, `ovf`, `ova`, or `docker`.

   The value might be `null` (JSON null data type).

  Possible values: `aki`, `ami`, `ari`, `bare`, `compressed`, `docker`, `ova`, `ovf`

* `--disk-format <DISK_FORMAT>` — The format of the disk.

   Values may vary based on the configuration available in a particular OpenStack cloud. See the [Image Schema](#image-schema) response from the cloud itself for the valid values available.

   Example formats are: `ami`, `ari`, `aki`, `vhd`, `vhdx`, `vmdk`, `raw`, `qcow2`, `vdi`, `ploop` or `iso`.

   The value might be `null` (JSON null data type).

   **Newton changes**: The `vhdx` disk format is a supported value.

   **Ocata changes**: The `ploop` disk format is a supported value.

  Possible values: `aki`, `ami`, `ari`, `iso`, `ploop`, `qcow2`, `raw`, `vdi`, `vhd`, `vhdx`, `vmdk`

* `--id <ID>` — A unique, user-defined image UUID, in the format:

   ```text nnnnnnnn-nnnn-nnnn-nnnn-nnnnnnnnnnnn

   ```

   Where **n** is a hexadecimal digit from 0 to f, or F.

   For example:

   ```text b2173dd3-7ad6-4362-baa6-a68bce3565cb

   ```

   If you omit this value, the API generates a UUID for the image. If you specify a value that has already been assigned, the request fails with a `409` response code.
* `--locations <JSON>` — A set of URLs to access the image file kept in external store
* `--min-disk <MIN_DISK>` — Amount of disk space in GB that is required to boot the image
* `--min-ram <MIN_RAM>` — Amount of RAM in MB that is required to boot the image
* `--name <NAME>` — The name of the image
* `--os-hidden <OS_HIDDEN>` — If true, image will not appear in default image list response

  Possible values: `true`, `false`

* `--owner <OWNER>` — Owner of the image
* `--protected <PROTECTED>` — Image protection for deletion. Valid value is `true` or `false`. Default is `false`

  Possible values: `true`, `false`

* `--tags <TAGS>` — List of tags for this image. Each tag is a string of at most 255 chars. The maximum number of tags allowed on an image is set by the operator
* `--visibility <VISIBILITY>` — Visibility for this image. Valid value is one of: `public`, `private`, `shared`, or `community`. At most sites, only an administrator can make an image `public`. Some sites may restrict what users can make an image `community`. Some sites may restrict what users can perform member operations on a `shared` image. *Since the Image API v2.5, the default value is `shared`.*

  Possible values: `community`, `private`, `public`, `shared`

* `--property <key=value>` — Additional properties to be sent with the request



## `osc image image deactivate`

Deactivates an image. (Since Image API v2.3)

By default, this operation is restricted to administrators only.

**Usage:** `osc image image deactivate [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/actions/deactivate API

###### **Options:**

* `--property <key=value>`



## `osc image image delete`

(Since Image API v2.0) Deletes an image.

You cannot delete images with the `protected` attribute set to `true` (boolean).

Preconditions

Synchronous Postconditions

Normal response codes: 204

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image image delete <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id} API



## `osc image image download`

Downloads binary image data. *(Since Image API v2.0)*

Example call: `curl -i -X GET -H "X-Auth-Token: $token" $image_url/v2/images/{image_id}/file`

The response body contains the raw binary data that represents the actual virtual disk. The `Content-Type` header contains the `application/octet-stream` value. The `Content-MD5` header contains an MD5 checksum of the image data. Use this checksum to verify the integrity of the image data.

**Preconditions**

**Synchronous Postconditions**

Normal response codes: 200, 204, 206

Error response codes: 400, 403, 404, 416

**Usage:** `osc image image download [OPTIONS] <IMAGE_ID>`

###### **Arguments:**

* `<IMAGE_ID>` — image_id parameter for /v2/images/{image_id}/file API

###### **Options:**

* `--file <FILE>` — Destination filename (using "-" will print object to stdout)



## `osc image image list`

Lists public virtual machine (VM) images. *(Since Image API v2.0)*

**Pagination**

Returns a subset of the larger collection of images and a link that you can use to get the next set of images. You should always check for the presence of a `next` link and use it as the URI in a subsequent HTTP GET request. You should follow this pattern until a `next` link is no longer provided.

The `next` link preserves any query parameters that you send in your initial request. You can use the `first` link to jump back to the first page of the collection. If you prefer to paginate through images manually, use the `limit` and `marker` parameters.

**Query Filters**

The list operation accepts query parameters to filter the response.

A client can provide direct comparison filters by using most image attributes, such as `name=Ubuntu`, `visibility=public`, and so on.

To filter using image tags, use the filter `tag` (note the singular). To filter on multiple tags, include each tag separately in the query. For example, to find images with the tag **ready**, include `tag=ready` in your query string. To find images tagged with **ready** and **approved**, include `tag=ready&tag=approved` in your query string. (Note that only images containing *both* tags will be included in the response.)

A client cannot use any `link` in the json-schema, such as self, file, or schema, to filter the response.

You can list VM images that have a status of `active`, `queued`, or `saving`.

**The** `in` **Operator**

As a convenience, you may specify several values for any of the following fields by using the `in` operator:

For most of these, usage is straight forward. For example, to list images in queued or saving status, use:

`GET /v2/images?status=in:saving,queued`

To find images in a particular list of image IDs, use:

`GET /v2/images?id=in:3afb79c1-131a-4c38-a87c-bc4b801d14e6,2e011209-660f-44b5-baf2-2eb4babae53d`

Using the `in` operator with the `name` property of images can be a bit trickier, depending upon how creatively you have named your images. The general rule is that if an image name contains a comma (`,`), you must enclose the entire name in quotation marks (`"`). As usual, you must URL encode any characters that require it.

For example, to find images named `glass, darkly` or `share me`, you would use the following filter specification:

`GET v2/images?name=in:"glass,%20darkly",share%20me`

As with regular filtering by name, you must specify the complete name you are looking for. Thus, for example, the query string `name=in:glass,share` will only match images with the exact name `glass` or the exact name `share`. It will not find an image named `glass, darkly` or an image named `share me`.

**Size Comparison Filters**

You can use the `size_min` and `size_max` query parameters to filter images that are greater than or less than the image size. The size, in bytes, is the size of an image on disk.

For example, to filter the container to include only images that are from 1 to 4 MB, set the `size_min` query parameter to `1048576` and the `size_max` query parameter to `4194304`.

**Time Comparison Filters**

You can use a *comparison operator* along with the `created_at` or `updated_at` fields to filter your results. Specify the operator first, a colon (`:`) as a separator, and then the time in [ISO 8601 Format](https://en.wikipedia.org/wiki/ISO_8601). Available comparison operators are:

For example:

**Sorting**

You can use query parameters to sort the results of this operation.

To sort the response, use the `sort_key` and `sort_dir` query parameters:

Alternatively, specify the `sort` query parameter:

Normal response codes: 200

Error response codes: 400, 401, 403

**Usage:** `osc image image list [OPTIONS]`

###### **Options:**

* `--created-at <CREATED_AT>` — Specify a comparison filter based on the date and time when the resource was created
* `--id <ID>` — id filter parameter
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--member-status <MEMBER_STATUS>` — Filters the response by a member status. A valid value is accepted, pending, rejected, or all. Default is accepted

  Possible values: `accepted`, `all`, `pending`, `rejected`

* `--name <NAME>` — Filters the response by a name, as a string. A valid value is the name of an image
* `--os-hidden <OS_HIDDEN>` — When true, filters the response to display only "hidden" images. By default, "hidden" images are not included in the image-list response. (Since Image API v2.7)

  Possible values: `true`, `false`

* `--owner <OWNER>` — Filters the response by a project (also called a “tenant”) ID. Shows only images that are shared with you by the specified owner
* `--protected <PROTECTED>` — Filters the response by the ‘protected’ image property. A valid value is one of ‘true’, ‘false’ (must be all lowercase). Any other value will result in a 400 response

  Possible values: `true`, `false`

* `--size-max <SIZE_MAX>` — Filters the response by a maximum image size, in bytes
* `--size-min <SIZE_MIN>` — Filters the response by a minimum image size, in bytes
* `--sort <SORT>` — Sorts the response by one or more attribute and sort direction combinations. You can also set multiple sort keys and directions. Default direction is desc. Use the comma (,) character to separate multiple values. For example: `sort=name:asc,status:desc`
* `--sort-dir <SORT_DIR>` — Sorts the response by a set of one or more sort direction and attribute (sort_key) combinations. A valid value for the sort direction is asc (ascending) or desc (descending). If you omit the sort direction in a set, the default is desc

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sorts the response by an attribute, such as name, id, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key image attribute
* `--status <STATUS>` — Filters the response by an image status
* `--tag <TAG>` — Filters the response by the specified tag value. May be repeated, but keep in mind that you're making a conjunctive query, so only images containing all the tags specified will appear in the response
* `--updated-at <UPDATED_AT>` — Specify a comparison filter based on the date and time when the resource was most recently modified
* `--visibility <VISIBILITY>` — Filters the response by an image visibility value. A valid value is public, private, community, shared, or all. (Note that if you filter on shared, the images included in the response will only be those where your member status is accepted unless you explicitly include a member_status filter in the request.) If you omit this parameter, the response shows public, private, and those shared images with a member status of accepted

  Possible values: `all`, `community`, `private`, `public`, `shared`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc image image reactivate`

Reactivates an image. (Since Image API v2.3)

By default, this operation is restricted to administrators only

**Usage:** `osc image image reactivate [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/actions/reactivate API

###### **Options:**

* `--property <key=value>`



## `osc image image set`

Updates an image. *(Since Image API v2.0)*

Conceptually, you update an image record by patching the JSON representation of the image, passing a request body conforming to one of the following media types:

Attempting to make a PATCH call using some other media type will provoke a response code of 415 (Unsupported media type).

The `application/openstack-images-v2.1-json-patch` media type provides a useful and compatible subset of the functionality defined in JavaScript Object Notation (JSON) Patch [RFC6902](http://tools.ietf.org/html/rfc6902), which defines the `application/json-patch+json` media type.

For information about the PATCH method and the available media types, see [Image API v2 HTTP PATCH media types](http://specs.openstack.org/openstack/glance-specs/specs/api/v2/http-patch-image-api-v2.html).

Attempting to modify some image properties will cause the entire request to fail with a 403 (Forbidden) response code:

Attempting to add a location path to an image that is not in `queued` or `active` state will result in a 409 (Conflict) response code *(since Image API v2.4)*.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 409, 413, 415

**Usage:** `osc image image set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id} API

###### **Options:**

* `--container-format <CONTAINER_FORMAT>` — Format of the image container.

   Values may vary based on the configuration available in a particular OpenStack cloud. See the [Image Schema](#image-schema) response from the cloud itself for the valid values available.

   Example formats are: `ami`, `ari`, `aki`, `bare`, `ovf`, `ova`, or `docker`.

   The value might be `null` (JSON null data type).

  Possible values: `aki`, `ami`, `ari`, `bare`, `compressed`, `docker`, `ova`, `ovf`

* `--disk-format <DISK_FORMAT>` — The format of the disk.

   Values may vary based on the configuration available in a particular OpenStack cloud. See the [Image Schema](#image-schema) response from the cloud itself for the valid values available.

   Example formats are: `ami`, `ari`, `aki`, `vhd`, `vhdx`, `vmdk`, `raw`, `qcow2`, `vdi`, `ploop` or `iso`.

   The value might be `null` (JSON null data type).

   **Newton changes**: The `vhdx` disk format is a supported value.

   **Ocata changes**: The `ploop` disk format is a supported value.

  Possible values: `aki`, `ami`, `ari`, `iso`, `ploop`, `qcow2`, `raw`, `vdi`, `vhd`, `vhdx`, `vmdk`

* `--locations <JSON>` — A list of objects, each of which describes an image location. Each object contains a `url` key, whose value is a URL specifying a location, and a `metadata` key, whose value is a dict of key:value pairs containing information appropriate to the use of whatever external store is indicated by the URL. *This list appears only if the* `show_multiple_locations` *option is set to* `true` *in the Image service’s configuration file.* **Because it presents a security risk, this option is disabled by default.**
* `--min-disk <MIN_DISK>` — Amount of disk space in GB that is required to boot the image. The value might be `null` (JSON null data type)
* `--min-ram <MIN_RAM>` — Amount of RAM in MB that is required to boot the image. The value might be `null` (JSON null data type)
* `--name <NAME>` — The name of the image. Value might be `null` (JSON null data type)
* `--os-hidden <OS_HIDDEN>` — This field controls whether an image is displayed in the default image-list response. A “hidden” image is out of date somehow (for example, it may not have the latest updates applied) and hence should not be a user’s first choice, but it’s not deleted because it may be needed for server rebuilds. By hiding it from the default image list, it’s easier for end users to find and use a more up-to-date version of this image. *(Since Image API v2.7)*

  Possible values: `true`, `false`

* `--owner <OWNER>` — An identifier for the owner of the image, usually the project (also called the “tenant”) ID. The value might be `null` (JSON null data type)
* `--protected <PROTECTED>` — A boolean value that must be `false` or the image cannot be deleted

  Possible values: `true`, `false`

* `--tags <TAGS>` — List of tags for this image, possibly an empty list
* `--visibility <VISIBILITY>` — Image visibility, that is, the access permission for the image

  Possible values: `community`, `private`, `public`, `shared`

* `--property <key=value>` — Additional properties to be sent with the request



## `osc image image show`

Shows details for an image. *(Since Image API v2.0)*

The response body contains a single image entity.

Preconditions

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc image image show <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id} API



## `osc image image upload`

Uploads binary image data. *(Since Image API v2.0)*

Set the `Content-Type` request header to `application/octet-stream`.

A multiple store backend support is introduced in the Rocky release as a part of the EXPERIMENTAL Image API v2.8.

Beginning with API version 2.8, an optional `X-Image-Meta-Store` header may be added to the request. When present, the image data will be placed into the backing store whose identifier is the value of this header. If the store identifier specified is not recognized, a 400 (Bad Request) response is returned. When the header is not present, the image data is placed into the default backing store.

Example call:

**Preconditions**

Before you can store binary image data, you must meet the following preconditions:

**Synchronous Postconditions**

**Troubleshooting**

Normal response codes: 204

Error response codes: 400, 401, 403, 404, 409, 410, 413, 415, 503

**Usage:** `osc image image upload [OPTIONS] <IMAGE_ID>`

###### **Arguments:**

* `<IMAGE_ID>` — image_id parameter for /v2/images/{image_id}/file API

###### **Options:**

* `--file <FILE>` — Source filename (using "-" will read object from stdout)



## `osc image metadef`

Metadef commands

**Usage:** `osc image metadef <COMMAND>`

###### **Subcommands:**

* `namespace` — Metadata definition namespaces
* `resource-type` — Metadata definition namespaces



## `osc image metadef namespace`

Metadata definition namespaces

Creates, lists, shows details for, updates, and deletes metadata definition namespaces. Defines namespaces that can contain property definitions, object definitions, and resource type associations.

**Usage:** `osc image metadef namespace <COMMAND>`

###### **Subcommands:**

* `create` — Create namespace
* `delete` — Delete namespace
* `list` — List namespaces
* `object` — Metadata definition objects
* `property` — Metadata definition properties
* `resource-type-association` — Metadata definition resource types
* `set` — Update namespace
* `show` — Get namespace details
* `tag` — Metadata definition tags



## `osc image metadef namespace create`

Creates a namespace.

A namespace must be unique across all users. Attempting to create an already existing namespace will result in a 409 (Conflict) response.

The `Location` response header contains the newly-created URI for the namespace.

Normal response codes: 201

Error response codes: 400, 401, 403, 409

**Usage:** `osc image metadef namespace create [OPTIONS] --namespace <NAMESPACE>`

###### **Options:**

* `--description <DESCRIPTION>` — The description of the namespace
* `--display-name <DISPLAY_NAME>` — User-friendly name to use in a UI to display the namespace name
* `--namespace <NAMESPACE>` — An identifier (a name) for the namespace. The value must be unique across all users
* `--objects <JSON>`
* `--owner <OWNER>` — Owner of the namespace
* `--properties <key=value>`
* `--protected <PROTECTED>` — Namespace protection for deletion. A valid value is `true` or `false`. Default is `false`

  Possible values: `true`, `false`

* `--resource-type-associations <JSON>`
* `--tags <TAGS>`
* `--visibility <VISIBILITY>` — The namespace visibility. A valid value is `public` or `private`. Default is `private`

  Possible values: `private`, `public`




## `osc image metadef namespace delete`

Deletes a namespace and its properties, objects, and any resource type associations.

A successful operation returns the HTTP `204` (No Content) response code.

Normal response codes: 204

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace delete <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name} API



## `osc image metadef namespace list`

Lists available namespaces.

Returns a list of namespaces to which the authenticated user has access. If the list is too large to fit in a single response, either because of operator configuration or because you’ve included a `limit` query parameter in the request to restrict the response size, the response will contain a link that you can use to get the next page of namespaces. Check for the presence of a `next` link and use it as the URI in a subsequent HTTP GET request. Follow this pattern until a `next` link is no longer provided.

The `next` link preserves any query parameters that you send in your initial request. You can use the `first` link to return to the first page in the collection. If you prefer to paginate through namespaces manually, use the `limit` and `marker` parameters.

The list operation accepts the `resource_types` and `visibility` query parameters, which you can use to filter the response.

To sort the results of this operation, use the `sort_key` and `sort_dir` parameters. The API uses the natural sorting order in the namespace attribute that you provide as the `sort_key` parameter.

Normal response codes: 200

Error response codes: 401, 403, 404

**Usage:** `osc image metadef namespace list`



## `osc image metadef namespace object`

Metadata definition objects

Creates, lists, shows details for, updates, and deletes metadata definition objects.

**Usage:** `osc image metadef namespace object <COMMAND>`

###### **Subcommands:**

* `create` — Create object
* `delete` — Delete object
* `list` — List objects
* `purge` — Command without description in OpenAPI
* `set` — Update object
* `show` — Show object



## `osc image metadef namespace object create`

Creates an object definition in a namespace.

Normal response codes: 201

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace object create [OPTIONS] --name <NAME> <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API

###### **Options:**

* `--description <DESCRIPTION>` — Detailed description of the object
* `--name <NAME>` — The name of the object, suitable for use as an identifier. A Name is limited to 80 chars in length
* `--properties <key=value>` — A set of key:value pairs, where each value is a *property* entity
* `--required <REQUIRED>` — A list of the names of properties that are required on this object



## `osc image metadef namespace object delete`

Deletes an object definition from a namespace.

When you successfully delete an object from a namespace, the response is empty and the response code is `204`.

Normal response codes: 204

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace object delete <NAMESPACE_NAME> <OBJECT_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API
* `<OBJECT_NAME>` — object_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API



## `osc image metadef namespace object list`

Lists object definitions in a namespace.

Returns a subset of the larger collection of namespaces and a link that you can use to get the next set of namespaces. You should always check for the presence of a `next` link and use it as the URI in a subsequent HTTP GET request. You should follow this pattern until a `next` link is no longer provided. The next link preserves any query parameters that you send in your initial request. You can use the `first` link to jump back to the first page of the collection. If you prefer to paginate through namespaces manually, use the `limit` and `marker` parameters.

Use the `resource_types` and `visibility` query parameters to filter the response.

For example, set the `resource_types` query parameter to `OS::Glance::Image,OS::Nova::Flavor` to filter the response to include only namespaces that are associated with the given resource types.

You can sort the results of this operation by using the `sort_key` and `sort_dir` parameters. The API uses the natural sorting of whatever namespace attribute is provided as the `sort_key`.

Normal response codes: 200

Error response codes: 401, 403, 404

**Usage:** `osc image metadef namespace object list <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API



## `osc image metadef namespace object purge`

Command without description in OpenAPI

**Usage:** `osc image metadef namespace object purge <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API



## `osc image metadef namespace object set`

Updates an object definition in a namespace.

The object resource is completely replaced by what you specify in the request body. Thus, if you leave out any of the optional parameters, and they exist in the current object, they will be eliminated by this call.

It is possible to change the name of the object with this call; if you do, note that the URL for the object (specified by the `self` field) will change.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace object set [OPTIONS] --name <NAME> <NAMESPACE_NAME> <OBJECT_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API
* `<OBJECT_NAME>` — object_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API

###### **Options:**

* `--description <DESCRIPTION>` — Detailed description of the object
* `--name <NAME>` — The name of the object, suitable for use as an identifier. A Name is limited to 80 chars in length
* `--properties <key=value>` — A set of key:value pairs, where each value is a *property* entity
* `--required <REQUIRED>` — A list of the names of properties that are required on this object



## `osc image metadef namespace object show`

Shows the definition for an object.

The response body shows a single object entity.

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace object show <NAMESPACE_NAME> <OBJECT_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API
* `<OBJECT_NAME>` — object_name parameter for /v2/metadefs/namespaces/{namespace_name}/objects/{object_name} API



## `osc image metadef namespace property`

Metadata definition properties

Creates, lists, shows details for, updates, and deletes metadata definition properties.

**Usage:** `osc image metadef namespace property <COMMAND>`

###### **Subcommands:**

* `create` — Create property
* `delete` — Remove property definition
* `list` — List properties
* `purge` — Command without description in OpenAPI
* `set` — Update property definition
* `show` — Show property definition



## `osc image metadef namespace property create`

Creates a property definition in a namespace.

The schema is a subset of the JSON property definition schema.

Normal response codes: 201

Error response codes: 400, 404

**Usage:** `osc image metadef namespace property create [OPTIONS] --name <NAME> --title <TITLE> --type <TYPE> <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API

###### **Options:**

* `--additional-items <ADDITIONAL_ITEMS>` — Describes extra items, if you use tuple typing. If the value of `items` is an array (tuple typing) and the instance is longer than the list of schemas in `items`, the additional items are described by the schema in this property. If this value is `false`, the instance cannot be longer than the list of schemas in `items`. If this value is `true`, that is equivalent to the empty schema (anything goes)

  Possible values: `true`, `false`

* `--default <JSON>`
* `--description <DESCRIPTION>` — Detailed description of the property
* `--enum <ENUM>` — Enumerated list of property values
* `--items <JSON>` — Schema for the items in an array
* `--maximum <MAXIMUM>` — Maximum allowed numerical value
* `--max-items <MAX_ITEMS>` — Maximum allowed string length
* `--max-length <MAX_LENGTH>` — Maximum allowed string length
* `--minimum <MINIMUM>` — Minimum allowed numerical value
* `--min-items <MIN_ITEMS>` — Minimum allowed string length
* `--min-length <MIN_LENGTH>` — Minimum allowed string length
* `--name <NAME>`
* `--operators <OPERATORS>` — Operators property description
* `--pattern <PATTERN>` — A regular expression ( [ECMA 262](http://www.ecma-international.org/publications/standards/Ecma-262.htm) ) that a string value must match
* `--readonly <READONLY>` — Indicates whether this is a read-only property

  Possible values: `true`, `false`

* `--required <REQUIRED>`
* `--title <TITLE>` — The title of the property
* `--type <TYPE>` — The property type

  Possible values: `array`, `boolean`, `integer`, `number`, `object`, `string`

* `--unique-items <UNIQUE_ITEMS>` — Indicates whether all values in the array must be distinct

  Possible values: `true`, `false`




## `osc image metadef namespace property delete`

Removes a property definition from a namespace.

When you successfully delete a property from a namespace, the response is empty and the response code is `204`.

Normal response codes: 204

Error response codes: 401, 403, 404

**Usage:** `osc image metadef namespace property delete <NAMESPACE_NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
* `<PROPERTY_NAME>` — property_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API



## `osc image metadef namespace property list`

Lists property definitions in a namespace.

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace property list <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API



## `osc image metadef namespace property purge`

Command without description in OpenAPI

**Usage:** `osc image metadef namespace property purge <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API



## `osc image metadef namespace property set`

Updates a property definition.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace property set [OPTIONS] --name <NAME> --title <TITLE> --type <TYPE> <NAMESPACE_NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
* `<PROPERTY_NAME>` — property_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API

###### **Options:**

* `--additional-items <ADDITIONAL_ITEMS>` — Describes extra items, if you use tuple typing. If the value of `items` is an array (tuple typing) and the instance is longer than the list of schemas in `items`, the additional items are described by the schema in this property. If this value is `false`, the instance cannot be longer than the list of schemas in `items`. If this value is `true`, that is equivalent to the empty schema (anything goes)

  Possible values: `true`, `false`

* `--default <JSON>`
* `--description <DESCRIPTION>` — The description of the namespace
* `--enum <ENUM>` — Enumerated list of property values
* `--items <JSON>` — Schema for the items in an array
* `--maximum <MAXIMUM>` — Maximum allowed numerical value
* `--max-items <MAX_ITEMS>` — Maximum allowed string length
* `--max-length <MAX_LENGTH>` — Maximum allowed string length
* `--minimum <MINIMUM>` — Minimum allowed numerical value
* `--min-items <MIN_ITEMS>` — Minimum allowed string length
* `--min-length <MIN_LENGTH>` — Minimum allowed string length
* `--name <NAME>` — The name of the property. A Name is limited to 80 chars in length
* `--operators <OPERATORS>` — Operators property description
* `--pattern <PATTERN>` — A regular expression ( [ECMA 262](http://www.ecma-international.org/publications/standards/Ecma-262.htm) ) that a string value must match
* `--readonly <READONLY>` — Indicates whether this is a read-only property

  Possible values: `true`, `false`

* `--required <REQUIRED>`
* `--title <TITLE>` — The title of the property
* `--type <TYPE>` — The property type

  Possible values: `array`, `boolean`, `integer`, `number`, `object`, `string`

* `--unique-items <UNIQUE_ITEMS>` — Indicates whether all values in the array must be distinct

  Possible values: `true`, `false`




## `osc image metadef namespace property show`

Shows the definition for a property.

If you use the `resource_type` query parameter, the API removes the prefix of the resource type from the property name before it submits the query. This enables you to look for a property name that starts with a prefix from an associated resource type.

The response body shows a single property entity.

Normal response codes: 200

Error response codes: 401, 403, 404

**Usage:** `osc image metadef namespace property show <NAMESPACE_NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
* `<PROPERTY_NAME>` — property_name parameter for /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API



## `osc image metadef namespace resource-type-association`

Metadata definition resource types

Lists resource types. Also, creates, lists, and removes resource type associations in a namespace.

**Usage:** `osc image metadef namespace resource-type-association <COMMAND>`

###### **Subcommands:**

* `create` — Create resource type association
* `delete` — Command without description in OpenAPI
* `list` — List resource type associations



## `osc image metadef namespace resource-type-association create`

Creates a resource type association between a namespace and the resource type specified in the body of the request.

Normal response codes: 201

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace resource-type-association create [OPTIONS] --name <NAME> <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/resource_types/{resource_type} API

###### **Options:**

* `--name <NAME>` — Resource type names should be aligned with Heat resource types whenever possible: https://docs.openstack.org/heat/latest/template_guide/openstack.html
* `--prefix <PREFIX>` — Prefix for any properties in the namespace that you want to apply to the resource type. If you specify a prefix, you must append a prefix separator, such as the colon (`:`) character
* `--properties-target <PROPERTIES_TARGET>` — Some resource types allow more than one key and value pair for each instance. For example, the Image service allows both user and image metadata on volumes. The `properties_target` parameter enables a namespace target to remove the ambiguity



## `osc image metadef namespace resource-type-association delete`

Command without description in OpenAPI

**Usage:** `osc image metadef namespace resource-type-association delete <NAMESPACE_NAME> <RESOURCE_TYPE>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/resource_types/{resource_type} API
* `<RESOURCE_TYPE>` — resource_type parameter for /v2/metadefs/namespaces/{namespace_name}/resource_types/{resource_type} API



## `osc image metadef namespace resource-type-association list`

Lists resource type associations in a namespace.

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace resource-type-association list <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/resource_types/{resource_type} API



## `osc image metadef namespace set`

Updates a namespace.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace set [OPTIONS] --namespace <NAMESPACE> <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name} API

###### **Options:**

* `--description <DESCRIPTION>` — The description of the namespace
* `--display-name <DISPLAY_NAME>` — User-friendly name to use in a UI to display the namespace name
* `--namespace <NAMESPACE>` — An identifier (a name) for the namespace. The value must be unique across all users
* `--objects <JSON>`
* `--owner <OWNER>` — Owner of the namespace
* `--properties <key=value>`
* `--protected <PROTECTED>` — Namespace protection for deletion. A valid value is `true` or `false`. Default is `false`

  Possible values: `true`, `false`

* `--resource-type-associations <JSON>`
* `--tags <TAGS>`
* `--visibility <VISIBILITY>` — The namespace visibility. A valid value is `public` or `private`. Default is `private`

  Possible values: `private`, `public`




## `osc image metadef namespace show`

Gets details for a namespace.

The response body shows a single namespace entity with all details including properties, objects, and resource type associations.

If the namespace contains a resource type association that specifies a prefix, you may optionally include the name of the resource type as a query parameter. In that case, the prefix will be applied to all property names in the response. (See below for an example.)

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace show <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name} API



## `osc image metadef namespace tag`

Metadata definition tags

Creates, lists, shows details for, updates, and deletes metadata definition tags.

**Usage:** `osc image metadef namespace tag <COMMAND>`

###### **Subcommands:**

* `create` — Create tag definition
* `delete` — Delete tag definition
* `list` — List tags
* `purge` — Delete all tag definitions
* `set` — Update tag definition
* `show` — Get tag definition



## `osc image metadef namespace tag create`

Adds a tag to the list of namespace tag definitions.

Normal response codes: 201

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace tag create --name <NAME> <NAMESPACE_NAME> <TAG_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API
* `<TAG_NAME>` — tag_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API

###### **Options:**

* `--name <NAME>`



## `osc image metadef namespace tag delete`

Deletes a tag definition within a namespace.

When you successfully delete a tag from a namespace, the response is empty and the response code is `204`.

Normal response codes: 204

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace tag delete <NAMESPACE_NAME> <TAG_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API
* `<TAG_NAME>` — tag_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API



## `osc image metadef namespace tag list`

Lists the tag definitions within a namespace.

To manually paginate through the list of tags, use the `limit` and `marker` parameters.

To sort the results of this operation use the `sort_key` and `sort_dir` parameters. The API uses the natural sort order of the tag attribute of the `sort_key` parameter.

Normal response codes: 200

Error response codes: 401, 403, 404

**Usage:** `osc image metadef namespace tag list <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API



## `osc image metadef namespace tag purge`

Deletes all tag definitions within a namespace.

When you successfully delete the tags from a namespace, the response is empty and the response code is `204`.

Normal response codes: 204

Error response codes: 403, 404

**Usage:** `osc image metadef namespace tag purge <NAMESPACE_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API



## `osc image metadef namespace tag set`

Renames a tag definition.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc image metadef namespace tag set --name <NAME> <NAMESPACE_NAME> <TAG_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API
* `<TAG_NAME>` — tag_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API

###### **Options:**

* `--name <NAME>` — The name of the tag. A Name is limited to 80 chars in length



## `osc image metadef namespace tag show`

Gets a definition for a tag.

The response body shows a single tag entity.

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc image metadef namespace tag show <NAMESPACE_NAME> <TAG_NAME>`

###### **Arguments:**

* `<NAMESPACE_NAME>` — namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API
* `<TAG_NAME>` — tag_name parameter for /v2/metadefs/namespaces/{namespace_name}/tags/{tag_name} API



## `osc image metadef resource-type`

Metadata definition namespaces

Creates, lists, shows details for, updates, and deletes metadata definition namespaces. Defines namespaces that can contain property definitions, object definitions, and resource type associations.

**Usage:** `osc image metadef resource-type <COMMAND>`

###### **Subcommands:**

* `list` — List resource types



## `osc image metadef resource-type list`

Lists all available resource types.

Using the other API calls in this section, you can create and maintain *resource type associations* between metadata definition namespaces and the resource types that are returned by this call.

Normal response codes: 200

Error response codes: 400, 401, 404

**Usage:** `osc image metadef resource-type list`



## `osc image schema`

Schema commands

**Usage:** `osc image schema <COMMAND>`

###### **Subcommands:**

* `image` — Show Image Schema
* `images` — Show Images Schema
* `member` — Show Member Schema
* `members` — Show Members Schema
* `metadef` — Metadata definition schemas



## `osc image schema image`

Show Image Schema

**Usage:** `osc image schema image <COMMAND>`

###### **Subcommands:**

* `show` — Show Image Schema



## `osc image schema image show`

Show Image Schema

**Usage:** `osc image schema image show`



## `osc image schema images`

Show Images Schema

**Usage:** `osc image schema images <COMMAND>`

###### **Subcommands:**

* `show` — Show Images Schema



## `osc image schema images show`

Show Images Schema

**Usage:** `osc image schema images show`



## `osc image schema member`

Show Member Schema

**Usage:** `osc image schema member <COMMAND>`

###### **Subcommands:**

* `show` — Show Member Schema



## `osc image schema member show`

Show Member Schema

**Usage:** `osc image schema member show`



## `osc image schema members`

Show Members Schema

**Usage:** `osc image schema members <COMMAND>`

###### **Subcommands:**

* `show` — Show Members Schema



## `osc image schema members show`

Show Members Schema

**Usage:** `osc image schema members show`



## `osc image schema metadef`

Metadata definition schemas

Gets a JSON-schema document that represents a metadata definition entity.

**Usage:** `osc image schema metadef <COMMAND>`

###### **Subcommands:**

* `namespace` — Metadef Namespace Schema operations
* `namespaces` — Metadef Namespaces Schema operations
* `object` — Metadef Object Schema operations
* `objects` — Metadef Objects Schema operations
* `properties` — Metadef Properties Schema operations
* `property` — Metadef Property Schema operations
* `resource-type` — Metadef ResourceType Schema operations
* `resource-types` — Metadef ResourceTypes Schema operations
* `tag` — Metadef Tag Schema operations
* `tags` — Metadef Tags Schema operations



## `osc image schema metadef namespace`

Metadef Namespace Schema operations

**Usage:** `osc image schema metadef namespace <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition namespace schema



## `osc image schema metadef namespace show`

Shows a JSON schema document that represents a metadata definition *namespace* entity.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef namespace show`



## `osc image schema metadef namespaces`

Metadef Namespaces Schema operations

**Usage:** `osc image schema metadef namespaces <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition namespaces schema



## `osc image schema metadef namespaces show`

Shows a JSON schema document that represents a metadata definition *namespaces* entity.

A namespaces entity is a container for *namespace* entities.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef namespaces show`



## `osc image schema metadef object`

Metadef Object Schema operations

**Usage:** `osc image schema metadef object <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition object schema



## `osc image schema metadef object show`

Shows a JSON schema document that represents a metadata definition *object* entity.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef object show`



## `osc image schema metadef objects`

Metadef Objects Schema operations

**Usage:** `osc image schema metadef objects <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition objects schema



## `osc image schema metadef objects show`

Shows a JSON schema document that represents a metadata definition *objects* entity.

An objects entity is a container for *object* entities.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef objects show`



## `osc image schema metadef properties`

Metadef Properties Schema operations

**Usage:** `osc image schema metadef properties <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition properties schema



## `osc image schema metadef properties show`

Shows a JSON schema document that represents a metadata definition *properties* entity.

A properties entity is a container for *property* entities.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200 Error response codes: 400, 401

**Usage:** `osc image schema metadef properties show`



## `osc image schema metadef property`

Metadef Property Schema operations

**Usage:** `osc image schema metadef property <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition property schema



## `osc image schema metadef property show`

Shows a JSON schema document that represents a metadata definition *property* entity.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef property show`



## `osc image schema metadef resource-type`

Metadef ResourceType Schema operations

**Usage:** `osc image schema metadef resource-type <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition namespace resource type association schema



## `osc image schema metadef resource-type show`

Shows a JSON schema document that represents a metadata definition namespace *resource type association* entity.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef resource-type show`



## `osc image schema metadef resource-types`

Metadef ResourceTypes Schema operations

**Usage:** `osc image schema metadef resource-types <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition namespace resource type associations schema



## `osc image schema metadef resource-types show`

Shows a JSON schema document that represents a metadata definition namespace *resource type associations* entity.

A resource type associations entity is a container for *resource type association* entities.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef resource-types show`



## `osc image schema metadef tag`

Metadef Tag Schema operations

**Usage:** `osc image schema metadef tag <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition tag schema



## `osc image schema metadef tag show`

Shows a JSON schema document that represents a metadata definition *tag* entity.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef tag show`



## `osc image schema metadef tags`

Metadef Tags Schema operations

**Usage:** `osc image schema metadef tags <COMMAND>`

###### **Subcommands:**

* `show` — Show metadata definition tags schema



## `osc image schema metadef tags show`

Shows a JSON schema document that represents a metadata definition *tags* entity.

A tags entity is a container for *tag* entities.

The following schema document is an example. The authoritative response is the actual response to the API call.

Normal response codes: 200

Error response codes: 400, 401

**Usage:** `osc image schema metadef tags show`



## `osc load-balancer`

Load Balancer service operations

**Usage:** `osc load-balancer <COMMAND>`

###### **Subcommands:**

* `amphorae` — Amphorae (Octavia) commands
* `availability-zone` — AvailabilityZone (Octavia) commands
* `availability-zone-profile` — AvailabilityZoneProfile (Octavia) commands
* `flavor` — Flavor (Octavia) commands
* `flavor-profile` — FlavorProfile (Octavia) commands
* `healthmonitor` — Healthmonitor (Octavia) commands
* `l7policy` — L7Policy (Octavia) commands
* `listener` — Listener (Octavia) commands
* `loadbalancer` — Loadbalancer (Octavia) commands
* `pool` — Pool (Octavia) commands
* `provider` — Provider (Octavia) commands
* `quota` — Quota commands
* `version` — Version (Octavia) commands



## `osc load-balancer amphorae`

Amphorae (Octavia) commands

**Usage:** `osc load-balancer amphorae <COMMAND>`

###### **Subcommands:**

* `config` — Configure Amphora
* `delete` — Remove an Amphora
* `failover` — Failover Amphora
* `list` — List Amphora
* `show` — Show Amphora details
* `stats` — Show Amphora Statistics



## `osc load-balancer amphorae config`

Configure Amphora

**Usage:** `osc load-balancer amphorae config [OPTIONS] <AMPHORA_ID>`

###### **Arguments:**

* `<AMPHORA_ID>` — amphora_id parameter for /v2/octavia/amphorae/{amphora_id}/config API

###### **Options:**

* `--property <key=value>`



## `osc load-balancer amphorae delete`

Removes an amphora and its associated configuration.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**New in version 2.20**

**Usage:** `osc load-balancer amphorae delete <AMPHORA_ID>`

###### **Arguments:**

* `<AMPHORA_ID>` — amphora_id parameter for /v2/octavia/amphorae/{amphora_id} API



## `osc load-balancer amphorae failover`

Failover Amphora

**Usage:** `osc load-balancer amphorae failover [OPTIONS] <AMPHORA_ID>`

###### **Arguments:**

* `<AMPHORA_ID>` — amphora_id parameter for /v2/octavia/amphorae/{amphora_id}/failover API

###### **Options:**

* `--property <key=value>`



## `osc load-balancer amphorae list`

Lists all amphora for the project.

If you are not an administrative user, the service returns the HTTP `Forbidden (403)` response code.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

The list might be empty.

**Usage:** `osc load-balancer amphorae list`



## `osc load-balancer amphorae show`

Shows the details of an amphora.

If you are not an administrative user, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer amphorae show <AMPHORA_ID>`

###### **Arguments:**

* `<AMPHORA_ID>` — amphora_id parameter for /v2/octavia/amphorae/{amphora_id} API



## `osc load-balancer amphorae stats`

Show the statistics for an amphora.

If you are not an administrative user, the service returns the HTTP `Forbidden (403)` response code.

Use the `fields` query parameter to control which fields are returned in the response body.

**New in version 2.3**

**Usage:** `osc load-balancer amphorae stats <AMPHORA_ID>`

###### **Arguments:**

* `<AMPHORA_ID>` — amphora_id parameter for /v2/octavia/amphorae/{amphora_id}/stats API



## `osc load-balancer availability-zone`

AvailabilityZone (Octavia) commands

**Usage:** `osc load-balancer availability-zone <COMMAND>`

###### **Subcommands:**

* `create` — Creates an Availability Zone
* `delete` — Deletes an Availability Zone
* `list` — Lists all Availability Zones
* `set` — Command without description in OpenAPI
* `show` — Gets an Availability Zone's detail



## `osc load-balancer availability-zone create`

Creates an Availability Zone

**Usage:** `osc load-balancer availability-zone create [OPTIONS] --availability-zone-profile-id <AVAILABILITY_ZONE_PROFILE_ID> --name <NAME>`

###### **Options:**

* `--availability-zone-profile-id <AVAILABILITY_ZONE_PROFILE_ID>`
* `--description <DESCRIPTION>`
* `--enabled <ENABLED>`

  Possible values: `true`, `false`

* `--name <NAME>`



## `osc load-balancer availability-zone delete`

Deletes an Availability Zone

**Usage:** `osc load-balancer availability-zone delete <ID>`

###### **Arguments:**

* `<ID>` — availabilityzone_id parameter for /v2/lbaas/availabilityzones/{availabilityzone_id} API



## `osc load-balancer availability-zone list`

Lists all Availability Zones

**Usage:** `osc load-balancer availability-zone list`



## `osc load-balancer availability-zone set`

Command without description in OpenAPI

**Usage:** `osc load-balancer availability-zone set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — availabilityzone_id parameter for /v2/lbaas/availabilityzones/{availabilityzone_id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--enabled <ENABLED>`

  Possible values: `true`, `false`




## `osc load-balancer availability-zone show`

Gets an Availability Zone's detail

**Usage:** `osc load-balancer availability-zone show <ID>`

###### **Arguments:**

* `<ID>` — availabilityzone_id parameter for /v2/lbaas/availabilityzones/{availabilityzone_id} API



## `osc load-balancer availability-zone-profile`

AvailabilityZoneProfile (Octavia) commands

**Usage:** `osc load-balancer availability-zone-profile <COMMAND>`

###### **Subcommands:**

* `create` — Creates an Availability Zone Profile
* `delete` — Deletes an Availability Zone Profile
* `list` — Lists all Availability Zone Profiles
* `set` — Updates an Availability Zone Profile
* `show` — Gets an Availability Zone Profile's detail



## `osc load-balancer availability-zone-profile create`

Creates an Availability Zone Profile

**Usage:** `osc load-balancer availability-zone-profile create --availability-zone-data <AVAILABILITY_ZONE_DATA> --name <NAME> --provider-name <PROVIDER_NAME>`

###### **Options:**

* `--availability-zone-data <AVAILABILITY_ZONE_DATA>`
* `--name <NAME>`
* `--provider-name <PROVIDER_NAME>`



## `osc load-balancer availability-zone-profile delete`

Deletes an Availability Zone Profile

**Usage:** `osc load-balancer availability-zone-profile delete <ID>`

###### **Arguments:**

* `<ID>` — availabilityzoneprofile_id parameter for /v2/lbaas/availabilityzoneprofiles/{availabilityzoneprofile_id} API



## `osc load-balancer availability-zone-profile list`

Lists all Availability Zone Profiles

**Usage:** `osc load-balancer availability-zone-profile list`



## `osc load-balancer availability-zone-profile set`

Updates an Availability Zone Profile

**Usage:** `osc load-balancer availability-zone-profile set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — availabilityzoneprofile_id parameter for /v2/lbaas/availabilityzoneprofiles/{availabilityzoneprofile_id} API

###### **Options:**

* `--availability-zone-data <AVAILABILITY_ZONE_DATA>`
* `--name <NAME>`
* `--provider-name <PROVIDER_NAME>`



## `osc load-balancer availability-zone-profile show`

Gets an Availability Zone Profile's detail

**Usage:** `osc load-balancer availability-zone-profile show <ID>`

###### **Arguments:**

* `<ID>` — availabilityzoneprofile_id parameter for /v2/lbaas/availabilityzoneprofiles/{availabilityzoneprofile_id} API



## `osc load-balancer flavor`

Flavor (Octavia) commands

**Usage:** `osc load-balancer flavor <COMMAND>`

###### **Subcommands:**

* `create` — Creates a flavor
* `delete` — Deletes a Flavor
* `list` — Lists all flavors
* `set` — Command without description in OpenAPI
* `show` — Gets a flavor's detail



## `osc load-balancer flavor create`

Creates a flavor

**Usage:** `osc load-balancer flavor create [OPTIONS] --flavor-profile-id <FLAVOR_PROFILE_ID> --name <NAME>`

###### **Options:**

* `--description <DESCRIPTION>`
* `--enabled <ENABLED>`

  Possible values: `true`, `false`

* `--flavor-profile-id <FLAVOR_PROFILE_ID>`
* `--name <NAME>`



## `osc load-balancer flavor delete`

Deletes a Flavor

**Usage:** `osc load-balancer flavor delete <ID>`

###### **Arguments:**

* `<ID>` — flavor_id parameter for /v2/lbaas/flavors/{flavor_id} API



## `osc load-balancer flavor list`

Lists all flavors

**Usage:** `osc load-balancer flavor list`



## `osc load-balancer flavor set`

Command without description in OpenAPI

**Usage:** `osc load-balancer flavor set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — flavor_id parameter for /v2/lbaas/flavors/{flavor_id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--enabled <ENABLED>`

  Possible values: `true`, `false`

* `--name <NAME>`



## `osc load-balancer flavor show`

Gets a flavor's detail

**Usage:** `osc load-balancer flavor show <ID>`

###### **Arguments:**

* `<ID>` — flavor_id parameter for /v2/lbaas/flavors/{flavor_id} API



## `osc load-balancer flavor-profile`

FlavorProfile (Octavia) commands

**Usage:** `osc load-balancer flavor-profile <COMMAND>`

###### **Subcommands:**

* `create` — Creates a flavor Profile
* `delete` — Deletes a Flavor Profile
* `list` — Lists all flavor profiles
* `set` — Updates a flavor Profile
* `show` — Gets a flavor profile's detail



## `osc load-balancer flavor-profile create`

Creates a flavor Profile

**Usage:** `osc load-balancer flavor-profile create --flavor-data <FLAVOR_DATA> --name <NAME> --provider-name <PROVIDER_NAME>`

###### **Options:**

* `--flavor-data <FLAVOR_DATA>`
* `--name <NAME>`
* `--provider-name <PROVIDER_NAME>`



## `osc load-balancer flavor-profile delete`

Deletes a Flavor Profile

**Usage:** `osc load-balancer flavor-profile delete <ID>`

###### **Arguments:**

* `<ID>` — flavorprofile_id parameter for /v2/lbaas/flavorprofiles/{flavorprofile_id} API



## `osc load-balancer flavor-profile list`

Lists all flavor profiles

**Usage:** `osc load-balancer flavor-profile list`



## `osc load-balancer flavor-profile set`

Updates a flavor Profile

**Usage:** `osc load-balancer flavor-profile set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — flavorprofile_id parameter for /v2/lbaas/flavorprofiles/{flavorprofile_id} API

###### **Options:**

* `--flavor-data <FLAVOR_DATA>`
* `--name <NAME>`
* `--provider-name <PROVIDER_NAME>`



## `osc load-balancer flavor-profile show`

Gets a flavor profile's detail

**Usage:** `osc load-balancer flavor-profile show <ID>`

###### **Arguments:**

* `<ID>` — flavorprofile_id parameter for /v2/lbaas/flavorprofiles/{flavorprofile_id} API



## `osc load-balancer healthmonitor`

Healthmonitor (Octavia) commands

**Usage:** `osc load-balancer healthmonitor <COMMAND>`

###### **Subcommands:**

* `create` — Create Health Monitor
* `delete` — Remove a Health Monitor
* `list` — List Health Monitors
* `set` — Update a Health Monitor
* `show` — Show Health Monitor details



## `osc load-balancer healthmonitor create`

Creates a health monitor on a pool.

Health monitors define how the load balancer monitors backend servers to determine if they are available to service requests.

This operation provisions a new health monitor by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, the API returns a response object that contains a unique ID and the status of provisioning the health monitor.

In the response, the health monitor [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/healthmonitors/{healthmonitor_id}` to view the progress of the provisioning operation. When the health monitor status changes to `ACTIVE`, the health monitor is successfully provisioned and is ready for further configuration.

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

Specifying a project_id is deprecated. The health monitor will inherit the project_id of the parent load balancer.

At a minimum, you must specify these health monitor attributes:

Some attributes receive default values if you omit them from the request:

To create a health monitor, the parent load balancer must have an `ACTIVE` provisioning status.

**Usage:** `osc load-balancer healthmonitor create [OPTIONS] --delay <DELAY> --max-retries <MAX_RETRIES> --pool-id <POOL_ID> --timeout <TIMEOUT> --type <TYPE>`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--delay <DELAY>` — The time, in seconds, between sending probes to members
* `--domain-name <DOMAIN_NAME>` — The domain name, which be injected into the HTTP Host Header to the backend server for HTTP health check.

   **New in version 2.10**
* `--expected-codes <EXPECTED_CODES>` — The list of HTTP status codes expected in response from the member to declare it healthy. Specify one of the following values:

   - A single value, such as `200` - A list, such as `200, 202` - A range, such as `200-204`

   The default is 200.
* `--http-method <HTTP_METHOD>` — The HTTP method that the health monitor uses for requests. One of `CONNECT`, `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT`, or `TRACE`. The default is `GET`

  Possible values: `connect`, `delete`, `get`, `head`, `options`, `patch`, `post`, `put`, `trace`

* `--http-version <HTTP_VERSION>` — The HTTP version. One of `1.0` or `1.1`. The default is `1.0`.

   **New in version 2.10**
* `--max-retries <MAX_RETRIES>` — The number of successful checks before changing the `operating status` of the member to `ONLINE`. A valid value is from `1` to `10`
* `--max-retries-down <MAX_RETRIES_DOWN>` — The number of allowed check failures before changing the `operating status` of the member to `ERROR`. A valid value is from `1` to `10`. The default is `3`
* `--name <NAME>` — Human-readable name of the resource
* `--pool-id <POOL_ID>` — The ID of the pool
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource. (deprecated)
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--tenant-id <TENANT_ID>`
* `--timeout <TIMEOUT>` — The maximum time, in seconds, that a monitor waits to connect before it times out. This value must be less than the delay value
* `--type <TYPE>` — The type of health monitor. One of `HTTP`, `HTTPS`, `PING`, `SCTP`, `TCP`, `TLS-HELLO`, or `UDP-CONNECT`

  Possible values: `http`, `https`, `ping`, `sctp`, `tcp`, `tls-hello`, `udp-connect`

* `--url-path <URL_PATH>` — The HTTP URL path of the request sent by the monitor to test the health of a backend member. Must be a string that begins with a forward slash (`/`). The default URL path is `/`



## `osc load-balancer healthmonitor delete`

Removes a health monitor and its associated configuration from the project.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer healthmonitor delete <ID>`

###### **Arguments:**

* `<ID>` — healthmonitor_id parameter for /v2/lbaas/healthmonitors/{healthmonitor_id} API



## `osc load-balancer healthmonitor list`

Lists all health monitors for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list health monitors for other projects.

The list might be empty.

**Usage:** `osc load-balancer healthmonitor list`



## `osc load-balancer healthmonitor set`

Update an existing health monitor.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the health monitor provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the health monitor object for changes.

This operation returns the updated health monitor object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

**Usage:** `osc load-balancer healthmonitor set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — healthmonitor_id parameter for /v2/lbaas/healthmonitors/{healthmonitor_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--delay <DELAY>` — The time, in seconds, between sending probes to members
* `--domain-name <DOMAIN_NAME>` — The domain name, which be injected into the HTTP Host Header to the backend server for HTTP health check.

   **New in version 2.10**
* `--expected-codes <EXPECTED_CODES>` — The list of HTTP status codes expected in response from the member to declare it healthy. Specify one of the following values:

   - A single value, such as `200` - A list, such as `200, 202` - A range, such as `200-204`

   The default is 200.
* `--http-method <HTTP_METHOD>` — The HTTP method that the health monitor uses for requests. One of `CONNECT`, `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT`, or `TRACE`. The default is `GET`

  Possible values: `connect`, `delete`, `get`, `head`, `options`, `patch`, `post`, `put`, `trace`

* `--http-version <HTTP_VERSION>` — The HTTP version. One of `1.0` or `1.1`. The default is `1.0`.

   **New in version 2.10**
* `--max-retries <MAX_RETRIES>` — The number of successful checks before changing the `operating status` of the member to `ONLINE`. A valid value is from `1` to `10`
* `--max-retries-down <MAX_RETRIES_DOWN>` — The number of allowed check failures before changing the `operating status` of the member to `ERROR`. A valid value is from `1` to `10`. The default is `3`
* `--name <NAME>` — Human-readable name of the resource
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--timeout <TIMEOUT>` — The maximum time, in seconds, that a monitor waits to connect before it times out. This value must be less than the delay value
* `--url-path <URL_PATH>` — The HTTP URL path of the request sent by the monitor to test the health of a backend member. Must be a string that begins with a forward slash (`/`). The default URL path is `/`



## `osc load-balancer healthmonitor show`

Shows the details of a health monitor.

If you are not an administrative user and the parent load balancer does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer healthmonitor show <ID>`

###### **Arguments:**

* `<ID>` — healthmonitor_id parameter for /v2/lbaas/healthmonitors/{healthmonitor_id} API



## `osc load-balancer l7policy`

L7Policy (Octavia) commands

**Usage:** `osc load-balancer l7policy <COMMAND>`

###### **Subcommands:**

* `create` — Create an L7 Policy
* `delete` — Remove a L7 Policy
* `list` — List L7 Policies
* `rule` — `L7Policy Rule` commands
* `set` — Update a L7 Policy
* `show` — Show L7 Policy details



## `osc load-balancer l7policy create`

Creates a L7 policy.

This operation provisions a new L7 policy by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, the API returns a response object that contains a unique ID and the status of provisioning the L7 policy.

In the response, the L7 policy [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/l7policies/{l7policy_id}` to view the progress of the provisioning operation. When the L7 policy status changes to `ACTIVE`, the L7 policy is successfully provisioned and is ready for further configuration.

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

All the rules associated with a given policy are logically ANDead together. A request must match all the policy’s rules to match the policy.

If you need to express a logical OR operation between rules, then do this by creating multiple policies with the same action.

If a new policy is created with a position that matches that of an existing policy, then the new policy is inserted at the given position.

L7 policies with `action` of `REDIRECT_TO_URL` will return the default HTTP `Found (302)` response code with the `redirect_url`. Also, specify `redirect_http_code` to configure the needed HTTP response code, such as, 301, 302, 303, 307 and 308.

L7 policies with `action` of `REJECT` will return a `Forbidden (403)` response code to the requester.

**Usage:** `osc load-balancer l7policy create [OPTIONS] --action <ACTION> --listener-id <LISTENER_ID>`

###### **Options:**

* `--action <ACTION>` — The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`, `REDIRECT_TO_URL`, or `REJECT`

  Possible values: `redirect-prefix`, `redirect-to-pool`, `redirect-to-url`, `reject`

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--listener-id <LISTENER_ID>` — The ID of the listener
* `--name <NAME>` — Human-readable name of the resource
* `--position <POSITION>` — The position of this policy on the listener. Positions start at 1
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource
* `--redirect-http-code <REDIRECT_HTTP_CODE>` — Requests matching this policy will be redirected to the specified URL or Prefix URL with the HTTP response code. Valid if `action` is `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302, 303, 307, or 308. Default is 302.

   **New in version 2.9**
* `--redirect-pool-id <REDIRECT_POOL_ID>` — Requests matching this policy will be redirected to the pool with this ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some restrictions, See [Protocol Combinations (Listener/Pool)](#valid-protocol)
* `--redirect-prefix <REDIRECT_PREFIX>` — Requests matching this policy will be redirected to this Prefix URL. Only valid if `action` is `REDIRECT_PREFIX`
* `--redirect-url <REDIRECT_URL>` — Requests matching this policy will be redirected to this URL. Only valid if `action` is `REDIRECT_TO_URL`
* `--rules <JSON>`
* `--tags <TAGS>`
* `--tenant-id <TENANT_ID>`



## `osc load-balancer l7policy delete`

Removes a L7 policy and its associated configuration from the project.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer l7policy delete <ID>`

###### **Arguments:**

* `<ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id} API



## `osc load-balancer l7policy list`

Lists all L7 policies for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list L7 policies for other projects.

The list might be empty.

**Usage:** `osc load-balancer l7policy list`



## `osc load-balancer l7policy rule`

`L7Policy Rule` commands

**Usage:** `osc load-balancer l7policy rule <COMMAND>`

###### **Subcommands:**

* `create` — Create an L7 Rule
* `delete` — Remove a L7 Rule
* `list` — List L7 Rules
* `set` — Update a L7 Rule
* `show` — Show L7 Rule details



## `osc load-balancer l7policy rule create`

Creates a L7 rule.

This operation provisions a new L7 rule by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, the API returns a response object that contains a unique ID and the status of provisioning the L7 rule.

In the response, the L7 rule [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/l7policies/{l7policy_id}/rules/{l7rule_id}` to view the progress of the provisioning operation. When the L7 rule status changes to `ACTIVE`, the L7 rule is successfully provisioned and is ready for further configuration.

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

All the rules associated with a given policy are logically ANDead together. A request must match all the policy’s rules to match the policy.

If you need to express a logical OR operation between rules, then do this by creating multiple policies with the same action.

**Usage:** `osc load-balancer l7policy rule create [OPTIONS] --compare-type <COMPARE_TYPE> --type <TYPE> --value <VALUE> <L7POLICY_ID>`

###### **Arguments:**

* `<L7POLICY_ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--compare-type <COMPARE_TYPE>` — The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`, `EQUAL_TO`, `REGEX`, or `STARTS_WITH`

  Possible values: `contains`, `ends-with`, `equal-to`, `regex`, `starts-with`

* `--invert <INVERT>` — When `true` the logic of the rule is inverted. For example, with invert `true`, equal to would become not equal to. Default is `false`

  Possible values: `true`, `false`

* `--key <KEY>` — The key to use for the comparison. For example, the name of the cookie to evaluate
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--tenant-id <TENANT_ID>`
* `--type <TYPE>` — The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`, `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`

  Possible values: `cookie`, `file-type`, `header`, `host-name`, `path`, `ssl-conn-has-cert`, `ssl-dn-field`, `ssl-verify-result`

* `--value <VALUE>` — The value to use for the comparison. For example, the file type to compare



## `osc load-balancer l7policy rule delete`

Removes a L7 rule and its associated configuration from the project.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer l7policy rule delete <L7POLICY_ID> <ID>`

###### **Arguments:**

* `<L7POLICY_ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
* `<ID>` — rule_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API



## `osc load-balancer l7policy rule list`

Lists all L7 rules for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list L7 policies for other projects.

The list might be empty.

**Usage:** `osc load-balancer l7policy rule list <L7POLICY_ID>`

###### **Arguments:**

* `<L7POLICY_ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API



## `osc load-balancer l7policy rule set`

Updates a L7 rule.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the L7 rule provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the L7 rule object for changes.

This operation returns the updated L7 rule object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

**Usage:** `osc load-balancer l7policy rule set [OPTIONS] <L7POLICY_ID> <ID>`

###### **Arguments:**

* `<L7POLICY_ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
* `<ID>` — rule_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--compare-type <COMPARE_TYPE>` — The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`, `EQUAL_TO`, `REGEX`, or `STARTS_WITH`

  Possible values: `contains`, `ends-with`, `equal-to`, `regex`, `starts-with`

* `--invert <INVERT>` — When `true` the logic of the rule is inverted. For example, with invert `true`, equal to would become not equal to. Default is `false`

  Possible values: `true`, `false`

* `--key <KEY>` — The key to use for the comparison. For example, the name of the cookie to evaluate
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--type <TYPE>` — The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`, `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`

  Possible values: `cookie`, `file-type`, `header`, `host-name`, `path`, `ssl-conn-has-cert`, `ssl-dn-field`, `ssl-verify-result`

* `--value <VALUE>` — The value to use for the comparison. For example, the file type to compare



## `osc load-balancer l7policy rule show`

Shows the details of a L7 rule.

If you are not an administrative user and the L7 rule object does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer l7policy rule show <L7POLICY_ID> <ID>`

###### **Arguments:**

* `<L7POLICY_ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API
* `<ID>` — rule_id parameter for /v2/lbaas/l7policies/{l7policy_id}/rules/{rule_id} API



## `osc load-balancer l7policy set`

Updates a L7 policy.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the L7 policy provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the L7 policy object for changes.

This operation returns the updated L7 policy object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

If a policy is updated with a position that matches that of an existing policy, then the updated policy is inserted at the given position.

**Usage:** `osc load-balancer l7policy set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id} API

###### **Options:**

* `--action <ACTION>` — The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`, `REDIRECT_TO_URL`, or `REJECT`

  Possible values: `redirect-prefix`, `redirect-to-pool`, `redirect-to-url`, `reject`

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--name <NAME>` — Human-readable name of the resource
* `--position <POSITION>` — The position of this policy on the listener. Positions start at 1
* `--redirect-http-code <REDIRECT_HTTP_CODE>` — Requests matching this policy will be redirected to the specified URL or Prefix URL with the HTTP response code. Valid if `action` is `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302, 303, 307, or 308. Default is 302.

   **New in version 2.9**
* `--redirect-pool-id <REDIRECT_POOL_ID>` — Requests matching this policy will be redirected to the pool with this ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some restrictions, See [Protocol Combinations (Listener/Pool)](#valid-protocol)
* `--redirect-prefix <REDIRECT_PREFIX>` — Requests matching this policy will be redirected to this Prefix URL. Only valid if `action` is `REDIRECT_PREFIX`
* `--redirect-url <REDIRECT_URL>` — Requests matching this policy will be redirected to this URL. Only valid if `action` is `REDIRECT_TO_URL`
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**



## `osc load-balancer l7policy show`

Shows the details of a L7 policy.

If you are not an administrative user and the L7 policy object does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer l7policy show <ID>`

###### **Arguments:**

* `<ID>` — l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id} API



## `osc load-balancer listener`

Listener (Octavia) commands

**Usage:** `osc load-balancer listener <COMMAND>`

###### **Subcommands:**

* `create` — Create Listener
* `delete` — Remove a Listener
* `list` — List Listeners
* `set` — Update a Listener
* `show` — Show Listener details
* `stats` — Get Listener statistics



## `osc load-balancer listener create`

Creates a listener for a load balancer.

The listener configures a port and protocol for the load balancer to listen on for incoming requests. A load balancer may have zero or more listeners configured.

This operation provisions a new listener by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, the API returns a response object that contains a unique ID and the status of provisioning the listener.

In the response, the listener [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/listeners/{listener_id}` to view the progress of the provisioning operation. When the listener status changes to `ACTIVE`, the listener is successfully provisioned and is ready for further configuration.

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

Specifying a project_id is deprecated. The listener will inherit the project_id of the parent load balancer.

You can configure all documented features of the listener at creation time by specifying the additional elements or attributes in the request.

To create a listener, the parent load balancer must have an `ACTIVE` provisioning status.

**Usage:** `osc load-balancer listener create [OPTIONS] --loadbalancer-id <LOADBALANCER_ID> --protocol <PROTOCOL> --protocol-port <PROTOCOL_PORT>`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--allowed-cidrs <ALLOWED_CIDRS>` — A list of IPv4, IPv6 or mix of both CIDRs. The default is all allowed. When a list of CIDRs is provided, the default switches to deny all.

   **New in version 2.12**
* `--alpn-protocols <ALPN_PROTOCOLS>`
* `--client-authentication <CLIENT_AUTHENTICATION>` — The TLS client authentication mode. One of the options `NONE`, `OPTIONAL` or `MANDATORY`.

   **New in version 2.8**

  Possible values: `mandatory`, `none`, `optional`

* `--client-ca-tls-container-ref <CLIENT_CA_TLS_CONTAINER_REF>` — The ref of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format client CA certificate bundle for `TERMINATED_HTTPS` listeners.

   **New in version 2.8**
* `--client-crl-container-ref <CLIENT_CRL_CONTAINER_REF>` — The URI of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format CA revocation list file for `TERMINATED_HTTPS` listeners.

   **New in version 2.8**
* `--connection-limit <CONNECTION_LIMIT>` — The maximum number of connections permitted for this listener. Default value is -1 which represents infinite connections or a default value defined by the provider driver
* `--default-pool <JSON>` — A pool object
* `--default-pool-id <DEFAULT_POOL_ID>` — The ID of the pool used by the listener if no L7 policies match. The pool has some restrictions. See [Protocol Combinations (Listener/Pool)](#valid-protocol)
* `--default-tls-container-ref <DEFAULT_TLS_CONTAINER_REF>` — The URI of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PKCS12 format certificate/key bundle for `TERMINATED_HTTPS` listeners. DEPRECATED: A secret container of type “certificate” containing the certificate and key for `TERMINATED_HTTPS` listeners
* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--hsts-include-subdomains <HSTS_INCLUDE_SUBDOMAINS>` — Defines whether the `includeSubDomains` directive should be added to the Strict-Transport-Security HTTP response header. This requires setting the `hsts_max_age` option as well in order to become effective.

   **New in version 2.27**

  Possible values: `true`, `false`

* `--hsts-max-age <HSTS_MAX_AGE>` — The value of the `max_age` directive for the Strict-Transport-Security HTTP response header. Setting this enables HTTP Strict Transport Security (HSTS) for the TLS-terminated listener.

   **New in version 2.27**
* `--hsts-preload <HSTS_PRELOAD>` — Defines whether the `preload` directive should be added to the Strict-Transport-Security HTTP response header. This requires setting the `hsts_max_age` option as well in order to become effective.

   **New in version 2.27**

  Possible values: `true`, `false`

* `--insert-headers <key=value>` — A dictionary of optional headers to insert into the request before it is sent to the backend `member`. See [Supported HTTP Header Insertions](#header-insertions). Both keys and values are always specified as strings
* `--l7policies <JSON>` — A list of L7 policy objects
* `--loadbalancer-id <LOADBALANCER_ID>` — The ID of the load balancer
* `--name <NAME>` — Human-readable name of the resource
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource. (deprecated)
* `--protocol <PROTOCOL>` — The protocol for the resource. One of `HTTP`, `HTTPS`, `SCTP`, `PROMETHEUS`, `TCP`, `TERMINATED_HTTPS`, or `UDP`

  Possible values: `http`, `https`, `prometheus`, `sctp`, `tcp`, `terminated-https`, `udp`

* `--protocol-port <PROTOCOL_PORT>` — The protocol port number for the resource
* `--sni-container-refs <SNI_CONTAINER_REFS>` — A list of URIs to the [key manager service](https://docs.openstack.org/barbican/latest/) secrets containing PKCS12 format certificate/key bundles for `TERMINATED_HTTPS` listeners. (DEPRECATED) Secret containers of type “certificate” containing the certificates and keys for `TERMINATED_HTTPS` listeners
* `--tags <TAGS>`
* `--tenant-id <TENANT_ID>`
* `--timeout-client-data <TIMEOUT_CLIENT_DATA>` — Frontend client inactivity timeout in milliseconds. Default: 50000.

   **New in version 2.1**
* `--timeout-member-connect <TIMEOUT_MEMBER_CONNECT>` — Backend member connection timeout in milliseconds. Default: 5000.

   **New in version 2.1**
* `--timeout-member-data <TIMEOUT_MEMBER_DATA>` — Backend member inactivity timeout in milliseconds. Default: 50000.

   **New in version 2.1**
* `--timeout-tcp-inspect <TIMEOUT_TCP_INSPECT>` — Time, in milliseconds, to wait for additional TCP packets for content inspection. Default: 0.

   **New in version 2.1**
* `--tls-ciphers <TLS_CIPHERS>`
* `--tls-versions <TLS_VERSIONS>`



## `osc load-balancer listener delete`

Removes a listener and its associated configuration from the project.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer listener delete <ID>`

###### **Arguments:**

* `<ID>` — listener_id parameter for /v2/lbaas/listeners/{listener_id} API



## `osc load-balancer listener list`

Lists all listeners for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list listeners for other projects.

The list might be empty.

**Usage:** `osc load-balancer listener list`



## `osc load-balancer listener set`

Update an existing listener.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the listener provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the listener object for changes.

This operation returns the updated listener object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

**Usage:** `osc load-balancer listener set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — listener_id parameter for /v2/lbaas/listeners/{listener_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--allowed-cidrs <ALLOWED_CIDRS>` — A list of IPv4, IPv6 or mix of both CIDRs. The default is all allowed. When a list of CIDRs is provided, the default switches to deny all.

   **New in version 2.12**
* `--alpn-protocols <ALPN_PROTOCOLS>` — A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2

   **New in version 2.20**
* `--client-authentication <CLIENT_AUTHENTICATION>` — The TLS client authentication mode. One of the options `NONE`, `OPTIONAL` or `MANDATORY`.

   **New in version 2.8**

  Possible values: `mandatory`, `none`, `optional`

* `--client-ca-tls-container-ref <CLIENT_CA_TLS_CONTAINER_REF>` — The ref of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format client CA certificate bundle for `TERMINATED_HTTPS` listeners.

   **New in version 2.8**
* `--client-crl-container-ref <CLIENT_CRL_CONTAINER_REF>` — The URI of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format CA revocation list file for `TERMINATED_HTTPS` listeners.

   **New in version 2.8**
* `--connection-limit <CONNECTION_LIMIT>` — The maximum number of connections permitted for this listener. Default value is -1 which represents infinite connections or a default value defined by the provider driver
* `--default-pool-id <DEFAULT_POOL_ID>` — The ID of the pool used by the listener if no L7 policies match. The pool has some restrictions. See [Protocol Combinations (Listener/Pool)](#valid-protocol)
* `--default-tls-container-ref <DEFAULT_TLS_CONTAINER_REF>` — The URI of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PKCS12 format certificate/key bundle for `TERMINATED_HTTPS` listeners. DEPRECATED: A secret container of type “certificate” containing the certificate and key for `TERMINATED_HTTPS` listeners
* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--hsts-include-subdomains <HSTS_INCLUDE_SUBDOMAINS>` — Defines whether the `includeSubDomains` directive should be added to the Strict-Transport-Security HTTP response header. This requires setting the `hsts_max_age` option as well in order to become effective.

   **New in version 2.27**

  Possible values: `true`, `false`

* `--hsts-max-age <HSTS_MAX_AGE>` — The value of the `max_age` directive for the Strict-Transport-Security HTTP response header. Setting this enables HTTP Strict Transport Security (HSTS) for the TLS-terminated listener.

   **New in version 2.27**
* `--hsts-preload <HSTS_PRELOAD>` — Defines whether the `preload` directive should be added to the Strict-Transport-Security HTTP response header. This requires setting the `hsts_max_age` option as well in order to become effective.

   **New in version 2.27**

  Possible values: `true`, `false`

* `--insert-headers <key=value>` — A dictionary of optional headers to insert into the request before it is sent to the backend `member`. See [Supported HTTP Header Insertions](#header-insertions). Both keys and values are always specified as strings
* `--name <NAME>` — Human-readable name of the resource
* `--sni-container-refs <SNI_CONTAINER_REFS>` — A list of URIs to the [key manager service](https://docs.openstack.org/barbican/latest/) secrets containing PKCS12 format certificate/key bundles for `TERMINATED_HTTPS` listeners. (DEPRECATED) Secret containers of type “certificate” containing the certificates and keys for `TERMINATED_HTTPS` listeners
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--timeout-client-data <TIMEOUT_CLIENT_DATA>` — Frontend client inactivity timeout in milliseconds. Default: 50000.

   **New in version 2.1**
* `--timeout-member-connect <TIMEOUT_MEMBER_CONNECT>` — Backend member connection timeout in milliseconds. Default: 5000.

   **New in version 2.1**
* `--timeout-member-data <TIMEOUT_MEMBER_DATA>` — Backend member inactivity timeout in milliseconds. Default: 50000.

   **New in version 2.1**
* `--timeout-tcp-inspect <TIMEOUT_TCP_INSPECT>` — Time, in milliseconds, to wait for additional TCP packets for content inspection. Default: 0.

   **New in version 2.1**
* `--tls-ciphers <TLS_CIPHERS>` — List of ciphers in OpenSSL format (colon-separated). See [https://www.openssl.org/docs/man1.1.1/man1/ciphers.html](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html)

   **New in version 2.15**
* `--tls-versions <TLS_VERSIONS>` — A list of TLS protocol versions. Available versions: SSLv3, TLSv1, TLSv1.1, TLSv1.2, TLSv1.3

   **New in version 2.17**



## `osc load-balancer listener show`

Shows the details of a listener.

If you are not an administrative user and the parent load balancer does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer listener show <ID>`

###### **Arguments:**

* `<ID>` — listener_id parameter for /v2/lbaas/listeners/{listener_id} API



## `osc load-balancer listener stats`

Shows the current statistics for a listener.

This operation returns the statistics of a listener object identified by listener_id.

If you are not an administrative user and the parent load balancer does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer listener stats <ID>`

###### **Arguments:**

* `<ID>` — listener_id parameter for /v2/lbaas/listeners/{listener_id}/stats API



## `osc load-balancer loadbalancer`

Loadbalancer (Octavia) commands

**Usage:** `osc load-balancer loadbalancer <COMMAND>`

###### **Subcommands:**

* `create` — Create a Load Balancer
* `delete` — Remove a Load Balancer
* `failover` — Failover a load balancer
* `list` — List Load Balancers
* `set` — Update a Load Balancer
* `show` — Show Load Balancer details
* `stats` — Get Load Balancer statistics
* `status` — Get the Load Balancer status tree



## `osc load-balancer loadbalancer create`

Creates a load balancer.

This operation provisions a new load balancer by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, the API returns a response object that contains a unique ID and the status of provisioning the load balancer.

In the response, the load balancer [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/loadbalancers/{loadbalancer_id}` to view the progress of the provisioning operation. When the load balancer status changes to `ACTIVE`, the load balancer is successfully provisioned and is ready for further configuration.

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

Administrative users can specify a project ID that is different than their own to create load balancers for other projects.

An optional `flavor_id` attribute can be used to create the load balancer using a pre-configured octavia flavor. Flavors are created by the operator to allow custom load balancer configurations, such as allocating more memory for the load balancer.

An optional `vip_qos_policy_id` attribute from Neutron can be used to apply QoS policies on a loadbalancer VIP, also could pass a ‘null’ value to remove QoS policies.

You can also specify the `provider` attribute when you create a load balancer. The `provider` attribute specifies which backend should be used to create the load balancer. This could be the default provider (`octavia`) or a vendor supplied `provider` if one has been installed. Setting both a flavor_id and a provider will result in a conflict error if the provider does not match the provider of the configured flavor profiles.

Specifying a Virtual IP (VIP) is mandatory. There are three ways to specify a VIP network for the load balancer:

Additional VIPs may also be specified in the `additional_vips` field, by providing a list of JSON objects containing a `subnet_id` and optionally an `ip_address`. All additional subnets must be part of the same network as the primary VIP.

**Usage:** `osc load-balancer loadbalancer create [OPTIONS]`

###### **Options:**

* `--additional-vips <JSON>` — A list of JSON objects defining “additional VIPs”. The format for these is `{"subnet_id": <subnet_id>, "ip_address": <ip_address>}`, where the `subnet_id` field is mandatory and the `ip_address` field is optional. Additional VIP subnets must all belong to the same network as the primary VIP.

   **New in version 2.26**
* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--availability-zone <AVAILABILITY_ZONE>` — An availability zone name
* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--flavor-id <FLAVOR_ID>` — The ID of the flavor
* `--listeners <JSON>` — The associated listener IDs, if any
* `--name <NAME>` — Human-readable name of the resource
* `--pools <JSON>`
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource
* `--provider <PROVIDER>` — Provider name for the load balancer. Default is `octavia`
* `--tags <TAGS>`
* `--tenant-id <TENANT_ID>`
* `--vip-address <VIP_ADDRESS>` — The IP address of the Virtual IP (VIP)
* `--vip-network-id <VIP_NETWORK_ID>` — The ID of the network for the Virtual IP (VIP). One of `vip_network_id`, `vip_port_id`, or `vip_subnet_id` must be specified
* `--vip-port-id <VIP_PORT_ID>` — The ID of the Virtual IP (VIP) port. One of `vip_network_id`, `vip_port_id`, or `vip_subnet_id` must be specified
* `--vip-qos-policy-id <VIP_QOS_POLICY_ID>` — The ID of the QoS Policy which will apply to the Virtual IP (VIP)
* `--vip-subnet-id <VIP_SUBNET_ID>` — The ID of the subnet for the Virtual IP (VIP). One of `vip_network_id`, `vip_port_id`, or `vip_subnet_id` must be specified



## `osc load-balancer loadbalancer delete`

Removes a load balancer and its associated configuration from the project.

The optional parameter `cascade` when defined as `true` will delete all child objects of the load balancer.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer loadbalancer delete <ID>`

###### **Arguments:**

* `<ID>` — loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id} API



## `osc load-balancer loadbalancer failover`

Failover a load balancer

**Usage:** `osc load-balancer loadbalancer failover [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id}/failover API

###### **Options:**

* `--property <key=value>`



## `osc load-balancer loadbalancer list`

Lists all load balancers for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list load balancers for other projects.

The list might be empty.

**Usage:** `osc load-balancer loadbalancer list`



## `osc load-balancer loadbalancer set`

Updates a load balancer.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the load balancer provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the load balancer object for changes.

This operation returns the updated load balancer object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

**Usage:** `osc load-balancer loadbalancer set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`)

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--name <NAME>` — Human-readable name of the resource
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--vip-qos-policy-id <VIP_QOS_POLICY_ID>` — The ID of the QoS Policy which will apply to the Virtual IP (VIP)



## `osc load-balancer loadbalancer show`

Shows the details of a load balancer.

If you are not an administrative user and the load balancer object does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer loadbalancer show <ID>`

###### **Arguments:**

* `<ID>` — loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id} API



## `osc load-balancer loadbalancer stats`

Shows the current statistics for a load balancer.

This operation returns the statistics of a load balancer object identified by loadbalancer_id.

If you are not an administrative user and the load balancer object does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer loadbalancer stats <ID>`

###### **Arguments:**

* `<ID>` — loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id}/stats API



## `osc load-balancer loadbalancer status`

Shows the status tree for a load balancer.

This operation returns a status tree for a load balancer object, by load balancer ID.

`provisioning_status` is the status associated with lifecycle of the resource. See [Provisioning Status Codes](#prov-status) for descriptions of the status codes.

`operating_status` is the observed status of the resource. See [Operating Status Codes](#op-status) for descriptions of the status codes.

If you are not an administrative user and the load balancer object does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

If the operation succeeds, the returned element is a status tree that contains the load balancer and all provisioning and operating statuses for its children.

**Usage:** `osc load-balancer loadbalancer status <ID>`

###### **Arguments:**

* `<ID>` — loadbalancer_id parameter for /v2/lbaas/loadbalancers/{loadbalancer_id}/status API



## `osc load-balancer pool`

Pool (Octavia) commands

**Usage:** `osc load-balancer pool <COMMAND>`

###### **Subcommands:**

* `create` — Create Pool
* `delete` — Remove a Pool
* `list` — List Pools
* `member` — Pool Member commands
* `set` — Update a Pool
* `show` — Show Pool details



## `osc load-balancer pool create`

Creates a pool for a load balancer.

The pool defines how requests should be balanced across the backend member servers.

This operation provisions a pool by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, the API returns a response object, which contains a unique ID.

In the response, the pool [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/pools/{pool_id}` to view the progress of the provisioning operation. When the pool status changes to `ACTIVE`, the pool is successfully provisioned and is ready for further configuration.

At a minimum, you must specify these pool attributes:

Some attributes receive default values if you omit them from the request:

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

Specifying a project_id is deprecated. The pool will inherit the project_id of the parent load balancer.

You can configure all documented features of the pool at creation time by specifying the additional elements or attributes in the request.

To create a pool, the parent load balancer must have an `ACTIVE` provisioning status.

`SOURCE_IP_PORT` algorithm is available from version 2.13.

**Usage:** `osc load-balancer pool create [OPTIONS] --lb-algorithm <LB_ALGORITHM> --protocol <PROTOCOL>`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--alpn-protocols <ALPN_PROTOCOLS>` — A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2

   **New in version 2.24**
* `--ca-tls-container-ref <CA_TLS_CONTAINER_REF>` — The reference of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format CA certificate bundle for `tls_enabled` pools.

   **New in version 2.8**
* `--crl-container-ref <CRL_CONTAINER_REF>` — The reference of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format CA revocation list file for `tls_enabled` pools
* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--healthmonitor <JSON>` — Defines mandatory and optional attributes of a POST request
* `--lb-algorithm <LB_ALGORITHM>` — The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`, `ROUND_ROBIN`, `SOURCE_IP`, or `SOURCE_IP_PORT`

  Possible values: `least-connections`, `round-robin`, `source-ip`, `source-ip-port`

* `--listener-id <LISTENER_ID>` — The ID of the listener for the pool. Either `listener_id` or `loadbalancer_id` must be specified. The listener has some restrictions, See [Protocol Combinations (Listener/Pool)](#valid-protocol)
* `--loadbalancer-id <LOADBALANCER_ID>` — The ID of the load balancer for the pool. Either `listener_id` or `loadbalancer_id` must be specified
* `--members <JSON>`
* `--name <NAME>` — Human-readable name of the resource
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource. (deprecated)
* `--protocol <PROTOCOL>` — The protocol for the resource. One of `HTTP`, `HTTPS`, `PROXY`, `PROXYV2`, `SCTP`, `TCP`, or `UDP`

  Possible values: `http`, `https`, `proxy`, `proxyv2`, `sctp`, `tcp`, `udp`

* `--cookie-name <COOKIE_NAME>`
* `--persistence-granularity <PERSISTENCE_GRANULARITY>`
* `--persistence-timeout <PERSISTENCE_TIMEOUT>`
* `--type <TYPE>`

  Possible values: `app-cookie`, `http-cookie`, `source-ip`

* `--tags <TAGS>`
* `--tenant-id <TENANT_ID>`
* `--tls-ciphers <TLS_CIPHERS>` — List of ciphers in OpenSSL format (colon-separated). See [https://www.openssl.org/docs/man1.1.1/man1/ciphers.html](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html)

   **New in version 2.15**
* `--tls-container-ref <TLS_CONTAINER_REF>` — The reference to the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PKCS12 format certificate/key bundle for `tls_enabled` pools for TLS client authentication to the member servers.

   **New in version 2.8**
* `--tls-enabled <TLS_ENABLED>` — When `true` connections to backend member servers will use TLS encryption. Default is `false`.

   **New in version 2.8**

  Possible values: `true`, `false`

* `--tls-versions <TLS_VERSIONS>` — A list of TLS protocol versions. Available versions: SSLv3, TLSv1, TLSv1.1, TLSv1.2, TLSv1.3

   **New in version 2.17**



## `osc load-balancer pool delete`

Removes a pool and its associated configuration from the load balancer.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer pool delete <ID>`

###### **Arguments:**

* `<ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id} API



## `osc load-balancer pool list`

Lists all pools for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list pools for other projects.

The list might be empty.

**Usage:** `osc load-balancer pool list`



## `osc load-balancer pool member`

Pool Member commands

**Usage:** `osc load-balancer pool member <COMMAND>`

###### **Subcommands:**

* `create` — Create Member
* `delete` — Remove a Member
* `list` — List Members
* `set` — Update a Member
* `show` — Show Member details



## `osc load-balancer pool member create`

This operation provisions a member and adds it to a pool by using the configuration that you define in the request object. After the API validates the request and starts the provisioning process, it returns a response object, which contains a unique ID.

In the response, the member [provisioning status](#prov-status) is `ACTIVE`, `PENDING_CREATE`, or `ERROR`.

If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/pools/{pool_id}/members/{member_id}` to view the progress of the provisioning operation. When the member status changes to `ACTIVE`, the member is successfully provisioned and is ready for further configuration.

If the API cannot fulfill the request due to insufficient data or data that is not valid, the service returns the HTTP `Bad Request (400)` response code with information about the failure in the response body. Validation errors require that you correct the error and submit the request again.

At a minimum, you must specify these member attributes:

Some attributes receive default values if you omit them from the request:

If you omit the `subnet_id` parameter, the `vip_subnet_id` for the parent load balancer will be used for the member subnet UUID.

The member `address` does not necessarily need to be a member of the `subnet_id` subnet. Members can be routable from the subnet specified either via the default route or by using `host_routes` defined on the subnet.

Administrative users can specify a project ID that is different than their own to create members for other projects.

`monitor_address` and/or `monitor_port` can be used to have the health monitor, if one is configured for the pool, connect to an alternate IP address and port when executing a health check on the member.

To create a member, the load balancer must have an `ACTIVE` provisioning status.

**Usage:** `osc load-balancer pool member create [OPTIONS] --address <ADDRESS> --protocol-port <PROTOCOL_PORT> <POOL_ID>`

###### **Arguments:**

* `<POOL_ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API

###### **Options:**

* `--address <ADDRESS>` — The IP address of the resource
* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--backup <BACKUP>` — Is the member a backup? Backup members only receive traffic when all non-backup members are down.

   **New in version 2.1**

  Possible values: `true`, `false`

* `--monitor-address <MONITOR_ADDRESS>` — An alternate IP address used for health monitoring a backend member. Default is `null` which monitors the member `address`
* `--monitor-port <MONITOR_PORT>` — An alternate protocol port used for health monitoring a backend member. Default is `null` which monitors the member `protocol_port`
* `--name <NAME>` — Human-readable name of the resource
* `--project-id <PROJECT_ID>` — The ID of the project owning this resource. (deprecated)
* `--protocol-port <PROTOCOL_PORT>` — The protocol port number for the resource
* `--subnet-id <SUBNET_ID>` — The subnet ID the member service is accessible from
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--tenant-id <TENANT_ID>`
* `--weight <WEIGHT>` — The weight of a member determines the portion of requests or connections it services compared to the other members of the pool. For example, a member with a weight of 10 receives five times as many requests as a member with a weight of 2. A value of 0 means the member does not receive new connections but continues to service existing connections. A valid value is from `0` to `256`. Default is `1`



## `osc load-balancer pool member delete`

Removes a member and its associated configuration from the pool.

The API immediately purges any and all configuration data, depending on the configuration settings. You cannot recover it.

**Usage:** `osc load-balancer pool member delete <POOL_ID> <ID>`

###### **Arguments:**

* `<POOL_ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
* `<ID>` — member_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API



## `osc load-balancer pool member list`

Lists all members for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list members for other projects.

The list might be empty.

**Usage:** `osc load-balancer pool member list <POOL_ID>`

###### **Arguments:**

* `<POOL_ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API



## `osc load-balancer pool member set`

Update an existing member.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the member provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the member object for changes.

Setting the member weight to `0` means that the member will not receive new requests but will finish any existing connections. This “drains” the backend member of active connections.

This operation returns the updated member object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

**Usage:** `osc load-balancer pool member set [OPTIONS] <POOL_ID> <ID>`

###### **Arguments:**

* `<POOL_ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
* `<ID>` — member_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--backup <BACKUP>` — Is the member a backup? Backup members only receive traffic when all non-backup members are down.

   **New in version 2.1**

  Possible values: `true`, `false`

* `--monitor-address <MONITOR_ADDRESS>` — An alternate IP address used for health monitoring a backend member. Default is `null` which monitors the member `address`
* `--monitor-port <MONITOR_PORT>` — An alternate protocol port used for health monitoring a backend member. Default is `null` which monitors the member `protocol_port`
* `--name <NAME>` — Human-readable name of the resource
* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--weight <WEIGHT>` — The weight of a member determines the portion of requests or connections it services compared to the other members of the pool. For example, a member with a weight of 10 receives five times as many requests as a member with a weight of 2. A value of 0 means the member does not receive new connections but continues to service existing connections. A valid value is from `0` to `256`. Default is `1`



## `osc load-balancer pool member show`

Shows the details of a pool member.

If you are not an administrative user and the parent load balancer does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer pool member show <POOL_ID> <ID>`

###### **Arguments:**

* `<POOL_ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
* `<ID>` — member_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API



## `osc load-balancer pool set`

Update an existing pool.

If the request is valid, the service returns the `Accepted (202)` response code. To confirm the update, check that the pool provisioning status is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll the pool object for changes.

This operation returns the updated pool object with the `ACTIVE`, `PENDING_UPDATE`, or `ERROR` provisioning status.

**Usage:** `osc load-balancer pool set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id} API

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--alpn-protocols <ALPN_PROTOCOLS>` — A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2

   **New in version 2.24**
* `--ca-tls-container-ref <CA_TLS_CONTAINER_REF>` — The reference of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format CA certificate bundle for `tls_enabled` pools.

   **New in version 2.8**
* `--crl-container-ref <CRL_CONTAINER_REF>` — The reference of the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PEM format CA revocation list file for `tls_enabled` pools
* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--lb-algorithm <LB_ALGORITHM>` — The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`, `ROUND_ROBIN`, or `SOURCE_IP`

  Possible values: `least-connections`, `round-robin`, `source-ip`, `source-ip-port`

* `--name <NAME>` — Human-readable name of the resource
* `--cookie-name <COOKIE_NAME>`
* `--persistence-granularity <PERSISTENCE_GRANULARITY>`
* `--persistence-timeout <PERSISTENCE_TIMEOUT>`
* `--type <TYPE>`

  Possible values: `app-cookie`, `http-cookie`, `source-ip`

* `--tags <TAGS>` — A list of simple strings assigned to the resource.

   **New in version 2.5**
* `--tls-ciphers <TLS_CIPHERS>` — List of ciphers in OpenSSL format (colon-separated). See [https://www.openssl.org/docs/man1.1.1/man1/ciphers.html](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html)

   **New in version 2.15**
* `--tls-container-ref <TLS_CONTAINER_REF>` — The reference to the [key manager service](https://docs.openstack.org/castellan/latest/) secret containing a PKCS12 format certificate/key bundle for `tls_enabled` pools for TLS client authentication to the member servers.

   **New in version 2.8**
* `--tls-enabled <TLS_ENABLED>` — When `true` connections to backend member servers will use TLS encryption. Default is `false`.

   **New in version 2.8**

  Possible values: `true`, `false`

* `--tls-versions <TLS_VERSIONS>` — A list of TLS protocol versions. Available versions: SSLv3, TLSv1, TLSv1.1, TLSv1.2, TLSv1.3

   **New in version 2.17**



## `osc load-balancer pool show`

Shows the details of a pool.

If you are not an administrative user and the parent load balancer does not belong to your project, the service returns the HTTP `Forbidden (403)` response code.

This operation does not require a request body.

**Usage:** `osc load-balancer pool show <ID>`

###### **Arguments:**

* `<ID>` — pool_id parameter for /v2/lbaas/pools/{pool_id} API



## `osc load-balancer provider`

Provider (Octavia) commands

**Usage:** `osc load-balancer provider <COMMAND>`

###### **Subcommands:**

* `availability-zone-capability` — AvailabilityZoneCapability (Octavia) commands
* `flavor-capability` — FlavorCapability (Octavia) commands
* `list` — List Providers



## `osc load-balancer provider availability-zone-capability`

AvailabilityZoneCapability (Octavia) commands

**Usage:** `osc load-balancer provider availability-zone-capability <COMMAND>`

###### **Subcommands:**

* `list` — Show Provider Availability Zone Capabilities



## `osc load-balancer provider availability-zone-capability list`

Shows the provider driver availability zone capabilities. These are the features of the provider driver that can be configured in an Octavia availability zone. This API returns a list of dictionaries with the name and description of each availability zone capability of the provider.

The list might be empty and a provider driver may not implement this feature.

**New in version 2.14**

**Usage:** `osc load-balancer provider availability-zone-capability list <PROVIDER>`

###### **Arguments:**

* `<PROVIDER>` — provider parameter for /v2/lbaas/providers/{provider}/availability_zone_capabilities API



## `osc load-balancer provider flavor-capability`

FlavorCapability (Octavia) commands

**Usage:** `osc load-balancer provider flavor-capability <COMMAND>`

###### **Subcommands:**

* `list` — Show Provider Flavor Capabilities



## `osc load-balancer provider flavor-capability list`

Shows the provider driver flavor capabilities. These are the features of the provider driver that can be configured in an Octavia flavor. This API returns a list of dictionaries with the name and description of each flavor capability of the provider.

The list might be empty and a provider driver may not implement this feature.

**New in version 2.6**

**Usage:** `osc load-balancer provider flavor-capability list <PROVIDER>`

###### **Arguments:**

* `<PROVIDER>` — provider parameter for /v2/lbaas/providers/{provider}/flavor_capabilities API



## `osc load-balancer provider list`

Lists all enabled provider drivers.

Use the `fields` query parameter to control which fields are returned in the response body.

The list might be empty.

**Usage:** `osc load-balancer provider list`



## `osc load-balancer quota`

Quota commands

**Usage:** `osc load-balancer quota <COMMAND>`

###### **Subcommands:**

* `delete` — Reset a Quota
* `list` — List Quota
* `set` — Update a Quota
* `show` — Show Project Quota



## `osc load-balancer quota delete`

Reset a Quota

**Usage:** `osc load-balancer quota delete <PROJECT_ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v2/lbaas/quotas/{project_id} API



## `osc load-balancer quota list`

Lists all quotas for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to list quotas for other projects.

If the quota is listed as `null` the quota is using the deployment default quota settings.

A quota of `-1` means the quota is unlimited.

The list might be empty.

**Usage:** `osc load-balancer quota list`



## `osc load-balancer quota set`

Updates a quota for a project.

If the request is valid, the service returns the `Accepted (202)` response code.

This operation returns the updated quota object.

If the quota is specified as `null` the quota will use the deployment default quota settings.

Specifying a quota of `-1` means the quota is unlimited.

Specifying a quota of `0` means the project cannot create any of the resource.

**Usage:** `osc load-balancer quota set [OPTIONS] <PROJECT_ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v2/lbaas/quotas/{project_id} API

###### **Options:**

* `--health-monitor <HEALTH_MONITOR>`
* `--healthmonitor <HEALTHMONITOR>` — The configured health monitor quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited
* `--l7policy <L7POLICY>` — The configured l7policy quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited
* `--l7rule <L7RULE>` — The configured l7rule quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited
* `--listener <LISTENER>` — The configured listener quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited
* `--load-balancer <LOAD_BALANCER>`
* `--loadbalancer <LOADBALANCER>` — The configured load balancer quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited
* `--member <MEMBER>` — The configured member quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited
* `--pool <POOL>` — The configured pool quota limit. A setting of `null` means it is using the deployment default quota. A setting of `-1` means unlimited



## `osc load-balancer quota show`

Show the quota for the project.

Use the `fields` query parameter to control which fields are returned in the response body. Additionally, you can filter results by using query string parameters. For information, see [Filtering and column selection](#filtering).

Administrative users can specify a project ID that is different than their own to show quota for other projects.

A quota of `-1` means the quota is unlimited.

**Usage:** `osc load-balancer quota show <PROJECT_ID>`

###### **Arguments:**

* `<PROJECT_ID>` — project_id parameter for /v2/lbaas/quotas/{project_id} API



## `osc load-balancer version`

Version (Octavia) commands

**Usage:** `osc load-balancer version <COMMAND>`

###### **Subcommands:**

* `get` — Command without description in OpenAPI



## `osc load-balancer version get`

Command without description in OpenAPI

**Usage:** `osc load-balancer version get`



## `osc network`

Network (Neutron) commands

**Usage:** `osc network <COMMAND>`

###### **Subcommands:**

* `address-group` — Address groups
* `address-scope` — Address scopes
* `availability-zone` — Availability Zones commands
* `extension` — Extensions commands
* `floating-ip` — Floating IP commands
* `network` — Network commands
* `port` — Port commands
* `rbac-policy` — RbacPolicy commands
* `router` — Router commands
* `security-group` — SecurityGroup commands
* `security-group-rule` — SecurityGroupRule commands
* `subnet` — Subnet commands
* `subnetpool` — SubnetPool commands



## `osc network address-group`

Address groups

Lists, creates, shows details for, updates, and deletes address groups.

**Usage:** `osc network address-group <COMMAND>`

###### **Subcommands:**

* `add-address` — Command without description in OpenAPI
* `create` — Create address group
* `delete` — Delete an address group
* `list` — List address groups
* `remove-address` — Command without description in OpenAPI
* `set` — Update an address group
* `show` — Show address group



## `osc network address-group add-address`

Command without description in OpenAPI

**Usage:** `osc network address-group add-address [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-groups/{id} API

###### **Options:**

* `--addresses <ADDRESSES>` — A list of IP addresses



## `osc network address-group create`

Creates an address group.

Normal response codes: 201

Error response codes: 400, 401, 403, 404

**Usage:** `osc network address-group create [OPTIONS]`

###### **Options:**

* `--addresses <ADDRESSES>` — A list of IP addresses
* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--project-id <PROJECT_ID>`



## `osc network address-group delete`

Deletes an address group.

Normal response codes: 204

Error response codes: 401, 404, 412

**Usage:** `osc network address-group delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-groups/{id} API



## `osc network address-group list`

Lists address groups that the project has access to.

Default policy settings return only the address groups owned by the project of the user submitting the request, unless the user has administrative role.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network address-group list [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — description query parameter for /v2.0/address-groups API
* `--id <ID>` — id query parameter for /v2.0/address-groups API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/address-groups API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--project-id <PROJECT_ID>` — project_id query parameter for /v2.0/address-groups API
* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network address-group remove-address`

Command without description in OpenAPI

**Usage:** `osc network address-group remove-address [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-groups/{id} API

###### **Options:**

* `--addresses <ADDRESSES>` — A list of IP addresses



## `osc network address-group set`

Updates an address group.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 412

**Usage:** `osc network address-group set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-groups/{id} API

###### **Options:**

* `--description <DESCRIPTION>` — A human-readable description for the resource
* `--name <NAME>` — Human-readable name of the resource. Default is an empty string



## `osc network address-group show`

Shows information for an address group.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network address-group show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-groups/{id} API



## `osc network address-scope`

Address scopes

Lists, creates, shows details for, updates, and deletes address scopes.

**Usage:** `osc network address-scope <COMMAND>`

###### **Subcommands:**

* `create` — Create address scope
* `delete` — Delete an address scope
* `list` — List address scopes
* `set` — Update an address scope
* `show` — Show address scope



## `osc network address-scope create`

Creates an address scope.

Normal response codes: 201

Error response codes: 400, 401, 403, 404

**Usage:** `osc network address-scope create [OPTIONS]`

###### **Options:**

* `--ip-version <IP_VERSION>` — The IP protocol version. Valid value is `4` or `6`
* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--shared <SHARED>` — Indicates whether this resource is shared across all projects. By default, only administrative users can change this value

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies



## `osc network address-scope delete`

Deletes an address scope.

Normal response codes: 204

Error response codes: 401, 404, 412

**Usage:** `osc network address-scope delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-scopes/{id} API



## `osc network address-scope list`

Lists address scopes that the project has access to.

Default policy settings return only the address scopes owned by the project of the user submitting the request, unless the user has administrative role.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network address-scope list [OPTIONS]`

###### **Options:**

* `--id <ID>` — id query parameter for /v2.0/address-scopes API
* `--ip-version <IP_VERSION>` — ip_version query parameter for /v2.0/address-scopes API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/address-scopes API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--shared <SHARED>` — shared query parameter for /v2.0/address-scopes API

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/address-scopes API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network address-scope set`

Updates an address scope.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 412

**Usage:** `osc network address-scope set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-scopes/{id} API

###### **Options:**

* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--shared <SHARED>` — Indicates whether this resource is shared across all projects. By default, only administrative users can change this value

  Possible values: `true`, `false`




## `osc network address-scope show`

Shows information for an address scope.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network address-scope show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/address-scopes/{id} API



## `osc network availability-zone`

Availability Zones commands

**Usage:** `osc network availability-zone <COMMAND>`

###### **Subcommands:**

* `list` — List all availability zones



## `osc network availability-zone list`

Lists all availability zones.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network availability-zone list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/availability_zones API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--resource <RESOURCE>` — resource query parameter for /v2.0/availability_zones API
* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--state <STATE>` — state query parameter for /v2.0/availability_zones API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network extension`

Extensions commands

**Usage:** `osc network extension <COMMAND>`

###### **Subcommands:**

* `list` — List extensions
* `show` — Show extension details



## `osc network extension list`

Lists available extensions.

Lists available Networking API v2.0 extensions.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network extension list`



## `osc network extension show`

Shows details for an extension, by alias. The response shows the extension name and its alias. To show details for an extension, you specify the alias.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network extension show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/extensions/{id} API



## `osc network floating-ip`

Floating IP commands

**Usage:** `osc network floating-ip <COMMAND>`

###### **Subcommands:**

* `create` — Create floating IP
* `delete` — Delete floating IP
* `list` — List floating IPs
* `port-forwarding` — Floating IPs port forwarding
* `set` — Update floating IP
* `show` — Show floating IP details
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network floating-ip create`

Creates a floating IP, and, if you specify port information, associates the floating IP with an internal port.

To associate the floating IP with an internal port, specify the port ID attribute in the request body. If you do not specify a port ID in the request, you can issue a PUT request instead of a POST request.

Default policy settings enable only administrative users to set floating IP addresses and some non-administrative users might require a floating IP address. If you do not specify a floating IP address in the request, the operation automatically allocates one.

By default, this operation associates the floating IP address with a single fixed IP address that is configured on an OpenStack Networking port. If a port has multiple IP addresses, you must specify the `fixed_ip_address` attribute in the request body to associate a fixed IP address with the floating IP address.

You can create floating IPs on only external networks. When you create a floating IP, you must specify the ID of the network on which you want to create the floating IP. Alternatively, you can create a floating IP on a subnet in the external network, based on the costs and quality of that subnet.

You must configure an IP address with the internal OpenStack Networking port that is associated with the floating IP address.

The operation returns the `Bad Request (400)` response code for one of reasons:

If the port ID is not valid, this operation returns `404` response code.

The operation returns the `Conflict (409)` response code for one of reasons:

Normal response codes: 201

Error response codes: 400, 401, 404, 409

**Usage:** `osc network floating-ip create [OPTIONS] --floating-network-id <FLOATING_NETWORK_ID>`

###### **Options:**

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--dns-domain <DNS_DOMAIN>` — A valid DNS domain
* `--dns-name <DNS_NAME>` — A valid DNS name
* `--fixed-ip-address <FIXED_IP_ADDRESS>` — The fixed IP address that is associated with the floating IP. If an internal port has multiple associated IP addresses, the service chooses the first IP address unless you explicitly define a fixed IP address in the `fixed_ip_address` parameter
* `--floating-ip-address <FLOATING_IP_ADDRESS>` — The floating IP address
* `--floating-network-id <FLOATING_NETWORK_ID>` — The ID of the network associated with the floating IP
* `--port-id <PORT_ID>` — The ID of a port associated with the floating IP. To associate the floating IP with a fixed IP at creation time, you must specify the identifier of the internal port
* `--qos-policy-id <QOS_POLICY_ID>` — The ID of the QoS policy associated with the floating IP
* `--subnet-id <SUBNET_ID>` — The subnet ID on which you want to create the floating IP
* `--tenant-id <TENANT_ID>` — The ID of the project



## `osc network floating-ip delete`

Deletes a floating IP and, if present, its associated port.

This example deletes a floating IP:

Normal response codes: 204

Error response codes: 401, 404, 412

**Usage:** `osc network floating-ip delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/floatingips/{id} API



## `osc network floating-ip list`

Lists floating IPs visible to the user.

Default policy settings return only the floating IPs owned by the user’s project, unless the user has admin role.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

This example request lists floating IPs in JSON format:

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network floating-ip list [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — description query parameter for /v2.0/floatingips API
* `--fixed-ip-address <FIXED_IP_ADDRESS>` — fixed_ip_address query parameter for /v2.0/floatingips API
* `--floating-ip-address <FLOATING_IP_ADDRESS>` — floating_ip_address query parameter for /v2.0/floatingips API
* `--floating-network-id <FLOATING_NETWORK_ID>` — floating_network_id query parameter for /v2.0/floatingips API
* `--id <ID>` — id query parameter for /v2.0/floatingips API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/floatingips API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/floatingips API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--port-id <PORT_ID>` — port_id query parameter for /v2.0/floatingips API
* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/floatingips API
* `--router-id <ROUTER_ID>` — router_id query parameter for /v2.0/floatingips API
* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--status <STATUS>` — status query parameter for /v2.0/floatingips API
* `--tags <TAGS>` — tags query parameter for /v2.0/floatingips API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/floatingips API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/floatingips API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network floating-ip port-forwarding`

Floating IPs port forwarding

Lists, creates, shows details for, updates, and deletes floating IPs port forwardings.

## Port forwarding with port ranges

The floating-ip-port-forwarding-port-ranges extension adds the new attributes internal_port_range and external_port_range to the floating IP port forwardings. The value of these new attributes should be a string that represents a colon separated port range. You can not use the attributes internal_port_range and external_port_range with the attributes internal_port and external_port in the same request.

## Port forwarding rule description

The floating-ip-port-forwarding-description extension adds the description attribute to the floating IP port forwardings. The value of the description attribute contains a text describing the rule, which helps users to manage/find easily theirs rules.

**Usage:** `osc network floating-ip port-forwarding <COMMAND>`

###### **Subcommands:**

* `create` — Create port forwarding
* `delete` — Delete a floating IP port forwarding
* `list` — List floating IP port forwardings
* `set` — Update a port forwarding
* `show` — Show port forwarding



## `osc network floating-ip port-forwarding create`

Creates a floating IP port forwarding.

Normal response codes: 201

Error response codes: 400, 404

**Usage:** `osc network floating-ip port-forwarding create [OPTIONS] <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API

###### **Options:**

* `--description <DESCRIPTION>` — A text describing the rule, which helps users to manage/find easily theirs rules
* `--external-port <EXTERNAL_PORT>` — The TCP/UDP/other protocol port number of the port forwarding’s floating IP address
* `--external-port-range <EXTERNAL_PORT_RANGE>` — The TCP/UDP/other protocol port range of the port forwarding’s floating IP address
* `--internal-ip-address <INTERNAL_IP_ADDRESS>` — The fixed IPv4 address of the Neutron port associated to the floating IP port forwarding
* `--internal-port <INTERNAL_PORT>` — The TCP/UDP/other protocol port number of the Neutron port fixed IP address associated to the floating ip port forwarding
* `--internal-port-id <INTERNAL_PORT_ID>` — The ID of the Neutron port associated to the floating IP port forwarding
* `--internal-port-range <INTERNAL_PORT_RANGE>` — The TCP/UDP/other protocol port range of the Neutron port fixed IP address associated to the floating ip port forwarding
* `--project-id <PROJECT_ID>`
* `--protocol <PROTOCOL>` — The IP protocol used in the floating IP port forwarding

  Possible values: `dccp`, `icmp`, `ipv6-icmp`, `sctp`, `tcp`, `udp`




## `osc network floating-ip port-forwarding delete`

Deletes a floating IP port forwarding.

Normal response codes: 204

Error response codes: 404

**Usage:** `osc network floating-ip port-forwarding delete <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API



## `osc network floating-ip port-forwarding list`

Lists floating IP port forwardings that the project has access to.

Default policy settings return only the port forwardings associated to floating IPs owned by the project of the user submitting the request, unless the user has administrative role.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 400, 404

**Usage:** `osc network floating-ip port-forwarding list [OPTIONS] <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API

###### **Options:**

* `--description <DESCRIPTION>` — description query parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings API
* `--external-port <EXTERNAL_PORT>` — external_port query parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings API
* `--external-port-range <EXTERNAL_PORT_RANGE>` — external_port_range query parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings API
* `--id <ID>` — id query parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings API
* `--internal-port-id <INTERNAL_PORT_ID>` — internal_port_id query parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--protocol <PROTOCOL>` — protocol query parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings API

  Possible values: `dccp`, `icmp`, `ipv6-icmp`, `sctp`, `tcp`, `udp`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network floating-ip port-forwarding set`

Updates a floating IP port forwarding.

Normal response codes: 200

Error response codes: 400, 404

**Usage:** `osc network floating-ip port-forwarding set [OPTIONS] <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API

###### **Options:**

* `--description <DESCRIPTION>`
* `--external-port <EXTERNAL_PORT>` — The TCP/UDP/other protocol port number of the port forwarding’s floating IP address
* `--external-port-range <EXTERNAL_PORT_RANGE>` — The TCP/UDP/other protocol port range of the port forwarding’s floating IP address
* `--internal-ip-address <INTERNAL_IP_ADDRESS>` — The fixed IPv4 address of the Neutron port associated to the floating IP port forwarding
* `--internal-port <INTERNAL_PORT>` — The TCP/UDP/other protocol port number of the Neutron port fixed IP address associated to the floating ip port forwarding
* `--internal-port-id <INTERNAL_PORT_ID>` — The ID of the Neutron port associated to the floating IP port forwarding
* `--internal-port-range <INTERNAL_PORT_RANGE>` — The TCP/UDP/other protocol port range of the Neutron port fixed IP address associated to the floating ip port forwarding
* `--protocol <PROTOCOL>` — The IP protocol used in the floating IP port forwarding

  Possible values: `dccp`, `icmp`, `ipv6-icmp`, `sctp`, `tcp`, `udp`




## `osc network floating-ip port-forwarding show`

Shows information for a floating IP port forwarding.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 400, 404

**Usage:** `osc network floating-ip port-forwarding show <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API



## `osc network floating-ip set`

Updates a floating IP and its association with an internal port.

The association process is the same as the process for the create floating IP operation.

To disassociate a floating IP from a port, set the `port_id` attribute to null or omit it from the request body.

This example updates a floating IP:

Depending on the request body that you submit, this request associates a port with or disassociates a port from a floating IP.

Normal response codes: 200

Error response codes: 400, 401, 404, 409, 412

**Usage:** `osc network floating-ip set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/floatingips/{id} API

###### **Options:**

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--fixed-ip-address <FIXED_IP_ADDRESS>` — The fixed IP address that is associated with the floating IP. If an internal port has multiple associated IP addresses, the service chooses the first IP address unless you explicitly define a fixed IP address in the `fixed_ip_address` parameter
* `--port-id <PORT_ID>` — The ID of a port associated with the floating IP. To associate the floating IP with a fixed IP, you must specify the ID of the internal port. To disassociate the floating IP, `null` should be specified
* `--qos-policy-id <QOS_POLICY_ID>`



## `osc network floating-ip show`

Shows details for a floating IP.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

This example request shows details for a floating IP in JSON format. This example also filters the result by the `fixed_ip_address` and `floating_ip_address` fields.

Normal response codes: 200

Error response codes: 401, 403, 404

**Usage:** `osc network floating-ip show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/floatingips/{id} API



## `osc network floating-ip tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network floating-ip tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network floating-ip tag add`

Command without description in OpenAPI

**Usage:** `osc network floating-ip tag add <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag check`

Command without description in OpenAPI

**Usage:** `osc network floating-ip tag check <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag delete`

Command without description in OpenAPI

**Usage:** `osc network floating-ip tag delete <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag list`

Command without description in OpenAPI

**Usage:** `osc network floating-ip tag list [OPTIONS] <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network floating-ip tag purge`

Command without description in OpenAPI

**Usage:** `osc network floating-ip tag purge <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network network`

Network commands

**Usage:** `osc network network <COMMAND>`

###### **Subcommands:**

* `create` — Create network
* `delete` — Delete network
* `dhcp-agent` — DHCP agent scheduler
* `list` — List networks
* `show` — Show network details
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network network create`

Creates a network.

A request body is optional. An administrative user can specify another project ID, which is the project that owns the network, in the request body.

Normal response codes: 201

Error response codes: 400, 401

**Usage:** `osc network network create [OPTIONS]`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the network, which is up (`true`) or down (`false`)

  Possible values: `true`, `false`

* `--availability-zone-hints <AVAILABILITY_ZONE_HINTS>` — The availability zone candidate for the network
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--dns-domain <DNS_DOMAIN>` — A valid DNS domain
* `--ha <HA>`

  Possible values: `true`, `false`

* `--is-default <IS_DEFAULT>` — The network is default or not

  Possible values: `true`, `false`

* `--mtu <MTU>` — The maximum transmission unit (MTU) value to address fragmentation. Minimum value is 68 for IPv4, and 1280 for IPv6
* `--name <NAME>` — Human-readable name of the network
* `--port-security-enabled <PORT_SECURITY_ENABLED>` — The port security status of the network. Valid values are enabled (`true`) and disabled (`false`). This value is used as the default value of `port_security_enabled` field of a newly created port

  Possible values: `true`, `false`

* `--provider-network-type <PROVIDER_NETWORK_TYPE>`
* `--provider-physical-network <PROVIDER_PHYSICAL_NETWORK>`
* `--provider-segmentation-id <PROVIDER_SEGMENTATION_ID>`
* `--qos-policy-id <QOS_POLICY_ID>` — The ID of the QoS policy associated with the network
* `--router-external <ROUTER_EXTERNAL>` — Indicates whether the network has an external routing facility that’s not managed by the networking service

  Possible values: `true`, `false`

* `--segments <JSON>` — A list of provider `segment` objects
* `--shared <SHARED>` — Indicates whether this resource is shared across all projects. By default, only administrative users can change this value

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies



## `osc network network delete`

Deletes a network and its associated resources.

Normal response codes: 204

Error response codes: 401, 404, 409, 412

**Usage:** `osc network network delete <ID>`

###### **Arguments:**

* `<ID>` — network_id parameter for /v2.0/networks/{network_id} API



## `osc network network dhcp-agent`

DHCP agent scheduler

The DHCP agent scheduler extension (dhcp_agent_scheduler) enables administrators to assign DHCP servers for Neutron networks to given Neutron DHCP agents, and retrieve mappings between Neutron networks and DHCP agents.

**Usage:** `osc network network dhcp-agent <COMMAND>`

###### **Subcommands:**

* `list` — List DHCP agents hosting a network



## `osc network network dhcp-agent list`

Lists DHCP agents hosting a network.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401, 403

**Usage:** `osc network network dhcp-agent list [OPTIONS] <NETWORK_ID>`

###### **Arguments:**

* `<NETWORK_ID>` — network_id parameter for /v2.0/networks/{network_id}/dhcp-agents/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network network list`

Lists networks to which the project has access.

Default policy settings return only networks that the project who submits the request owns, unless an administrative user submits the request. In addition, networks shared with the project who submits the request are also returned.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

You can also use the `tags`, `tags-any`, `not-tags`, `not-tags-any` query parameter to filter the response with tags. For information, see [REST API Impact](http://specs.openstack.org/openstack/neutron-specs/specs/mitaka/add-tags-to-core-resources.html#rest-api-impact).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network network list [OPTIONS]`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — admin_state_up query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — description query parameter for /v2.0/networks API
* `--id <ID>` — id query parameter for /v2.0/networks API
* `--is-default <IS_DEFAULT>` — is_default query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--mtu <MTU>` — mtu query parameter for /v2.0/networks API
* `--name <NAME>` — name query parameter for /v2.0/networks API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/networks API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/networks API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--provider-network-type <PROVIDER_NETWORK_TYPE>` — provider:network_type query parameter for /v2.0/networks API
* `--provider-physical-network <PROVIDER_PHYSICAL_NETWORK>` — provider:physical_network query parameter for /v2.0/networks API
* `--provider-segmentation-id <PROVIDER_SEGMENTATION_ID>` — provider:segmentation_id query parameter for /v2.0/networks API
* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/networks API
* `--router-external <ROUTER_EXTERNAL>` — router:external query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--shared <SHARED>` — shared query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--status <STATUS>` — status query parameter for /v2.0/networks API
* `--tags <TAGS>` — tags query parameter for /v2.0/networks API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/networks API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/networks API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network network show`

Shows details for a network.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network network show <ID>`

###### **Arguments:**

* `<ID>` — network_id parameter for /v2.0/networks/{network_id} API



## `osc network network tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network network tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network network tag add`

Command without description in OpenAPI

**Usage:** `osc network network tag add <NETWORK_ID> <ID>`

###### **Arguments:**

* `<NETWORK_ID>` — network_id parameter for /v2.0/networks/{network_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/networks/{network_id}/tags/{id} API



## `osc network network tag check`

Command without description in OpenAPI

**Usage:** `osc network network tag check <NETWORK_ID> <ID>`

###### **Arguments:**

* `<NETWORK_ID>` — network_id parameter for /v2.0/networks/{network_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/networks/{network_id}/tags/{id} API



## `osc network network tag delete`

Command without description in OpenAPI

**Usage:** `osc network network tag delete <NETWORK_ID> <ID>`

###### **Arguments:**

* `<NETWORK_ID>` — network_id parameter for /v2.0/networks/{network_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/networks/{network_id}/tags/{id} API



## `osc network network tag list`

Command without description in OpenAPI

**Usage:** `osc network network tag list [OPTIONS] <NETWORK_ID>`

###### **Arguments:**

* `<NETWORK_ID>` — network_id parameter for /v2.0/networks/{network_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network network tag purge`

Command without description in OpenAPI

**Usage:** `osc network network tag purge <NETWORK_ID>`

###### **Arguments:**

* `<NETWORK_ID>` — network_id parameter for /v2.0/networks/{network_id}/tags/{id} API



## `osc network port`

Port commands

**Usage:** `osc network port <COMMAND>`

###### **Subcommands:**

* `create` — Create port
* `delete` — Delete port
* `list` — List ports
* `show` — Show port details
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network port create`

Creates a port on a network.

To define the network in which to create the port, specify the `network_id` attribute in the request body.

Normal response codes: 201

Error response codes: 400, 401, 403, 404

**Usage:** `osc network port create [OPTIONS]`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--allowed-address-pairs <JSON>` — A set of zero or more allowed address pair objects each where address pair object contains an `ip_address` and `mac_address`. While the `ip_address` is required, the `mac_address` will be taken from the port if not specified. The value of `ip_address` can be an IP Address or a CIDR (if supported by the underlying extension plugin). A server connected to the port can send a packet with source address which matches one of the specified allowed address pairs
* `--binding-host-id <BINDING_HOST_ID>` — The ID of the host where the port resides. The default is an empty string
* `--binding-profile <key=value>` — A dictionary that enables the application running on the specific host to pass and receive vif port information specific to the networking back-end. This field is only meant for machine-machine communication for compute services like Nova, Ironic or Zun to pass information to a Neutron back-end. It should not be used by multiple services concurrently or by cloud end users. The existing counterexamples (`capabilities: [switchdev]` for Open vSwitch hardware offload and `trusted=true` for Trusted Virtual Functions) are due to be cleaned up. The networking API does not define a specific format of this field. The default is an empty dictionary. If you update it with null then it is treated like {} in the response. Since the port-mac-address-override extension the `device_mac_address` field of the binding:profile can be used to provide the MAC address of the physical device a direct-physical port is being bound to. If provided, then the `mac_address` field of the port resource will be updated to the MAC from the active binding
* `--binding-vnic-type <BINDING_VNIC_TYPE>` — The type of vNIC which this port should be attached to. This is used to determine which mechanism driver(s) to be used to bind the port. The valid values are `normal`, `macvtap`, `direct`, `baremetal`, `direct-physical`, `virtio-forwarder`, `smart-nic` and `remote-managed`. What type of vNIC is actually available depends on deployments. The default is `normal`

  Possible values: `accelerator-direct`, `accelerator-direct-physical`, `baremetal`, `direct`, `direct-physical`, `macvtap`, `normal`, `remote-managed`, `smart-nic`, `vdpa`, `virtio-forwarder`

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--device-id <DEVICE_ID>` — The ID of the device that uses this port. For example, a server instance or a logical router
* `--device-owner <DEVICE_OWNER>` — The entity type that uses this port. For example, `compute:nova` (server instance), `network:dhcp` (DHCP agent) or `network:router_interface` (router interface)
* `--device-profile <DEVICE_PROFILE>`
* `--dns-domain <DNS_DOMAIN>` — A valid DNS domain
* `--dns-name <DNS_NAME>` — A valid DNS name
* `--extra-dhcp-opts <JSON>` — A set of zero or more extra DHCP option pairs. An option pair consists of an option value and name
* `--fixed-ips <JSON>` — The IP addresses for the port. If you would like to assign multiple IP addresses for the port, specify multiple entries in this field. Each entry consists of IP address (`ip_address`) and the subnet ID from which the IP address is assigned (`subnet_id`).

   - If you specify both a subnet ID and an IP address, OpenStack Networking tries to allocate the IP address on that subnet to the port. - If you specify only a subnet ID, OpenStack Networking allocates an available IP from that subnet to the port. - If you specify only an IP address, OpenStack Networking tries to allocate the IP address if the address is a valid IP for any of the subnets on the specified network.
* `--hints <key=value>` — Admin-only. A dict, at the top level keyed by mechanism driver aliases (as defined in setup.cfg). To following values can be used to control Open vSwitch’s Userspace Tx packet steering feature:

   - `{"openvswitch": {"other_config": {"tx-steering": "hash"}}}` - `{"openvswitch": {"other_config": {"tx-steering": "thread"}}}`

   If omitted the default is defined by Open vSwitch. The field cannot be longer than 4095 characters.
* `--mac-address <MAC_ADDRESS>` — The MAC address of the port. If unspecified, a MAC address is automatically generated
* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--network-id <NETWORK_ID>` — The ID of the attached network
* `--numa-affinity-policy <NUMA_AFFINITY_POLICY>` — The port NUMA affinity policy requested during the virtual machine scheduling. Values: `None`, `required`, `preferred` or `legacy`

  Possible values: `legacy`, `preferred`, `required`, `socket`

* `--port-security-enabled <PORT_SECURITY_ENABLED>` — The port security status. A valid value is enabled (`true`) or disabled (`false`). If port security is enabled for the port, security group rules and anti-spoofing rules are applied to the traffic on the port. If disabled, no such rules are applied

  Possible values: `true`, `false`

* `--propagate-uplink-status <PROPAGATE_UPLINK_STATUS>` — The uplink status propagation of the port. Valid values are enabled (`true`) and disabled (`false`)

  Possible values: `true`, `false`

* `--qos-policy-id <QOS_POLICY_ID>` — QoS policy associated with the port
* `--security-groups <SECURITY_GROUPS>` — The IDs of security groups applied to the port
* `--tags <TAGS>`
* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies



## `osc network port delete`

Deletes a port.

Any IP addresses that are associated with the port are returned to the respective subnets allocation pools.

Normal response codes: 204

Error response codes: 401, 403, 404, 412

**Usage:** `osc network port delete <ID>`

###### **Arguments:**

* `<ID>` — port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs API



## `osc network port list`

Lists ports to which the user has access.

Default policy settings return only those ports that are owned by the project of the user who submits the request, unless the request is submitted by a user with administrative rights.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

If the `ip-substring-filtering` extension is enabled, the Neutron API supports IP address substring filtering on the `fixed_ips` attribute. If you specify an IP address substring (`ip_address_substr`) in an entry of the `fixed_ips` attribute, the Neutron API will list all ports that have an IP address matching the substring.

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network port list [OPTIONS]`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — admin_state_up query parameter for /v2.0/ports API

  Possible values: `true`, `false`

* `--binding-host-id <BINDING_HOST_ID>` — binding:host_id query parameter for /v2.0/ports API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/ports API
* `--device-id <DEVICE_ID>` — device_id query parameter for /v2.0/ports API
* `--device-owner <DEVICE_OWNER>` — device_owner query parameter for /v2.0/ports API
* `--fixed-ips <FIXED_IPS>` — fixed_ips query parameter for /v2.0/ports API
* `--id <ID>` — id query parameter for /v2.0/ports API
* `--ip-allocation <IP_ALLOCATION>` — ip_allocation query parameter for /v2.0/ports API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--mac-address <MAC_ADDRESS>` — mac_address query parameter for /v2.0/ports API
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/ports API
* `--network-id <NETWORK_ID>` — network_id query parameter for /v2.0/ports API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/ports API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/ports API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/ports API
* `--security-groups <SECURITY_GROUPS>` — security_groups query parameter for /v2.0/ports API
* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--status <STATUS>` — status query parameter for /v2.0/ports API
* `--tags <TAGS>` — tags query parameter for /v2.0/ports API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/ports API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/ports API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network port show`

Shows details for a port.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network port show <ID>`

###### **Arguments:**

* `<ID>` — port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs API



## `osc network port tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network port tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network port tag add`

Command without description in OpenAPI

**Usage:** `osc network port tag add <PORT_ID> <ID>`

###### **Arguments:**

* `<PORT_ID>` — port_id parameter for /v2.0/ports/{port_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/ports/{port_id}/tags/{id} API



## `osc network port tag check`

Command without description in OpenAPI

**Usage:** `osc network port tag check <PORT_ID> <ID>`

###### **Arguments:**

* `<PORT_ID>` — port_id parameter for /v2.0/ports/{port_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/ports/{port_id}/tags/{id} API



## `osc network port tag delete`

Command without description in OpenAPI

**Usage:** `osc network port tag delete <PORT_ID> <ID>`

###### **Arguments:**

* `<PORT_ID>` — port_id parameter for /v2.0/ports/{port_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/ports/{port_id}/tags/{id} API



## `osc network port tag list`

Command without description in OpenAPI

**Usage:** `osc network port tag list [OPTIONS] <PORT_ID>`

###### **Arguments:**

* `<PORT_ID>` — port_id parameter for /v2.0/ports/{port_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network port tag purge`

Command without description in OpenAPI

**Usage:** `osc network port tag purge <PORT_ID>`

###### **Arguments:**

* `<PORT_ID>` — port_id parameter for /v2.0/ports/{port_id}/tags/{id} API



## `osc network rbac-policy`

RbacPolicy commands

**Usage:** `osc network rbac-policy <COMMAND>`

###### **Subcommands:**

* `create` — Create RBAC policy
* `delete` — Delete RBAC policy
* `list` — List RBAC policies
* `set` — Update RBAC policy
* `show` — Show RBAC policy details



## `osc network rbac-policy create`

Create RBAC policy for given tenant.

Normal response codes: 201

Error response codes: 400, 401

**Usage:** `osc network rbac-policy create [OPTIONS]`

###### **Options:**

* `--action <ACTION>` — Action for the RBAC policy which is `access_as_external` or `access_as_shared`
* `--object-id <OBJECT_ID>` — The ID of the `object_type` resource. An `object_type` of `network` returns a network ID, an `object_type` of `qos-policy` returns a QoS policy ID, an `object_type` of `security-group` returns a security group ID, an `object_type` of `address-scope` returns a address scope ID, an `object_type` of `subnetpool` returns a subnetpool ID and an `object_type` of `address-group` returns an address group ID
* `--object-type <OBJECT_TYPE>` — The type of the object that the RBAC policy affects. Types include `qos-policy`, `network`, `security-group`, `address-scope`, `subnetpool` or `address-group`
* `--target-tenant <TARGET_TENANT>` — The ID of the tenant to which the RBAC policy will be enforced. Please note that Neutron does not perform any type of validation that the value provided is actually the ID of the existing project. If, for example, the name of the project is provided here, it will be accepted by the Neutron API, but the RBAC rule created will not work as expected
* `--tenant-id <TENANT_ID>`



## `osc network rbac-policy delete`

Delete an RBAC policy.

Normal response codes: 204

Error response codes: 401, 404, 409

**Usage:** `osc network rbac-policy delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/rbac-policies/{id} API



## `osc network rbac-policy list`

List RBAC policies that belong to a given tenant.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network rbac-policy list [OPTIONS]`

###### **Options:**

* `--action <ACTION>` — action query parameter for /v2.0/rbac-policies API
* `--id <ID>` — id query parameter for /v2.0/rbac-policies API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--object-id <OBJECT_ID>` — object_id query parameter for /v2.0/rbac-policies API
* `--object-type <OBJECT_TYPE>` — object_type query parameter for /v2.0/rbac-policies API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--target-tenant <TARGET_TENANT>` — target_tenant query parameter for /v2.0/rbac-policies API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/rbac-policies API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network rbac-policy set`

Update RBAC policy for given tenant.

Normal response codes: 200

Error response codes: 400, 401, 403, 404

**Usage:** `osc network rbac-policy set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/rbac-policies/{id} API

###### **Options:**

* `--target-tenant <TARGET_TENANT>` — The ID of the tenant to which the RBAC policy will be enforced. Please note that Neutron does not perform any type of validation that the value provided is actually the ID of the existing project. If, for example, the name of the project is provided here, it will be accepted by the Neutron API, but the RBAC rule created will not work as expected



## `osc network rbac-policy show`

Show details for a given RBAC policy.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network rbac-policy show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/rbac-policies/{id} API



## `osc network router`

Router commands

**Usage:** `osc network router <COMMAND>`

###### **Subcommands:**

* `add-external-gateway` — Add external gateways to router
* `add-extraroute` — Add extra routes to router
* `add-router-interface` — Add interface to router
* `conntrack-helper` — Lists, creates, shows details for, updates, and deletes router conntrack helper (CT) target rules
* `create` — Create router
* `delete` — Delete router
* `l3-agent` — L3 agent scheduler
* `list` — List routers
* `remove-external-gateway` — Remove external gateways from router
* `remove-extraroute` — Remove extra routes from router
* `remove-router-interface` — Remove interface from router
* `show` — Show router details
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network router add-external-gateway`

Add external gateways to router

**Usage:** `osc network router add-external-gateway [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API

###### **Options:**

* `--external-gateways <JSON>` — The list of external gateways of the router



## `osc network router add-extraroute`

Add extra routes to router

**Usage:** `osc network router add-extraroute [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API

###### **Options:**

* `--routes <JSON>` — The extra routes configuration for L3 router. A list of dictionaries with `destination` and `nexthop` parameters. It is available when `extraroute` extension is enabled



## `osc network router add-router-interface`

Add interface to router

**Usage:** `osc network router add-router-interface [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API

###### **Options:**

* `--port-id <PORT_ID>` — The ID of the port. One of subnet_id or port_id must be specified
* `--subnet-id <SUBNET_ID>` — The ID of the subnet. One of subnet_id or port_id must be specified



## `osc network router conntrack-helper`

Lists, creates, shows details for, updates, and deletes router conntrack helper (CT) target rules

**Usage:** `osc network router conntrack-helper <COMMAND>`

###### **Subcommands:**

* `create` — Create conntrack helper
* `delete` — Delete a conntrack helper
* `list` — List router conntrack helpers
* `set` — Update a conntrack helper
* `show` — Show conntrack helper



## `osc network router conntrack-helper create`

Creates a router conntrack helper.

Normal response codes: 201

Error response codes: 400, 404

**Usage:** `osc network router conntrack-helper create [OPTIONS] <ROUTER_ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API

###### **Options:**

* `--helper <HELPER>` — The netfilter conntrack helper module
* `--port <PORT>` — The network port for the netfilter conntrack target rule
* `--project-id <PROJECT_ID>`
* `--protocol <PROTOCOL>` — The network protocol for the netfilter conntrack target rule

  Possible values: `dccp`, `icmp`, `ipv6-icmp`, `sctp`, `tcp`, `udp`




## `osc network router conntrack-helper delete`

Deletes a router conntrack helper.

Normal response codes: 204

Error response codes: 404

**Usage:** `osc network router conntrack-helper delete <ROUTER_ID> <ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API
* `<ID>` — id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API



## `osc network router conntrack-helper list`

Lists router conntrack helpers associated with a router.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 400, 404

**Usage:** `osc network router conntrack-helper list [OPTIONS] <ROUTER_ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API

###### **Options:**

* `--helper <HELPER>` — helper query parameter for /v2.0/routers/{router_id}/conntrack_helpers API
* `--id <ID>` — id query parameter for /v2.0/routers/{router_id}/conntrack_helpers API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--port <PORT>` — port query parameter for /v2.0/routers/{router_id}/conntrack_helpers API
* `--protocol <PROTOCOL>` — protocol query parameter for /v2.0/routers/{router_id}/conntrack_helpers API

  Possible values: `dccp`, `icmp`, `ipv6-icmp`, `sctp`, `tcp`, `udp`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network router conntrack-helper set`

Updates a router conntrack helper.

Normal response codes: 200

Error response codes: 400, 404

**Usage:** `osc network router conntrack-helper set [OPTIONS] <ROUTER_ID> <ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API
* `<ID>` — id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API

###### **Options:**

* `--helper <HELPER>` — The netfilter conntrack helper module
* `--port <PORT>` — The network port for the netfilter conntrack target rule
* `--protocol <PROTOCOL>` — The network protocol for the netfilter conntrack target rule

  Possible values: `dccp`, `icmp`, `ipv6-icmp`, `sctp`, `tcp`, `udp`




## `osc network router conntrack-helper show`

Shows information for a router conntrack helper.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 400, 404

**Usage:** `osc network router conntrack-helper show <ROUTER_ID> <ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API
* `<ID>` — id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API



## `osc network router create`

Creates a logical router.

This operation creates a logical router. The logical router does not have any internal interface and it is not associated with any subnet. You can optionally specify an external gateway for a router at create time. The external gateway for the router must be plugged into an external network. An external network has its `router:external` extended field set to `true`. To specify an external gateway, the ID of the external network must be passed in the `network_id` parameter of the `external_gateway_info` attribute in the request body.

Normal response codes: 201

Error response codes: 400, 401

**Usage:** `osc network router create [OPTIONS]`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--availability-zone-hints <AVAILABILITY_ZONE_HINTS>` — The availability zone candidates for the router. It is available when `router_availability_zone` extension is enabled
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--distributed <DISTRIBUTED>` — `true` indicates a distributed router. It is available when `dvr` extension is enabled

  Possible values: `true`, `false`

* `--enable-ndp-proxy <ENABLE_NDP_PROXY>` — Enable NDP proxy attribute. Default is `false`, To persist this attribute value, set the `enable_ndp_proxy_by_default` option in the `neutron.conf` file. It is available when `router-extend-ndp-proxy` extension is enabled

  Possible values: `true`, `false`

* `--enable-snat <ENABLE_SNAT>`

  Possible values: `true`, `false`

* `--external-fixed-ips <JSON>`
* `--network-id <NETWORK_ID>`
* `--flavor-id <FLAVOR_ID>` — The ID of the flavor associated with the router
* `--ha <HA>` — `true` indicates a highly-available router. It is available when `l3-ha` extension is enabled

  Possible values: `true`, `false`

* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies



## `osc network router delete`

Deletes a logical router and, if present, its external gateway interface.

This operation fails if the router has attached interfaces. Use the remove router interface operation to remove all router interfaces before you delete the router.

Normal response codes: 204

Error response codes: 401, 404, 409, 412

**Usage:** `osc network router delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API



## `osc network router l3-agent`

L3 agent scheduler

The L3 agent scheduler extension (l3_agent_scheduler) allows administrators to assign Neutron routers to Neutron L3 agents, and retrieve mappings between Neutron routers and L3 agents.

**Usage:** `osc network router l3-agent <COMMAND>`

###### **Subcommands:**

* `list` — List L3 agents hosting a router



## `osc network router l3-agent list`

Lists l3 agents hosting a specific router.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network router l3-agent list [OPTIONS] <ROUTER_ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/l3-agents/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network router list`

Lists logical routers that the project who submits the request can access.

Default policy settings return only those routers that the project who submits the request owns, unless an administrative user submits the request.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network router list [OPTIONS]`

###### **Options:**

* `--admin-state-up <ADMIN_STATE_UP>` — admin_state_up query parameter for /v2.0/routers API

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — description query parameter for /v2.0/routers API
* `--enable-ndp-proxy <ENABLE_NDP_PROXY>` — enable_ndp_proxy query parameter for /v2.0/routers API

  Possible values: `true`, `false`

* `--id <ID>` — id query parameter for /v2.0/routers API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/routers API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/routers API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/routers API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/routers API
* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--tags <TAGS>` — tags query parameter for /v2.0/routers API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/routers API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/routers API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network router remove-external-gateway`

Remove external gateways from router

**Usage:** `osc network router remove-external-gateway [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API

###### **Options:**

* `--external-gateways <JSON>` — The list of external gateways of the router



## `osc network router remove-extraroute`

Remove extra routes from router

**Usage:** `osc network router remove-extraroute [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API

###### **Options:**

* `--routes <JSON>` — The extra routes configuration for L3 router. A list of dictionaries with `destination` and `nexthop` parameters. It is available when `extraroute` extension is enabled



## `osc network router remove-router-interface`

Remove interface from router

**Usage:** `osc network router remove-router-interface [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API

###### **Options:**

* `--port-id <PORT_ID>` — The ID of the port. One of subnet_id or port_id must be specified
* `--subnet-id <SUBNET_ID>` — The ID of the subnet. One of subnet_id or port_id must be specified



## `osc network router show`

Shows details for a router.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 403, 404

**Usage:** `osc network router show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API



## `osc network router tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network router tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network router tag add`

Command without description in OpenAPI

**Usage:** `osc network router tag add <ROUTER_ID> <ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/routers/{router_id}/tags/{id} API



## `osc network router tag check`

Command without description in OpenAPI

**Usage:** `osc network router tag check <ROUTER_ID> <ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/routers/{router_id}/tags/{id} API



## `osc network router tag delete`

Command without description in OpenAPI

**Usage:** `osc network router tag delete <ROUTER_ID> <ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/routers/{router_id}/tags/{id} API



## `osc network router tag list`

Command without description in OpenAPI

**Usage:** `osc network router tag list [OPTIONS] <ROUTER_ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network router tag purge`

Command without description in OpenAPI

**Usage:** `osc network router tag purge <ROUTER_ID>`

###### **Arguments:**

* `<ROUTER_ID>` — router_id parameter for /v2.0/routers/{router_id}/tags/{id} API



## `osc network security-group`

SecurityGroup commands

**Usage:** `osc network security-group <COMMAND>`

###### **Subcommands:**

* `create` — Create security group
* `delete` — Delete security group
* `list` — List security groups
* `set` — Update security group
* `show` — Show security group
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network security-group create`

Creates an OpenStack Networking security group.

This operation creates a security group with default security group rules for the IPv4 and IPv6 ether types.

Normal response codes: 201

Error response codes: 400, 401, 409

**Usage:** `osc network security-group create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--name <NAME>` — Human-readable name of the resource
* `--stateful <STATEFUL>` — Indicates if the security group is stateful or stateless

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — The ID of the project



## `osc network security-group delete`

Deletes an OpenStack Networking security group.

This operation deletes an OpenStack Networking security group and its associated security group rules, provided that a port is not associated with the security group. If a port is associated with the security group 409 (Conflict) is returned.

This operation does not require a request body. This operation does not return a response body.

Normal response codes: 204

Error response codes: 401, 404, 409, 412

**Usage:** `osc network security-group delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/security-groups/{id} API



## `osc network security-group list`

Lists OpenStack Networking security groups to which the project has access.

The response is an array of `security_group` objects which contains a list of `security_group_rules` objects.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network security-group list [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — description query parameter for /v2.0/security-groups API
* `--id <ID>` — id query parameter for /v2.0/security-groups API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/security-groups API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/security-groups API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/security-groups API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/security-groups API
* `--shared <SHARED>` — shared query parameter for /v2.0/security-groups API

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--tags <TAGS>` — tags query parameter for /v2.0/security-groups API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/security-groups API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/security-groups API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network security-group set`

Updates a security group.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 412

**Usage:** `osc network security-group set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/security-groups/{id} API

###### **Options:**

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--name <NAME>` — Human-readable name of the resource
* `--stateful <STATEFUL>`

  Possible values: `true`, `false`




## `osc network security-group show`

Shows details for a security group.

The associated security group rules are contained in the response.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network security-group show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/security-groups/{id} API



## `osc network security-group tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network security-group tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network security-group tag add`

Command without description in OpenAPI

**Usage:** `osc network security-group tag add <SECURITY_GROUP_ID> <ID>`

###### **Arguments:**

* `<SECURITY_GROUP_ID>` — security_group_id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API



## `osc network security-group tag check`

Command without description in OpenAPI

**Usage:** `osc network security-group tag check <SECURITY_GROUP_ID> <ID>`

###### **Arguments:**

* `<SECURITY_GROUP_ID>` — security_group_id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API



## `osc network security-group tag delete`

Command without description in OpenAPI

**Usage:** `osc network security-group tag delete <SECURITY_GROUP_ID> <ID>`

###### **Arguments:**

* `<SECURITY_GROUP_ID>` — security_group_id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API



## `osc network security-group tag list`

Command without description in OpenAPI

**Usage:** `osc network security-group tag list [OPTIONS] <SECURITY_GROUP_ID>`

###### **Arguments:**

* `<SECURITY_GROUP_ID>` — security_group_id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network security-group tag purge`

Command without description in OpenAPI

**Usage:** `osc network security-group tag purge <SECURITY_GROUP_ID>`

###### **Arguments:**

* `<SECURITY_GROUP_ID>` — security_group_id parameter for /v2.0/security-groups/{security_group_id}/tags/{id} API



## `osc network security-group-rule`

SecurityGroupRule commands

**Usage:** `osc network security-group-rule <COMMAND>`

###### **Subcommands:**

* `create` — Create security group rule
* `delete` — Delete security group rule
* `list` — List security group rules
* `set` — Command without description in OpenAPI
* `show` — Show security group rule



## `osc network security-group-rule create`

Creates an OpenStack Networking security group rule.

Normal response codes: 201

Error response codes: 400, 401, 404, 409

**Usage:** `osc network security-group-rule create [OPTIONS]`

###### **Options:**

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--direction <DIRECTION>` — Ingress or egress, which is the direction in which the security group rule is applied

  Possible values: `egress`, `ingress`

* `--ethertype <ETHERTYPE>` — Must be IPv4 or IPv6, and addresses represented in CIDR must match the ingress or egress rules

  Possible values: `ipv4`, `ipv6`

* `--port-range-max <PORT_RANGE_MAX>` — The maximum port number in the range that is matched by the security group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this value must be greater than or equal to the `port_range_min` attribute value. If the protocol is ICMP, this value must be an ICMP code
* `--port-range-min <PORT_RANGE_MIN>` — The minimum port number in the range that is matched by the security group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this value must be less than or equal to the `port_range_max` attribute value. If the protocol is ICMP, this value must be an ICMP type
* `--protocol <PROTOCOL>` — The IP protocol can be represented by a string, an integer, or `null`. Valid string or integer values are `any` or `0`, `ah` or `51`, `dccp` or `33`, `egp` or `8`, `esp` or `50`, `gre` or `47`, `icmp` or `1`, `icmpv6` or `58`, `igmp` or `2`, `ipip` or `4`, `ipv6-encap` or `41`, `ipv6-frag` or `44`, `ipv6-icmp` or `58`, `ipv6-nonxt` or `59`, `ipv6-opts` or `60`, `ipv6-route` or `43`, `ospf` or `89`, `pgm` or `113`, `rsvp` or `46`, `sctp` or `132`, `tcp` or `6`, `udp` or `17`, `udplite` or `136`, `vrrp` or `112`. Additionally, any integer value between \[0-255\] is also valid. The string `any` (or integer `0`) means `all` IP protocols. See the constants in `neutron_lib.constants` for the most up-to-date list of supported strings
* `--remote-address-group-id <REMOTE_ADDRESS_GROUP_ID>`
* `--remote-group-id <REMOTE_GROUP_ID>` — The remote group UUID to associate with this security group rule. You can specify either the `remote_group_id` or `remote_ip_prefix` attribute in the request body
* `--remote-ip-prefix <REMOTE_IP_PREFIX>` — The remote IP prefix that is matched by this security group rule
* `--security-group-id <SECURITY_GROUP_ID>` — The security group ID to associate with this security group rule
* `--tenant-id <TENANT_ID>`



## `osc network security-group-rule delete`

Deletes a rule from an OpenStack Networking security group.

Normal response codes: 204

Error response codes: 401, 404, 412

**Usage:** `osc network security-group-rule delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/security-group-rules/{id} API



## `osc network security-group-rule list`

Lists a summary of all OpenStack Networking security group rules that the project can access.

The list provides the ID for each security group rule.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network security-group-rule list [OPTIONS]`

###### **Options:**

* `--belongs-to-default-sg <BELONGS_TO_DEFAULT_SG>` — belongs_to_default_sg query parameter for /v2.0/security-group-rules API

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — description query parameter for /v2.0/security-group-rules API
* `--direction <DIRECTION>` — direction query parameter for /v2.0/security-group-rules API

  Possible values: `egress`, `ingress`

* `--ethertype <ETHERTYPE>` — ethertype query parameter for /v2.0/security-group-rules API

  Possible values: `IPv4`, `IPv6`

* `--id <ID>` — id query parameter for /v2.0/security-group-rules API
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--normalized-cidr <NORMALIZED_CIDR>` — normalized_cidr query parameter for /v2.0/security-group-rules API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--port-range-max <PORT_RANGE_MAX>` — port_range_max query parameter for /v2.0/security-group-rules API
* `--port-range-min <PORT_RANGE_MIN>` — port_range_min query parameter for /v2.0/security-group-rules API
* `--protocol <PROTOCOL>` — protocol query parameter for /v2.0/security-group-rules API
* `--remote-address-group-id <REMOTE_ADDRESS_GROUP_ID>` — remote_address_group_id query parameter for /v2.0/security-group-rules API
* `--remote-group-id <REMOTE_GROUP_ID>` — remote_group_id query parameter for /v2.0/security-group-rules API
* `--remote-ip-prefix <REMOTE_IP_PREFIX>` — remote_ip_prefix query parameter for /v2.0/security-group-rules API
* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/security-group-rules API
* `--security-group-id <SECURITY_GROUP_ID>` — security_group_id query parameter for /v2.0/security-group-rules API
* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/security-group-rules API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network security-group-rule set`

Command without description in OpenAPI

**Usage:** `osc network security-group-rule set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/security-group-rules/{id} API

###### **Options:**

* `--description <DESCRIPTION>`



## `osc network security-group-rule show`

Shows detailed information for a security group rule.

The response body contains the following information about the security group rule:

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network security-group-rule show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/security-group-rules/{id} API



## `osc network subnet`

Subnet commands

**Usage:** `osc network subnet <COMMAND>`

###### **Subcommands:**

* `create` — Create subnet
* `delete` — Delete subnet
* `list` — List subnets
* `set` — Update subnet
* `show` — Show subnet details
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network subnet create`

Creates a subnet on a network.

OpenStack Networking does not try to derive the correct IP version from the CIDR. If you do not specify the `gateway_ip` attribute, OpenStack Networking allocates an address from the CIDR for the gateway for the subnet.

To specify a subnet without a gateway, set the `gateway_ip` attribute to `null` in the request body. If you do not specify the `allocation_pools` attribute, OpenStack Networking automatically allocates pools for covering all IP addresses in the CIDR, excluding the address reserved for the subnet gateway. Otherwise, you can explicitly specify allocation pools as shown in the following example.

When you specify both the `allocation_pools` and `gateway_ip` attributes, you must ensure that the gateway IP does not overlap with the allocation pools; otherwise, the call returns the `Conflict (409)` response code.

A subnet can have one or more name servers and host routes. Hosts in this subnet use the name servers. Devices with IP addresses from this subnet, not including the local subnet route, use the host routes.

Specify the `ipv6_ra_mode` and `ipv6_address_mode` attributes to create subnets that support IPv6 configurations, such as stateless address autoconfiguration (SLAAC), DHCPv6 stateful, and DHCPv6 stateless configurations.

A subnet can optionally be associated with a network segment when it is created by specifying the `segment_id` of a valid segment on the specified network. A network with subnets associated in this way is called a routed network. On any given network, all of the subnets must be associated with segments or none of them can be. Neutron enforces this invariant. Currently, routed networks are only supported for provider networks.

Normal response codes: 201

Error response codes: 400, 401, 403, 404, 409

**Usage:** `osc network subnet create [OPTIONS] --ip-version <IP_VERSION> --network-id <NETWORK_ID>`

###### **Options:**

* `--allocation-pools <JSON>` — Allocation pools with `start` and `end` IP addresses for this subnet. If allocation_pools are not specified, OpenStack Networking automatically allocates pools for covering all IP addresses in the CIDR, excluding the address reserved for the subnet gateway by default
* `--cidr <CIDR>` — The CIDR of the subnet
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--dns-nameservers <DNS_NAMESERVERS>` — List of dns name servers associated with the subnet. Default is an empty list
* `--dns-publish-fixed-ip <DNS_PUBLISH_FIXED_IP>` — Whether to publish DNS records for IPs from this subnet. Default is `false`

  Possible values: `true`, `false`

* `--enable-dhcp <ENABLE_DHCP>` — Indicates whether dhcp is enabled or disabled for the subnet. Default is `true`

  Possible values: `true`, `false`

* `--gateway-ip <GATEWAY_IP>` — Gateway IP of this subnet. If the value is `null` that implies no gateway is associated with the subnet. If the gateway_ip is not specified, OpenStack Networking allocates an address from the CIDR for the gateway for the subnet by default
* `--host-routes <JSON>` — Additional routes for the subnet. A list of dictionaries with `destination` and `nexthop` parameters. Default value is an empty list
* `--ip-version <IP_VERSION>` — The IP protocol version. Value is `4` or `6`
* `--ipv6-address-mode <IPV6_ADDRESS_MODE>` — The IPv6 address modes specifies mechanisms for assigning IP addresses. Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless`

  Possible values: `dhcpv6-stateful`, `dhcpv6-stateless`, `slaac`

* `--ipv6-ra-mode <IPV6_RA_MODE>` — The IPv6 router advertisement specifies whether the networking service should transmit ICMPv6 packets, for a subnet. Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless`

  Possible values: `dhcpv6-stateful`, `dhcpv6-stateless`, `slaac`

* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--network-id <NETWORK_ID>` — The ID of the network to which the subnet belongs
* `--prefixlen <PREFIXLEN>` — The prefix length to use for subnet allocation from a subnet pool. If not specified, the `default_prefixlen` value of the subnet pool will be used
* `--segment-id <SEGMENT_ID>` — The ID of a network segment the subnet is associated with. It is available when `segment` extension is enabled
* `--service-types <SERVICE_TYPES>` — The service types associated with the subnet
* `--subnetpool-id <SUBNETPOOL_ID>` — The ID of the subnet pool associated with the subnet
* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies
* `--use-default-subnetpool <USE_DEFAULT_SUBNETPOOL>` — Whether to allocate this subnet from the default subnet pool

  Possible values: `true`, `false`




## `osc network subnet delete`

Deletes a subnet.

The operation fails if subnet IP addresses are still allocated.

Normal response codes: 204

Error response codes: 401, 404, 412

**Usage:** `osc network subnet delete <ID>`

###### **Arguments:**

* `<ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id} API



## `osc network subnet list`

Lists subnets that the project has access to.

Default policy settings return only subnets owned by the project of the user submitting the request, unless the user has administrative role. You can control which attributes are returned by using the fields query parameter. You can filter results by using query string parameters.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network subnet list [OPTIONS]`

###### **Options:**

* `--cidr <CIDR>` — cidr query parameter for /v2.0/subnets API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/subnets API
* `--enable-dhcp <ENABLE_DHCP>` — enable_dhcp query parameter for /v2.0/subnets API

  Possible values: `true`, `false`

* `--gateway-ip <GATEWAY_IP>` — gateway_ip query parameter for /v2.0/subnets API
* `--id <ID>` — id query parameter for /v2.0/subnets API
* `--ip-version <IP_VERSION>` — ip_version query parameter for /v2.0/subnets API
* `--ipv6-address-mode <IPV6_ADDRESS_MODE>` — ipv6_address_mode query parameter for /v2.0/subnets API

  Possible values: `dhcpv6-stateful`, `dhcpv6-stateless`, `slaac`

* `--ipv6-ra-mode <IPV6_RA_MODE>` — ipv6_ra_mode query parameter for /v2.0/subnets API

  Possible values: `dhcpv6-stateful`, `dhcpv6-stateless`, `slaac`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — name query parameter for /v2.0/subnets API
* `--network-id <NETWORK_ID>` — network_id query parameter for /v2.0/subnets API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/subnets API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/subnets API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/subnets API
* `--router-external <ROUTER_EXTERNAL>` — The membership of a subnet to an external network

  Possible values: `true`, `false`

* `--segment-id <SEGMENT_ID>` — segment_id query parameter for /v2.0/subnets API
* `--shared <SHARED>` — shared query parameter for /v2.0/subnets API

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--subnetpool-id <SUBNETPOOL_ID>` — subnetpool_id query parameter for /v2.0/subnets API
* `--tags <TAGS>` — tags query parameter for /v2.0/subnets API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/subnets API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/subnets API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network subnet set`

Updates a subnet.

Some attributes, such as IP version (ip_version), CIDR (cidr), and segment (segment_id) cannot be updated. Attempting to update these attributes results in a `400 Bad Request` error.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 412

**Usage:** `osc network subnet set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id} API

###### **Options:**

* `--allocation-pools <JSON>` — Allocation pools with `start` and `end` IP addresses for this subnet. If allocation_pools are not specified, OpenStack Networking automatically allocates pools for covering all IP addresses in the CIDR, excluding the address reserved for the subnet gateway by default
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--dns-nameservers <DNS_NAMESERVERS>` — List of dns name servers associated with the subnet. Default is an empty list
* `--dns-publish-fixed-ip <DNS_PUBLISH_FIXED_IP>` — Whether to publish DNS records for IPs from this subnet. Default is `false`

  Possible values: `true`, `false`

* `--enable-dhcp <ENABLE_DHCP>` — Indicates whether dhcp is enabled or disabled for the subnet. Default is `true`

  Possible values: `true`, `false`

* `--gateway-ip <GATEWAY_IP>` — Gateway IP of this subnet. If the value is `null` that implies no gateway is associated with the subnet. If the gateway_ip is not specified, OpenStack Networking allocates an address from the CIDR for the gateway for the subnet by default
* `--host-routes <JSON>` — Additional routes for the subnet. A list of dictionaries with `destination` and `nexthop` parameters. Default value is an empty list
* `--name <NAME>` — Human-readable name of the resource
* `--segment-id <SEGMENT_ID>` — The ID of a network segment the subnet is associated with. It is available when `segment` extension is enabled
* `--service-types <SERVICE_TYPES>` — The service types associated with the subnet



## `osc network subnet show`

Shows details for a subnet.

Use the fields query parameter to filter the results.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network subnet show <ID>`

###### **Arguments:**

* `<ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id} API



## `osc network subnet tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network subnet tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network subnet tag add`

Command without description in OpenAPI

**Usage:** `osc network subnet tag add <SUBNET_ID> <ID>`

###### **Arguments:**

* `<SUBNET_ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API



## `osc network subnet tag check`

Command without description in OpenAPI

**Usage:** `osc network subnet tag check <SUBNET_ID> <ID>`

###### **Arguments:**

* `<SUBNET_ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API



## `osc network subnet tag delete`

Command without description in OpenAPI

**Usage:** `osc network subnet tag delete <SUBNET_ID> <ID>`

###### **Arguments:**

* `<SUBNET_ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API



## `osc network subnet tag list`

Command without description in OpenAPI

**Usage:** `osc network subnet tag list [OPTIONS] <SUBNET_ID>`

###### **Arguments:**

* `<SUBNET_ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network subnet tag purge`

Command without description in OpenAPI

**Usage:** `osc network subnet tag purge <SUBNET_ID>`

###### **Arguments:**

* `<SUBNET_ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id}/tags/{id} API



## `osc network subnetpool`

SubnetPool commands

**Usage:** `osc network subnetpool <COMMAND>`

###### **Subcommands:**

* `add-prefixes` — Add prefixes
* `create` — Create subnet pool
* `delete` — Delete subnet pool
* `list` — List subnet pools
* `remove-prefixes` — Remove prefixes
* `set` — Update subnet pool
* `show` — Show subnet pool
* `tag` — Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource



## `osc network subnetpool add-prefixes`

Add prefixes

**Usage:** `osc network subnetpool add-prefixes [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/subnetpools/{id}/remove_prefixes API

###### **Options:**

* `--property <key=value>`



## `osc network subnetpool create`

Creates a subnet pool.

Normal response codes: 201

Error response codes: 400, 401, 403, 404

**Usage:** `osc network subnetpool create [OPTIONS]`

###### **Options:**

* `--address-scope-id <ADDRESS_SCOPE_ID>` — An address scope to assign to the subnet pool
* `--default-prefixlen <DEFAULT_PREFIXLEN>` — The size of the prefix to allocate when the `cidr` or `prefixlen` attributes are omitted when you create the subnet. Default is `min_prefixlen`
* `--default-quota <DEFAULT_QUOTA>` — A per-project quota on the prefix space that can be allocated from the subnet pool for project subnets. Default is no quota is enforced on allocations from the subnet pool. For IPv4 subnet pools, `default_quota` is measured in units of /32. For IPv6 subnet pools, `default_quota` is measured units of /64. All projects that use the subnet pool have the same prefix quota applied
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--is-default <IS_DEFAULT>` — The subnetpool is default pool or not

  Possible values: `true`, `false`

* `--max-prefixlen <MAX_PREFIXLEN>` — The maximum prefix size that can be allocated from the subnet pool. For IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is `128`
* `--min-prefixlen <MIN_PREFIXLEN>` — The smallest prefix that can be allocated from a subnet pool. For IPv4 subnet pools, default is `8`. For IPv6 subnet pools, default is `64`
* `--name <NAME>` — Human-readable name of the resource
* `--prefixes <PREFIXES>` — A list of subnet prefixes to assign to the subnet pool. The API merges adjacent prefixes and treats them as a single prefix. Each subnet prefix must be unique among all subnet prefixes in all subnet pools that are associated with the address scope
* `--shared <SHARED>` — Indicates whether this resource is shared across all projects. By default, only administrative users can change this value

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — The ID of the project



## `osc network subnetpool delete`

Deletes a subnet pool.

The operation fails if any subnets allocated from the subnet pool are still in use.

Normal response codes: 204

Error response codes: 401, 404, 412

**Usage:** `osc network subnetpool delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/subnetpools/{id}/remove_prefixes API



## `osc network subnetpool list`

Lists subnet pools that the project has access to.

Default policy settings return only the subnet pools owned by the project of the user submitting the request, unless the user has administrative role.

Standard query parameters are supported on the URI. For more information, see [Filtering and Column Selection](#filtering).

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Pagination query parameters are supported if Neutron configuration supports it by overriding `allow_pagination=false`. For more information, see [Pagination](#pagination).

Sorting query parameters are supported if Neutron configuration supports it with `allow_sorting=true`. For more information, see [Sorting](#sorting).

Normal response codes: 200

Error response codes: 401

**Usage:** `osc network subnetpool list [OPTIONS]`

###### **Options:**

* `--address-scope-id <ADDRESS_SCOPE_ID>` — address_scope_id query parameter for /v2.0/subnetpools API
* `--default-prefixlen <DEFAULT_PREFIXLEN>` — default_prefixlen query parameter for /v2.0/subnetpools API
* `--default-quota <DEFAULT_QUOTA>` — default_quota query parameter for /v2.0/subnetpools API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/subnetpools API
* `--id <ID>` — id query parameter for /v2.0/subnetpools API
* `--ip-version <IP_VERSION>` — ip_version query parameter for /v2.0/subnetpools API
* `--is-default <IS_DEFAULT>` — is_default query parameter for /v2.0/subnetpools API

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--max-prefixlen <MAX_PREFIXLEN>` — max_prefixlen query parameter for /v2.0/subnetpools API
* `--min-prefixlen <MIN_PREFIXLEN>` — min_prefixlen query parameter for /v2.0/subnetpools API
* `--name <NAME>` — name query parameter for /v2.0/subnetpools API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/subnetpools API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/subnetpools API
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/subnetpools API
* `--shared <SHARED>` — shared query parameter for /v2.0/subnetpools API

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--tags <TAGS>` — tags query parameter for /v2.0/subnetpools API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/subnetpools API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/subnetpools API
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network subnetpool remove-prefixes`

Remove prefixes

**Usage:** `osc network subnetpool remove-prefixes [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/subnetpools/{id}/remove_prefixes API

###### **Options:**

* `--property <key=value>`



## `osc network subnetpool set`

Updates a subnet pool.

Normal response codes: 200

Error response codes: 400, 401, 403, 404, 412

**Usage:** `osc network subnetpool set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/subnetpools/{id}/remove_prefixes API

###### **Options:**

* `--address-scope-id <ADDRESS_SCOPE_ID>` — An address scope to assign to the subnet pool
* `--default-prefixlen <DEFAULT_PREFIXLEN>` — The size of the prefix to allocate when the `cidr` or `prefixlen` attributes are omitted when you create the subnet. Default is `min_prefixlen`
* `--default-quota <DEFAULT_QUOTA>` — A per-project quota on the prefix space that can be allocated from the subnet pool for project subnets. Default is no quota is enforced on allocations from the subnet pool. For IPv4 subnet pools, `default_quota` is measured in units of /32. For IPv6 subnet pools, `default_quota` is measured units of /64. All projects that use the subnet pool have the same prefix quota applied
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--is-default <IS_DEFAULT>` — The subnetpool is default pool or not

  Possible values: `true`, `false`

* `--max-prefixlen <MAX_PREFIXLEN>` — The maximum prefix size that can be allocated from the subnet pool. For IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is `128`
* `--min-prefixlen <MIN_PREFIXLEN>` — The smallest prefix that can be allocated from a subnet pool. For IPv4 subnet pools, default is `8`. For IPv6 subnet pools, default is `64`
* `--name <NAME>` — Human-readable name of the resource
* `--prefixes <PREFIXES>` — A list of subnet prefixes to assign to the subnet pool. The API merges adjacent prefixes and treats them as a single prefix. Each subnet prefix must be unique among all subnet prefixes in all subnet pools that are associated with the address scope



## `osc network subnetpool show`

Shows information for a subnet pool.

Use the `fields` query parameter to control which fields are returned in the response body. For more information, see [Fields](#fields).

Normal response codes: 200

Error response codes: 401, 404

**Usage:** `osc network subnetpool show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/subnetpools/{id}/remove_prefixes API



## `osc network subnetpool tag`

Lists tags, creates, replaces or deletes one or more tags for a resource, checks the existence of a tag for a resource

**Usage:** `osc network subnetpool tag <COMMAND>`

###### **Subcommands:**

* `add` — Command without description in OpenAPI
* `check` — Command without description in OpenAPI
* `delete` — Command without description in OpenAPI
* `list` — Command without description in OpenAPI
* `purge` — Command without description in OpenAPI



## `osc network subnetpool tag add`

Command without description in OpenAPI

**Usage:** `osc network subnetpool tag add <SUBNETPOOL_ID> <ID>`

###### **Arguments:**

* `<SUBNETPOOL_ID>` — subnetpool_id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API



## `osc network subnetpool tag check`

Command without description in OpenAPI

**Usage:** `osc network subnetpool tag check <SUBNETPOOL_ID> <ID>`

###### **Arguments:**

* `<SUBNETPOOL_ID>` — subnetpool_id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API



## `osc network subnetpool tag delete`

Command without description in OpenAPI

**Usage:** `osc network subnetpool tag delete <SUBNETPOOL_ID> <ID>`

###### **Arguments:**

* `<SUBNETPOOL_ID>` — subnetpool_id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API



## `osc network subnetpool tag list`

Command without description in OpenAPI

**Usage:** `osc network subnetpool tag list [OPTIONS] <SUBNETPOOL_ID>`

###### **Arguments:**

* `<SUBNETPOOL_ID>` — subnetpool_id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--page-reverse <PAGE_REVERSE>` — Reverse the page direction

  Possible values: `true`, `false`

* `--sort-dir <SORT_DIR>` — Sort direction. This is an optional feature and may be silently ignored by the server

  Possible values: `asc`, `desc`

* `--sort-key <SORT_KEY>` — Sort results by the attribute. This is an optional feature and may be silently ignored by the server
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network subnetpool tag purge`

Command without description in OpenAPI

**Usage:** `osc network subnetpool tag purge <SUBNETPOOL_ID>`

###### **Arguments:**

* `<SUBNETPOOL_ID>` — subnetpool_id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id} API



## `osc object-store`

Object Store service (Swift) commands

**Usage:** `osc object-store <COMMAND>`

###### **Subcommands:**

* `account` — Account commands
* `container` — Container commands
* `object` — Object commands



## `osc object-store account`

Account commands

**Usage:** `osc object-store account <COMMAND>`

###### **Subcommands:**

* `show` — Shows metadata for an account. Because the storage system can store large amounts of data, take care when you represent the total bytes response as an integer; when possible, convert it to a 64-bit unsigned integer if your platform supports that primitive type. Do not include metadata headers in this request
* `set` — Creates, updates, or deletes account metadata. To create, update, or delete custom metadata, use the X-Account-Meta-{name} request header, where {name} is the name of the metadata item. Account metadata operations work differently than how object metadata operations work. Depending on the contents of your POST account metadata request, the Object Storage API updates the metadata as shown in the following table: TODO: fill the rest To delete a metadata header, send an empty value for that header, such as for the X-Account-Meta-Book header. If the tool you use to communicate with Object Storage, such as an older version of cURL, does not support empty headers, send the X-Remove-Account- Meta-{name} header with an arbitrary value. For example, X-Remove-Account-Meta-Book: x. The operation ignores the arbitrary value



## `osc object-store account show`

Shows metadata for an account. Because the storage system can store large amounts of data, take care when you represent the total bytes response as an integer; when possible, convert it to a 64-bit unsigned integer if your platform supports that primitive type. Do not include metadata headers in this request

**Usage:** `osc object-store account show`



## `osc object-store account set`

Creates, updates, or deletes account metadata. To create, update, or delete custom metadata, use the X-Account-Meta-{name} request header, where {name} is the name of the metadata item. Account metadata operations work differently than how object metadata operations work. Depending on the contents of your POST account metadata request, the Object Storage API updates the metadata as shown in the following table: TODO: fill the rest To delete a metadata header, send an empty value for that header, such as for the X-Account-Meta-Book header. If the tool you use to communicate with Object Storage, such as an older version of cURL, does not support empty headers, send the X-Remove-Account- Meta-{name} header with an arbitrary value. For example, X-Remove-Account-Meta-Book: x. The operation ignores the arbitrary value

**Usage:** `osc object-store account set [OPTIONS]`

###### **Options:**

* `--property <key=value>` — Property to be set



## `osc object-store container`

Container commands

**Usage:** `osc object-store container <COMMAND>`

###### **Subcommands:**

* `create` — Creates a container. You do not need to check whether a container already exists before issuing a PUT operation because the operation is idempotent: It creates a container or updates an existing container, as appropriate
* `delete` — Deletes an empty container. This operation fails unless the container is empty. An empty container has no objects
* `list` — Shows details for an account and lists containers, sorted by name, in the account
* `prune` — Prune objects in a container
* `set` — Creates, updates, or deletes custom metadata for a container
* `show` — Shows container metadata, including the number of objects and the total bytes of all objects stored in the container



## `osc object-store container create`

Creates a container. You do not need to check whether a container already exists before issuing a PUT operation because the operation is idempotent: It creates a container or updates an existing container, as appropriate

**Usage:** `osc object-store container create <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container



## `osc object-store container delete`

Deletes an empty container. This operation fails unless the container is empty. An empty container has no objects

**Usage:** `osc object-store container delete <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container



## `osc object-store container list`

Shows details for an account and lists containers, sorted by name, in the account

**Usage:** `osc object-store container list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — For an integer value n, limits the number of results to n
* `--marker <MARKER>` — For a string value, x, constrains the list to items whose names are greater than x
* `--end-marker <END_MARKER>` — For a string value, x, constrains the list to items whose names are less than x
* `--format <FORMAT>` — The response format. Valid values are json, xml, or plain. The default is plain. If you append the format=xml or format=json query parameter to the storage account URL, the response shows extended container information serialized in that format. If you append the format=plain query parameter, the response lists the container names separated by newlines
* `--prefix <PREFIX>` — Only objects with this prefix will be returned. When combined with a delimiter query, this enables API users to simulate and traverse the objects in a container as if they were in a directory tree
* `--delimiter <DELIMITER>` — The delimiter is a single character used to split object names to present a pseudo-directory hierarchy of objects. When combined with a prefix query, this enables API users to simulate and traverse the objects in a container as if they were in a directory tree
* `--reverse <REVERSE>` — By default, listings are returned sorted by name, ascending. If you include the reverse=true query parameter, the listing will be returned sorted by name, descending

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc object-store container prune`

Prune objects in a container

**Usage:** `osc object-store container prune [OPTIONS] <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name

###### **Options:**

* `--prefix <PREFIX>` — Only objects with this prefix will be deleted



## `osc object-store container set`

Creates, updates, or deletes custom metadata for a container

**Usage:** `osc object-store container set [OPTIONS] <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container

###### **Options:**

* `--property <key=value>` — Property to be set



## `osc object-store container show`

Shows container metadata, including the number of objects and the total bytes of all objects stored in the container

**Usage:** `osc object-store container show <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container



## `osc object-store object`

Object commands

**Usage:** `osc object-store object <COMMAND>`

###### **Subcommands:**

* `delete` — Permanently deletes an object from the object store. Object deletion occurs immediately at request time. Any subsequent GET, HEAD, POST, or DELETE operations will return a 404 Not Found error code. For static large object manifests, you can add the ?multipart- manifest=delete query parameter. This operation deletes the segment objects and, if all deletions succeed, this operation deletes the manifest object. A DELETE request made to a symlink path will delete the symlink rather than the target object. An alternative to using the DELETE operation is to use the POST operation with the bulk-delete query parameter
* `download` — Downloads the object content and gets the object metadata. This operation returns the object metadata in the response headers and the object content in the response body
* `list` — Shows details for a container and lists objects, sorted by name, in the container. Specify query parameters in the request to filter the list and return a subset of objects. Omit query parameters to return a list of objects that are stored in the container, up to 10,000 names. The 10,000 maximum value is configurable. To view the value for the cluster, issue a GET /info request
* `show` — Shows object metadata
* `upload` — Creates an object with data content and metadata, or replaces an existing object with data content and metadata. The PUT operation always creates an object. If you use this operation on an existing object, you replace the existing object and metadata rather than modifying the object. Consequently, this operation returns the Created (201) response code. If you use this operation to copy a manifest object, the new object is a normal object and not a copy of the manifest. Instead it is a concatenation of all the segment objects. This means that you cannot copy objects larger than 5 GB. Note that the provider may have limited the characters which are allowed in an object name. Any name limits are exposed under the name_check key in the /info discoverability response. Regardless of name_check limitations, names must be URL quoted UTF-8. To create custom metadata, use the X-Object-Meta-name header, where name is the name of the metadata item



## `osc object-store object delete`

Permanently deletes an object from the object store. Object deletion occurs immediately at request time. Any subsequent GET, HEAD, POST, or DELETE operations will return a 404 Not Found error code. For static large object manifests, you can add the ?multipart- manifest=delete query parameter. This operation deletes the segment objects and, if all deletions succeed, this operation deletes the manifest object. A DELETE request made to a symlink path will delete the symlink rather than the target object. An alternative to using the DELETE operation is to use the POST operation with the bulk-delete query parameter

**Usage:** `osc object-store object delete [OPTIONS] <CONTAINER> <OBJECT>`

###### **Arguments:**

* `<CONTAINER>` — The unique name for the account. An account is also known as the project or tenant
* `<OBJECT>` — The unique name for the object

###### **Options:**

* `--multipart-manifest <MULTIPART_MANIFEST>` — If you include the multipart-manifest=get query parameter and the object is a large object, the object contents are not returned. Instead, the manifest is returned in the X-Object-Manifest response header for dynamic large objects or in the response body for static large objects



## `osc object-store object download`

Downloads the object content and gets the object metadata. This operation returns the object metadata in the response headers and the object content in the response body

**Usage:** `osc object-store object download [OPTIONS] <CONTAINER> <OBJECT>`

###### **Arguments:**

* `<CONTAINER>` — The unique name for the account. An account is also known as the project or tenant
* `<OBJECT>` — The unique name for the object

###### **Options:**

* `--multipart-manifest <MULTIPART_MANIFEST>` — If you include the multipart-manifest=get query parameter and the object is a large object, the object contents are not returned. Instead, the manifest is returned in the X-Object-Manifest response header for dynamic large objects or in the response body for static large objects
* `--temp-url-sig <TEMP_URL_SIG>` — Used with temporary URLs to sign the request with an HMAC-SHA1 cryptographic signature that defines the allowed HTTP method, expiration date, full path to the object, and the secret key for the temporary URL. For more information about temporary URLs, see Temporary URL middleware
* `--temp-url-expires <TEMP_URL_EXPIRES>` — The date and time in UNIX Epoch time stamp format or ISO 8601 UTC timestamp when the signature for temporary URLs expires. For example, 1440619048 or 2015-08-26T19:57:28Z is equivalent to Mon, Wed, 26 Aug 2015 19:57:28 GMT. For more information about temporary URLs, see Temporary URL middleware
* `--filename <FILENAME>` — Overrides the default file name. Object Storage generates a default file name for GET temporary URLs that is based on the object name. Object Storage returns this value in the Content-Disposition response header. Browsers can interpret this file name value as a file attachment to save. For more information about temporary URLs, see Temporary URL middleware
* `--symlink <SYMLINK>` — If you include the symlink=get query parameter and the object is a symlink, then the response will include data and metadata from the symlink itself rather than from the target
* `--file <FILE>` — Destination filename (using "-" will print object to stdout)



## `osc object-store object list`

Shows details for a container and lists objects, sorted by name, in the container. Specify query parameters in the request to filter the list and return a subset of objects. Omit query parameters to return a list of objects that are stored in the container, up to 10,000 names. The 10,000 maximum value is configurable. To view the value for the cluster, issue a GET /info request

**Usage:** `osc object-store object list [OPTIONS] <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container

###### **Options:**

* `--limit <LIMIT>` — For an integer value n, limits the number of results to n
* `--marker <MARKER>` — For a string value, x, constrains the list to items whose names are greater than x
* `--end-marker <END_MARKER>` — For a string value, x, constrains the list to items whose names are less than x
* `--format <FORMAT>` — The response format. Valid values are json, xml, or plain. The default is plain. If you append the format=xml or format=json query parameter to the storage account URL, the response shows extended container information serialized in that format. If you append the format=plain query parameter, the response lists the container names separated by newlines
* `--prefix <PREFIX>` — Only objects with this prefix will be returned. When combined with a delimiter query, this enables API users to simulate and traverse the objects in a container as if they were in a directory tree
* `--delimiter <DELIMITER>` — The delimiter is a single character used to split object names to present a pseudo-directory hierarchy of objects. When combined with a prefix query, this enables API users to simulate and traverse the objects in a container as if they were in a directory tree
* `--reverse <REVERSE>` — By default, listings are returned sorted by name, ascending. If you include the reverse=true query parameter, the listing will be returned sorted by name, descending

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc object-store object show`

Shows object metadata

**Usage:** `osc object-store object show [OPTIONS] <CONTAINER> <OBJECT>`

###### **Arguments:**

* `<CONTAINER>` — The unique name for the account. An account is also known as the project or tenant
* `<OBJECT>` — The unique name for the object

###### **Options:**

* `--multipart-manifest <MULTIPART_MANIFEST>` — If you include the multipart-manifest=get query parameter and the object is a large object, the object contents are not returned. Instead, the manifest is returned in the X-Object-Manifest response header for dynamic large objects or in the response body for static large objects
* `--temp-url-sig <TEMP_URL_SIG>` — Used with temporary URLs to sign the request with an HMAC-SHA1 cryptographic signature that defines the allowed HTTP method, expiration date, full path to the object, and the secret key for the temporary URL. For more information about temporary URLs, see Temporary URL middleware
* `--temp-url-expires <TEMP_URL_EXPIRES>` — The date and time in UNIX Epoch time stamp format or ISO 8601 UTC timestamp when the signature for temporary URLs expires. For example, 1440619048 or 2015-08-26T19:57:28Z is equivalent to Mon, Wed, 26 Aug 2015 19:57:28 GMT. For more information about temporary URLs, see Temporary URL middleware
* `--filename <FILENAME>` — Overrides the default file name. Object Storage generates a default file name for GET temporary URLs that is based on the object name. Object Storage returns this value in the Content-Disposition response header. Browsers can interpret this file name value as a file attachment to save. For more information about temporary URLs, see Temporary URL middleware
* `--symlink <SYMLINK>` — If you include the symlink=get query parameter and the object is a symlink, then the response will include data and metadata from the symlink itself rather than from the target



## `osc object-store object upload`

Creates an object with data content and metadata, or replaces an existing object with data content and metadata. The PUT operation always creates an object. If you use this operation on an existing object, you replace the existing object and metadata rather than modifying the object. Consequently, this operation returns the Created (201) response code. If you use this operation to copy a manifest object, the new object is a normal object and not a copy of the manifest. Instead it is a concatenation of all the segment objects. This means that you cannot copy objects larger than 5 GB. Note that the provider may have limited the characters which are allowed in an object name. Any name limits are exposed under the name_check key in the /info discoverability response. Regardless of name_check limitations, names must be URL quoted UTF-8. To create custom metadata, use the X-Object-Meta-name header, where name is the name of the metadata item

**Usage:** `osc object-store object upload [OPTIONS] <CONTAINER> <OBJECT>`

###### **Arguments:**

* `<CONTAINER>` — The unique name for the account. An account is also known as the project or tenant
* `<OBJECT>` — The unique name for the object

###### **Options:**

* `--multipart-manifest <MULTIPART_MANIFEST>` — If you include the multipart-manifest=get query parameter and the object is a large object, the object contents are not returned. Instead, the manifest is returned in the X-Object-Manifest response header for dynamic large objects or in the response body for static large objects
* `--temp-url-sig <TEMP_URL_SIG>` — Used with temporary URLs to sign the request with an HMAC-SHA1 cryptographic signature that defines the allowed HTTP method, expiration date, full path to the object, and the secret key for the temporary URL. For more information about temporary URLs, see Temporary URL middleware
* `--temp-url-expires <TEMP_URL_EXPIRES>` — The date and time in UNIX Epoch time stamp format or ISO 8601 UTC timestamp when the signature for temporary URLs expires. For example, 1440619048 or 2015-08-26T19:57:28Z is equivalent to Mon, Wed, 26 Aug 2015 19:57:28 GMT. For more information about temporary URLs, see Temporary URL middleware
* `--filename <FILENAME>` — Overrides the default file name. Object Storage generates a default file name for GET temporary URLs that is based on the object name. Object Storage returns this value in the Content-Disposition response header. Browsers can interpret this file name value as a file attachment to save. For more information about temporary URLs, see Temporary URL middleware
* `--symlink <SYMLINK>` — If you include the symlink=get query parameter and the object is a symlink, then the response will include data and metadata from the symlink itself rather than from the target
* `--file <FILE>` — Source filename (using "-" will read object from stdout)



## `osc completion`

Output shell completion code for the specified shell (bash, zsh, fish, or powershell). The shell code must be evaluated to provide interactive completion of `osc` commands.  This can be done by sourcing it from the .bash_profile.

Examples:

Enable completion at a shell start:

`echo 'source <(osc completion bash)' >>~/.bashrc`

**Usage:** `osc completion [SHELL]`

###### **Arguments:**

* `<SHELL>` — If provided, outputs the completion file for given shell

  Default value: `bash`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
