# `uefisettings`

## About

`uefisettings` is a tool to read and write BIOS settings on a local host. It currently supports one interface:

* HiiDB (used in OCP and also has partial support in other platforms)

---

## Install

```sh
cargo install uefisettings
```

## Build manually

```sh
cd /tmp
git clone https://github.com/linuxboot/uefisettings
cd uefisettings
cargo install --path .
~/.cargo/bin/uefisettings --help
```

---

## Usage examples

### Reset TPM

```sh
uefisettings hii set 'Pending operation' 'TPM Clear'
```

### Check if TXT is enabled

```sh
if [[ "$(uefisettings hii get --json 'Enable Intel(R) TXT' | jq -r '.responses | .[].question.answer')" = "Enable" ]]; then
    # Do something if TXT is enabled
fi
```

---

## Available commands

```plain
SUBCOMMANDS:
    get         Auto-identify backend and get the current value of a question
    help        Print this message or the help of the given subcommand(s)
    hii         Commands which work on machines exposing the UEFI HiiDB
    set         Auto-identify backend and set/change the value of a question
```

`hii`:

```plain
SUBCOMMANDS:
    extract-db      Dump HiiDB into a file
    get             Get the current value of a question
    help            Print this message or the help of the given subcommand(s)
    list-strings    List all strings-id, string pairs in HiiDB
    set             Set/change the value of a question
    show-ifr        Show a human readable representation of the Hii Forms
```

---

## Changing UEFI settings with automation

To change BIOS settings from Linux terminal there are usually next ways available:

* On [Open Compute Project](https://www.opencompute.org/) hardware - read/parse a binary database called Hii (defined in the UEFI spec) and manipulate binary files in the `/sys` file system to change settings. This approach may also work on some non-OCP consumer hardware like your laptops.
* Use different proprietary tools like SCELNX from AMI or conrep from HPE. However, these require additional kernel modules to be loaded.

But this tool is an unified opensource approach to manipulate UEFI settings on any platform.

---

## How it works

### For OCP hardware

* Extract HiiDB from `/dev/mem` after getting offsets from `efivarfs`.
* Library (`hii`) in Rust partially implements the UEFI Hii specification.
* It parses the HiiDB op-codes into a DOM tree like representation which can be read by machines and humans.
* It can ask questions about UEFI settings from HiiDB and get their answers.
* Change UEFI settings by calculating the correct offsets and writing to entries in `efivarfs`.
