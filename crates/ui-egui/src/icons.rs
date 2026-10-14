//! Bundled images: each Craft App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Craft App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "freeandiy-photo" => egui::include_image!("../../../assets/app-icons/freeandiy-photo.png"),
        "freeandiy-vector" => egui::include_image!("../../../assets/app-icons/freeandiy-vector.png"),
        "freeandiy-film" => egui::include_image!("../../../assets/app-icons/freeandiy-film.png"),
        "freeandiy-light" => egui::include_image!("../../../assets/app-icons/freeandiy-light.png"),
        "freeandiy-pdf" => egui::include_image!("../../../assets/app-icons/freeandiy-pdf.png"),
        "freeandiy-effects" => egui::include_image!("../../../assets/app-icons/freeandiy-effects.png"),
        "freeandiy-layout" => egui::include_image!("../../../assets/app-icons/freeandiy-layout.png"),
        "freeandiy-write" => egui::include_image!("../../../assets/app-icons/freeandiy-write.png"),
        "freeandiy-slides" => egui::include_image!("../../../assets/app-icons/freeandiy-slides.png"),
        "freeandiy-sheets" => egui::include_image!("../../../assets/app-icons/freeandiy-sheets.png"),
        "freeandiy-sound" => egui::include_image!("../../../assets/app-icons/freeandiy-sound.png"),
        _ => return None,
    })
}

/// The FreeANDIY wordmark (the name set in IBM Plex Mono): ink for light backgrounds, paper for
/// dark ones. © Free Andy Agency, not covered by the MIT/Apache license
/// (`freeandiy/assets/brand/LICENSE-brand.txt`).
pub fn brand_logo(dark_background: bool) -> ImageSource<'static> {
    if dark_background {
        egui::include_image!("../../../assets/brand/freeandiy-wordmark-paper.png")
    } else {
        egui::include_image!("../../../assets/brand/freeandiy-wordmark-ink.png")
    }
}

/// The wordmark's width / height (its PNG: 880 × 130).
pub const ARTCRAFT_LOGO_ASPECT: f32 = 880.0 / 130.0;

/// A social service's logo, by its name in `catalog::SOCIAL`. FreeANDIY lists no services.
pub fn social(_service: &str) -> Option<ImageSource<'static>> {
    None
}

/// The launcher's mark (its app icon tile, the angel Andy). © Free Andy Agency.
pub fn brand_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/app-icon/freeandiy-launcher-mark.png")
}
