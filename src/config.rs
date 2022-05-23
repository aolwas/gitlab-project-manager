use serde::Deserialize;
use serde_yaml;
use std::fs;
use std::path::Path;

use gitlab::api::common::{ProtectedAccessLevel, VisibilityLevel};
use gitlab::api::projects::{
    FeatureAccessLevel, FeatureAccessLevelPublic, MergeMethod, SquashOption,
};

#[derive(Deserialize)]
#[serde(remote = "VisibilityLevel")]
enum VisibilityLevelRef {
    Public,
    Internal,
    Private,
}

#[derive(Deserialize)]
#[serde(remote = "ProtectedAccessLevel")]
pub enum ProtectedAccessLevelRef {
    Developer,
    Maintainer,
    Admin,
    NoAccess,
}

#[derive(Deserialize)]
#[serde(remote = "FeatureAccessLevel")]
enum FeatureAccessLevelRef {
    Disabled,
    Private,
    Enabled,
}

#[derive(Deserialize)]
#[serde(remote = "FeatureAccessLevelPublic")]
enum FeatureAccessLevelPublicRef {
    Disabled,
    Private,
    Enabled,
    Public,
}

#[derive(Deserialize)]
#[serde(remote = "MergeMethod")]
enum MergeMethodRef {
    Merge,
    RebaseMerge,
    FastForward,
}

#[derive(Deserialize)]
#[serde(remote = "SquashOption")]
enum SquashOptionRef {
    Never,
    Always,
    DefaultOn,
    DefaultOff,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub gitlab: Gitlab,
    pub projects: Vec<Project>,
}

#[derive(Deserialize, Debug)]
pub struct Gitlab {
    pub host: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    #[serde(default = "default_branch")]
    pub default_branch: String,

    #[serde(default = "default_protected_access_level_admin")]
    #[serde(with = "ProtectedAccessLevelRef")]
    pub default_branch_push_protected_access_level: ProtectedAccessLevel,

    #[serde(default = "default_protected_access_level_developer")]
    #[serde(with = "ProtectedAccessLevelRef")]
    pub default_branch_merge_protected_access_level: ProtectedAccessLevel,

    //     #[serde(default = "default_feature_level_access_disabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub issues_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_enabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub repository_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_disabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub container_registry_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_enabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub merge_requests_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_enabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub forking_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_disabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub builds_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_disabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub wiki_access_level: FeatureAccessLevel,
    //     #[serde(default = "default_feature_level_access_disabled")]
    //     #[serde(with = "FeatureAccessLevelRef")]
    //     pub snippets_access_level: FeatureAccessLevel,
    #[serde(default = "default_feature_level_access_public_disabled")]
    #[serde(with = "FeatureAccessLevelPublicRef")]
    pub pages_access_level: FeatureAccessLevelPublic,

    #[serde(default = "default_feature_level_access_disabled")]
    #[serde(with = "FeatureAccessLevelRef")]
    pub operations_access_level: FeatureAccessLevel,

    #[serde(default = "default_feature_level_access_public_disabled")]
    #[serde(with = "FeatureAccessLevelPublicRef")]
    pub requirements_access_level: FeatureAccessLevelPublic,

    #[serde(default = "default_feature_level_access_disabled")]
    #[serde(with = "FeatureAccessLevelRef")]
    pub analytics_access_level: FeatureAccessLevel,

    #[serde(default = "default_false")]
    pub emails_disabled: bool,

    #[serde(default = "default_false")]
    pub container_registry_enabled: bool,

    #[serde(default = "default_visibility_level_internal")]
    #[serde(with = "VisibilityLevelRef")]
    pub visibility: VisibilityLevel,

    #[serde(default = "default_false")]
    pub public_builds: bool,

    #[serde(default = "default_true")]
    pub only_allow_merge_if_pipeline_succeeds: bool,

    #[serde(default = "default_false")]
    pub allow_merge_on_skipped_pipeline: bool,

    #[serde(default = "default_true")]
    pub only_allow_merge_if_all_discussions_are_resolved: bool,

    #[serde(default = "default_merge_method_fastforward")]
    #[serde(with = "MergeMethodRef")]
    pub merge_method: MergeMethod,

    #[serde(default = "default_squash_option_default_off")]
    #[serde(with = "SquashOptionRef")]
    pub squash_option: SquashOption,

    #[serde(default = "default_false")]
    pub merge_pipelines_enabled: bool,

    #[serde(default = "default_false")]
    pub merge_trains_enabled: bool,

    #[serde(default = "default_true")]
    pub remove_source_branch_after_merge: bool,

    #[serde(default = "default_true")]
    pub printing_merge_requests_link_enabled: bool,

    #[serde(default = "default_false")]
    pub lfs_enabled: bool,

    #[serde(default = "default_false")]
    pub request_access_enabled: bool,

    #[serde(default = "default_false")]
    pub auto_devops_enabled: bool,

    #[serde(default = "default_u64_one")]
    pub approvals_before_merge: u64,

    #[serde(default = "default_false")]
    pub mirror: bool,

    #[serde(default = "default_false")]
    pub package_enabled: bool,

    #[serde(default = "default_false")]
    pub service_desk_enabled: bool,

    #[serde(default = "default_false")]
    pub issues_enabled: bool,

    #[serde(default = "default_true")]
    pub merge_requests_enabled: bool,

    #[serde(default = "default_false")]
    pub jobs_enabled: bool,

    #[serde(default = "default_false")]
    pub wiki_enabled: bool,

    #[serde(default = "default_false")]
    pub snippets_enabled: bool,
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_u64_one() -> u64 {
    1
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_feature_level_access_disabled() -> FeatureAccessLevel {
    FeatureAccessLevel::Disabled
}

fn default_feature_level_access_public_disabled() -> FeatureAccessLevelPublic {
    FeatureAccessLevelPublic::Disabled
}

fn default_visibility_level_internal() -> VisibilityLevel {
    VisibilityLevel::Internal
}

fn default_merge_method_fastforward() -> MergeMethod {
    MergeMethod::FastForward
}

fn default_squash_option_default_off() -> SquashOption {
    SquashOption::DefaultOff
}

fn default_protected_access_level_admin() -> ProtectedAccessLevel {
    ProtectedAccessLevel::Admin
}

fn default_protected_access_level_developer() -> ProtectedAccessLevel {
    ProtectedAccessLevel::Developer
}

impl Config {
    pub fn from_file(config_path: Option<&Path>) -> Config {
        let path = config_path.unwrap_or(Path::new("config.yaml"));
        let contents = fs::read_to_string(path)
            .expect(format!("Could not read file `{}`", path.display()).as_str());
        let config: Config = serde_yaml::from_str(&contents)
            .expect(format!("Unable to load config from `{}`", path.display()).as_str());
        config
    }
}
