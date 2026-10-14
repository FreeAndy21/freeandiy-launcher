<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. New file, not part of upstream ArtCraft Launcher. -->
# FreeANDIY Launcher: changes from upstream ArtCraft Launcher

This file lists what the FreeANDIY fork changes in ArtCraft Launcher (Apache-2.0 section 4(b)).
The exact changes are generated from `freeandiy/` (see `NOTICE`); `git diff upstream-main main`
shows every changed file.

## Unreleased

### What it installs
- The FreeANDIY apps instead of the Craft App Suite: FreeANDIY Photo, Vector, Film, Light, PDF,
  Effects, Layout, Write, Slides, Sheets and Sound, from the `FreeAndy21/freeandiy-*` GitHub
  releases only. CADCraft and FigCraft are not FreeANDIY apps.
- Release tags `vX.Y.Z-fa.N` are read as release X.Y.Z (a FreeANDIY build of it), not as a
  pre-release.
- The launcher updates itself from `FreeAndy21/freeandiy-launcher`.

### Removed
- Upstream's release feed (`api.storyteller.ai`): the launcher asks GitHub directly. A feed can
  still be named with `FREEANDIY_LAUNCHER_FEED_URL` for testing.
- The screenshots on the app pages: upstream's pictures repository carries no licence.
- The ArtCraft name, wordmark, logos and "A" mark (`docs/brand/`, the header logo, the tray icon,
  the app icon), as their terms require of forks.
- The ArtCraft Discord and social links (header, footer, Settings, tray menu) and the social icons.
- Upstream's Nix flake and its release and platform workflows (ArtCraft's shared actions); ours are
  `freeandiy*.yml`.

### Branding
- Product name **FreeANDIY Launcher**, application id `agency.freeandy.launcher`, executable
  `freeandiy-launcher`; the cargo packages keep their names.
- The FreeANDIY wordmark in the header, the angel Andy in the tray, FreeANDIY app icons.
- MSI publisher and package vendor Free Andy Agency.

### Added
- `NOTICE` section and `ATTRIBUTION.md` rows for FreeANDIY's assets and fonts; packages, the
  Windows portable ZIP included, ship the font licences and the brand terms.
