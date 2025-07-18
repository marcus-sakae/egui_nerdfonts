use std::sync::Arc;

pub mod variants;
pub use variants::*;

pub fn add_to_fonts(fonts: &mut egui::FontDefinitions, variant: Variant) {
    fonts
        .font_data
        .insert("nerdfonts".into(), Arc::new(variant.font_data()));

    if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        font_keys.push("nerdfonts".into());
    }
}
