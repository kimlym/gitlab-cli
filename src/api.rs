use std::fmt::{self, Display, Formatter};

use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::gitlab::GitlabConfig;

pub struct ListProject {
    pub search: String,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

// https://docs.gitlab.com/ee/api/projects.html#list-all-projects
pub fn list_project(config: &GitlabConfig, project: ListProject) -> Result<Vec<Project>, Error> {
    let project_list = Client::new()
        .get(format!("{}/{}", config.url, "api/v4/projects"))
        .header("PRIVATE-TOKEN", &config.token)
        .query(&[("search", project.search)])
        .send()?
        .json::<Vec<Project>>()?;

    Ok(project_list)
}

pub struct ListBranch {
    pub id: i32,
    pub search: String,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Branch {
    pub name: String,
    pub merged: bool,
    pub protected: bool,
    pub developers_can_push: bool,
    pub developers_can_merge: bool,
}

pub fn list_branch(config: &GitlabConfig, branch: ListBranch) -> Result<Vec<Branch>, Error> {
    let branch_list = Client::new()
        .get(format!(
            "{}/{}/{}/{}",
            config.url, "api/v4/projects", branch.id, "repository/branches"
        ))
        .header("PRIVATE-TOKEN", &config.token)
        .query(&[("search", branch.search)])
        .send()?
        .json::<Vec<Branch>>()?;

    Ok(branch_list)
}

pub struct CreateBranch {
    pub id: i32,
    pub branch: String,
    pub ref_branch: String,
}

pub fn create_branch(config: &GitlabConfig, branch: CreateBranch) -> Result<Branch, Error> {
    let created_branch = Client::new()
        .post(format!(
            "{}/{}/{}/{}",
            config.url, "api/v4/projects", branch.id, "repository/branches"
        ))
        .header("PRIVATE-TOKEN", &config.token)
        .query(&[("branch", branch.branch), ("ref", branch.ref_branch)])
        .send()?
        .json::<Branch>()?;

    Ok(created_branch)
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct MergeRequest {
    #[tabled(rename = "id")]
    pub iid: i32,
    pub title: String,
    pub source_branch: String,
    pub target_branch: String,
    pub author: Author,
    pub project_id: i32,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub username: String,
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn list_merge_requests(
    config: &GitlabConfig,
    project_id: i32,
) -> Result<Vec<MergeRequest>, Error> {
    let merge_request_list = Client::new()
        .get(format!(
            "{}/{}/{}/{}",
            config.url, "api/v4/projects", project_id, "merge_requests"
        ))
        .header("PRIVATE-TOKEN", &config.token)
        .query(&[("state", "opened")])
        .send()?
        .json::<Vec<MergeRequest>>()?;

    Ok(merge_request_list)
}

#[cfg(test)]
mod tests {
    use crate::print;

    use super::*;

    #[test]
    fn should_list_project_work() {
        let project_list = list_project(
            &GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            ListProject {
                search: String::from("123"),
            },
        )
        .unwrap();

        assert_eq!(20, project_list.len());
        println!("{:?}", project_list);
    }

    #[test]
    fn should_list_branch_work() {
        let branch_list = list_branch(
            &GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            ListBranch {
                id: 38276649,
                search: String::from(""),
            },
        )
        .unwrap();

        assert_eq!(1, branch_list.len());
        println!("{:?}", branch_list);
    }

    #[test]
    fn should_create_branch_successfully() {
        let _ = create_branch(
            &GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            CreateBranch {
                id: 38276649,
                branch: String::from("new-branch"),
                ref_branch: String::from("main"),
            },
        );
    }

    #[test]
    fn should_list_merge_requests_successfully() {
        let list = list_merge_requests(
            &&GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            38276649,
        )
        .unwrap();

        assert_eq!(0, list.len());
    }
}
