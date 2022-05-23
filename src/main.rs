use clap::Parser;
use gitlab::api::{self, projects, ApiError, Query};
use gitlab::types as glt;
use gitlab::Gitlab;
use std::path::PathBuf;

mod config;

use config::{Config, Project};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,
}

enum ApplySpecMode {
    CREATE,
    UPDATE,
}

fn apply_project_spec(client: &Gitlab, project_spec: &Project, mode: ApplySpecMode) {
    match mode {
        ApplySpecMode::UPDATE => {
            let project_endpoint = projects::EditProject::builder()
                .project(project_spec.name.as_ref())
                .default_branch(&project_spec.default_branch)
                .emails_disabled(project_spec.emails_disabled)
                .container_registry_enabled(project_spec.container_registry_enabled)
                .visibility(project_spec.visibility)
                .public_builds(project_spec.public_builds)
                .only_allow_merge_if_pipeline_succeeds(
                    project_spec.only_allow_merge_if_pipeline_succeeds,
                )
                .allow_merge_on_skipped_pipeline(project_spec.allow_merge_on_skipped_pipeline)
                .only_allow_merge_if_all_discussions_are_resolved(
                    project_spec.only_allow_merge_if_all_discussions_are_resolved,
                )
                .merge_method(project_spec.merge_method)
                .squash_option(project_spec.squash_option)
                .merge_pipelines_enabled(project_spec.merge_pipelines_enabled)
                .merge_trains_enabled(project_spec.merge_trains_enabled)
                .remove_source_branch_after_merge(project_spec.remove_source_branch_after_merge)
                .printing_merge_requests_link_enabled(
                    project_spec.printing_merge_requests_link_enabled,
                )
                .lfs_enabled(project_spec.lfs_enabled)
                .request_access_enabled(project_spec.request_access_enabled)
                .auto_devops_enabled(project_spec.auto_devops_enabled)
                .approvals_before_merge(project_spec.approvals_before_merge)
                .mirror(project_spec.mirror)
                .packages_enabled(project_spec.package_enabled)
                .service_desk_enabled(project_spec.service_desk_enabled)
                .issues_enabled(project_spec.issues_enabled)
                .merge_requests_enabled(project_spec.merge_requests_enabled)
                .jobs_enabled(project_spec.jobs_enabled)
                .wiki_enabled(project_spec.wiki_enabled)
                .snippets_enabled(project_spec.snippets_enabled)
                .build()
                .unwrap();
            let _: () = api::ignore(project_endpoint).query(client).unwrap();
        }
        ApplySpecMode::CREATE => {
            let project_endpoint = projects::CreateProject::builder()
                .path(&project_spec.name)
                .default_branch(&project_spec.default_branch)
                .emails_disabled(project_spec.emails_disabled)
                .container_registry_enabled(project_spec.container_registry_enabled)
                .visibility(project_spec.visibility)
                .public_builds(project_spec.public_builds)
                .only_allow_merge_if_pipeline_succeeds(
                    project_spec.only_allow_merge_if_pipeline_succeeds,
                )
                .allow_merge_on_skipped_pipeline(project_spec.allow_merge_on_skipped_pipeline)
                .only_allow_merge_if_all_discussions_are_resolved(
                    project_spec.only_allow_merge_if_all_discussions_are_resolved,
                )
                .merge_method(project_spec.merge_method)
                .squash_option(project_spec.squash_option)
                .merge_pipelines_enabled(project_spec.merge_pipelines_enabled)
                .merge_trains_enabled(project_spec.merge_trains_enabled)
                .remove_source_branch_after_merge(project_spec.remove_source_branch_after_merge)
                .printing_merge_request_link_enabled(
                    project_spec.printing_merge_requests_link_enabled,
                )
                .lfs_enabled(project_spec.lfs_enabled)
                .request_access_enabled(project_spec.request_access_enabled)
                .auto_devops_enabled(project_spec.auto_devops_enabled)
                .approvals_before_merge(project_spec.approvals_before_merge)
                .mirror(project_spec.mirror)
                .packages_enabled(project_spec.package_enabled)
                .issues_enabled(project_spec.issues_enabled)
                .merge_requests_enabled(project_spec.merge_requests_enabled)
                .jobs_enabled(project_spec.jobs_enabled)
                .wiki_enabled(project_spec.wiki_enabled)
                .snippets_enabled(project_spec.snippets_enabled)
                .build()
                .unwrap();
            let _: () = api::ignore(project_endpoint).query(client).unwrap();
        }
    }

    // first unprotect branch before updating
    let unprotect_branch_endpoint = projects::protected_branches::UnprotectBranch::builder()
        .project(project_spec.name.as_ref())
        .name(&project_spec.default_branch)
        .build()
        .unwrap();

    api::ignore(unprotect_branch_endpoint)
        .query(client)
        .unwrap();

    let protect_branch_endpoint = projects::protected_branches::ProtectBranch::builder()
        .project(project_spec.name.as_ref())
        .name(&project_spec.default_branch)
        .push_access_level(project_spec.default_branch_push_protected_access_level)
        .merge_access_level(project_spec.default_branch_merge_protected_access_level)
        .build()
        .unwrap();

    let _: () = api::ignore(protect_branch_endpoint).query(client).unwrap();
}

fn create_or_update(client: &Gitlab, project_spec: &Project) {
    let endpoint = projects::Project::builder()
        .project(project_spec.name.as_ref())
        .build()
        .unwrap();
    let result: Result<glt::Project, _> = endpoint.query(client);
    match result {
        Ok(project) => {
            println!(
                "{} project exists with id {}, updating it\n",
                project_spec.name, project.id
            );
            apply_project_spec(client, project_spec, ApplySpecMode::UPDATE);
        }
        Err(ApiError::Gitlab { msg }) => match msg.as_str() {
            "404 Project Not Found" => {
                println!("{} project not found, creating it\n", project_spec.name);
                apply_project_spec(client, project_spec, ApplySpecMode::CREATE);
            }
            _ => eprintln!(
                "Error while querying for {} project: {}\n",
                project_spec.name, msg
            ),
        },
        Err(err) => eprintln!("Unexpected error: {:?}\n", err),
    }
}

fn main() {
    let cli = Cli::parse();
    let config = Config::from_file(cli.config.as_deref());
    println!("Config: {:?}\n", config);
    let client = Gitlab::new(config.gitlab.host, config.gitlab.token).unwrap();
    for spec in config.projects.into_iter() {
        create_or_update(&client, &spec)
    }
}
