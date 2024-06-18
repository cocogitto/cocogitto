use std::collections::HashMap;

use crate::{monorepo::MonoRepoPackage, BumpProfile, Settings, SETTINGS};

#[derive(Copy, Clone)]
pub enum HookType {
    PreBump,
    PostBump,
}

pub trait Hooks {
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile>;
    fn pre_bump_hooks(&self) -> &Vec<String>;
    fn post_bump_hooks(&self) -> &Vec<String>;

    fn get_hooks(&self, hook_type: HookType) -> &Vec<String> {
        match hook_type {
            HookType::PreBump => self.pre_bump_hooks(),
            HookType::PostBump => self.post_bump_hooks(),
        }
    }

    fn get_profile_hooks(&self, profile: &str, hook_type: HookType) -> &Vec<String> {
        let profile = self
            .bump_profiles()
            .get(profile)
            .expect("Bump profile not found");
        match hook_type {
            HookType::PreBump => &profile.pre_bump_hooks,
            HookType::PostBump => &profile.post_bump_hooks,
        }
    }
}

impl Hooks for Settings {
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile> {
        &self.bump_profiles
    }

    fn pre_bump_hooks(&self) -> &Vec<String> {
        &self.pre_bump_hooks
    }

    fn post_bump_hooks(&self) -> &Vec<String> {
        &self.post_bump_hooks
    }
}

impl Hooks for MonoRepoPackage {
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile> {
        &self.bump_profiles
    }

    fn pre_bump_hooks(&self) -> &Vec<String> {
        self.pre_bump_hooks
            .as_ref()
            .unwrap_or(&SETTINGS.pre_package_bump_hooks)
    }

    fn post_bump_hooks(&self) -> &Vec<String> {
        self.post_bump_hooks
            .as_ref()
            .unwrap_or(&SETTINGS.post_package_bump_hooks)
    }
}
