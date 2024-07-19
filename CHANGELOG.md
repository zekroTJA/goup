# Changelog

## v1.7.0

- Add progress bar for downloading SDKs and spinners for longer running tasks
- Add `rm` alias to `drop` command

## v1.6.0

- Add Fish support [[#10](https://github.com/zekroTJA/goup/issues/10)]

## v1.5.0

- Add Nushell support [[#9](https://github.com/zekroTJA/goup/pull/9)] - by @cptpiepmatz

## v1.4.1

- Allow versions passed to commands be prefixed with `v` or `V` (example `v1.22.0`).

## v1.4.0

- Updated the `clean` command:
  - The command does now remove all installed SDKs instead of the selected one by default.
  - Added the `--all` flag to remove all SDKs – including the selected one.
  - Removed the yes-no accept prompt and removed the `--yes` flag.

- Updated the `use` command:
  - When no specific `VERSION` parameter is passed to the command and if an unstable instance is installed, the latest unstable instance will be used if available and if the latest version is not a stable one. Otherwise, the latest stable instance is installed. 

## v1.3.0

- Implemented a shell abstraction to infer which shell is currently used. This adds support for GitBash on Windows! 🎉

- Fixed a bug where unstable versions were suggested when checking for new updates with a stable version installed.

- Added alias `up` to `use` command.

- Added version aliases for `use` command (`latest` and `s` for `stable` and `rc` for `unstable`).

## v1.2.0

- Windows with PowerShell is now supported! 🥳

- The parameter `goup env -p` (or long `--profile`) has now been renamed to `-a` (long `--apply`). The old parameter is added as an invisible alias and can still be used.

## v1.1.0

- A new command [`check`](https://github.com/zekroTJA/goup/blob/main/docs/commands.md#check) has been added which can be used to check for new upstream versions compared to the currently used one.
  ![](https://user-images.githubusercontent.com/16734205/236545310-b6aa6956-93c8-4b6a-b50e-27332dd52104.gif)

- Remote versions are now fetched via the GitHub REST API and `git ls-remote --tags` is only used as fallback. This should improve the performance significantly. [#1]

- A warning is now printed using the commands `ls`, `current` and `use` when the required environment variables are not set.

- A better about description has been added to the `env` command when displaying the long help using `help env` or `env --help`.


## v1.0.0

- Initial release.