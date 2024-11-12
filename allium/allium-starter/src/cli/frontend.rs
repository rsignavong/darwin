use super::error::Result;
use derive_new::new;
use heck::ToKebabCase;
use serde::{Deserialize, Deserializer};
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process::Command,
};
use tracing::info;

#[derive(Deserialize)]
struct PackageJson {
    #[serde(
        alias = "_applicationPort",
        deserialize_with = "PackageJson::deserialize_application_port"
    )]
    application_port: u16,
}

impl PackageJson {
    pub fn deserialize_application_port<'de, D>(d: D) -> std::result::Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(d)?;
        s.parse::<u16>().map_err(serde::de::Error::custom)
    }
}

#[derive(new)]
pub struct Frontend {
    project_path: PathBuf,
}

impl Frontend {
    pub fn install(&self) -> Result<()> {
        info!("Install MFs");

        let mut mf_dir_path = self.project_path.clone();
        mf_dir_path.push("frontend/allium-system");

        let mf_dir = mf_dir_path.canonicalize()?;
        let mut mf_output = Command::new("npm")
            .args(["run", "install:mf"])
            .current_dir(mf_dir)
            .spawn()?;
        mf_output.wait()?;

        let mut webcomponents_dir_path = self.project_path.clone();
        webcomponents_dir_path.push("frontend/webcomponents");

        let webcomponents_dir = webcomponents_dir_path.canonicalize()?;
        let mut mf_output = Command::new("npm")
            .args(["install"])
            .current_dir(webcomponents_dir)
            .spawn()?;
        mf_output.wait()?;

        let mut frontend_dir_path = self.project_path.clone();
        frontend_dir_path.push("frontend");

        let frontend_dir = frontend_dir_path.canonicalize()?;
        let mut mf_output = Command::new("npm")
            .args(["install"])
            .current_dir(&frontend_dir)
            .spawn()?;
        mf_output.wait()?;

        for entry in fs::read_dir(frontend_dir)? {
            let path = entry?.path();
            if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
                if path.is_dir()
                    && file_name != "allium-system"
                    && file_name != "webcomponents"
                    && file_name != "node_modules"
                    && file_name != "src"
                {
                    info!("Installing dependencies for maestro {:?}", path);
                    let mut maestro_output = Command::new("npm")
                        .args(["install"])
                        .current_dir(path)
                        .spawn()?;
                    maestro_output.wait()?;
                }
            }
        }

        let mut styles_dir_path = self.project_path.clone();
        styles_dir_path.push("styles");

        let styles_dir = styles_dir_path.canonicalize()?;
        for entry in fs::read_dir(styles_dir)? {
            let path = entry?.path();
            if path.is_dir() {
                info!(
                    "Installing dependencies for styles of application {:?}",
                    path
                );
                let mut styles_output = Command::new("npm")
                    .args(["install"])
                    .current_dir(path)
                    .spawn()?;
                styles_output.wait()?;
            }
        }

        info!("Frontend install up \u{1f43c}!");

        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        info!("Build MFs");

        let mut project_dir_path = self.project_path.clone();
        project_dir_path.push("frontend/allium-system");

        let project_dir = project_dir_path.canonicalize()?;
        let mut gen_output = Command::new("npm")
            .args(["run", "build"])
            .current_dir(project_dir)
            .spawn()?;
        gen_output.wait()?;

        info!("Frontend build up \u{1f43c}!");

        Ok(())
    }

    pub fn build_web_components(&self) -> Result<()> {
        info!("Build Web components");

        let mut project_dir_path = self.project_path.clone();
        project_dir_path.push("frontend/webcomponents");

        let project_dir = project_dir_path.canonicalize()?;
        let mut gen_output = Command::new("npm")
            .args(["run", "build"])
            .current_dir(project_dir)
            .spawn()?;
        gen_output.wait()?;

        info!("Web components build up \u{1f43c}!");

        Ok(())
    }

    pub fn open(&self, maestro_application: &str) -> Result<()> {
        let mut pkg_json_path = self.project_path.clone();
        pkg_json_path.push(format!(
            "frontend/{}/package.json",
            maestro_application.to_kebab_case() + "-app"
        ));

        let pkg_json = pkg_json_path.canonicalize()?;
        let mut file = File::open(pkg_json)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let package_json: PackageJson = serde_json::from_slice(&buffer)?;
        let url = format!("https://localhost:{}", package_json.application_port);

        let mut open_output = Command::new("open").args(&[url]).spawn()?;
        open_output.wait()?;

        info!("Open application \u{1f43c}!");

        Ok(())
    }
}
