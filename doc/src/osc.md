# Command-Line Help for `osc`

This document contains the help content for the `osc` command-line program.

**Command Overview:**

* [`osc`↴](#osc)
* [`osc block-storage`↴](#osc-block-storage)
* [`osc block-storage volume`↴](#osc-block-storage-volume)
* [`osc block-storage volume list`↴](#osc-block-storage-volume-list)
* [`osc block-storage volume show`↴](#osc-block-storage-volume-show)
* [`osc compute`↴](#osc-compute)
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
* [`osc compute keypair delete`↴](#osc-compute-keypair-delete)
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
* [`osc network router`↴](#osc-network-router)
* [`osc network router list`↴](#osc-network-router-list)
* [`osc network router show`↴](#osc-network-router-show)
* [`osc network router create`↴](#osc-network-router-create)
* [`osc network router delete`↴](#osc-network-router-delete)
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
* [`osc catalog`↴](#osc-catalog)
* [`osc catalog list`↴](#osc-catalog-list)
* [`osc api`↴](#osc-api)

## `osc`

OpenStack client rewritten in Rust

**Usage:** `osc [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `block-storage` — Block Storage (Volume) service (Cinder) commands
* `compute` — Compute service (Nova) commands
* `image` — Image (Glance) commands
* `network` — Network (Neutron) commands
* `object-store` — Object Store service (Swift) commands
* `catalog` — Shows current catalog information
* `api` — Perform direct REST API requests with authorization

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

* `list` — List Volumes
* `show` — Show single volume



## `osc block-storage volume list`

List Volumes

**Usage:** `osc block-storage volume list [OPTIONS]`

###### **Options:**

* `--project-id <PROJECT_ID>` — The UUID of the project in a multi-tenancy cloud
* `--all-projects <ALL_PROJECTS>` — all_projects filter parameter

  Possible values: `true`, `false`

* `--name <NAME>` — Name filter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc block-storage volume show`

Show single volume

**Usage:** `osc block-storage volume show [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Volume ID

###### **Options:**

* `--project-id <PROJECT_ID>` — The UUID of the project in a multi-tenancy cloud



## `osc compute`

Compute service (Nova) commands

**Usage:** `osc compute [OPTIONS]
       compute <COMMAND>`

###### **Subcommands:**

* `server` — Server (VM) commands
* `flavor` — Flavor commands
* `keypair` — Keypair commands

###### **Options:**

* `--os-compute-api-version <OS_COMPUTE_API_VERSION>` — Compute API microversion



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

* `--min-disk <MIN_DISK>` — Filters the response by a minimum disk space, in GiB. For example, 100
* `--min-ram <MIN_RAM>` — Filters the response by a minimum RAM, in MiB. For example, 512
* `--is-public <IS_PUBLIC>` — This parameter is only applicable to users with the administrative role. For all other non-admin users, the parameter is ignored and only public flavors will be returned. Filters the flavor list based on whether the flavor is public or private. If the value of this parameter is not specified, it is treated as True. If the value is specified, 1, t, true, on, y and yes are treated as True. 0, f, false, off, n and no are treated as False (they are case-insensitive). If the value is None (case-insensitive) both public and private flavors will be listed in a single request

  Possible values: `true`, `false`

* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--marker <MARKER>` — The ID of the last-seen item. Use the limit parameter to make an initial limited request and use the ID of the last-seen item from the response as the marker parameter value in a subsequent limited request
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute flavor show`

Show single Server

**Usage:** `osc compute flavor show <ID>`

###### **Arguments:**

* `<ID>` — Flavor ID



## `osc compute keypair`

Keypair commands

**Usage:** `osc compute keypair <COMMAND>`

###### **Subcommands:**

* `list` — List Keypairs
* `show` — Show single Keypair
* `create` — Create Keypair
* `delete` — Delete Keypair



## `osc compute keypair list`

List Keypairs

**Usage:** `osc compute keypair list [OPTIONS]`

###### **Options:**

* `--user-id <USER_ID>` — This allows administrative users to operate key-pairs of specified user ID. New in version 2.10
* `--limit <LIMIT>` — Requests a page size of items. Returns a number of items up to a limit value. Use the limit parameter to make an initial limited request and use the last-seen item from the response as the marker parameter value in a subsequent limited request. New in version 2.35
* `--marker <MARKER>` — The last-seen item. Use the limit parameter to make an initial limited request and use the last-seen item from the response as the marker parameter value in a subsequent limited request. New in version 2.35
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc compute keypair show`

Show single Keypair

**Usage:** `osc compute keypair show [OPTIONS] <KEYPAIR_NAME>`

###### **Arguments:**

* `<KEYPAIR_NAME>` — This allows administrative users to operate key-pairs of specified user ID. New in version 2.10

###### **Options:**

* `--user-id <USER_ID>` — This allows administrative users to operate key-pairs of specified user ID. New in version 2.10



## `osc compute keypair create`

Create Keypair

**Usage:** `osc compute keypair create [OPTIONS] --name <NAME> --public-key <PUBLIC_KEY>`

###### **Options:**

* `--name <NAME>` — A name for the keypair which will be used to reference it later. Note: Since microversion 2.92, allowed characters are ASCII letters [a-zA-Z], digits [0-9] and the following special characters: [@._- ]
* `--public-key <PUBLIC_KEY>` — The public ssh key to import. Was optional before microversion 2.92 : if you were omitting this value, a keypair was generated for you
* `--xtype <XTYPE>` — The type of the keypair. Allowed values are ssh or x509. New in version 2.2
* `--user-id <USER_ID>` — The user_id for a keypair



## `osc compute keypair delete`

Delete Keypair

**Usage:** `osc compute keypair delete [OPTIONS] <KEYPAIR_NAME>`

###### **Arguments:**

* `<KEYPAIR_NAME>` — This allows administrative users to operate key-pairs of specified user ID. New in version 2.10

###### **Options:**

* `--user-id <USER_ID>` — This allows administrative users to operate key-pairs of specified user ID. New in version 2.10



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
* `show` — Show single Image
* `create` — Create Image
* `set` — Update Image
* `download` — Download Image
* `upload` — Upload Image
* `delete` — Delete Image
* `deactivate` — Deactivate Image
* `reactivate` — Reactivate Image



## `osc image image list`

List Images

**Usage:** `osc image image list [OPTIONS]`

###### **Options:**

* `--limit <LIMIT>` — limit filter parameter
* `--marker <MARKER>` — marker filter parameter
* `--id <ID>` — id filter parameter
* `--name <NAME>` — name filter parameter
* `--visibility <VISIBILITY>` — visibility filter parameter
* `--member-status <MEMBER_STATUS>` — member_status filter parameter
* `--owner <OWNER>` — owner filter parameter
* `--status <STATUS>` — status filter parameter
* `--size-min <SIZE_MIN>` — size_min filter parameter
* `--size-max <SIZE_MAX>` — size_max filter parameter
* `--protected <PROTECTED>` — protected filter parameter
* `--is-hidden <IS_HIDDEN>` — is_hidden filter parameter

  Possible values: `true`, `false`

* `--sort-key <SORT_KEY>` — sort_key filter parameter
* `--sort-dir <SORT_DIR>` — sort_dir filter parameter
* `--sort <SORT>` — sort filter parameter
* `--tag <TAG>` — tag filter parameter
* `--created-at <CREATED_AT>` — created_at filter parameter
* `--updated-at <UPDATED_AT>` — updated_at filter parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc image image show`

Show single Image

**Usage:** `osc image image show <ID>`

###### **Arguments:**

* `<ID>` — Image ID



## `osc image image create`

Create Image

**Usage:** `osc image image create [OPTIONS]`

###### **Options:**

* `--container-format <CONTAINER_FORMAT>` — The container format refers to whether the VM image is in a file format that also contains metadata about the actual VM. Container formats include OVF and Amazon AMI. In addition, a VM image might not have a container format - instead, the image is just a blob of unstructured data
* `--disk-format <DISK_FORMAT>` — The format of the disk. Values may vary based on the configuration available in a particular OpenStack cloud. See the Image Schema response from the cloud itself for the valid values available. Example formats are: ami, ari, aki, vhd, vhdx, vmdk, raw, qcow2, vdi, ploop or iso
* `--min-disk <MIN_DISK>` — Amount of disk space in GB that is required to boot the image
* `--min-ram <MIN_RAM>` — Amount of RAM in MB that is required to boot the image
* `--name <NAME>` — The name of the image
* `--is-protected <IS_PROTECTED>` — Image protection for deletion. Valid value is true or false. Default is false

  Possible values: `true`, `false`

* `--tags <TAGS>` — List of tags for this image. Each tag is a string of at most 255 chars. The maximum number of tags allowed on an image is set by the operator
* `--visibility <VISIBILITY>` — Visibility for this image. Valid value is one of: ``public``, ``private``, ``shared``, or ``community``. At most sites, only an administrator can make an image public. Some sites may restrict what users can make an image community. Some sites may restrict what users can perform member operations on a shared image. Since the Image API v2.5, the default value is ``shared``



## `osc image image set`

Update Image

**Usage:** `osc image image set [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Image ID

###### **Options:**

* `--architecture <ARCHITECTURE>` — The CPU architecture that must be supported by the hypervisor
* `--has-auto-disk-config <HAS_AUTO_DISK_CONFIG>` — If true, the root partition on the disk is automatically resized before the instance boots
* `--container-format <CONTAINER_FORMAT>` — The container format refers to whether the VM image is in a file format that also contains metadata about the actual VM. Container formats include OVF and Amazon AMI. In addition, a VM image might not have a container format - instead, the image is just a blob of unstructured data
* `--disk-format <DISK_FORMAT>` — Valid values are: aki, ari, ami, raw, iso, vhd, vdi, qcow2, or vmdk. The disk format of a VM image is the format of the underlying disk image. Virtual appliance vendors have different formats for laying out the information contained in a VM disk image
* `--hw-cpu-cores <HW_CPU_CORES>` — The preferred number of cores to expose to the guest
* `--hw-cpu-policy <HW_CPU_POLICY>` — Used to pin the virtual CPUs (vCPUs) of instances to the host's physical CPU cores (pCPUs)
* `--hw-cpu-thread-policy <HW_CPU_THREAD_POLICY>` — Defines how hardware CPU threads in a simultaneous multithreading-based (SMT) architecture be used
* `--hw-cpu-threads <HW_CPU_THREADS>` — The preferred number of threads to expose to the guest
* `--hw-disk-bus <HW_DISK_BUS>` — Specifies the type of disk controller to attach disk devices to. One of scsi, virtio, uml, xen, ide, or usb
* `--hw-machine-type <HW_MACHINE_TYPE>` — For libvirt: Enables booting an ARM system using the specified machine type. For Hyper-V: Specifies whether the Hyper-V instance will be a generation 1 or generation 2 VM
* `--hw-qemu-guest-agent <HW_QEMU_GUEST_AGENT>` — A string boolean, which if "true", QEMU guest agent will be exposed to the instance
* `--hw-rng-model <HW_RNG_MODEL>` — Adds a random-number generator device to the image's instances
* `--hw-scsi-model <HW_SCSI_MODEL>` — Enables the use of VirtIO SCSI (virtio-scsi) to provide block device access for compute instances; by default, instances use VirtIO Block (virtio-blk)
* `--hw-serial-port-count <HW_SERIAL_PORT_COUNT>` — Specifies the count of serial ports that should be provided
* `--hw-video-model <HW_VIDEO_MODEL>` — The video image driver used
* `--hw-video-ram <HW_VIDEO_RAM>` — Maximum RAM for the video image
* `--hw-vif-model <HW_VIF_MODEL>` — Specifies the model of virtual network interface device to use
* `--hw-watchdog-action <HW_WATCHDOG_ACTION>` — Enables a virtual hardware watchdog device that carries out the specified action if the server hangs
* `--hypervisor-type <HYPERVISOR_TYPE>` — The hypervisor type. Note that qemu is used for both QEMU and KVM hypervisor types
* `--needs-config-drive <NEEDS_CONFIG_DRIVE>` — Specifies whether the image needs a config drive. `mandatory` or `optional` (default if property is not used)
* `--instance-type-rxtx-factor <INSTANCE_TYPE_RXTX_FACTOR>` — Optional property allows created servers to have a different bandwidth cap than that defined in the network they are attached to
* `--instance-uuid <INSTANCE_UUID>` — create this image
* `--kernel-id <KERNEL_ID>` — The ID of an image stored in the Image service that should be used as the kernel when booting an AMI-style image
* `--locations <LOCATIONS>` — A list of URLs to access the image file in external store. This list appears if the show_multiple_locations option is set to true in the Image service's configuration file
* `--min-disk <MIN_DISK>` — The minimum disk size in GB that is required to boot the image
* `--min-ram <MIN_RAM>` — The minimum amount of RAM in MB that is required to boot the image
* `--name <NAME>` — The name of the image
* `--os-admin-user <OS_ADMIN_USER>` — The operating system admin username
* `--os-command-line <OS_COMMAND_LINE>` — The kernel command line to be used by the libvirt driver, instead of the default
* `--os-distro <OS_DISTRO>` — The common name of the operating system distribution in lowercase
* `--is-hidden <IS_HIDDEN>` — This field controls whether an image is displayed in the default image- list response

  Possible values: `true`, `false`

* `--os-require-quiesce <OS_REQUIRE_QUIESCE>` — If true, require quiesce on snapshot via QEMU guest agent

  Possible values: `true`, `false`

* `--needs-secure-boot <NEEDS_SECURE_BOOT>` — Secure Boot is a security standard. When the instance starts, Secure Boot first examines software such as firmware and OS by their signature and only allows them to run if the signatures are valid
* `--os-shutdown-timeout <OS_SHUTDOWN_TIMEOUT>` — Time for graceful shutdown
* `--os-type <OS_TYPE>` — The operating system installed on the image
* `--os-version <OS_VERSION>` — The operating system version as specified by the distributor
* `--owner-id <OWNER_ID>` — The ID of the owner, or project, of the image. (backwards compat)
* `--is-protected <IS_PROTECTED>` — Defines whether the image can be deleted

  Possible values: `true`, `false`

* `--ramdisk-id <RAMDISK_ID>` — The ID of image stored in the Image service that should be used as the ramdisk when booting an AMI-style image
* `--store <STORE>` — When present, Glance will attempt to store the disk image data in the backing store indicated by the value of the header. When not present, Glance will store the disk image data in the backing store that is marked default. Valid values are: file, s3, rbd, swift, cinder, gridfs, sheepdog, or vsphere
* `--tags <TAGS>` — List of tags for this image, possibly an empty list
* `--url <URL>` — The URL to access the image file kept in external store
* `--visibility <VISIBILITY>` — The image visibility
* `--vm-mode <VM_MODE>` — The virtual machine mode. This represents the host/guest ABI (application binary interface) used for the virtual machine
* `--vmware-adaptertype <VMWARE_ADAPTERTYPE>` — The virtual SCSI or IDE controller used by the hypervisor
* `--vmware-ostype <VMWARE_OSTYPE>` — A VMware GuestID which describes the operating system installed in the image



## `osc image image download`

Download Image

**Usage:** `osc image image download [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Image ID

###### **Options:**

* `--file <FILE>` — Destination filename (using "-" will print object to stdout)



## `osc image image upload`

Upload Image

**Usage:** `osc image image upload [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Image ID

###### **Options:**

* `--file <FILE>` — Source filename (using "-" will read object from stdout)



## `osc image image delete`

Delete Image

**Usage:** `osc image image delete <ID>`

###### **Arguments:**

* `<ID>` — Image ID



## `osc image image deactivate`

Deactivate Image

**Usage:** `osc image image deactivate <ID>`

###### **Arguments:**

* `<ID>` — Image ID



## `osc image image reactivate`

Reactivate Image

**Usage:** `osc image image reactivate <ID>`

###### **Arguments:**

* `<ID>` — Image ID



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

* `router` — Router commands



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

* `--limit <LIMIT>` — limit filter parameter
* `--marker <MARKER>` — marker filter parameter
* `--description <DESCRIPTION>` — description filter parameter
* `--flavor-id <FLAVOR_ID>` — flavor_id filter parameter
* `--name <NAME>` — name filter parameter
* `--status <STATUS>` — status filter parameter
* `--project-id <PROJECT_ID>` — project_id filter parameter
* `--is-admin-state-up <IS_ADMIN_STATE_UP>` — is_admin_state_up filter parameter

  Possible values: `true`, `false`

* `--is-distributed <IS_DISTRIBUTED>` — is_distributed filter parameter

  Possible values: `true`, `false`

* `--is-ha <IS_HA>` — is_ha filter parameter

  Possible values: `true`, `false`

* `--tags <TAGS>` — tags filter parameter
* `--any-tags <ANY_TAGS>` — any_tags filter parameter
* `--not-tags <NOT_TAGS>` — not_tags filter parameter
* `--not-any-tags <NOT_ANY_TAGS>` — not_any_tags filter parameter
* `--max-items <MAX_ITEMS>` — Total limit of entities count to return. Use this when there are too many entries

  Default value: `10000`



## `osc network router show`

Show single Router

**Usage:** `osc network router show <ID>`

###### **Arguments:**

* `<ID>` — Router ID



## `osc network router create`

Create single Router

**Usage:** `osc network router create [OPTIONS]`

###### **Options:**

* `--is-admin-state-up <IS_ADMIN_STATE_UP>` — The administrative state of the router, which is up ``True`` or down ``False``

  Possible values: `true`, `false`

* `--availability-zone-hints <AVAILABILITY_ZONE_HINTS>` — Availability zone hints to use when scheduling the router
* `--description <DESCRIPTION>` — The router description
* `--is-distributed <IS_DISTRIBUTED>` — The distributed state of the router, which is distributed ``True`` or not ``False``

  Possible values: `true`, `false`

* `--enable-ndp-proxy <ENABLE_NDP_PROXY>` — The ndp proxy state of the router

  Possible values: `true`, `false`

* `--external-gateway-info <JSON_VALUE>` — The external gateway information of the router. If the router has an external gateway, this would be a dict with network_id, enable_snat, external_fixed_ips and qos_policy_id. Otherwise, this would be null
* `--flavor-id <FLAVOR_ID>` — The ID of the flavor
* `--is-ha <IS_HA>` — The highly-available state of the router, which is highly available ``True`` or not ``False``

  Possible values: `true`, `false`

* `--name <NAME>` — The router name
* `--project-id <PROJECT_ID>` — The ID of the project this router is associated with
* `--tenant-id <TENANT_ID>` — Tenant_id (deprecated attribute)



## `osc network router delete`

Delete single Router

**Usage:** `osc network router delete <ID>`

###### **Arguments:**

* `<ID>` — Router ID



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



## `osc catalog`

Shows current catalog information

**Usage:** `osc catalog <COMMAND>`

###### **Subcommands:**

* `list` — List catalog command arguments



## `osc catalog list`

List catalog command arguments

**Usage:** `osc catalog list`



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



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
