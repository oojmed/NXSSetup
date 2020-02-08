# NXSSetup Changelog

## v0.3.0

### Features

- Added numbers as input for selecting options in windows
- WINDOWS ONLY: No longer exit after exporting to a dir, go back to the finished window to allow multiple exports (this fixes dir export not working on Windows)

### Misc. Changes

- Bumped version to `v0.3.0`
- Slightly changed readme (`README.md`)
- Renamed `None` CFW option to `Skip`
- Finished window: If skipped CFW / no CFW is chosen, only say `Skipped`, instead of `Downloaded None`, etc.
- Added additional information to help
- Removed a lot of unneeded / in the way debug logging:
    - Argument logging at startup
    - Click debug when rendering windows
    - JSON response when downloading releases

### Fixes

- Stopped `(debug)` appearing on pre-build / download
- Added missing space between self-build and brackets in the finished window
- Fixed CFW not showing in finished screen when prebuilding
- Fixed CFW window showing when attempting to go back from the debug build choice window


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