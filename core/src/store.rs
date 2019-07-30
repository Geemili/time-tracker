use crate::{Meta, Patch, PatchRef};
use std::error::Error;

pub trait Store {
    type Error: Error;

    fn get_device_meta(&self, device_id: &str) -> Result<Meta, Self::Error>;
    fn get_patch(&self, patch_ref: &str) -> Result<Patch, Self::Error>;
}

