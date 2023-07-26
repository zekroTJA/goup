# Changelog

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