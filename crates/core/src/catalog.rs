//! The FreeANDIY apps: the thirteen apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

use serde::{Deserialize, Serialize};

/// Which generation of the suite an app belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppGroup {
    /// The seven original apps.
    Classic,
    /// The apps added after them.
    New,
}

impl AppGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::New => "New",
        }
    }
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`freeandiy-photo`).
    pub id: &'static str,
    /// Display name (`FreeANDIY Photo`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    pub group: AppGroup,
    /// Brand colour (sRGB), for surfaces: freeandiy-web.vercel.app's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (FreeANDIY PDF shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (FreeANDIY PDF's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `agency.freeandy.<app>`: macOS bundle id, Flatpak id and Linux desktop-file id (FreeANDIY:
    /// the id without its `freeandiy-` prefix).
    pub fn bundle_id(&self) -> String {
        format!("agency.freeandy.{}", self.id.strip_prefix("freeandiy-").unwrap_or(self.id))
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page on the FreeANDIY website.
    pub fn website(&self) -> String {
        format!("https://freeandiy-web.vercel.app/{}", self.id.strip_prefix("freeandiy-").unwrap_or(self.id))
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    group: AppGroup,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, group, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// The FreeANDIY apps, in display order: the seven classic apps, then the four new ones
/// (FreeANDIY: CADCraft and FigCraft are not part of it).
pub const SUITE: [CraftApp; 11] = [
    app(
        "freeandiy-photo",
        "FreeANDIY Photo",
        "Image editing",
        AppGroup::Classic,
        [[0xa3, 0xdd, 0xfa], [0x54, 0x67, 0x73], [0xa3, 0xdd, 0xfa]],
        "FreeAndy21/freeandiy-photo",
    ),
    app(
        "freeandiy-vector",
        "FreeANDIY Vector",
        "Vector illustration",
        AppGroup::Classic,
        [[0xff, 0xd9, 0x66], [0x77, 0x66, 0x3b], [0xff, 0xd9, 0x66]],
        "FreeAndy21/freeandiy-vector",
    ),
    app(
        "freeandiy-film",
        "FreeANDIY Film",
        "Video editing, color and sound",
        AppGroup::Classic,
        [[0xea, 0x99, 0x99], [0x6f, 0x4d, 0x4e], [0xea, 0x99, 0x99]],
        "FreeAndy21/freeandiy-film",
    ),
    app(
        "freeandiy-light",
        "FreeANDIY Light",
        "Photo library and raw development",
        AppGroup::Classic,
        [[0x7d, 0xc1, 0xe3], [0x45, 0x5d, 0x6a], [0x7d, 0xc1, 0xe3]],
        "FreeAndy21/freeandiy-light",
    ),
    CraftApp {
        former_ids: &["printcraft"],
        former_names: &["PrintCraft"],
        ..app(
            "freeandiy-pdf",
            "FreeANDIY PDF",
            "Reading, organizing and protecting PDFs",
            AppGroup::Classic,
            [[0xa5, 0xb9, 0x9c], [0x54, 0x5a, 0x4f], [0xa5, 0xb9, 0x9c]],
            "FreeAndy21/freeandiy-pdf",
        )
    },
    app(
        "freeandiy-effects",
        "FreeANDIY Effects",
        "Motion graphics and visual effects",
        AppGroup::Classic,
        [[0xff, 0xcc, 0x33], [0x77, 0x61, 0x27], [0xff, 0xcc, 0x33]],
        "FreeAndy21/freeandiy-effects",
    ),
    app(
        "freeandiy-layout",
        "FreeANDIY Layout",
        "Page layout and publishing",
        AppGroup::Classic,
        [[0xf2, 0xdf, 0xa6], [0x72, 0x68, 0x53], [0xf2, 0xdf, 0xa6]],
        "FreeAndy21/freeandiy-layout",
    ),
    app(
        "freeandiy-write",
        "FreeANDIY Write",
        "Writing and document design",
        AppGroup::New,
        [[0xa7, 0xe4, 0x8b], [0x55, 0x6a, 0x49], [0xa7, 0xe4, 0x8b]],
        "FreeAndy21/freeandiy-write",
    ),
    app(
        "freeandiy-slides",
        "FreeANDIY Slides",
        "Presentations and slide shows",
        AppGroup::New,
        [[0xc6, 0xdc, 0xbb], [0x61, 0x67, 0x5b], [0xc6, 0xdc, 0xbb]],
        "FreeAndy21/freeandiy-slides",
    ),
    app(
        "freeandiy-sheets",
        "FreeANDIY Sheets",
        "Spreadsheets and data",
        AppGroup::New,
        [[0x85, 0xce, 0x64], [0x48, 0x62, 0x3a], [0x85, 0xce, 0x64]],
        "FreeAndy21/freeandiy-sheets",
    ),
    app(
        "freeandiy-sound",
        "FreeANDIY Sound",
        "Recording, editing and mixing audio",
        AppGroup::New,
        [[0xd4, 0x7c, 0x7c], [0x66, 0x42, 0x43], [0xd4, 0x7c, 0x7c]],
        "FreeAndy21/freeandiy-sound",
    ),
];

/// The FreeANDIY website.
pub const WEBSITE_URL: &str = "https://freeandiy-web.vercel.app";
/// The GitHub account every FreeANDIY app's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
pub const GITHUB_OWNER: &str = "FreeAndy21";
/// FreeANDIY on GitHub: every app's source.
pub const GITHUB_URL: &str = "https://github.com/FreeAndy21/freeandiy";
/// Social accounts: (service, URL), shown as small icons. FreeANDIY lists none.
pub const SOCIAL: [(&str, &str); 0] = [];

/// Where to report a problem or ask a question: FreeANDIY's issue tracker.
pub const ISSUES_URL: &str = "https://github.com/FreeAndy21/freeandiy/issues";

/// FreeANDIY Launcher itself, described like a Craft App so it can update itself with the same
/// machinery (asset names `freeandiy-launcher-<ver>-…`, bundle id `agency.freeandy.launcher`,
/// `FreeANDIY Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "freeandiy-launcher",
    "FreeANDIY Launcher",
    "Installs and updates the FreeANDIY apps",
    AppGroup::New,
    [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
    "FreeAndy21/freeandiy-launcher",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "FreeAndy21/freeandiy-launcher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "freeandiy-launcher";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_seven_classic_and_four_new_apps_with_unique_ids() {
        assert_eq!(SUITE.iter().filter(|a| a.group == AppGroup::Classic).count(), 7);
        assert_eq!(SUITE.iter().filter(|a| a.group == AppGroup::New).count(), 4);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("FreeAndy21/{}", a.id));
            assert_eq!(a.name.to_lowercase().replace(' ', "-"), a.id);
        }
    }

    #[test]
    fn bundle_ids_and_pages_drop_the_brand_prefix() {
        let pdf = CraftApp::by_id("freeandiy-pdf").unwrap();
        assert_eq!(pdf.stems().collect::<Vec<_>>(), ["freeandiy-pdf", "printcraft"]);
        assert_eq!(pdf.bundle_id(), "agency.freeandy.pdf");
        assert_eq!(pdf.website(), "https://freeandiy-web.vercel.app/pdf");
        assert_eq!(LAUNCHER.bundle_id(), "agency.freeandy.launcher");
    }
}
