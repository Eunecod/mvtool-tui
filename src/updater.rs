// src/updater.rs

use self_update::{ backends::github::Update, cargo_crate_version, update::ReleaseUpdate };

pub struct Updater
{
    builder: self_update::backends::github::UpdateBuilder,
    status: Option<Box<dyn ReleaseUpdate>>
}

impl Default for Updater
{
    fn default() -> Self
    {
        return Self { builder: Update::configure(), status: None };
    }
}

impl Updater
{
    pub fn new() -> Self
    {
        return Self::default();
    }

    pub fn fetch(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        self.builder
            .repo_owner("Eunecod")
            .repo_name("mvtool-tui")
            .bin_name("mvtool_1.1.3.exe")
            .show_download_progress(true)
            .show_output(true)
            .no_confirm(true)
            .current_version(cargo_crate_version!());

        self.status = Some(self.builder.build()?);
        if let Some(ref status) = self.status
        {
            self_update::version::bump_is_greater(cargo_crate_version!(), &status.get_latest_release().unwrap().version)?;
        }

        return Ok(());
    }

    pub fn update(&self)
    {
        if let Some(ref status) = self.status
        {
            let _ = status.update();
        }
    }
}