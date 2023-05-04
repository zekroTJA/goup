# Available Commands

- [clean](#clean): `Remove all installed SDKs`
- [current](#current): `Display the currently selected version of Go`
- [drop](#drop): `Drop an installed SDK`
- [env](#env): `Print env variables required to use goup`
- [ls](#ls): `Display currently installed SDKs`
- [ls-remote](#ls-remote): `List all upstream versions`
- [use](#use): `Install a version of Go`

## clean

```
Remove all installed SDKs

Usage: goup clean

Options:
  -h, --help  Print help
```

## current

```
Display the currently selected version of Go

Usage: goup current

Options:
  -h, --help  Print help
```

## drop

```
Drop an installed SDK

Usage: goup drop <VERSION>

Arguments:
  <VERSION>  The version which should be dropped

Options:
  -h, --help  Print help
```

## env

```
Print env variables required to use goup

Usage: goup env [OPTIONS]

Options:
  -p, --profile  Apply the environment variables to your .profile
  -h, --help     Print help
```

## ls

```
Display currently installed SDKs

Usage: goup ls

Options:
  -h, --help  Print help
```

## ls-remote

```
List all upstream versions

Usage: goup ls-remote [OPTIONS]

Options:
  -f, --filter <FILTER>  Filter versions by release type [default: all] [possible values: stable, unstable, all]
  -h, --help             Print help
```

## use

```
Install a version of Go

Usage: goup use [VERSION]

Arguments:
  [VERSION]  Specify a specific version or select the latest stable or unstable release [default: stable]

Options:
  -h, --help  Print help
```


