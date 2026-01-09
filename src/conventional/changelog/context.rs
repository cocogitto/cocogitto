use serde::Serialize;
use tera::Context;

use crate::{conventional::changelog::release::Release, git::oid::OidOf};

/// A wrapper to append remote repository information to template context
#[derive(Debug)]
pub struct RemoteContext {
    pub remote: String,
    pub repository: String,
    pub owner: String,
}

#[derive(Debug)]
pub struct MonoRepoContext {
    pub package_lock: bool,
    pub packages: Vec<PackageBumpContext>,
}

#[derive(Debug, Serialize)]
pub struct PackageBumpContext {
    pub package_name: String,
    pub package_path: String,
    pub version: OidOf,
    pub from: Option<OidOf>,
}

#[derive(Debug)]
pub struct PackageContext {
    pub package_name: String,
}

pub(crate) trait ToContext {
    fn to_context(&self) -> Context;
}

impl ToContext for MonoRepoContext {
    fn to_context(&self) -> Context {
        let mut context = tera::Context::new();
        context.insert("package_lock", &self.package_lock);
        context.insert("packages", &self.packages);
        context
    }
}

impl ToContext for PackageContext {
    fn to_context(&self) -> Context {
        let mut context = tera::Context::new();
        context.insert("package_name", &self.package_name);
        context
    }
}

impl ToContext for RemoteContext {
    fn to_context(&self) -> Context {
        let mut context = tera::Context::new();
        context.insert("platform", &format!("https://{}", self.remote.as_str()));
        context.insert("owner", self.owner.as_str());
        context.insert(
            "repository_url",
            &format!("https://{}/{}/{}", self.remote, self.owner, self.repository),
        );

        context
    }
}

impl RemoteContext {
    pub fn try_new(
        remote: Option<String>,
        repository: Option<String>,
        owner: Option<String>,
    ) -> Option<Self> {
        match (remote, repository, owner) {
            (Some(remote), Some(repository), Some(owner)) => Some(Self {
                remote,
                repository,
                owner,
            }),
            (None, None, None) => None,
            _ => panic!("Changelog remote context should be set. Missing one of 'remote', 'repository', 'owner' in changelog configuration")
        }
    }
}

impl ToContext for &Release {
    fn to_context(&self) -> Context {
        Context::from_serialize(self).expect("Valid release")
    }
}

impl ToContext for &mut Release {
    fn to_context(&self) -> Context {
        Context::from_serialize(self).expect("Valid release")
    }
}
