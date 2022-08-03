use structopt::StructOpt;

use crate::{
    api::{CreateBranch, ListBranch, ListProject},
    cli::{Action::*, Branch, CommandLineArgs, Config, MergeRequest, Project},
    print::print,
};

mod api;
mod cli;
mod file;
mod gitlab;
mod print;

fn main() {
    let args = CommandLineArgs::from_args();

    let config = gitlab::read_config();

    match args.action {
        Config(config) => match config {
            Config::Display => {
                println!("{}", serde_json::to_string(&gitlab::read_config()).unwrap());
            }
            Config::Set { url, token } => {
                gitlab::write_config(gitlab::GitlabConfig { url, token });
                println!("{}", serde_json::to_string(&gitlab::read_config()).unwrap());
            }
        },
        Project(project) => match project {
            Project::List { search_name } => {
                let project_list = api::list_project(
                    &config,
                    ListProject {
                        search: search_name.unwrap_or_default(),
                    },
                )
                .unwrap();

                print("Projects", project_list);
            }
        },
        Branch(branch) => match branch {
            Branch::List {
                project_id,
                search_name,
            } => {
                let branch_list = api::list_branch(
                    &config,
                    ListBranch {
                        id: project_id,
                        search: search_name.unwrap_or_default(),
                    },
                )
                .unwrap();

                print("Branch List", branch_list);
            }
            Branch::Create {
                project_id,
                name,
                base_branch,
            } => {
                let new_branch = api::create_branch(
                    &config,
                    CreateBranch {
                        id: project_id,
                        branch: name,
                        ref_branch: base_branch,
                    },
                )
                .unwrap();
                print("New Branch Created", vec![new_branch]);
            }
        },
        MergeRequest(merge_request) => match merge_request {
            MergeRequest::List { project_id } => {
                let mrege_request_list = api::list_merge_requests(&config, project_id).unwrap();

                print("Merge Requests", mrege_request_list);
            }
        },
    }
}
