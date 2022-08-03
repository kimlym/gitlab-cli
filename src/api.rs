use reqwest::Error;
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
    let project_list = reqwest::blocking::Client::new()
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
    let branch_list = reqwest::blocking::Client::new()
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
    let created_branch = reqwest::blocking::Client::new()
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

#[cfg(test)]
mod tests {
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
}
