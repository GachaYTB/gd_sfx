use std::{ops::Range, path::PathBuf};

use anyhow::Result;
use educe::Educe;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use strum::EnumIter;

use crate::{paths, localized_enum};

static SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| {
    paths::runtime::PROJECT_DIRS.config_local_dir()
        .join("settings.json")
});

#[derive(Educe, Serialize, Deserialize, Debug)]
#[educe(Default, Clone, PartialEq)]
pub struct PersistentSettings {
    #[educe(Default = get_gd_folder())]
    pub gd_folder: String,

    pub search_filter_mode: SearchFilterMode,

    pub sfx_select_mode: SfxSelectMode,

    #[educe(Default = true)]
    pub play_sfx_on_click: bool,

    #[educe(Default = String::from("en_US"))]
    pub locale: String,

    #[educe(Default = 0..14500)]
    pub download_ids_range: Range<u32>,

    #[serde(skip)]
    #[educe(Clone(method(ignore_option)), PartialEq(ignore))]
    last_state: Option<Box<PersistentSettings>>,
}

localized_enum! {
    #[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, EnumIter)]
    pub enum SearchFilterMode = "settings.search_filter_mode" {
        #[default]
        GrayOut = "gray_out",
        Hide = "hide",
    }
}

localized_enum! {
    #[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, EnumIter)]
    pub enum SfxSelectMode = "settings.sfx_select_mode" {
        #[default]
        Hover = "hover",
        Click = "click",
    }
}

fn get_gd_folder() -> String {
    paths::runtime::GD_FOLDER
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_default()
}

fn ignore_option<T>(_: &Option<T>) -> Option<T> { None }

impl PersistentSettings {
    pub fn load_or_default() -> Self {
        let mut settings: PersistentSettings = gdsfx_data::read_json_file(&*SETTINGS_FILE)
            .unwrap_or_default();

        settings.set_last_state();
        settings
    }

    pub fn try_save_if_changed(&mut self) -> Result<()> {
        if self.has_changed() {
            let json_data = serde_json::to_string(self).expect("derived serialization shouldn't fail");
            
            gdsfx_data::create_parent_dirs(&*SETTINGS_FILE)?;
            gdsfx_data::write_file(&*SETTINGS_FILE, json_data)?;

            self.set_last_state();
        }
        Ok(())
    }

    fn has_changed(&self) -> bool {
        self.last_state.as_ref()
            .map(|last| self.ne(last))
            .unwrap_or(true) // has not saved before
    }

    fn set_last_state(&mut self) {
        self.last_state = Some(Box::new(self.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_last_state() {
        let mut settings = PersistentSettings::default();
        assert_eq!(settings.last_state, None);
        
        settings.set_last_state();
        assert_eq!(settings.last_state, Some(Box::new(settings.clone())));

        let last_state = settings.last_state.as_ref().unwrap();

        // last_state shouldn't be cloned
        assert_eq!(last_state.last_state, None);
        // and also shouldn't be considered when checking equality
        assert_eq!(settings, **last_state);
    }

    #[test]
    fn test_change_detection() {
        let mut settings = PersistentSettings::default();
        settings.set_last_state();
        assert!(!settings.has_changed());

        settings.locale = String::from("de_AT");
        assert!(settings.has_changed());

        settings.set_last_state();
        assert!(!settings.has_changed());
    }
}
