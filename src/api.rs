use std::fmt::{self, Display, Formatter};

use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};
use tabled::Tabled;
use terminal_link::Link;

use crate::config::GitlabConfig;

pub struct ListProject {
    pub search: String,
}

#[derive(Debug, Serialize, Deserialize, Tabled, PartialEq)]
pub struct Project {
    #[tabled(display_with("Self::display_with_url", args))]
    pub id: i32,
    pub name: String,
    #[tabled(skip)]
    pub web_url: String,
}

impl Project {
    fn display_with_url(&self) -> String {
        Link::new(&self.id.to_string(), &self.web_url).to_string()
    }
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

// https://docs.gitlab.com/ee/api/projects.html#get-single-project
pub fn get_project(config: &GitlabConfig, project_id: i32) -> Result<Project, Error> {
    let project = Client::new()
        .get(format!(
            "{}/{}/{}",
            config.url, "api/v4/projects", project_id
        ))
        .header("PRIVATE-TOKEN", &config.token)
        .send()?
        .json::<Project>()?;

    Ok(project)
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
    // https://github.com/zhiburt/tabled#format-fields
    #[tabled(rename = "mr id", display_with("Self::display_with_url", args))]
    pub iid: i32,
    pub title: String,
    #[tabled(rename = "source")]
    pub source_branch: String,
    #[tabled(rename = "target")]
    pub target_branch: String,
    pub author: Author,
    #[tabled(rename = "project")]
    pub project_id: i32,
    pub state: String,
    #[tabled(skip)]
    pub web_url: String,
}

impl MergeRequest {
    fn display_with_url(&self) -> String {
        Link::new(&self.iid.to_string(), &self.web_url).to_string()
    }
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

// https://docs.gitlab.com/ee/api/merge_requests.html#get-single-mr
pub fn get_merge_request(
    config: &GitlabConfig,
    project_id: i32,
    merge_request_iid: i32,
) -> Result<MergeRequest, Error> {
    let merge_request = Client::new()
        .get(format!(
            "{}/{}/{}/{}/{}",
            config.url, "api/v4/projects", project_id, "merge_requests", merge_request_iid
        ))
        .header("PRIVATE-TOKEN", &config.token)
        .send()?
        .json::<MergeRequest>()?;

    Ok(merge_request)
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
    fn should_get_project_work() {
        let project = get_project(
            &GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            38276649,
        )
        .unwrap();

        let expected = Project {
            id: 38276649,
            name: String::from("project-with-new-file-67034aa0efaeb123"),
            web_url: String::from("https://gitlab.com/gitlab-qa-sandbox-group-3/qa-test-2022-08-02-09-36-20-a3fb36bffdee7599/project-with-new-file-67034aa0efaeb123"),
        };
        println!("{:?}", project);
        assert_eq!(expected, project);
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
            &GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            15513260,
        )
        .unwrap();

        assert_eq!(20, list.len());
        println!("{:?}", list);
    }

    #[test]
    fn should_get_single_merge_request_successfully() {
        let mr = get_merge_request(
            &GitlabConfig {
                url: String::from("https://gitlab.com"),
                token: String::from(""),
            },
            15513260,
            133,
        )
        .unwrap();

        println!("merge: {:?}", mr);
        if let Some(_mr) = Option::Some(mr) {
            assert!(true);
            assert_eq!("Manual job rules", _mr.title);
        } else {
            assert!(false);
        }
    }
}
