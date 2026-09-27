#!/usr/bin/env sh

#
# Generic installer script for installing OpenStack Rust binary tools from
# GitHub Releases.
#
# This script installs a binary from a specific GitHub repository.
# It automatically detects the OS and architecture, downloads the
# appropriate release asset, verifies its checksum, and installs it.
#
# It is based on the cargo-dist template that was used earlier.
#
# USAGE:
#   curl -sSL https://github.com/gtema/openstack/blob/main/install.sh | sh
#
# To install a specific version, you can set the VERSION variable:
#   curl -sSL https://github.com/gtema/openstack/blob/main/install.sh | INSTALLER_VERSION=v1.2.3 sh
#
# To install a specific tool (openstack_cli by default, or openstack_tui), you
# can set the TOOL variable:
#   curl -sSL https://github.com/gtema/openstack/blob/main/install.sh | INSTALLER_TOOL=openstack_tui sh
#
# REQUIREMENTS:
#   - curl: to download files
#   - grep, cut, sed: for parsing the API response (standard in most shells)
#   - tar: to extract .tar.gz archives
#   - sha256sum: to verify file checksums (part of coreutils)

set -e
set -o nounset

# --- CONFIGURATION -----------------------------------------------------------
# The GitHub repository to install from (format: owner/repo)
REPO="gtema/openstack"
# The name of the binary to install.
TOOL=${INSTALLER_TOOL:-openstack_cli}
#VERSION=${INSTALLER_VERSION:}
# The name of the checksums file in your GitHub Release assets.
CHECKSUM_FILENAME="sha256.sum"
ARTIFACT_DOWNLOAD_URL="https://github.com/gtema/openstack/releases/download"
PRINT_VERBOSE=${INSTALLER_PRINT_VERBOSE:-0}
PRINT_QUIET=${INSTALLER_PRINT_QUIET:-0}

# -----------------------------------------------------------------------------

# Helper function to print error messages and exit.
err() {
    if [ "0" = "$PRINT_QUIET" ]; then
        local red
        local reset
        red=$(tput setaf 1 2>/dev/null || echo '')
        reset=$(tput sgr0 2>/dev/null || echo '')
        say "${red}ERROR${reset}: $1" >&2
    fi
    exit 1
}

say() {
    if [ "0" = "$PRINT_QUIET" ]; then
        echo "$1"
    fi
}

say_verbose() {
    if [ "1" = "$PRINT_VERBOSE" ]; then
        echo "$1"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
    return $?
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

# Helper function to check for required commands.
need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "Required command '$1' is not installed. Please install it first."
    fi
}

# This is just for indicating that commands' results are being
# intentionally ignored. Usually, because it's being executed
# as part of error handling.
ignore() {
    "$@"
}


check_proc() {
    # Check for /proc by looking for the /proc/self/exe link
    # This is only run on Linux
    if ! test -L /proc/self/exe ; then
        err "fatal: Unable to find /proc/self/exe.  Is /proc mounted?  Installation cannot proceed without /proc."
    fi
}

get_bitness() {
    need_cmd head
    # Architecture detection without dependencies beyond coreutils.
    # ELF files start out "\x7fELF", and the following byte is
    #   0x01 for 32-bit and
    #   0x02 for 64-bit.
    # The printf builtin on some shells like dash only supports octal
    # escape sequences, so we use those.
    local _current_exe_head
    _current_exe_head=$(head -c 5 /proc/self/exe )
    if [ "$_current_exe_head" = "$(printf '\177ELF\001')" ]; then
        echo 32
    elif [ "$_current_exe_head" = "$(printf '\177ELF\002')" ]; then
        echo 64
    else
        err "unknown platform bitness"
    fi
}

is_host_amd64_elf() {
    need_cmd head
    need_cmd tail
    # ELF e_machine detection without dependencies beyond coreutils.
    # Two-byte field at offset 0x12 indicates the CPU,
    # but we're interested in it being 0x3E to indicate amd64, or not that.
    local _current_exe_machine
    _current_exe_machine=$(head -c 19 /proc/self/exe | tail -c 1)
    [ "$_current_exe_machine" = "$(printf '\076')" ]
}

get_endianness() {
    local cputype=$1
    local suffix_eb=$2
    local suffix_el=$3

    # detect endianness without od/hexdump, like get_bitness() does.
    need_cmd head
    need_cmd tail

    local _current_exe_endianness
    _current_exe_endianness="$(head -c 6 /proc/self/exe | tail -c 1)"
    if [ "$_current_exe_endianness" = "$(printf '\001')" ]; then
        echo "${cputype}${suffix_el}"
    elif [ "$_current_exe_endianness" = "$(printf '\002')" ]; then
        echo "${cputype}${suffix_eb}"
    else
        err "unknown platform endianness"
    fi
}

get_architecture() {
    local _ostype
    local _cputype
    _ostype="$(uname -s)"
    _cputype="$(uname -m)"
    local _clibtype="gnu"
    local _local_glibc

    if [ "$_ostype" = Linux ]; then
        if [ "$(uname -o)" = Android ]; then
            _ostype=Android
        fi
        if ldd --version 2>&1 | grep -q 'musl'; then
            _clibtype="musl-dynamic"
        else
            # Assume all other linuxes are glibc (even if wrong, static libc fallback will apply)
            _clibtype="gnu"
        fi
    fi

    if [ "$_ostype" = Darwin ] && [ "$_cputype" = i386 ]; then
        # Darwin `uname -m` lies
        if sysctl hw.optional.x86_64 | grep -q ': 1'; then
            _cputype=x86_64
        fi
    fi

    if [ "$_ostype" = Darwin ] && [ "$_cputype" = x86_64 ]; then
        # Rosetta on aarch64
        if [ "$(sysctl -n hw.optional.arm64 2>/dev/null)" = "1" ]; then
            _cputype=aarch64
        fi
    fi

    if [ "$_ostype" = SunOS ]; then
        # Both Solaris and illumos presently announce as "SunOS" in "uname -s"
        # so use "uname -o" to disambiguate.  We use the full path to the
        # system uname in case the user has coreutils uname first in PATH,
        # which has historically sometimes printed the wrong value here.
        if [ "$(/usr/bin/uname -o)" = illumos ]; then
            _ostype=illumos
        fi

        # illumos systems have multi-arch userlands, and "uname -m" reports the
        # machine hardware name; e.g., "i86pc" on both 32- and 64-bit x86
        # systems.  Check for the native (widest) instruction set on the
        # running kernel:
        if [ "$_cputype" = i86pc ]; then
            _cputype="$(isainfo -n)"
        fi
    fi

    case "$_ostype" in

        Android)
            _ostype=linux-android
            ;;

        Linux)
            check_proc
            _ostype=unknown-linux-$_clibtype
            _bitness=$(get_bitness)
            ;;

        FreeBSD)
            _ostype=unknown-freebsd
            ;;

        NetBSD)
            _ostype=unknown-netbsd
            ;;

        DragonFly)
            _ostype=unknown-dragonfly
            ;;

        Darwin)
            _ostype=apple-darwin
            ;;

        illumos)
            _ostype=unknown-illumos
            ;;

        MINGW* | MSYS* | CYGWIN* | Windows_NT)
            _ostype=pc-windows-gnu
            ;;

        *)
            err "unrecognized OS type: $_ostype"
            ;;

    esac

    case "$_cputype" in

        i386 | i486 | i686 | i786 | x86)
            _cputype=i686
            ;;

        xscale | arm)
            _cputype=arm
            if [ "$_ostype" = "linux-android" ]; then
                _ostype=linux-androideabi
            fi
            ;;

        armv6l)
            _cputype=arm
            if [ "$_ostype" = "linux-android" ]; then
                _ostype=linux-androideabi
            else
                _ostype="${_ostype}eabihf"
            fi
            ;;

        armv7l | armv8l)
            _cputype=armv7
            if [ "$_ostype" = "linux-android" ]; then
                _ostype=linux-androideabi
            else
                _ostype="${_ostype}eabihf"
            fi
            ;;

        aarch64 | arm64)
            _cputype=aarch64
            ;;

        x86_64 | x86-64 | x64 | amd64)
            _cputype=x86_64
            ;;

        mips)
            _cputype=$(get_endianness mips '' el)
            ;;

        mips64)
            if [ "$_bitness" -eq 64 ]; then
                # only n64 ABI is supported for now
                _ostype="${_ostype}abi64"
                _cputype=$(get_endianness mips64 '' el)
            fi
            ;;

        ppc)
            _cputype=powerpc
            ;;

        ppc64)
            _cputype=powerpc64
            ;;

        ppc64le)
            _cputype=powerpc64le
            ;;

        s390x)
            _cputype=s390x
            ;;
        riscv64)
            _cputype=riscv64gc
            ;;
        loongarch64)
            _cputype=loongarch64
            ;;
        *)
            err "unknown CPU type: $_cputype"

    esac

    # Detect 64-bit linux with 32-bit userland
    if [ "${_ostype}" = unknown-linux-gnu ] && [ "${_bitness}" -eq 32 ]; then
        case $_cputype in
            x86_64)
                # 32-bit executable for amd64 = x32
                if is_host_amd64_elf; then {
                    err "x32 linux unsupported"
                }; else
                    _cputype=i686
                fi
                ;;
            mips64)
                _cputype=$(get_endianness mips '' el)
                ;;
            powerpc64)
                _cputype=powerpc
                ;;
            aarch64)
                _cputype=armv7
                if [ "$_ostype" = "linux-android" ]; then
                    _ostype=linux-androideabi
                else
                    _ostype="${_ostype}eabihf"
                fi
                ;;
            riscv64gc)
                err "riscv64 with 32-bit userland unsupported"
                ;;
        esac
    fi

    # treat armv7 systems without neon as plain arm
    if [ "$_ostype" = "unknown-linux-gnueabihf" ] && [ "$_cputype" = armv7 ]; then
        if ensure grep '^Features' /proc/cpuinfo | grep -q -v neon; then
            # At least one processor does not have NEON.
            _cputype=arm
        fi
    fi

    _arch="${_cputype}-${_ostype}"

    RETVAL="$_arch"
}


# This wraps curl or wget. Try curl first, if not installed,
# use wget instead.
downloader() {
    if check_cmd curl
    then _dld=curl
    elif check_cmd wget
    then _dld=wget
    else _dld='curl or wget' # to be used in error message of need_cmd
    fi

    if [ "$1" = --check ]
    then need_cmd "$_dld"
    elif [ "$_dld" = curl ]
    then curl -sSfL "$1" -o "$2"
    elif [ "$_dld" = wget ]
    then wget "$1" -O "$2"
    else err "Unknown downloader"   # should not reach here
    fi
}

get_release_data() {
    # Determine the version to install and get release assets JSON
    #local version
    local release_data
    if [ -n "${INSTALLER_VERSION:-}" ]; then
        release_data=$(curl -s "https://api.github.com/repos/${REPO}/releases/tags/${INSTALLER_VERSION}")
    else
        release_data=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest")
    fi

    if [ -z "$release_data" ] || [ "$release_data" = "null" ]; then
        err "Could not determine release data."
    fi
    echo "$release_data"
}

get_release_version() {
    # Determine the version to install and get release assets JSON
    local release_data="$1"
    if [ -n "${INSTALLER_VERSION:-}" ]; then
        version="$INSTALLER_VERSION"
    else
        version=$(echo "$1" | grep '"tag_name"' | cut -d'"' -f4)
    fi

    if [ -z "$version" ] || [ "$version" = "null" ]; then
        err "Could not determine release version."
    fi
    echo "$version"
}

check_glibc() {
    local _min_glibc_major="$1"
    local _min_glibc_series="$2"

    # Parsing version out from line 1 like:
    # ldd (Ubuntu GLIBC 2.35-0ubuntu3.1) 2.35
    _local_glibc="$(ldd --version | awk -F' ' '{ if (FNR<=1) print $NF }')"

    if [ "$(echo "${_local_glibc}" | awk -F. '{ print $1 }')" = "$_min_glibc_major" ] && [ "$(echo "${_local_glibc}" | awk -F. '{ print $2 }')" -ge "$_min_glibc_series" ]; then
        return 0
    else
        say "System glibc version (\`${_local_glibc}') is too old; checking alternatives" >&2
        return 1
    fi
}

select_archive_for_arch() {
    local _true_arch="$1"
    local _archive

    # try each archive, checking runtime conditions like libc versions
    # accepting the first one that matches, as it's the best match
    case "$_true_arch" in 
        "aarch64-apple-darwin")
            _archive="$TOOL-aarch64-apple-darwin.tar.xz"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            _archive="$TOOL-x86_64-apple-darwin.tar.xz"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "aarch64-pc-windows-msvc")
            _archive="$TOOL-x86_64-pc-windows-msvc.zip"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "x86_64-apple-darwin")
            _archive="$TOOL-x86_64-apple-darwin.tar.xz"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "x86_64-pc-windows-gnu")
            _archive="$TOOL-x86_64-pc-windows-msvc.zip"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "x86_64-pc-windows-msvc")
            _archive="$TOOL-x86_64-pc-windows-msvc.zip"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "x86_64-unknown-linux-gnu")
            _archive="$TOOL-x86_64-unknown-linux-gnu.tar.xz"
            if ! check_glibc "2" "39"; then
                _archive=""
            fi
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            _archive="$TOOL-x86_64-unknown-linux-musl.tar.xz"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "x86_64-unknown-linux-musl-dynamic")
            _archive="$TOOL-x86_64-unknown-linux-musl.tar.xz"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        "x86_64-unknown-linux-musl-static")
            _archive="$TOOL-x86_64-unknown-linux-musl.tar.xz"
            if [ -n "$_archive" ]; then
                echo "$_archive"
                return 0
            fi
            ;;
        *)
            err "there isn't a download for your platform $_true_arch"
            ;;
    esac
    err "no compatible downloads were found for your platform $_true_arch"
}

download_and_install() {
    local release_data=$( get_release_data )
    local ver=$( get_release_version "$release_data" )

    local _artifact_name
    _artifact_name="$(select_archive_for_arch "$_true_arch")" || return 1

    local _url
    local _dir
    local _zip_ext=".tar.gz"
    local _libs=""
    local _bins=""
    local _staticlibs=""

    if echo "$_artifact_name" | grep -Eq "\.zip$"; then
       _zip_ext=".zip" 
    fi

    case "$TOOL" in
        openstack_cli )
          _bins="osc"
          ;;
        openstack_tui )
          _bins="ostui"
          ;;
    esac

    _url="$ARTIFACT_DOWNLOAD_URL/$ver/$_artifact_name"
    _dir="$(ensure mktemp -d)" || return 1
    _file="$_dir/$_artifact_name"

    say "downloading $TOOL:$ver for ${_arch}" 1>&2
    say_verbose "  from $_url" 1>&2
    say_verbose "  to $_file" 1>&2

    ensure mkdir -p "$_dir"

    if ! downloader "$_url" "$_file"; then
      say "failed to download $_url"
      say "this may be a standard network error, but it may also indicate"
      say "that $TOOL's release process is not working. When in doubt"
      say "please feel free to open an issue!"
      exit 1
    fi

    # Verify the checksum
    _checksum_url="$ARTIFACT_DOWNLOAD_URL/$ver/$CHECKSUM_FILENAME"
    _checksum_file="$_dir/$CHECKSUM_FILENAME"
    say "downloading checksums file" 1>&2
    say_verbose "  from $_checksum_url" 1>&2
    say_verbose "  to $_checksum_file" 1>&2
    if ! downloader "$_checksum_url" "$_checksum_file"; then
      exit 1
    fi
    say "Verifying checksum..."
    (cd "${_dir}" && grep "${_artifact_name}" "$_checksum_file" | sha256sum --check - --strict --ignore-missing)
    if [ $? -ne 0 ]; then
      err "Checksum validation failed for ${_artifact_name}."
    fi
    say "Checksum verified."

    # unpack the archive
    case "$_zip_ext" in
        ".zip")
            ensure unzip -q "$_file" -d "$_dir"
            ;;

        ".tar."*)
            ensure tar xf "$_file" --strip-components 1 -C "$_dir"
            ;;
        *)
            err "unknown archive format: $_zip_ext"
            ;;
    esac

    install "$_dir" "$_bins" "$_libs" "$_staticlibs" "$_arch"
    local _retval=$?
    if [ "$_retval" != 0 ]; then
        return "$_retval"
    fi

    ignore rm -rf "$_dir"

    return "$_retval"
}

install() {
    # The actual path we're going to install to
    local _install_dir
    # A list of binaries which are shadowed in the PATH
    local _shadowed_bins=""

    # first try $CARGO_HOME, then fallback to $HOME/.cargo
    if [ -n "${CARGO_HOME:-}" ]; then
        _install_dir="$CARGO_HOME/bin"
    elif [ -n "${HOME:-}" ]; then
        _install_dir="$HOME/.cargo/bin"
    fi

    say "installing to $_install_dir"
    ensure mkdir -p "$_install_dir"

    # copy all the binaries to the install dir
    local _src_dir="$1"
    local _bins="$2"
    local _libs="$3"
    local _staticlibs="$4"
    local _arch="$5"
    for _bin_name in $_bins; do
        local _bin="$_src_dir/$_bin_name"
        ensure mv "$_bin" "$_install_dir"
        # unzip seems to need this chmod
        ensure chmod +x "$_install_dir/$_bin_name"
        say "  $_bin_name"
    done

    say "everything's installed!"

    type "$_bins" &>/dev/null || say "$_install_dir is NOT in PATH. Please consider adding it."

    _shadowed_bins="$(check_for_shadowed_bins "$_install_dir" "$_bins")"
    if [ -n "$_shadowed_bins" ]; then
        say "WARNING: The following commands are shadowed by other commands in your PATH:$_shadowed_bins"
    fi

}

check_for_shadowed_bins() {
    local _install_dir="$1"
    local _bins="$2"

    for _bin_name in $_bins; do
        if [ "$(command -v "$_bin_name")" != "$_install_dir/$_bin_name" ]; then
            _shadowed_bins="$_shadowed_bins $_bin_name"
        fi
    done

    echo "$_shadowed_bins"
}

# Basic preparations
prepare() {
    downloader --check
    need_cmd uname
    need_cmd mktemp
    need_cmd chmod
    need_cmd mkdir
    need_cmd rm
    need_cmd tar
    need_cmd grep
    need_cmd cat
    get_architecture || return 1
    _true_arch="$RETVAL"
}

usage() {
    # print help (this cat/EOF stuff is a "heredoc" string)
    cat <<EOF
install.sh

The installer for OpenStack Rust tools

This script detects what platform you're on and fetches an appropriate archive
from https://github.com/gtema/openstack/releases/download/latest then unpacks
the binaries and installs them to

    \$CARGO_HOME/bin (or \$HOME/.cargo/bin)

It will then add that dir to PATH by adding the appropriate line to your shell
profiles.

USAGE:
    install.sh [OPTIONS]

OPTIONS:
    -t, --target
            Target to install [openstack_cli, openstack_tui]. Default: 'openstack_cli'.
            You can also set this using the `INSTALLER_TOOL` environment variable.

    -v, --verbose
            Enable verbose output

    -q, --quiet
            Disable progress output

        --no-modify-path
            Don't configure the PATH environment variable

    -h, --help
            Print help information
EOF
}


# parse argv variables
while [ "$#" -gt 0 ]; do
  case "$1" in
    -h | --help)
        usage
        exit 0
        ;;
    -t | --target)
        TOOL="$2"
        shift 2
        ;;
    -q | --quiet)
        PRINT_QUIET=1
        shift 1
        ;;
    -v | --verbose)
        PRINT_VERBOSE=1
        shift 1
        ;;
    *)
        err "unknown option $1"
        ;;
  esac
done

prepare

download_and_install
