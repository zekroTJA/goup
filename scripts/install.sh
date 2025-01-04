#!/bin/sh

CLR_RED="\x1b[31m"
CLR_GREEN="\x1b[32m"
CLR_ORANGE="\x1b[38;5;215m"
CLR_RESET="\x1b[0m"


RELEASE_URL="https://api.github.com/repos/zekrotja/goup/releases/latest"
DEFAULT_INSTALL_PATH="/usr/local/bin/goup"


set -ue
set -o pipefail

fatal() {
    printf "${CLR_RED}fatal${CLR_RESET}: %s\n" "$1" >&2
    exit 1
}

info() {
    printf "${CLR_GREEN}info${CLR_RESET}: %s\n" "$1"
}

to_lower() {
    if cmd_exists awk; then 
        awk '{print tolower($1)}' <<< "$1"
    elif cmd_exists tr; then
        tr "[:upper:]" "[:lower:]" <<< "$1"
    else
        fatal "either 'awk' or 'tr' must be installed"
    fi
}

confirm() {
    printf "${CLR_ORANGE}warn${CLR_RESET}: %s (y/N) " "$1"
    local resp
    read -r resp
    case $(to_lower "${resp}") in
        "y" | "yes" ) return 0 ;;
        *) fatal "instalaltion canceled" ;;
    esac
}

cmd_exists() {
    command -v "$1" > /dev/null 2>&1
}

need_cmd() {
    if ! cmd_exists "$1"; then
        fatal "need command '$1' (command not found)"
    fi
}

request() {
    need_cmd curl

    # shellcheck disable=SC2068
    curl --proto '=https' --tlsv1.2 --silent --show-error --fail --location $@
}

main() {
    case "$(to_lower "${1-}")" in
        "-h" | "--help" | "?" )
            printf "Usage: install.sh [INSTALL_LOCATION]\n"
            exit 0
        ;;
    esac

    need_cmd uname
    need_cmd curl
    need_cmd jq
    need_cmd mktemp

    INSTALL_PATH=${INSTALL_PATH-"${1-}"}

    if [ -z "${INSTALL_PATH}" ]; then
        INSTALL_PATH=$(command -v "goup" || true)
    fi

    if [ -z "${INSTALL_PATH-}" ]; then
        INSTALL_PATH="$DEFAULT_INSTALL_PATH"
    fi

    if [ -f "$INSTALL_PATH" ]; then
        confirm "$INSTALL_PATH already exists; do you want to overwrite it?"
    fi

    local os
    local arch
    
    os=$(to_lower "$(uname --kernel-name)")
    arch=$(uname --machine)

    info "Getting latest release for $os $arch"
    local dl_file_url
    dl_file_url=$(request "$RELEASE_URL" \
        | jq --raw-output '
            .assets[]
                | select(
                        (.name | contains("'"$arch"'")) 
                    and (.name | contains("'"${os}"'"))
                ) 
                | [.name, .browser_download_url]
                | join(" ")'
    )

    if [ -z "$dl_file_url" ]; then
        fatal "Could not find any asset for $os $arch"
    fi

    local filename
    filename=$(cut --delimiter=' ' --field=1 <<< "$dl_file_url")

    local dl_url
    dl_url=$(cut --delimiter=' ' --field=2 <<< "$dl_file_url")

    local download_tmp_dir
    download_tmp_dir="$(mktemp --directory)"
    
    local download_tmp_file
    download_tmp_file="$download_tmp_dir/goup"

    info "Downloading asset $filename to $download_tmp_file"
    request "$dl_url" --output "$download_tmp_file"

    info "Moving $download_tmp_file to $INSTALL_PATH"
    mv "$download_tmp_file" "$INSTALL_PATH"
    chmod +x "$INSTALL_PATH"

    info "Cleaning up"
    rm --recursive "$download_tmp_dir"

    info "goup has been installed successfully to $INSTALL_PATH!"
}


# shellcheck disable=SC2068
main $@