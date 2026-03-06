use crate::{BumpProfile, HookType, MonoRepoPackage, Settings, SETTINGS};
use std::collections::HashMap;

/// # Hooks Trait
///
/// A trait that provides access to version bump hooks and profiles.
///
/// This trait defines methods for retrieving pre-bump and post-bump hooks,
/// as well as accessing hooks from specific bump profiles.
pub trait Hooks {
    /// Returns a reference to the map of bump profiles.
    ///
    /// # Returns
    ///
    /// * `&HashMap<String, BumpProfile>` - A map of profile names to their configurations
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile>;

    /// Returns a reference to the list of pre-bump hooks.
    ///
    /// Pre-bump hooks are executed before the version bump occurs.
    ///
    /// # Returns
    ///
    /// * `&Vec<String>` - A list of hook commands or scripts
    fn pre_bump_hooks(&self) -> &Vec<String>;

    /// Returns a reference to the list of post-bump hooks.
    ///
    /// Post-bump hooks are executed after the version bump occurs.
    ///
    /// # Returns
    ///
    /// * `&Vec<String>` - A list of hook commands or scripts
    fn post_bump_hooks(&self) -> &Vec<String>;

    /// Gets the hooks for a specific hook type.
    ///
    /// This method retrieves either pre-bump or post-bump hooks based on the
    /// provided `HookType`.
    ///
    /// # Arguments
    ///
    /// * `hook_type` - The type of hook to retrieve (`HookType::PreBump` or `HookType::PostBump`)
    ///
    /// # Returns
    ///
    /// * `&Vec<String>` - A list of hook commands or scripts for the specified type
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::{Hooks, HookType};
    /// use std::collections::HashMap;
    /// 
    /// struct MyConfig {
    ///     bump_profiles: HashMap<String, cocogitto_settings::BumpProfile>,
    ///     pre_bump_hooks: Vec<String>,
    ///     post_bump_hooks: Vec<String>,
    /// }
    /// 
    /// impl Hooks for MyConfig {
    ///     fn bump_profiles(&self) -> &HashMap<String, cocogitto_settings::BumpProfile> {
    ///         &self.bump_profiles
    ///     }
    ///     fn pre_bump_hooks(&self) -> &Vec<String> { &self.pre_bump_hooks }
    ///     fn post_bump_hooks(&self) -> &Vec<String> { &self.post_bump_hooks }
    /// }
    /// 
    /// let my_config = MyConfig {
    ///     bump_profiles: HashMap::new(),
    ///     pre_bump_hooks: vec!["echo 'pre-bump'".to_string()],
    ///     post_bump_hooks: vec!["echo 'post-bump'".to_string()],
    /// };
    /// 
    /// let hooks = my_config.get_hooks(HookType::PreBump);
    /// for hook in hooks {
    ///     println!("Pre-bump hook: {}", hook);
    /// }
    /// ```
    fn get_hooks(&self, hook_type: HookType) -> &Vec<String> {
        match hook_type {
            HookType::PreBump => self.pre_bump_hooks(),
            HookType::PostBump => self.post_bump_hooks(),
        }
    }

    /// Gets the hooks for a specific profile and hook type.
    ///
    /// This method retrieves hooks from a named bump profile for the specified
    /// hook type. It will panic if the requested profile does not exist.
    ///
    /// # Arguments
    ///
    /// * `profile` - The name of the bump profile to use
    /// * `hook_type` - The type of hook to retrieve (`HookType::PreBump` or `HookType::PostBump`)
    ///
    /// # Returns
    ///
    /// * `&Vec<String>` - A list of hook commands or scripts from the specified profile
    ///
    /// # Panics
    ///
    /// Panics if the specified bump profile does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::{Hooks, HookType, BumpProfile};
    /// use std::collections::HashMap;
    /// 
    /// struct MyConfig {
    ///     bump_profiles: HashMap<String, BumpProfile>,
    ///     pre_bump_hooks: Vec<String>,
    ///     post_bump_hooks: Vec<String>,
    /// }
    /// 
    /// impl Hooks for MyConfig {
    ///     fn bump_profiles(&self) -> &HashMap<String, BumpProfile> { &self.bump_profiles }
    ///     fn pre_bump_hooks(&self) -> &Vec<String> { &self.pre_bump_hooks }
    ///     fn post_bump_hooks(&self) -> &Vec<String> { &self.post_bump_hooks }
    /// }
    /// 
    /// let mut profiles = HashMap::new();
    /// profiles.insert("production".to_string(), BumpProfile {
    ///     pre_bump_hooks: vec!["echo 'production pre-bump'".to_string()],
    ///     post_bump_hooks: vec!["echo 'production post-bump'".to_string()],
    /// });
    /// 
    /// let my_config = MyConfig {
    ///     bump_profiles: profiles,
    ///     pre_bump_hooks: vec![],
    ///     post_bump_hooks: vec![],
    /// };
    /// 
    /// let hooks = my_config.get_profile_hooks("production", HookType::PostBump);
    /// for hook in hooks {
    ///     println!("Production post-bump hook: {}", hook);
    /// }
    /// ```
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
