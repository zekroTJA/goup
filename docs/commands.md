# Available Commands


> $ goup help

```
goup helps to install, update and switch between Go SDK versions in an as easy as possible way.

Simply use `goup env -a` to add the required environment variables and execute `eval "$(goup env)"` after, to apply the variables to your current terminal session. After that, download the latest version of Go using `goup use`.

Usage: goup <COMMAND>

Commands:
  check    Check for updates
  clean    Remove all installed SDKs [aliases: purge, prune]
  current  Display the currently selected version of Go [aliases: c]
  drop     Drop an installed SDK [aliases: delete, remove, rm]
  env      Print env variables required to use goup
  ls       Display currently installed SDKs [aliases: list]
  lsr      List all upstream versions [aliases: ls-remote, list-remote]
  use      Install a version of Go [aliases: u, up, select, install]
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


## Index


- [check](#check): `Check for updates`
- [clean](#clean): `Remove all installed SDKs`
- [current](#current): `Display the currently selected version of Go`
- [drop](#drop): `Drop an installed SDK`
- [env](#env): `This command prints all necessary environment variables and values required to use goup. `
- [lsr](#lsr): `List all upstream versions`
- [ls](#ls): `Display currently installed SDKs`
- [use](#use): `Install a version of Go`

## Details


### check

> $ goup help check

```
Check for updates

Usage: goup check [OPTIONS]

Options:
  -n, --notify  Only print when updates are available; Designed to be used in profile file
  -h, --help    Print help
```

### clean

> $ goup help clean

```
Remove all installed SDKs

Usage: goup clean [OPTIONS]

Options:
  -a, --all   Clean up **all** installed SDK versions
  -h, --help  Print help
```

### current

> $ goup help current

```
Display the currently selected version of Go

Usage: goup current

Options:
  -h, --help  Print help
```

### drop

> $ goup help drop

```
Drop an installed SDK

Usage: goup drop <VERSION>

Arguments:
  <VERSION>  The version which should be dropped

Options:
  -h, --help  Print help
```

### env

> $ goup help env

```
This command prints all necessary environment variables and values required to use goup. 

Using `goup env -p` appends the variables to your profile file (/home/r.hoffmann@intern.b12-group.de/.profile). After that, you can apply the changes to your current terminal session using `eval "$(goup env)"`.

Usage: goup env [OPTIONS]

Options:
  -a, --apply
          Apply the environment variables to your profile

  -h, --help
          Print help (see a summary with '-h')
```

### lsr

> $ goup help lsr

```
List all upstream versions

Usage: goup lsr [OPTIONS]

Options:
  -f, --filter <FILTER>  Filter versions by release type [default: all] [possible values: stable, unstable, all]
  -h, --help             Print help
```

### ls

> $ goup help ls

```
Display currently installed SDKs

Usage: goup ls

Options:
  -h, --help  Print help
```

### use

> $ goup help use

```
Install a version of Go

Usage: goup use [VERSION]

Arguments:
  [VERSION]  Specify a specific version or select the latest stable or unstable release

Options:
  -h, --help  Print help
```


