# Command-Line Help for `osc`

This document contains the help content for the `osc` command-line program.

**Command Overview:**

* [`osc`↴](#osc)
* [`osc api`↴](#osc-api)
* [`osc auth`↴](#osc-auth)
* [`osc auth login`↴](#osc-auth-login)
* [`osc auth show`↴](#osc-auth-show)
* [`osc block-storage`↴](#osc-block-storage)
* [`osc block-storage volume`↴](#osc-block-storage-volume)
* [`osc block-storage volume create`↴](#osc-block-storage-volume-create)
* [`osc block-storage volume create30`↴](#osc-block-storage-volume-create30)
* [`osc block-storage volume create313`↴](#osc-block-storage-volume-create313)
* [`osc block-storage volume create347`↴](#osc-block-storage-volume-create347)
* [`osc block-storage volume create353`↴](#osc-block-storage-volume-create353)
* [`osc block-storage volume delete`↴](#osc-block-storage-volume-delete)
* [`osc block-storage volume extend`↴](#osc-block-storage-volume-extend)
* [`osc block-storage volume list`↴](#osc-block-storage-volume-list)
* [`osc block-storage volume set`↴](#osc-block-storage-volume-set)
* [`osc block-storage volume set353`↴](#osc-block-storage-volume-set353)
* [`osc block-storage volume set30`↴](#osc-block-storage-volume-set30)
* [`osc block-storage volume show`↴](#osc-block-storage-volume-show)
* [`osc catalog`↴](#osc-catalog)
* [`osc catalog list`↴](#osc-catalog-list)
* [`osc compute`↴](#osc-compute)
* [`osc compute extension`↴](#osc-compute-extension)
* [`osc compute extension list`↴](#osc-compute-extension-list)
* [`osc compute extension show`↴](#osc-compute-extension-show)
* [`osc compute server`↴](#osc-compute-server)
* [`osc compute server list`↴](#osc-compute-server-list)
* [`osc compute server show`↴](#osc-compute-server-show)
* [`osc compute server pause`↴](#osc-compute-server-pause)
* [`osc compute flavor`↴](#osc-compute-flavor)
* [`osc compute flavor list`↴](#osc-compute-flavor-list)
* [`osc compute flavor show`↴](#osc-compute-flavor-show)
* [`osc compute keypair`↴](#osc-compute-keypair)
* [`osc compute keypair list`↴](#osc-compute-keypair-list)
* [`osc compute keypair show`↴](#osc-compute-keypair-show)
* [`osc compute keypair create`↴](#osc-compute-keypair-create)
* [`osc compute keypair create292`↴](#osc-compute-keypair-create292)
* [`osc compute keypair create210`↴](#osc-compute-keypair-create210)
* [`osc compute keypair create22`↴](#osc-compute-keypair-create22)
* [`osc compute keypair create21`↴](#osc-compute-keypair-create21)
* [`osc compute keypair create20`↴](#osc-compute-keypair-create20)
* [`osc compute keypair delete`↴](#osc-compute-keypair-delete)
* [`osc identity`↴](#osc-identity)
* [`osc identity application-credential`↴](#osc-identity-application-credential)
* [`osc identity application-credential create`↴](#osc-identity-application-credential-create)
* [`osc identity application-credential delete`↴](#osc-identity-application-credential-delete)
* [`osc identity application-credential list`↴](#osc-identity-application-credential-list)
* [`osc identity application-credential show`↴](#osc-identity-application-credential-show)
* [`osc identity access-rule`↴](#osc-identity-access-rule)
* [`osc identity access-rule delete`↴](#osc-identity-access-rule-delete)
* [`osc identity access-rule list`↴](#osc-identity-access-rule-list)
* [`osc identity access-rule show`↴](#osc-identity-access-rule-show)
* [`osc identity project`↴](#osc-identity-project)
* [`osc identity project create`↴](#osc-identity-project-create)
* [`osc identity project delete`↴](#osc-identity-project-delete)
* [`osc identity project list`↴](#osc-identity-project-list)
* [`osc identity project set`↴](#osc-identity-project-set)
* [`osc identity project show`↴](#osc-identity-project-show)
* [`osc identity user`↴](#osc-identity-user)
* [`osc identity user create`↴](#osc-identity-user-create)
* [`osc identity user delete`↴](#osc-identity-user-delete)
* [`osc identity user list`↴](#osc-identity-user-list)
* [`osc identity user set`↴](#osc-identity-user-set)
* [`osc identity user show`↴](#osc-identity-user-show)
* [`osc identity user password`↴](#osc-identity-user-password)
* [`osc identity user password set`↴](#osc-identity-user-password-set)
* [`osc identity user projects`↴](#osc-identity-user-projects)
* [`osc identity user groups`↴](#osc-identity-user-groups)
* [`osc image`↴](#osc-image)
* [`osc image image`↴](#osc-image-image)
* [`osc image image list`↴](#osc-image-image-list)
* [`osc image image show`↴](#osc-image-image-show)
* [`osc image image create`↴](#osc-image-image-create)
* [`osc image image set`↴](#osc-image-image-set)
* [`osc image image download`↴](#osc-image-image-download)
* [`osc image image upload`↴](#osc-image-image-upload)
* [`osc image image delete`↴](#osc-image-image-delete)
* [`osc image image deactivate`↴](#osc-image-image-deactivate)
* [`osc image image reactivate`↴](#osc-image-image-reactivate)
* [`osc image schema`↴](#osc-image-schema)
* [`osc image schema image`↴](#osc-image-schema-image)
* [`osc image schema image show`↴](#osc-image-schema-image-show)
* [`osc image schema images`↴](#osc-image-schema-images)
* [`osc image schema images show`↴](#osc-image-schema-images-show)
* [`osc image schema member`↴](#osc-image-schema-member)
* [`osc image schema member show`↴](#osc-image-schema-member-show)
* [`osc image schema members`↴](#osc-image-schema-members)
* [`osc image schema members show`↴](#osc-image-schema-members-show)
* [`osc network`↴](#osc-network)
* [`osc network availability-zone`↴](#osc-network-availability-zone)
* [`osc network availability-zone list`↴](#osc-network-availability-zone-list)
* [`osc network extension`↴](#osc-network-extension)
* [`osc network extension list`↴](#osc-network-extension-list)
* [`osc network extension show`↴](#osc-network-extension-show)
* [`osc network floating-ip`↴](#osc-network-floating-ip)
* [`osc network floating-ip create`↴](#osc-network-floating-ip-create)
* [`osc network floating-ip delete`↴](#osc-network-floating-ip-delete)
* [`osc network floating-ip list`↴](#osc-network-floating-ip-list)
* [`osc network floating-ip set`↴](#osc-network-floating-ip-set)
* [`osc network floating-ip show`↴](#osc-network-floating-ip-show)
* [`osc network floating-ip tag`↴](#osc-network-floating-ip-tag)
* [`osc network floating-ip tag add`↴](#osc-network-floating-ip-tag-add)
* [`osc network floating-ip tag check`↴](#osc-network-floating-ip-tag-check)
* [`osc network floating-ip tag delete`↴](#osc-network-floating-ip-tag-delete)
* [`osc network floating-ip tag list`↴](#osc-network-floating-ip-tag-list)
* [`osc network floating-ip tag purge`↴](#osc-network-floating-ip-tag-purge)
* [`osc network floating-ip tag replace`↴](#osc-network-floating-ip-tag-replace)
* [`osc network network`↴](#osc-network-network)
* [`osc network network list`↴](#osc-network-network-list)
* [`osc network network show`↴](#osc-network-network-show)
* [`osc network network create`↴](#osc-network-network-create)
* [`osc network network delete`↴](#osc-network-network-delete)
* [`osc network port`↴](#osc-network-port)
* [`osc network port list`↴](#osc-network-port-list)
* [`osc network port show`↴](#osc-network-port-show)
* [`osc network port create`↴](#osc-network-port-create)
* [`osc network port delete`↴](#osc-network-port-delete)
* [`osc network router`↴](#osc-network-router)
* [`osc network router list`↴](#osc-network-router-list)
* [`osc network router show`↴](#osc-network-router-show)
* [`osc network router create`↴](#osc-network-router-create)
* [`osc network router delete`↴](#osc-network-router-delete)
* [`osc network subnet`↴](#osc-network-subnet)
* [`osc network subnet list`↴](#osc-network-subnet-list)
* [`osc network subnet show`↴](#osc-network-subnet-show)
* [`osc network subnet create`↴](#osc-network-subnet-create)
* [`osc network subnet delete`↴](#osc-network-subnet-delete)
* [`osc object-store`↴](#osc-object-store)
* [`osc object-store account`↴](#osc-object-store-account)
* [`osc object-store account show`↴](#osc-object-store-account-show)
* [`osc object-store account set`↴](#osc-object-store-account-set)
* [`osc object-store container`↴](#osc-object-store-container)
* [`osc object-store container list`↴](#osc-object-store-container-list)
* [`osc object-store container show`↴](#osc-object-store-container-show)
* [`osc object-store container set`↴](#osc-object-store-container-set)
* [`osc object-store container create`↴](#osc-object-store-container-create)
* [`osc object-store container delete`↴](#osc-object-store-container-delete)
* [`osc object-store object`↴](#osc-object-store-object)
* [`osc object-store object list`↴](#osc-object-store-object-list)
* [`osc object-store object download`↴](#osc-object-store-object-download)
* [`osc object-store object upload`↴](#osc-object-store-object-upload)
* [`osc object-store object show`↴](#osc-object-store-object-show)
* [`osc object-store object delete`↴](#osc-object-store-object-delete)

## `osc`

OpenStack client rewritten in Rust

**Usage:** `osc [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `api` — Perform direct REST API requests with authorization
* `auth` — Cloud Authentication operations
* `block-storage` — Block Storage (Volume) service (Cinder) commands
* `catalog` — Shows current catalog information
* `compute` — Compute service (Nova) commands
* `identity` — Identity (Keystone) commands
* `image` — Image (Glance) commands
* `network` — Network (Neutron) commands
* `object-store` — Object Store service (Swift) commands

###### **Options:**

* `--os-cloud <OS_CLOUD>` — Name reference to the clouds.yaml entry for the cloud configuration
* `-o`, `--output <OUTPUT>` — Output format

  Possible values:
  - `json`:
    Json output
  - `yaml`:
    YAML output
  - `wide`:
    Wide (Human readable table with extra attributes)

* `-f`, `--fields <FIELDS>` — Fields to return in the output (only in normal and wide mode)
* `-v`, `--verbose` — Verbosity level. Repeat to increase level



## `osc api`

Perform direct REST API requests with authorization

**Usage:** `osc api [OPTIONS] <SERVICE> <URL>`

###### **Arguments:**

* `<SERVICE>` — Service name
* `<URL>` — Rest URL (relative to the endpoint information from the service catalog). Do not start URL with the "/" to respect endpoint version information

###### **Options:**

* `-m`, `--method <METHOD>` — HTTP Method

  Default value: `get`

  Possible values:
  - `head`:
    HEAD
  - `get`:
    GET
  - `put`:
    PUT
  - `post`:
    POST
  - `delete`:
    DELETE

* `--header <key=value>` — Additional headers
* `--body <BODY>` — Request body to be used



## `osc auth`

Cloud Authentication operations

**Usage:** `osc auth
       auth <COMMAND>`

###### **Subcommands:**

* `login` — Login to the cloud and get a valid authorization token
* `show` — Show current auth information



## `osc auth login`

Fetch a new valid authorization token for the cloud.

This command writes token to the stdout

**Usage:** `osc auth login [OPTIONS]`

###### **Options:**

* `--renew` — Require token renewal

  Possible values: `true`, `false`




## `osc auth show`

Show current authorization information for the cloud

This command returns authentication and authorization information for the currently active connection. It includes issue and expiration information, user data, list of granted roles and project/domain information.

**NOTE**: The command does not support selecting individual fields in the output, but it supports `-o json` command and returns full available information in json format what allows further processing with `jq`

**Usage:** `osc auth show`



## `osc block-storage`

Block Storage (Volume) service (Cinder) commands

**Usage:** `osc block-storage [OPTIONS]
       block-storage <COMMAND>`

###### **Subcommands:**

* `volume` — Volume commands

###### **Options:**

* `--os-volume-api-version <OS_VOLUME_API_VERSION>` — BlockStorage API microversion



## `osc block-storage volume`

Volume commands

**Usage:** `osc block-storage volume <COMMAND>`

###### **Subcommands:**

* `create` — Create new volume (with highest possible microversion)
* `create30` — Create new volume (microversion = 3.0)
* `create313` — Create new volume (microversion = 3.13)
* `create347` — Create new volume (microversion = 3.47)
* `create353` — Create new volume (microversion = 3.53)
* `delete` — Delete volume
* `extend` — Extend volume
* `list` — List Volumes
* `set` — Updates a volume (highest possible microversion).
* `set353` — Updates a volume (microversion = 3.53).
* `set30` — Updates a volume (microversion = 3.0).
* `show` — Show single volume details



## `osc block-storage volume create`

Create volume (with highest possible microversion)

To create a bootable volume, include the UUID of the image from which you want to create the volume in the imageRef attribute in the request body.

Since the Train release, every volume must have a volume type. It is optional to specify a volume type as part of your Create a volume request. If you do not specify one, a default volume type will be supplied for you. This type may vary according to what project you are in and how the operator has configured the Block Storage service. Use the Show default volume type request to determine your effective default volume type.

**Preconditions**

- You must have enough volume storage quota remaining to create a volume of size requested.

**Asynchronous Postconditions**

- With correct permissions, you can see the volume status as available through API calls.

- With correct access, you can see the created volume in the storage system that OpenStack Block Storage manages.

**Troubleshooting**

-  If volume status remains creating or shows another error status, the request failed. Ensure you meet the preconditions then investigate the storage back end.

- Volume is not created in the storage system that OpenStack Block Storage manages.

- The storage node needs enough free storage space to match the size of the volume creation request.

**Usage:** `osc block-storage volume create [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The volume name
* `--description <DESCRIPTION>` — The volume description
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage- multi-backend.html)
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--multiattach <MULTIATTACH>` — To enable this volume to attach to more than one server, set this value to `true`. Default is `false`. Note that support for multiattach volumes depends on the volume type being used. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--group-id <GROUP_ID>`
* `--backup-id <BACKUP_ID>` — The UUID of the backup
* `--os-sch-hnt-scheduler-hints <key=value>`



## `osc block-storage volume create30`

Create new volume (microversion = 3.0)

**Usage:** `osc block-storage volume create30 [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The volume name
* `--description <DESCRIPTION>` — The volume description
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage- multi-backend.html)
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--multiattach <MULTIATTACH>` — To enable this volume to attach to more than one server, set this value to `true`. Default is `false`. Note that support for multiattach volumes depends on the volume type being used. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--os-sch-hnt-scheduler-hints <key=value>`



## `osc block-storage volume create313`

Create new volume (microversion = 3.13)

**Usage:** `osc block-storage volume create313 [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The volume name
* `--description <DESCRIPTION>` — The volume description
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage- multi-backend.html)
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--multiattach <MULTIATTACH>` — To enable this volume to attach to more than one server, set this value to `true`. Default is `false`. Note that support for multiattach volumes depends on the volume type being used. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--group-id <GROUP_ID>`
* `--os-sch-hnt-scheduler-hints <key=value>`



## `osc block-storage volume create347`

Create new volume (microversion = 3.47)

**Usage:** `osc block-storage volume create347 [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The volume name
* `--description <DESCRIPTION>` — The volume description
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage- multi-backend.html)
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--multiattach <MULTIATTACH>` — To enable this volume to attach to more than one server, set this value to `true`. Default is `false`. Note that support for multiattach volumes depends on the volume type being used. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--group-id <GROUP_ID>`
* `--backup-id <BACKUP_ID>` — The UUID of the backup
* `--os-sch-hnt-scheduler-hints <key=value>`



## `osc block-storage volume create353`

Create new volume (microversion = 3.53)

**Usage:** `osc block-storage volume create353 [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The volume name
* `--description <DESCRIPTION>` — The volume description
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--volume-type <VOLUME_TYPE>` — The volume type (either name or ID). To create an environment with multiple-storage back ends, you must specify a volume type. Block Storage volume back ends are spawned as children to `cinder- volume`, and they are keyed from a unique queue. They are named `cinder- volume.HOST.BACKEND`. For example, `cinder- volume.ubuntu.lvmdriver`. When a volume is created, the scheduler chooses an appropriate back end to handle the request based on the volume type. Default is `None`. For information about how to use volume types to create multiple- storage back ends, see [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage- multi-backend.html)
* `--metadata <key=value>` — One or more metadata key and value pairs to be associated with the new volume
* `--snapshot-id <SNAPSHOT_ID>` — The UUID of the consistency group
* `--source-volid <SOURCE_VOLID>` — The UUID of the consistency group
* `--consistencygroup-id <CONSISTENCYGROUP_ID>` — The UUID of the consistency group
* `--size <SIZE>` — The size of the volume, in gibibytes (GiB)
* `--availability-zone <AVAILABILITY_ZONE>` — The name of the availability zone
* `--multiattach <MULTIATTACH>` — To enable this volume to attach to more than one server, set this value to `true`. Default is `false`. Note that support for multiattach volumes depends on the volume type being used. See [valid boolean values](#valid-boolean-values)

  Possible values: `true`, `false`

* `--image-id <IMAGE_ID>`
* `--image-ref <IMAGE_REF>` — The UUID of the image from which you want to create the volume. Required to create a bootable volume
* `--group-id <GROUP_ID>`
* `--backup-id <BACKUP_ID>` — The UUID of the backup
* `--os-sch-hnt-scheduler-hints <key=value>`



## `osc block-storage volume delete`

Deletes a volume.

**Preconditions**

- Volume status must be available, in-use, error, error_restoring, error_extending, error_managing, and must not be migrating, attached, awaiting-transfer, belong to a group, have snapshots or be disassociated from snapshots after volume transfer.

- The cascade option can be passed in the request if you want all snapshots of this volume to be deleted automatically, which should allow the volume deletion to succeed.

- You cannot delete a volume that is in a migration.

**Asynchronous Postconditions**

- The volume is deleted in volume index.

- The volume managed by OpenStack Block Storage is deleted in storage node.

**Troubleshooting**

- If volume status remains in deleting or becomes error_deleting the request failed. Ensure you meet the preconditions then investigate the storage back end.

- The volume managed by OpenStack Block Storage is not deleted from the storage system.

**Usage:** `osc block-storage volume delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API



## `osc block-storage volume extend`

Extends the size of a volume to a requested size, in gibibytes (GiB). Specify the os-extend action in the request body.

**Preconditions**

- Prior to microversion 3.42 the volume status must be available. Starting with microversion 3.42, attached volumes with status in-use may be able to be extended depending on policy and backend volume and compute driver constraints in the cloud. Note that reserved is not a valid state for extend.

- Sufficient amount of storage must exist to extend the volume.

- The user quota must have sufficient volume storage.

**Postconditions**

- If the request is processed successfully, the volume status will change to extending while the volume size is being extended.

- Upon successful completion of the extend operation, the volume status will go back to its original value.

- Starting with microversion 3.42, when extending the size of an attached volume, the Block Storage service will notify the Compute service that an attached volume has been extended. The Compute service will asynchronously process the volume size change for the related server instance. This can be monitored using the GET /servers/{server_id}/os-instance-actions API in the Compute service.

**Troubleshooting**

- An error_extending volume status indicates that the request failed. Ensure that you meet the preconditions and retry the request. If the request fails again, investigate the storage back end.

**Usage:** `osc block-storage volume extend --new-size <NEW_SIZE> <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API

###### **Options:**

* `--new-size <NEW_SIZE>`



## `osc block-storage volume list`

List Volumes

**Usage:** `osc block-storage volume list [OPTIONS]`

###### **Options:**

* `--all-tenans <ALL_TENANS>` — Shows details for all project. Admin only

  Possible values: `true`, `false`

* `--sort <SORT>` — Comma-separated list of sort keys and optional sort directions in the form of < key > [: < direction > ]. A valid direction is asc (ascending) or desc (descending)
* `--sort-key <SORT_KEY>` — Sorts by an attribute. A valid value is name, status, container_format, disk_format, size, id, created_at, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key attribute value. Deprecated in favour of the combined sort parameter
* `--sort-dir <SORT_DIR>` — Sorts by one or more sets of attribute and sort direction combinations. If you omit the sort direction in a set, default is desc. Deprecated in favour of the combined sort parameter
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--offset <OFFSET>` — Used in conjunction with limit to return a slice of items. offset is where to start in the list
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--with-count <WITH_COUNT>` — Whether to show count in API response or not, default is False

  Possible values: `true`, `false`

* `--created-at <CREATED_AT>` — Filters reuslts by a time that resources are created at with time comparison operators: gt/gte/eq/neq/lt/lte
* `--updated-at <UPDATED_AT>` — Filters reuslts by a time that resources are updated at with time comparison operators: gt/gte/eq/neq/lt/lte
* `--consumes-quota <CONSUMES_QUOTA>` — Filters results by consumes_quota field. Resources that don’t use quotas are usually temporary internal resources created to perform an operation. Default is to not filter by it. Filtering by this option may not be always possible in a cloud, see List Resource Filters to determine whether this filter is available in your cloud

  Possible values: `true`, `false`

* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage volume set`

Updates a volume (highest possible microversion).

**Usage:** `osc block-storage volume set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API

###### **Options:**

* `--name <NAME>`
* `--description <DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--metadata <key=value>`



## `osc block-storage volume set353`

Updates a volume (microversion = 3.53).

**Usage:** `osc block-storage volume set353 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API

###### **Options:**

* `--name <NAME>`
* `--description <DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--metadata <key=value>`



## `osc block-storage volume set30`

Updates a volume (microversion = 3.0).

**Usage:** `osc block-storage volume set30 [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API

###### **Options:**

* `--name <NAME>`
* `--description <DESCRIPTION>`
* `--display-name <DISPLAY_NAME>`
* `--display-description <DISPLAY_DESCRIPTION>`
* `--metadata <key=value>`



## `osc block-storage volume show`

Shows details for a volume.

**Preconditions**

- The volume must exist.

**Usage:** `osc block-storage volume show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v3/volumes/{id} API



## `osc catalog`

Shows current catalog information

**Usage:** `osc catalog <COMMAND>`

###### **Subcommands:**

* `list` — List catalog command arguments



## `osc catalog list`

List catalog command arguments

**Usage:** `osc catalog list`



## `osc compute`

Compute service (Nova) commands

**Usage:** `osc compute
       compute <COMMAND>`

###### **Subcommands:**

* `extension` — Extension commands
* `server` — Server (VM) commands
* `flavor` — Flavor commands
* `keypair` — Keypair commands



## `osc compute extension`

Extension commands

**Usage:** `osc compute extension <COMMAND>`

###### **Subcommands:**

* `list` — List Extensions
* `show` — Show single extension



## `osc compute extension list`

List Extensions

**Usage:** `osc compute extension list`



## `osc compute extension show`

Show single extension

**Usage:** `osc compute extension show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/extensions/{id} API



## `osc compute server`

Server (VM) commands

**Usage:** `osc compute server
       server <COMMAND>`

###### **Subcommands:**

* `list` — List Servers
* `show` — Show single Server
* `pause` — Pause Server



## `osc compute server list`

List Servers

**Usage:** `osc compute server list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — limit filter parameter
* `--marker <MARKER>` — marker filter parameter
* `--auto-disk-config <AUTO_DISK_CONFIG>` — auto_disk_config filter parameter
* `--availability-zone <AVAILABILITY_ZONE>` — availability_zone filter parameter
* `--created-at <CREATED_AT>` — created_at filter parameter
* `--description <DESCRIPTION>` — description filter parameter
* `--flavor <FLAVOR>` — flavor filter parameter
* `--hostname <HOSTNAME>` — hostname filter parameter
* `--image <IMAGE>` — image filter parameter
* `--kernel-id <KERNEL_ID>` — kernel_id filter parameter
* `--key-name <KEY_NAME>` — key_name filter parameter
* `--launch-index <LAUNCH_INDEX>` — launch_index filter parameter
* `--launched-at <LAUNCHED_AT>` — launched_at filter parameter
* `--locked-by <LOCKED_BY>` — locked_by filter parameter
* `--name <NAME>` — name filter parameter
* `--node <NODE>` — node filter parameter
* `--power-state <POWER_STATE>` — power_state filter parameter
* `--progress <PROGRESS>` — progress filter parameter
* `--project-id <PROJECT_ID>` — project_id filter parameter
* `--ramdisk-id <RAMDISK_ID>` — ramdisk_id filter parameter
* `--reservation-id <RESERVATION_ID>` — reservation_id filter parameter
* `--root-device-name <ROOT_DEVICE_NAME>` — root_device_name filter parameter
* `--status <STATUS>` — status filter parameter
* `--task-state <TASK_STATE>` — task_state filter parameter
* `--terminated-at <TERMINATED_AT>` — terminated_at filter parameter
* `--user-id <USER_ID>` — user_id filter parameter
* `--vm-state <VM_STATE>` — vm_state filter parameter
* `--sort-key <SORT_KEY>` — sort_key filter parameter
* `--sort-dir <SORT_DIR>` — sort_dir filter parameter
* `--access-ipv4 <ACCESS_IPV4>` — access_ipv4 filter parameter
* `--access-ipv6 <ACCESS_IPV6>` — access_ipv6 filter parameter
* `--has-config-drive <HAS_CONFIG_DRIVE>` — has_config_drive filter parameter
* `--deleted-only <DELETED_ONLY>` — deleted_only filter parameter
* `--compute-host <COMPUTE_HOST>` — compute_host filter parameter
* `--is-soft-deleted <IS_SOFT_DELETED>` — is_soft_deleted filter parameter
* `--ipv4-address <IPV4_ADDRESS>` — ipv4_address filter parameter
* `--ipv6-address <IPV6_ADDRESS>` — ipv6_address filter parameter
* `--changes-since <CHANGES_SINCE>` — changes_since filter parameter
* `--changes-before <CHANGES_BEFORE>` — changes_before filter parameter
* `--id <ID>` — id filter parameter
* `--all-projects <ALL_PROJECTS>` — all_projects filter parameter
* `--tags <TAGS>` — tags filter parameter
* `--any-tags <ANY_TAGS>` — any_tags filter parameter
* `--not-tags <NOT_TAGS>` — not_tags filter parameter
* `--not-any-tags <NOT_ANY_TAGS>` — not_any_tags filter parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute server show`

Show single Server

**Usage:** `osc compute server show <ID>`

###### **Arguments:**

* `<ID>` — Server ID



## `osc compute server pause`

Pause Server

**Usage:** `osc compute server pause <ID>`

###### **Arguments:**

* `<ID>` — Server ID



## `osc compute flavor`

Flavor commands

**Usage:** `osc compute flavor <COMMAND>`

###### **Subcommands:**

* `list` — List Servers
* `show` — Show single Server



## `osc compute flavor list`

List Servers

**Usage:** `osc compute flavor list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>`
* `--marker <MARKER>`
* `--is-public <IS_PUBLIC>`
* `--min-ram <MIN_RAM>`
* `--min-disk <MIN_DISK>`
* `--sort-key <SORT_KEY>`
* `--sort-dir <SORT_DIR>`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute flavor show`

Show single Server

**Usage:** `osc compute flavor show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/flavors/{id}/action API



## `osc compute keypair`

Keypair commands

**Usage:** `osc compute keypair <COMMAND>`

###### **Subcommands:**

* `list` — List keypairs
* `show` — Show single keypair details
* `create` — Imports (or generates) Keypair (with highest possible microversion)
* `create292` — Import keypair (microversion >= 2.92)
* `create210` — Import (or generate) keypair (2.10 <= microversion < 2.92)
* `create22` — Import (or generate) keypair (2.2 <= microversion < 2.10)
* `create21` — Import (or generate) keypair (2.1 <= microversion < 2.2)
* `create20` — Import (or generate) keypair (microversion == 2.0)
* `delete` — Delete keypair



## `osc compute keypair list`

List keypairs

**Usage:** `osc compute keypair list [OPTIONS]`

###### **Options:**

* `--user-id <USER_ID>`
* `--limit <LIMIT>`
* `--marker <MARKER>`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute keypair show`

Show single keypair details

Shows details for a keypair that is associated with the account.

**Usage:** `osc compute keypair show [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-keypairs/{id} API

###### **Options:**

* `--user-id <USER_ID>`



## `osc compute keypair create`

Imports (or generates) Keypair (with highest possible microversion)

**Usage:** `osc compute keypair create [OPTIONS] --name <NAME> --public-key <PUBLIC_KEY>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`

  Possible values: `ssh`, `x509`

* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--user-id <USER_ID>` — The user\_id for a keypair. This allows administrative users to upload keys for other users than themselves



## `osc compute keypair create292`

Import keypair (microversion >= 2.92)

**Usage:** `osc compute keypair create292 [OPTIONS] --name <NAME> --public-key <PUBLIC_KEY>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`

  Possible values: `ssh`, `x509`

* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--user-id <USER_ID>` — The user\_id for a keypair. This allows administrative users to upload keys for other users than themselves



## `osc compute keypair create210`

Import (or generate) keypair (2.10 <= microversion < 2.92)

**Usage:** `osc compute keypair create210 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`

  Possible values: `ssh`, `x509`

* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--user-id <USER_ID>` — The user\_id for a keypair. This allows administrative users to upload keys for other users than themselves



## `osc compute keypair create22`

Import (or generate) keypair (2.2 <= microversion < 2.10)

**Usage:** `osc compute keypair create22 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later
* `--type <TYPE>` — The type of the keypair. Allowed values are `ssh` or `x509`

  Possible values: `ssh`, `x509`

* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you



## `osc compute keypair create21`

Import (or generate) keypair (2.1 <= microversion < 2.2)

**Usage:** `osc compute keypair create21 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you



## `osc compute keypair create20`

Import (or generate) keypair (microversion == 2.0)

**Usage:** `osc compute keypair create20 [OPTIONS] --name <NAME>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you



## `osc compute keypair delete`

Delete keypair

**Usage:** `osc compute keypair delete [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.1/os-keypairs/{id} API

###### **Options:**

* `--user-id <USER_ID>`



## `osc identity`

Identity (Keystone) commands

**Usage:** `osc identity
       identity <COMMAND>`

###### **Subcommands:**

* `application-credential` — **Application Credentials**
* `access-rule` — **Application Credentials - Access Rules**
* `project` — Project commands
* `user` — User commands



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

Create application credential

**Usage:** `osc identity application-credential create [OPTIONS] --name <NAME> <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--name <NAME>` — The name of the application credential. Must be unique to a user
* `--description <DESCRIPTION>` — A description of the application credential’s purpose
* `--secret <SECRET>` — The secret that the application credential will be created with. If not provided, one will be generated
* `--expires-at <EXPIRES_AT>` — An optional expiry time for the application credential. If unset, the application credential does not expire
* `--roles <JSON>` — An optional list of role objects, identified by ID or name. The list may only contain roles that the user has assigned on the project. If not provided, the roles assigned to the application credential will be the same as the roles in the current token
* `--unrestricted <UNRESTRICTED>` — An optional flag to restrict whether the application credential may be used for the creation or destruction of other application credentials or trusts. Defaults to false

  Possible values: `true`, `false`

* `--access-rules <JSON>` — A list of `access\_rules` objects



## `osc identity application-credential delete`

Delete application credential

**Usage:** `osc identity application-credential delete <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — application_credential_id parameter for /v3/users/{user_id}/application_credentials/{application_credential_id} API



## `osc identity application-credential list`

List application credentials

**Usage:** `osc identity application-credential list [OPTIONS] <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--name <NAME>` — The name of the application credential. Must be unique to a user



## `osc identity application-credential show`

Show application credential details

**Usage:** `osc identity application-credential show <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — application_credential_id parameter for /v3/users/{user_id}/application_credentials/{application_credential_id} API



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

Delete access rule

**Usage:** `osc identity access-rule delete <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — access_rule_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity access-rule list`

List access rules

**Usage:** `osc identity access-rule list <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity access-rule show`

Show access rule details

**Usage:** `osc identity access-rule show <USER_ID> <ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API
* `<ID>` — access_rule_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity project`

Project commands

**Usage:** `osc identity project <COMMAND>`

###### **Subcommands:**

* `create` — Create project
* `delete` — Delete project
* `list` — List Projects
* `set` — Update project details
* `show` — Show project details



## `osc identity project create`

Create project

**Usage:** `osc identity project create [OPTIONS] --name <NAME>`

###### **Options:**

* `--description <DESCRIPTION>` — The description of the project
* `--domain-id <DOMAIN_ID>` — The ID of the domain for the project
* `--enabled <ENABLED>` — If set to `true`, project is enabled. If set to `false`, project is disabled. The default is `true`

  Possible values: `true`, `false`

* `--is-domain <IS_DOMAIN>` — If set to `true`, project is enabled. If set to `false`, project is disabled. The default is `true`

  Possible values: `true`, `false`

* `--parent-id <PARENT_ID>` — The ID of the parent of the project
* `--name <NAME>` — The name of the project, which must be unique within the owning domain. A project can have the same name as its domain
* `--tags <TAGS>` — A list of simple strings assigned to a project. Tags can be used to classify projects into groups
* `--immutable <IMMUTABLE>`

  Possible values: `true`, `false`




## `osc identity project delete`

Delete project

**Usage:** `osc identity project delete <ID>`

###### **Arguments:**

* `<ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API



## `osc identity project list`

List Projects

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

Update project details

**Usage:** `osc identity project set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API

###### **Options:**

* `--description <DESCRIPTION>` — The description of the project
* `--domain-id <DOMAIN_ID>` — The ID of the new domain for the project. The ability to change the domain of a project is now deprecated, and will be removed in subequent release. It is already disabled by default in most Identity service implementations
* `--enabled <ENABLED>` — If set to `true`, project is enabled. If set to `false`, project is disabled

  Possible values: `true`, `false`

* `--is-domain <IS_DOMAIN>` — If set to `true`, project is enabled. If set to `false`, project is disabled

  Possible values: `true`, `false`

* `--parent-id <PARENT_ID>`
* `--name <NAME>` — The name of the project, which must be unique within the owning domain. A project can have the same name as its domain
* `--tags <TAGS>` — A list of simple strings assigned to a project. Tags can be used to classify projects into groups
* `--immutable <IMMUTABLE>`

  Possible values: `true`, `false`




## `osc identity project show`

Show project details

**Usage:** `osc identity project show <ID>`

###### **Arguments:**

* `<ID>` — project_id parameter for /v3/projects/{project_id}/groups/{group_id}/roles API



## `osc identity user`

User commands

A user is an individual API consumer that is owned by a domain. A role explicitly associates a user with projects or domains. A user with no assigned roles has no access to OpenStack resources.

You can list, create, show details for, update, delete, and change the password for users.

You can also list groups, projects, and role assignments for a specified user.

**Usage:** `osc identity user <COMMAND>`

###### **Subcommands:**

* `create` — Create user
* `delete` — Delete user
* `list` — List Users
* `set` — Update user details
* `show` — Show user details
* `password` — User password operations
* `projects` — List projects for user
* `groups` — List groups to which a user belongs



## `osc identity user create`

Create user

**Usage:** `osc identity user create [OPTIONS] --name <NAME>`

###### **Options:**

* `--default-project-id <DEFAULT_PROJECT_ID>` — The ID of the default project for the user
* `--description <DESCRIPTION>` — The new description of the group
* `--domain-id <DOMAIN_ID>` — The ID of the domain
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--federated <JSON>` — List of federated objects associated with a user. Each object in the list contains the `idp\_id` and `protocols`. `protocols` is a list of objects, each of which contains `protocol\_id` and `unique\_id` of the protocol and user respectively. For example:
* `--name <NAME>` — The user name. Must be unique within the owning domain
* `--password <PASSWORD>` — The new password for the user
* `--ignore-change-password-upon-first-use <IGNORE_CHANGE_PASSWORD_UPON_FIRST_USE>`

  Possible values: `true`, `false`

* `--ignore-password-expiry <IGNORE_PASSWORD_EXPIRY>`

  Possible values: `true`, `false`

* `--ignore-lockout-failure-attempts <IGNORE_LOCKOUT_FAILURE_ATTEMPTS>`

  Possible values: `true`, `false`

* `--lock-password <LOCK_PASSWORD>`

  Possible values: `true`, `false`

* `--ignore-user-inactivity <IGNORE_USER_INACTIVITY>`

  Possible values: `true`, `false`

* `--multi-factor-auth-rules <MULTI_FACTOR_AUTH_RULES>`
* `--multi-factor-auth-enabled <MULTI_FACTOR_AUTH_ENABLED>`

  Possible values: `true`, `false`




## `osc identity user delete`

Delete user

**Usage:** `osc identity user delete <ID>`

###### **Arguments:**

* `<ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity user list`

List Users

**Usage:** `osc identity user list [OPTIONS]`

###### **Options:**

* `--domain-id <DOMAIN_ID>` — Filters the response by a domain ID
* `--enabled <ENABLED>` — If set to true, then only enabled projects will be returned. Any value other than 0 (including no value) will be interpreted as true

  Possible values: `true`, `false`

* `--idp-id <IDP_ID>` — Filters the response by a domain ID
* `--name <NAME>` — Filters the response by a resource name
* `--password-expires-at <PASSWORD_EXPIRES_AT>` — Filter results based on which user passwords have expired. The query should include an operator and a timestamp with a colon (:) separating the two, for example: `password_expires_at={operator}:{timestamp}`. Valid operators are: `lt`, `lte`, `gt`, `gte`, `eq`, and `neq`. Valid timestamps are of the form: YYYY-MM-DDTHH:mm:ssZ
* `--protocol-id <PROTOCOL_ID>` — Filters the response by a protocol ID
* `--unique-id <UNIQUE_ID>` — Filters the response by a unique ID



## `osc identity user set`

Update user details

**Usage:** `osc identity user set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--default-project-id <DEFAULT_PROJECT_ID>` — The ID of the default project for the user
* `--description <DESCRIPTION>` — The new description of the group
* `--domain-id <DOMAIN_ID>` — The ID of the domain
* `--enabled <ENABLED>` — If the user is enabled, this value is `true`. If the user is disabled, this value is `false`

  Possible values: `true`, `false`

* `--federated <JSON>` — List of federated objects associated with a user. Each object in the list contains the `idp\_id` and `protocols`. `protocols` is a list of objects, each of which contains `protocol\_id` and `unique\_id` of the protocol and user respectively. For example:
* `--name <NAME>` — The user name. Must be unique within the owning domain
* `--password <PASSWORD>` — The new password for the user
* `--ignore-change-password-upon-first-use <IGNORE_CHANGE_PASSWORD_UPON_FIRST_USE>`

  Possible values: `true`, `false`

* `--ignore-password-expiry <IGNORE_PASSWORD_EXPIRY>`

  Possible values: `true`, `false`

* `--ignore-lockout-failure-attempts <IGNORE_LOCKOUT_FAILURE_ATTEMPTS>`

  Possible values: `true`, `false`

* `--lock-password <LOCK_PASSWORD>`

  Possible values: `true`, `false`

* `--ignore-user-inactivity <IGNORE_USER_INACTIVITY>`

  Possible values: `true`, `false`

* `--multi-factor-auth-rules <MULTI_FACTOR_AUTH_RULES>`
* `--multi-factor-auth-enabled <MULTI_FACTOR_AUTH_ENABLED>`

  Possible values: `true`, `false`




## `osc identity user show`

Show user details

**Usage:** `osc identity user show <ID>`

###### **Arguments:**

* `<ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity user password`

User password commands

This subcommand allows user to change the password

**Usage:** `osc identity user password <COMMAND>`

###### **Subcommands:**

* `set` — Update user password



## `osc identity user password set`

Update user password

**Usage:** `osc identity user password set [OPTIONS] <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API

###### **Options:**

* `--original-password <ORIGINAL_PASSWORD>` — The original password for the user
* `--password <PASSWORD>` — The new password for the user



## `osc identity user projects`

List projects for user

**Usage:** `osc identity user projects <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc identity user groups`

List groups to which a user belongs

**Usage:** `osc identity user groups <USER_ID>`

###### **Arguments:**

* `<USER_ID>` — user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id} API



## `osc image`

Image (Glance) commands

**Usage:** `osc image
       image <COMMAND>`

###### **Subcommands:**

* `image` — Image commands
* `schema` — Schema commands



## `osc image image`

Image commands

**Usage:** `osc image image
       image <COMMAND>`

###### **Subcommands:**

* `list` — List Images
* `show` — Show single image
* `create` — Create image
* `set` — Update image
* `download` — Download image data
* `upload` — Upload image data
* `delete` — Download image data
* `deactivate` — Deactivate image
* `reactivate` — Reactivate image



## `osc image image list`

Lists public virtual machine (VM) images.

*Pagination*

Returns a subset of the larger collection of images and a link that you can use to get the next set of images. You should always check for the presence of a next link and use it as the URI in a subsequent HTTP GET request. You should follow this pattern until a next link is no longer provided.

The next link preserves any query parameters that you send in your initial request. You can use the first link to jump back to the first page of the collection. If you prefer to paginate through images manually, use the limit and marker parameters.

*Query Filters*

The list operation accepts query parameters to filter the response.

A client can provide direct comparison filters by using most image attributes, such as name=Ubuntu, visibility=public, and so on.

To filter using image tags, use the filter tag (note the singular). To filter on multiple tags, include each tag separately in the query. For example, to find images with the tag ready, include tag=ready in your query string. To find images tagged with ready and approved, include tag=ready&tag=approved in your query string. (Note that only images containing both tags will be included in the response.)

A client cannot use any link in the json-schema, such as self, file, or schema, to filter the response.

You can list VM images that have a status of active, queued, or saving.

*The `in` Operator*

As a convenience, you may specify several values for any of the following fields by using the in operator: [container_format, disk_format, id, name, status]

For most of these, usage is straight forward. For example, to list images in queued or saving status, use: `--status "in:saving,queued"`

To find images in a particular list of image IDs, use: `--id "in:3afb79c1-131a-4c38-a87c-bc4b801d14e6,2e011209-660f-44b5-baf2-2eb4babae53d"

Using the in operator with the name property of images can be a bit trickier, depending upon how creatively you have named your images. The general rule is that if an image name contains a comma (,), you must enclose the entire name in quotation marks ("). As usual, you must URL encode any characters that require it.

For example, to find images named glass, darkly or share me, you would use the following filter specification: `--name: 'in:"glass,%20darkly",share%20me'`

As with regular filtering by name, you must specify the complete name you are looking for. Thus, for example, the query `--name "in:glass,share"` will only match images with the exact name glass or the exact name share. It will not find an image named glass, darkly or an image named share me.

*Size Comparison Filters*

You can use the size_min and size_max query parameters to filter images that are greater than or less than the image size. The size, in bytes, is the size of an image on disk.

For example, to filter the container to include only images that are from 1 to 4 MB, set the size_min query parameter to 1048576 and the size_max query parameter to 4194304.

*Time Comparison Filters*

You can use a comparison operator along with the created_at or updated_at fields to filter your results. Specify the operator first, a colon (:) as a separator, and then the time in ISO 8601 Format. Available comparison operators are: [gt, gte, eq, neq, lt, lte]

**Usage:** `osc image image list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--name <NAME>` — Filters the response by a name, as a string. A valid value is the name of an image
* `--id <ID>` — id filter parameter
* `--owner <OWNER>` — Filters the response by a project (also called a “tenant”) ID. Shows only images that are shared with you by the specified owner
* `--protected <PROTECTED>` — Filters the response by the ‘protected’ image property. A valid value is one of ‘true’, ‘false’ (must be all lowercase). Any other value will result in a 400 response

  Possible values: `true`, `false`

* `--status <STATUS>` — Filters the response by an image status
* `--tag <TAG>` — Filters the response by the specified tag value. May be repeated, but keep in mind that you’re making a conjunctive query, so only images containing all the tags specified will appear in the response
* `--visibility <VISIBILITY>` — Filters the response by an image visibility value. A valid value is public, private, community, shared, or all. (Note that if you filter on shared, the images included in the response will only be those where your member status is accepted unless you explicitly include a member_status filter in the request.) If you omit this parameter, the response shows public, private, and those shared images with a member status of accepted
* `--os-hidden <OS_HIDDEN>` — When true, filters the response to display only "hidden" images. By default, "hidden" images are not included in the image-list response. (Since Image API v2.7)

  Possible values: `true`, `false`

* `--member-status <MEMBER_STATUS>` — Filters the response by a member status. A valid value is accepted, pending, rejected, or all. Default is accepted
* `--size-max <SIZE_MAX>` — Filters the response by a maximum image size, in bytes
* `--size-min <SIZE_MIN>` — Filters the response by a minimum image size, in bytes
* `--created-at <CREATED_AT>` — Specify a comparison filter based on the date and time when the resource was created
* `--updated-at <UPDATED_AT>` — Specify a comparison filter based on the date and time when the resource was most recently modified
* `--sort-dir <SORT_DIR>` — Sorts the response by a set of one or more sort direction and attribute (sort_key) combinations. A valid value for the sort direction is asc (ascending) or desc (descending). If you omit the sort direction in a set, the default is desc
* `--sort-key <SORT_KEY>` — Sorts the response by an attribute, such as name, id, or updated_at. Default is created_at. The API uses the natural sorting direction of the sort_key image attribute
* `--sort <SORT>` — Sorts the response by one or more attribute and sort direction combinations. You can also set multiple sort keys and directions. Default direction is desc. Use the comma (,) character to separate multiple values. For example: `sort=name:asc,status:desc`
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc image image show`

Show single image

**Usage:** `osc image image show <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API



## `osc image image create`

Creates a catalog record for an operating system disk image. (Since Image API v2.0)

The Location response header contains the URI for the image.

A multiple store backend support is introduced in the Rocky release as a part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new header OpenStack-image-store-ids which contains the list of available stores will be included in response. This header is only included if multiple backend stores are supported.

The response body contains the new image entity.

*Synchronous Postconditions*

With correct permissions, you can see the image status as queued through API calls.

**Usage:** `osc image image create [OPTIONS]`

###### **Options:**

* `--id <ID>`
* `--name <NAME>`
* `--visibility <VISIBILITY>`

  Possible values: `community`, `private`, `public`, `shared`

* `--protected <PROTECTED>`

  Possible values: `true`, `false`

* `--os-hidden <OS_HIDDEN>`

  Possible values: `true`, `false`

* `--owner <OWNER>`
* `--container-format <CONTAINER_FORMAT>`

  Possible values: `aki`, `ami`, `ari`, `bare`, `compressed`, `docker`, `ova`, `ovf`

* `--disk-format <DISK_FORMAT>`

  Possible values: `aki`, `ami`, `ari`, `iso`, `ploop`, `qcow2`, `raw`, `vdi`, `vhd`, `vhdx`, `vmdk`

* `--tags <TAGS>`
* `--min-ram <MIN_RAM>`
* `--min-disk <MIN_DISK>`
* `--locations <JSON>`
* `--property <key=value>` — Additional properties to be sent with the request



## `osc image image set`

Update image

**Usage:** `osc image image set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API

###### **Options:**

* `--name <NAME>`
* `--visibility <VISIBILITY>`

  Possible values: `community`, `private`, `public`, `shared`

* `--protected <PROTECTED>`

  Possible values: `true`, `false`

* `--os-hidden <OS_HIDDEN>`

  Possible values: `true`, `false`

* `--owner <OWNER>`
* `--container-format <CONTAINER_FORMAT>`

  Possible values: `aki`, `ami`, `ari`, `bare`, `compressed`, `docker`, `ova`, `ovf`

* `--disk-format <DISK_FORMAT>`

  Possible values: `aki`, `ami`, `ari`, `iso`, `ploop`, `qcow2`, `raw`, `vdi`, `vhd`, `vhdx`, `vmdk`

* `--tags <TAGS>`
* `--min-ram <MIN_RAM>`
* `--min-disk <MIN_DISK>`
* `--locations <JSON>`
* `--property <key=value>` — Additional properties to be sent with the request



## `osc image image download`

Downloads binary image data. (Since Image API v2.0)

The response body contains the raw binary data that represents the actual virtual disk. The Content-Type header contains the application/octet-stream value. The Content-MD5 header contains an MD5 checksum of the image data. Use this checksum to verify the integrity of the image data.

*Preconditions*

- The image must exist.

*Synchronous Postconditions*

- You can download the binary image data in your machine if the image has image data.

- If image data exists, the call returns the HTTP 200 response code for a full image download request.

- If image data exists, the call returns the HTTP 206 response code for a partial download request.

- If no image data exists, the call returns the HTTP 204 (No Content) response code.

- If no image record exists, the call returns the HTTP 404 response code for an attempted full image download request.

- For an unsatisfiable partial download request, the call returns the HTTP 416 response code.

**Usage:** `osc image image download [OPTIONS] <IMAGE_ID>`

###### **Arguments:**

* `<IMAGE_ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API

###### **Options:**

* `--file <FILE>` — Destination filename (using "-" will print object to stdout)



## `osc image image upload`

Uploads binary image data.

These operation may be restricted to administrators. Consult your cloud operator’s documentation for details.

*Preconditions* Before you can store binary image data, you must meet the following preconditions:

- The image must exist.

- You must set the disk and container formats in the image.

- The image status must be queued.

- Your image storage quota must be sufficient.

- The size of the data that you want to store must not exceed the size that the OpenStack Image service allows.

*Synchronous Postconditions*

- With correct permissions, you can see the image status as active through API calls.

- With correct access, you can see the stored data in the storage system that the OpenStack Image Service manages.

*Troubleshooting*

- If you cannot store the data, either your request lacks required information or you exceeded your allotted quota. Ensure that you meet the preconditions and run the request again. If the request fails again, review your API request.

- The storage back ends for storing the data must have enough free storage space to accommodate the size of the data.

**Usage:** `osc image image upload [OPTIONS] <IMAGE_ID>`

###### **Arguments:**

* `<IMAGE_ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API

###### **Options:**

* `--file <FILE>` — Source filename (using "-" will read object from stdout)



## `osc image image delete`

Deletes an image.

You cannot delete images with the protected attribute set to true (boolean).

*Preconditions*

- You can delete an image in any status except deleted.

- The protected attribute of the image cannot be true.

- You have permission to perform image deletion under the configured image deletion policy.

*Synchronous Postconditions*

- The response is empty and returns the HTTP 204 response code.

- The API deletes the image from the images index.

-  If the image has associated binary image data in the storage backend, the OpenStack Image service deletes the data.

**Usage:** `osc image image delete <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API



## `osc image image deactivate`

Deactivates an image. (Since Image API v2.3)

By default, this operation is restricted to administrators only.

**Usage:** `osc image image deactivate <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API



## `osc image image reactivate`

Reactivates an image. (Since Image API v2.3)

By default, this operation is restricted to administrators only

**Usage:** `osc image image reactivate <ID>`

###### **Arguments:**

* `<ID>` — image_id parameter for /v2/images/{image_id}/members/{member_id} API



## `osc image schema`

Schema commands

**Usage:** `osc image schema
       schema <COMMAND>`

###### **Subcommands:**

* `image` — Show Image Schema
* `images` — Show Images Schema
* `member` — Show Member Schema
* `members` — Show Members Schema



## `osc image schema image`

Show Image Schema

**Usage:** `osc image schema image
       image <COMMAND>`

###### **Subcommands:**

* `show` — Show Image Schema



## `osc image schema image show`

Show Image Schema

**Usage:** `osc image schema image show`



## `osc image schema images`

Show Images Schema

**Usage:** `osc image schema images
       images <COMMAND>`

###### **Subcommands:**

* `show` — Show Images Schema



## `osc image schema images show`

Show Images Schema

**Usage:** `osc image schema images show`



## `osc image schema member`

Show Member Schema

**Usage:** `osc image schema member
       member <COMMAND>`

###### **Subcommands:**

* `show` — Show Member Schema



## `osc image schema member show`

Show Member Schema

**Usage:** `osc image schema member show`



## `osc image schema members`

Show Members Schema

**Usage:** `osc image schema members
       members <COMMAND>`

###### **Subcommands:**

* `show` — Show Members Schema



## `osc image schema members show`

Show Members Schema

**Usage:** `osc image schema members show`



## `osc network`

Network (Neutron) commands

**Usage:** `osc network
       network <COMMAND>`

###### **Subcommands:**

* `availability-zone` — Availability Zones commands
* `extension` — Extensions commands
* `floating-ip` — Floating IP commands
* `network` — Network commands
* `port` — Port commands
* `router` — Router commands
* `subnet` — Subnet commands



## `osc network availability-zone`

Availability Zones commands

**Usage:** `osc network availability-zone
       availability-zone <COMMAND>`

###### **Subcommands:**

* `list` — List AvailabilityZones



## `osc network availability-zone list`

List AvailabilityZones

**Usage:** `osc network availability-zone list [OPTIONS]`

###### **Options:**

* `--name <NAME>` — name query parameter for /v2.0/availability_zones API
* `--resource <RESOURCE>` — resource query parameter for /v2.0/availability_zones API
* `--state <STATE>` — state query parameter for /v2.0/availability_zones API



## `osc network extension`

Extensions commands

**Usage:** `osc network extension
       extension <COMMAND>`

###### **Subcommands:**

* `list` — List Extensions
* `show` — show Extensions



## `osc network extension list`

List Extensions

**Usage:** `osc network extension list`



## `osc network extension show`

show Extensions

**Usage:** `osc network extension show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/extensions/{id} API



## `osc network floating-ip`

Floating IP commands

**Usage:** `osc network floating-ip
       floating-ip <COMMAND>`

###### **Subcommands:**

* `create` — Create single FloatingIP
* `delete` — Delete single FloatingIP
* `list` — List FloatingIPs
* `set` — Update FloatingIP attributes
* `show` — Show single FloatingIP
* `tag` — FloatingIP Tags management



## `osc network floating-ip create`

Create single FloatingIP

**Usage:** `osc network floating-ip create [OPTIONS] --floating-network-id <FLOATING_NETWORK_ID>`

###### **Options:**

* `--floating-ip-address <FLOATING_IP_ADDRESS>` — The floating IP address
* `--subnet-id <SUBNET_ID>` — The subnet ID on which you want to create the floating IP
* `--floating-network-id <FLOATING_NETWORK_ID>` — The ID of the network associated with the floating IP
* `--port-id <PORT_ID>` — The ID of a port associated with the floating IP. To associate the floating IP with a fixed IP at creation time, you must specify the identifier of the internal port
* `--fixed-ip-address <FIXED_IP_ADDRESS>` — The fixed IP address that is associated with the floating IP. If an internal port has multiple associated IP addresses, the service chooses the first IP address unless you explicitly define a fixed IP address in the `fixed\_ip\_address` parameter
* `--tenant-id <TENANT_ID>` — The ID of the project
* `--qos-policy-id <QOS_POLICY_ID>` — The ID of the QoS policy associated with the floating IP
* `--dns-name <DNS_NAME>` — A valid DNS name
* `--dns-domain <DNS_DOMAIN>` — A valid DNS domain
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string



## `osc network floating-ip delete`

Delete single FloatingIP

**Usage:** `osc network floating-ip delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/floatingips/{id} API



## `osc network floating-ip list`

List FloatingIPs

**Usage:** `osc network floating-ip list [OPTIONS]`

###### **Options:**

* `--floating-ip-address <FLOATING_IP_ADDRESS>` — floating_ip_address query parameter for /v2.0/floatingips API
* `--floating-network-id <FLOATING_NETWORK_ID>` — floating_network_id query parameter for /v2.0/floatingips API
* `--router-id <ROUTER_ID>` — router_id query parameter for /v2.0/floatingips API
* `--port-id <PORT_ID>` — port_id query parameter for /v2.0/floatingips API
* `--fixed-ip-address <FIXED_IP_ADDRESS>` — fixed_ip_address query parameter for /v2.0/floatingips API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/floatingips API
* `--status <STATUS>` — status query parameter for /v2.0/floatingips API
* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/floatingips API
* `--tags <TAGS>` — tags query parameter for /v2.0/floatingips API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/floatingips API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/floatingips API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/floatingips API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/floatingips API



## `osc network floating-ip set`

Update FloatingIP attributes

**Usage:** `osc network floating-ip set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/floatingips/{id} API

###### **Options:**

* `--port-id <PORT_ID>` — The ID of a port associated with the floating IP. To associate the floating IP with a fixed IP, you must specify the ID of the internal port. To disassociate the floating IP, `null` should be specified
* `--fixed-ip-address <FIXED_IP_ADDRESS>` — The fixed IP address that is associated with the floating IP. If an internal port has multiple associated IP addresses, the service chooses the first IP address unless you explicitly define a fixed IP address in the `fixed\_ip\_address` parameter
* `--qos-policy-id <QOS_POLICY_ID>`
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string



## `osc network floating-ip show`

Show single FloatingIP

**Usage:** `osc network floating-ip show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/floatingips/{id} API



## `osc network floating-ip tag`

FloatingIP Tags management

Shows details for, updates, and deletes tags. The maximum number of characters allowed in a tag is 60.

**Usage:** `osc network floating-ip tag
       tag <COMMAND>`

###### **Subcommands:**

* `add` — Add a tag
* `check` — Confirm tag presence
* `delete` — Remove a single tag
* `list` — List all tags
* `purge` — Remove all tags
* `replace` — Replace all tags



## `osc network floating-ip tag add`

Add a tag

Adds a tag on the resource.

**Usage:** `osc network floating-ip tag add <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag check`

Confirm tag presence

Confirms a given tag is set on the resource. This method does not return any reasonable response, but fails with "not found" when tag is not present.

**Usage:** `osc network floating-ip tag check <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag delete`

Remove a single tag

Removes a tag on the resource.

**Usage:** `osc network floating-ip tag delete <FLOATINGIP_ID> <ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API
* `<ID>` — id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag list`

List all tags

Obtains the tags for a resource.

**Usage:** `osc network floating-ip tag list <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag purge`

Remove all tags

Removes all tags on the resource.

**Usage:** `osc network floating-ip tag purge <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API



## `osc network floating-ip tag replace`

Replace all tags

Replaces all tags on the resource.

**Usage:** `osc network floating-ip tag replace [OPTIONS] <FLOATINGIP_ID>`

###### **Arguments:**

* `<FLOATINGIP_ID>` — floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id} API

###### **Options:**

* `--tags <TAGS>`



## `osc network network`

Network commands

**Usage:** `osc network network
       network <COMMAND>`

###### **Subcommands:**

* `list` — List Networks
* `show` — Show single Network
* `create` — Create single Network
* `delete` — Delete single Network



## `osc network network list`

List Networks

**Usage:** `osc network network list [OPTIONS]`

###### **Options:**

* `--id <ID>` — id query parameter for /v2.0/networks API
* `--name <NAME>` — name query parameter for /v2.0/networks API
* `--admin-state-up <ADMIN_STATE_UP>` — admin_state_up query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--status <STATUS>` — status query parameter for /v2.0/networks API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/networks API
* `--shared <SHARED>` — shared query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--router-external <ROUTER_EXTERNAL>` — router:external query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--mtu <MTU>` — mtu query parameter for /v2.0/networks API
* `--provider-network-type <PROVIDER_NETWORK_TYPE>` — provider:network_type query parameter for /v2.0/networks API
* `--provider-physical-network <PROVIDER_PHYSICAL_NETWORK>` — provider:physical_network query parameter for /v2.0/networks API
* `--provider-segmentation-id <PROVIDER_SEGMENTATION_ID>` — provider:segmentation_id query parameter for /v2.0/networks API
* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/networks API
* `--tags <TAGS>` — tags query parameter for /v2.0/networks API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/networks API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/networks API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/networks API
* `--is-default <IS_DEFAULT>` — is_default query parameter for /v2.0/networks API

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — description query parameter for /v2.0/networks API



## `osc network network show`

Show single Network

**Usage:** `osc network network show <ID>`

###### **Arguments:**

* `<ID>` — network_id parameter for /v2.0/networks/{network_id} API



## `osc network network create`

Create single Network

**Usage:** `osc network network create [OPTIONS]`

###### **Options:**

* `--name <NAME>` — Human-readable name of the network
* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the network, which is up (`true`) or down (`false`)

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies
* `--shared <SHARED>` — Indicates whether this resource is shared across all projects. By default, only administrative users can change this value

  Possible values: `true`, `false`

* `--router-external <ROUTER_EXTERNAL>` — Indicates whether the network has an external routing facility that’s not managed by the networking service

  Possible values: `true`, `false`

* `--segments <JSON>` — A list of provider `segment` objects
* `--mtu <MTU>` — The maximum transmission unit (MTU) value to address fragmentation. Minimum value is 68 for IPv4, and 1280 for IPv6
* `--availability-zone-hints <AVAILABILITY_ZONE_HINTS>` — The availability zone candidate for the network
* `--ha <HA>`

  Possible values: `true`, `false`

* `--port-security-enabled <PORT_SECURITY_ENABLED>` — The port security status of the network. Valid values are enabled (`true`) and disabled (`false`). This value is used as the default value of `port\_security\_enabled` field of a newly created port

  Possible values: `true`, `false`

* `--provider-network-type <PROVIDER_NETWORK_TYPE>`
* `--provider-physical-network <PROVIDER_PHYSICAL_NETWORK>`
* `--provider-segmentation-id <PROVIDER_SEGMENTATION_ID>`
* `--qos-policy-id <QOS_POLICY_ID>` — The ID of the QoS policy associated with the network
* `--is-default <IS_DEFAULT>` — The network is default or not

  Possible values: `true`, `false`

* `--dns-domain <DNS_DOMAIN>` — A valid DNS domain
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string



## `osc network network delete`

Delete single Network

**Usage:** `osc network network delete <ID>`

###### **Arguments:**

* `<ID>` — network_id parameter for /v2.0/networks/{network_id} API



## `osc network port`

Port commands

**Usage:** `osc network port
       port <COMMAND>`

###### **Subcommands:**

* `list` — List Ports
* `show` — Show single Port
* `create` — Create single Port
* `delete` — Delete single Port



## `osc network port list`

List Ports

**Usage:** `osc network port list [OPTIONS]`

###### **Options:**

* `--id <ID>` — id query parameter for /v2.0/ports API
* `--name <NAME>` — name query parameter for /v2.0/ports API
* `--network-id <NETWORK_ID>` — network_id query parameter for /v2.0/ports API
* `--admin-state-up <ADMIN_STATE_UP>` — admin_state_up query parameter for /v2.0/ports API

  Possible values: `true`, `false`

* `--mac-address <MAC_ADDRESS>` — mac_address query parameter for /v2.0/ports API
* `--fixed-ips <FIXED_IPS>` — fixed_ips query parameter for /v2.0/ports API
* `--device-id <DEVICE_ID>` — device_id query parameter for /v2.0/ports API
* `--device-owner <DEVICE_OWNER>` — device_owner query parameter for /v2.0/ports API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/ports API
* `--status <STATUS>` — status query parameter for /v2.0/ports API
* `--ip-allocation <IP_ALLOCATION>` — ip_allocation query parameter for /v2.0/ports API
* `--binding-host-id <BINDING_HOST_ID>` — binding:host_id query parameter for /v2.0/ports API
* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/ports API
* `--tags <TAGS>` — tags query parameter for /v2.0/ports API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/ports API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/ports API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/ports API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/ports API
* `--security-groups <SECURITY_GROUPS>` — security_groups query parameter for /v2.0/ports API



## `osc network port show`

Show single Port

**Usage:** `osc network port show <ID>`

###### **Arguments:**

* `<ID>` — port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs API



## `osc network port create`

Create single Port

**Usage:** `osc network port create [OPTIONS]`

###### **Options:**

* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--network-id <NETWORK_ID>` — The ID of the attached network
* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--mac-address <MAC_ADDRESS>` — The MAC address of the port. If unspecified, a MAC address is automatically generated
* `--fixed-ips <JSON>` — The IP addresses for the port. If you would like to assign multiple IP addresses for the port, specify multiple entries in this field. Each entry consists of IP address (`ip\_address`) and the subnet ID from which the IP address is assigned (`subnet\_id`)
* `--device-id <DEVICE_ID>` — The ID of the device that uses this port. For example, a server instance or a logical router
* `--device-owner <DEVICE_OWNER>` — The entity type that uses this port. For example, `compute:nova` (server instance), `network:dhcp` (DHCP agent) or `network:router\_interface` (router interface)
* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies
* `--allowed-address-pairs <JSON>` — A set of zero or more allowed address pair objects each where address pair object contains an `ip\_address` and `mac\_address`. While the `ip\_address` is required, the `mac\_address` will be taken from the port if not specified. The value of `ip\_address` can be an IP Address or a CIDR (if supported by the underlying extension plugin). A server connected to the port can send a packet with source address which matches one of the specified allowed address pairs
* `--extra-dhcp-opts <JSON>` — A set of zero or more extra DHCP option pairs. An option pair consists of an option value and name
* `--device-profile <DEVICE_PROFILE>`
* `--hints <key=value>` — Admin-only. A dict, at the top level keyed by mechanism driver aliases (as defined in setup.cfg). To following values can be used to control Open vSwitch’s Userspace Tx packet steering feature:
* `--numa-affinity-policy <NUMA_AFFINITY_POLICY>` — The port NUMA affinity policy requested during the virtual machine scheduling. Values: `None`, `required`, `preferred` or `legacy`

  Possible values: `legacy`, `preferred`, `required`

* `--binding-vnic-type <BINDING_VNIC_TYPE>` — The type of vNIC which this port should be attached to. This is used to determine which mechanism driver(s) to be used to bind the port. The valid values are `normal`, `macvtap`, `direct`, `baremetal`, `direct-physical`, `virtio-forwarder`, `smart-nic` and `remote-managed`. What type of vNIC is actually available depends on deployments. The default is `normal`

  Possible values: `accelerator-direct`, `accelerator-direct-physical`, `baremetal`, `direct`, `direct-physical`, `macvtap`, `normal`, `remote-managed`, `smart-nic`, `vdpa`, `virtio-forwarder`

* `--binding-host-id <BINDING_HOST_ID>` — The ID of the host where the port resides. The default is an empty string
* `--binding-profile <key=value>` — A dictionary that enables the application running on the specific host to pass and receive vif port information specific to the networking back- end. This field is only meant for machine-machine communication for compute services like Nova, Ironic or Zun to pass information to a Neutron back-end. It should not be used by multiple services concurrently or by cloud end users. The existing counterexamples (`capabilities: [switchdev]` for Open vSwitch hardware offload and `trusted=true` for Trusted Virtual Functions) are due to be cleaned up. The networking API does not define a specific format of this field. The default is an empty dictionary. If you update it with null then it is treated like {} in the response. Since the port-mac-address-override extension the `device\_mac\_address` field of the binding:profile can be used to provide the MAC address of the physical device a direct-physical port is being bound to. If provided, then the `mac\_address` field of the port resource will be updated to the MAC from the active binding
* `--port-security-enabled <PORT_SECURITY_ENABLED>` — The port security status. A valid value is enabled (`true`) or disabled (`false`). If port security is enabled for the port, security group rules and anti-spoofing rules are applied to the traffic on the port. If disabled, no such rules are applied

  Possible values: `true`, `false`

* `--qos-policy-id <QOS_POLICY_ID>` — QoS policy associated with the port
* `--tags <TAGS>`
* `--propagate-uplink-status <PROPAGATE_UPLINK_STATUS>` — The uplink status propagation of the port. Valid values are enabled (`true`) and disabled (`false`)

  Possible values: `true`, `false`

* `--dns-name <DNS_NAME>` — A valid DNS name
* `--dns-domain <DNS_DOMAIN>` — A valid DNS domain
* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--security-groups <SECURITY_GROUPS>` — The IDs of security groups applied to the port



## `osc network port delete`

Delete single Port

**Usage:** `osc network port delete <ID>`

###### **Arguments:**

* `<ID>` — port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs API



## `osc network router`

Router commands

**Usage:** `osc network router
       router <COMMAND>`

###### **Subcommands:**

* `list` — List Routers
* `show` — Show single Router
* `create` — Create single Router
* `delete` — Delete single Router



## `osc network router list`

List Routers

**Usage:** `osc network router list [OPTIONS]`

###### **Options:**

* `--name <NAME>` — name query parameter for /v2.0/routers API
* `--admin-state-up <ADMIN_STATE_UP>` — admin_state_up query parameter for /v2.0/routers API

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/routers API
* `--enable-ndp-proxy <ENABLE_NDP_PROXY>` — enable_ndp_proxy query parameter for /v2.0/routers API

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/routers API
* `--tags <TAGS>` — tags query parameter for /v2.0/routers API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/routers API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/routers API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/routers API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/routers API



## `osc network router show`

Show single Router

**Usage:** `osc network router show <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API



## `osc network router create`

Create single Router

**Usage:** `osc network router create [OPTIONS]`

###### **Options:**

* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--admin-state-up <ADMIN_STATE_UP>` — The administrative state of the resource, which is up (`true`) or down (`false`). Default is `true`

  Possible values: `true`, `false`

* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies
* `--network-id <NETWORK_ID>`
* `--enable-snat <ENABLE_SNAT>`

  Possible values: `true`, `false`

* `--external-fixed-ips <JSON>`
* `--ha <HA>` — `true` indicates a highly-available router. It is available when `l3-ha` extension is enabled

  Possible values: `true`, `false`

* `--enable-ndp-proxy <ENABLE_NDP_PROXY>` — Enable NDP proxy attribute. Default is `false`, To persist this attribute value, set the `enable\_ndp\_proxy\_by\_default` option in the `neutron.conf` file. It is available when `router-extend-ndp-proxy` extension is enabled

  Possible values: `true`, `false`

* `--flavor-id <FLAVOR_ID>` — The ID of the flavor associated with the router
* `--availability-zone-hints <AVAILABILITY_ZONE_HINTS>` — The availability zone candidates for the router. It is available when `router\_availability\_zone` extension is enabled
* `--distributed <DISTRIBUTED>` — `true` indicates a distributed router. It is available when `dvr` extension is enabled

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string



## `osc network router delete`

Delete single Router

**Usage:** `osc network router delete <ID>`

###### **Arguments:**

* `<ID>` — id parameter for /v2.0/routers/{id} API



## `osc network subnet`

Subnet commands

**Usage:** `osc network subnet
       subnet <COMMAND>`

###### **Subcommands:**

* `list` — List Subnets
* `show` — Show single Subnet
* `create` — Create single Subnet
* `delete` — Delete single Subnet



## `osc network subnet list`

List Subnets

**Usage:** `osc network subnet list [OPTIONS]`

###### **Options:**

* `--id <ID>` — id query parameter for /v2.0/subnets API
* `--name <NAME>` — name query parameter for /v2.0/subnets API
* `--ip-version <IP_VERSION>` — ip_version query parameter for /v2.0/subnets API
* `--network-id <NETWORK_ID>` — network_id query parameter for /v2.0/subnets API
* `--subnetpool-id <SUBNETPOOL_ID>` — subnetpool_id query parameter for /v2.0/subnets API
* `--cidr <CIDR>` — cidr query parameter for /v2.0/subnets API
* `--gateway-ip <GATEWAY_IP>` — gateway_ip query parameter for /v2.0/subnets API
* `--tenant-id <TENANT_ID>` — tenant_id query parameter for /v2.0/subnets API
* `--enable-dhcp <ENABLE_DHCP>` — enable_dhcp query parameter for /v2.0/subnets API

  Possible values: `true`, `false`

* `--ipv6-ra-mode <IPV6_RA_MODE>` — ipv6_ra_mode query parameter for /v2.0/subnets API
* `--ipv6-address-mode <IPV6_ADDRESS_MODE>` — ipv6_address_mode query parameter for /v2.0/subnets API
* `--shared <SHARED>` — shared query parameter for /v2.0/subnets API

  Possible values: `true`, `false`

* `--revision-number <REVISION_NUMBER>` — revision_number query parameter for /v2.0/subnets API
* `--tags <TAGS>` — tags query parameter for /v2.0/subnets API
* `--tags-any <TAGS_ANY>` — tags-any query parameter for /v2.0/subnets API
* `--not-tags <NOT_TAGS>` — not-tags query parameter for /v2.0/subnets API
* `--not-tags-any <NOT_TAGS_ANY>` — not-tags-any query parameter for /v2.0/subnets API
* `--description <DESCRIPTION>` — description query parameter for /v2.0/subnets API
* `--segment-id <SEGMENT_ID>` — segment_id query parameter for /v2.0/subnets API



## `osc network subnet show`

Show single Subnet

**Usage:** `osc network subnet show <ID>`

###### **Arguments:**

* `<ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id} API



## `osc network subnet create`

Create single Subnet

**Usage:** `osc network subnet create [OPTIONS] --ip-version <IP_VERSION> --network-id <NETWORK_ID>`

###### **Options:**

* `--name <NAME>` — Human-readable name of the resource. Default is an empty string
* `--ip-version <IP_VERSION>` — The IP protocol version. Value is `4` or `6`
* `--network-id <NETWORK_ID>` — The ID of the network to which the subnet belongs
* `--subnetpool-id <SUBNETPOOL_ID>` — The ID of the subnet pool associated with the subnet
* `--prefixlen <PREFIXLEN>` — The prefix length to use for subnet allocation from a subnet pool. If not specified, the `default\_prefixlen` value of the subnet pool will be used
* `--cidr <CIDR>` — The CIDR of the subnet
* `--gateway-ip <GATEWAY_IP>` — Gateway IP of this subnet. If the value is `null` that implies no gateway is associated with the subnet. If the gateway\_ip is not specified, OpenStack Networking allocates an address from the CIDR for the gateway for the subnet by default
* `--allocation-pools <JSON>` — Allocation pools with `start` and `end` IP addresses for this subnet. If allocation\_pools are not specified, OpenStack Networking automatically allocates pools for covering all IP addresses in the CIDR, excluding the address reserved for the subnet gateway by default
* `--dns-nameservers <DNS_NAMESERVERS>` — List of dns name servers associated with the subnet. Default is an empty list
* `--host-routes <JSON>` — Additional routes for the subnet. A list of dictionaries with `destination` and `nexthop` parameters. Default value is an empty list
* `--tenant-id <TENANT_ID>` — The ID of the project that owns the resource. Only administrative and users with advsvc role can specify a project ID other than their own. You cannot change this value through authorization policies
* `--enable-dhcp <ENABLE_DHCP>` — Indicates whether dhcp is enabled or disabled for the subnet. Default is `true`

  Possible values: `true`, `false`

* `--ipv6-ra-mode <IPV6_RA_MODE>` — The IPv6 router advertisement specifies whether the networking service should transmit ICMPv6 packets, for a subnet. Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless`

  Possible values: `dhcpv6-stateful`, `dhcpv6-stateless`, `slaac`

* `--ipv6-address-mode <IPV6_ADDRESS_MODE>` — The IPv6 address modes specifies mechanisms for assigning IP addresses. Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless`

  Possible values: `dhcpv6-stateful`, `dhcpv6-stateless`, `slaac`

* `--service-types <SERVICE_TYPES>` — The service types associated with the subnet
* `--use-default-subnetpool <USE_DEFAULT_SUBNETPOOL>` — Whether to allocate this subnet from the default subnet pool

  Possible values: `true`, `false`

* `--dns-publish-fixed-ip <DNS_PUBLISH_FIXED_IP>` — Whether to publish DNS records for IPs from this subnet. Default is `false`

  Possible values: `true`, `false`

* `--description <DESCRIPTION>` — A human-readable description for the resource. Default is an empty string
* `--segment-id <SEGMENT_ID>` — The ID of a network segment the subnet is associated with. It is available when `segment` extension is enabled



## `osc network subnet delete`

Delete single Subnet

**Usage:** `osc network subnet delete <ID>`

###### **Arguments:**

* `<ID>` — subnet_id parameter for /v2.0/subnets/{subnet_id} API



## `osc object-store`

Object Store service (Swift) commands

**Usage:** `osc object-store
       object-store <COMMAND>`

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

* `list` — Shows details for an account and lists containers, sorted by name, in the account
* `show` — Shows container metadata, including the number of objects and the total bytes of all objects stored in the container
* `set` — Creates, updates, or deletes custom metadata for a container
* `create` — Creates a container. You do not need to check whether a container already exists before issuing a PUT operation because the operation is idempotent: It creates a container or updates an existing container, as appropriate
* `delete` — Deletes an empty container. This operation fails unless the container is empty. An empty container has no objects



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



## `osc object-store container show`

Shows container metadata, including the number of objects and the total bytes of all objects stored in the container

**Usage:** `osc object-store container show <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container



## `osc object-store container set`

Creates, updates, or deletes custom metadata for a container

**Usage:** `osc object-store container set [OPTIONS] <CONTAINER>`

###### **Arguments:**

* `<CONTAINER>` — The unique (within an account) name for the container. The container name must be from 1 to 256 characters long and can start with any character and contain any pattern. Character set must be UTF-8. The container name cannot contain a slash (/) character because this character delimits the container and object name. For example, the path /v1/account/www/pages specifies the www container, not the www/pages container

###### **Options:**

* `--property <key=value>` — Property to be set



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



## `osc object-store object`

Object commands

**Usage:** `osc object-store object <COMMAND>`

###### **Subcommands:**

* `list` — Shows details for a container and lists objects, sorted by name, in the container. Specify query parameters in the request to filter the list and return a subset of objects. Omit query parameters to return a list of objects that are stored in the container, up to 10,000 names. The 10,000 maximum value is configurable. To view the value for the cluster, issue a GET /info request
* `download` — Downloads the object content and gets the object metadata. This operation returns the object metadata in the response headers and the object content in the response body
* `upload` — Creates an object with data content and metadata, or replaces an existing object with data content and metadata. The PUT operation always creates an object. If you use this operation on an existing object, you replace the existing object and metadata rather than modifying the object. Consequently, this operation returns the Created (201) response code. If you use this operation to copy a manifest object, the new object is a normal object and not a copy of the manifest. Instead it is a concatenation of all the segment objects. This means that you cannot copy objects larger than 5 GB. Note that the provider may have limited the characters which are allowed in an object name. Any name limits are exposed under the name_check key in the /info discoverability response. Regardless of name_check limitations, names must be URL quoted UTF-8. To create custom metadata, use the X-Object-Meta-name header, where name is the name of the metadata item
* `show` — Shows object metadata
* `delete` — Permanently deletes an object from the object store. Object deletion occurs immediately at request time. Any subsequent GET, HEAD, POST, or DELETE operations will return a 404 Not Found error code. For static large object manifests, you can add the ?multipart- manifest=delete query parameter. This operation deletes the segment objects and, if all deletions succeed, this operation deletes the manifest object. A DELETE request made to a symlink path will delete the symlink rather than the target object. An alternative to using the DELETE operation is to use the POST operation with the bulk-delete query parameter



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



## `osc object-store object delete`

Permanently deletes an object from the object store. Object deletion occurs immediately at request time. Any subsequent GET, HEAD, POST, or DELETE operations will return a 404 Not Found error code. For static large object manifests, you can add the ?multipart- manifest=delete query parameter. This operation deletes the segment objects and, if all deletions succeed, this operation deletes the manifest object. A DELETE request made to a symlink path will delete the symlink rather than the target object. An alternative to using the DELETE operation is to use the POST operation with the bulk-delete query parameter

**Usage:** `osc object-store object delete [OPTIONS] <CONTAINER> <OBJECT>`

###### **Arguments:**

* `<CONTAINER>` — The unique name for the account. An account is also known as the project or tenant
* `<OBJECT>` — The unique name for the object

###### **Options:**

* `--multipart-manifest <MULTIPART_MANIFEST>` — If you include the multipart-manifest=get query parameter and the object is a large object, the object contents are not returned. Instead, the manifest is returned in the X-Object-Manifest response header for dynamic large objects or in the response body for static large objects



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
