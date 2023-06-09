# Available Commands


> $ goup help

```
goup helps to install, update and switch between Go SDK versions in an as easy as possible way.

Simply use `goup env -p && source ~/profile` to add the required environment variables. After that, download the latest version of Go using `goup use`.

Usage: goup <COMMAND>

Commands:
  check
          Check for updates
  clean
          Remove all installed SDKs [aliases: purge, prune]
  current
          Display the currently selected version of Go [aliases: c]
  drop
          Drop an installed SDK [aliases: delete, remove]
  env
          Print env variables required to use goup
  ls
          Display currently installed SDKs [aliases: list]
  lsr
          List all upstream versions [aliases: ls-remote, list-remote]
  use
          Install a version of Go [aliases: u, select, install]
  help
          Print this message or the help of the given subcommand(s)

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
- [env](#env): `This command prints all necessary environment variables and values required to use goup.`
- [ls](#ls): `Display currently installed SDKs`
- [lsr](#lsr): `List all upstream versions`
- [use](#use): `Install a version of Go`

## Details


### check

> $ goup help check

```
Check for updates

Usage: goup check

Options:
  -h, --help  Print help
```

### clean

> $ goup help clean

```
Remove all installed SDKs

Usage: goup clean

Options:
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
Using `goup env -p` appends the variables to your `.profile` file in your $HOME directory. After that, you can apply the changes to your current terminal session using `source ~/.profile`.
If you only want to play around with goup, you can temporarily apply the environment variables to your current terminal session using the command`eval "$(goup env)"`.

Usage: goup env [OPTIONS]

Options:
  -p, --profile
          Apply the environment variables to your .profile

  -h, --help
          Print help (see a summary with '-h')
```

### ls

> $ goup help ls

```
Display currently installed SDKs

Usage: goup ls

Options:
  -h, --help  Print help
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

### use

> $ goup help use

```
Install a version of Go

Usage: goup use [VERSION]

Arguments:
  [VERSION]  Specify a specific version or select the latest stable or unstable release [default: stable]

Options:
  -h, --help  Print help
```


