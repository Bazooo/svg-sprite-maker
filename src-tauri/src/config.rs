use std::{fs, io, path};
use derive_builder::Builder;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, specta::Type)]
pub struct TransparencyGridColor(String, f32);

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationConfig {
    #[serde(skip)]
    path: path::PathBuf,
    pub settings: ApplicationConfigSettings,
}

#[derive(serde::Deserialize, serde::Serialize, Builder, Default, Debug, Clone, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationConfigSettings {
    // Generic
    pub auto_save_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_path: Option<path::PathBuf>,
    pub dark_mode: bool,

    // Transparency Grid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency_grid_color_1: Option<TransparencyGridColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency_grid_color_2: Option<TransparencyGridColor>,
}

impl ApplicationConfig {
    pub fn load(config_path: path::PathBuf) -> Self {
        if !config_path.exists() {
            return Self::default_with_path(config_path);
        }

        let Ok(config_file) = fs::File::open(config_path.clone()) else {
            return Self::default_with_path(config_path);
        };

        let config_reader = io::BufReader::new(config_file);

        let Ok(config) = serde_json::from_reader(config_reader) else {
            return Self::default_with_path(config_path);
        };

        Self {
            path: config_path,
            ..config
        }
    }

    pub fn update_settings<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ApplicationConfigSettingsBuilder) -> &mut ApplicationConfigSettingsBuilder
    {
        let mut builder: ApplicationConfigSettingsBuilder = self.settings.clone().into();

        let Ok(new_settings) = f(&mut builder).build() else {
            return;
        };

        self.settings = new_settings;
        self.save();
    }

    fn default_with_path(path: path::PathBuf) -> Self {
        Self {
            path,
            ..Self::default()
        }
    }

    fn save(&self) {
        fs::create_dir_all(self.path.clone().parent().unwrap()).unwrap();

        let mut options = fs::OpenOptions::new();
        options.write(true).truncate(true).create(true);

        let mut file = options.open(self.path.clone()).unwrap();

        serde_json::to_writer_pretty(&mut file, self).unwrap();
    }
}

impl From<ApplicationConfigSettings> for ApplicationConfigSettingsBuilder {
    fn from(settings: ApplicationConfigSettings) -> Self {
        Self {
            auto_save_enabled: Some(settings.auto_save_enabled),
            editor_path: Some(settings.editor_path),
            dark_mode: Some(settings.dark_mode),
            transparency_grid_color_1: Some(settings.transparency_grid_color_1),
            transparency_grid_color_2: Some(settings.transparency_grid_color_2),
        }
    }
}
