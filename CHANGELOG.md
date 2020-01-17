# NXSSetup Changelog

## v0.2.0

### Features

- Added debug or not debug builds selection window
- Added finished / completed / end window
- Use debug build selection for whether to have debug for Atmosphère
- Automatically download and use a replacement zip binary on Windows, as Atmosphère's Makefile required it
- Added changelog (`CHANGELOG.md`)

### Misc. Changes

- Updated dependencies
- Bumped version to `v0.2.0`
- Tidied / simplified / rewrote some backend UI code
- Removed old, unused code
- Atmosphère: Only `make dist`, don't `make` before, it's unneeded
- Reversed title colors (blue now on the left, red on the right)
- (Probably other things we missed)

### Fixes

- Don't exit window is checkbox clicked is not interactable
- Checkbox's remove escapes when adding checkbox's text to chosen
- Windows (UI) ignore escapes in checkbox's text when calculating padding with checkboxes and window width

## v0.1.0

Initial Release