use lazy_regex::{regex_is_match};
use serde::{Serialize, Deserialize};
use std::{fs, path::Path};

/// Application root configuration.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationRootConfig {
    pub package: ApplicationPackageConfig,
}

/// Application "package" configuration.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationPackageConfig {
    pub metadata: ApplicationPackageMetadataConfig,
}

/// Application "package.metadata" configuration.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationPackageMetadataConfig {
    pub rialight: ApplicationRialightConfig,
}

/// Application "package.metadata.rialight" configuration.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationRialightConfig {
    pub application: ApplicationConfig,
}

/// Application "package.metadata.rialight.application" configuration.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationConfig {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Clone, Debug)]
pub enum ApplicationConfigError {
    NotFound,
    InvalidId,
}

pub fn is_id_valid<S: AsRef<str>>(name: S) -> bool {
    regex_is_match!(r"[a-zA-Z0-9][a-zA-Z\-0-9.]*", name.as_ref())
}

impl ApplicationConfig {
    pub fn read(dir: impl AsRef<str>) -> Result<ApplicationRootConfig, ApplicationConfigError> {
        let application_config_path = Path::new(dir.as_ref()).join("Cargo.toml");
        if !(application_config_path.exists() && application_config_path.is_file()) {
            return Err(ApplicationConfigError::NotFound);
        }
        let application_root_config: ApplicationRootConfig = toml::from_str(std::str::from_utf8(&fs::read(application_config_path).unwrap()).unwrap()).unwrap();
        let application_config = application_root_config.package.metadata.rialight.application;
        if !is_id_valid(application_config.id.clone()) {
            return Err(ApplicationConfigError::InvalidId);
        }
        Ok(ApplicationRootConfig {
            package: ApplicationPackageConfig {
                metadata: ApplicationPackageMetadataConfig {
                    rialight: ApplicationRialightConfig {
                        application: ApplicationConfig {
                            id: application_config.id.clone(),
                            title: application_config.title.clone(),
                        },
                    },
                },
            },
        })
    }
}