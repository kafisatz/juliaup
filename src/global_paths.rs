use std::path::PathBuf;
use anyhow::{Result,bail,anyhow};
#[cfg(feature = "selfupdate")]
use anyhow::Context;
use crate::get_juliaup_target;
pub struct GlobalPaths {
    pub juliauphome: PathBuf,
    pub juliaupconfig: PathBuf,
    pub lockfile: PathBuf,
    pub versiondb: PathBuf,
    #[cfg(feature = "selfupdate")]
    pub juliaupselfhome: PathBuf,
    #[cfg(feature = "selfupdate")]
    pub juliaupselfconfig: PathBuf,
    #[cfg(feature = "selfupdate")]
    pub juliaupselfbin: PathBuf,
}

fn get_juliaup_home_path() -> Result<PathBuf> {
    let entry_sep = if std::env::consts::OS == "windows" {';'} else {':'};

    let path = match std::env::var("JULIA_DEPOT_PATH") {
        Ok(val) => {
            let path = PathBuf::from(val.split(entry_sep).next().unwrap()); // We can unwrap here because even when we split an empty string we should get a first element.

            if !path.is_absolute() {
                bail!("The `JULIA_DEPOT_PATH` environment variable contains a value that resolves to an an invalid path `{}`.", path.display());
            };

            path.join("juliaup")
        }
        Err(_) => {
    let path = dirs::home_dir()
        .ok_or_else(|| anyhow!(
            "Could not determine the path of the user home directory."
        ))?
        .join(".julia")
        .join("juliaup");

            if !path.is_absolute() {
                bail!(
                    "The system returned an invalid home directory path `{}`.",
                    path.display()
                );
            };

            path
        }
    };

    Ok(path)
}

pub fn get_paths() -> Result<GlobalPaths> {
    let juliauphome = get_juliaup_home_path()?;

    #[cfg(feature = "selfupdate")]
    let my_own_path = std::env::current_exe()
        .with_context(|| "Could not determine the path of the running exe.")?;

    #[cfg(feature = "selfupdate")]
    let juliaupselfbin = my_own_path.parent()
        .ok_or_else(|| anyhow!("Could not determine parent."))?
        .to_path_buf();

    let juliaupconfig = juliauphome.join("juliaup.json");

    let versiondb = juliauphome.join(format!("versiondb-{}.json", get_juliaup_target()));
    
    let lockfile = juliauphome.join(".juliaup-lock");

    #[cfg(feature = "selfupdate")]
    let juliaupselfhome = my_own_path
        .parent()
        .ok_or_else(|| anyhow!("Failed to get path of folder of own executable."))?
        .parent()
        .ok_or_else(|| anyhow!("Failed to get parent path of folder of own executable."))?
        .to_path_buf();

    #[cfg(feature = "selfupdate")]
    let juliaupselfconfig = juliaupselfhome
        .join("juliaupself.json");

    Ok(GlobalPaths {
        juliauphome,
        juliaupconfig,
        lockfile,
        versiondb,
        #[cfg(feature = "selfupdate")]
        juliaupselfhome,
        #[cfg(feature = "selfupdate")]
        juliaupselfconfig,
        #[cfg(feature = "selfupdate")]
        juliaupselfbin,
    })
}
