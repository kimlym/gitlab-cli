use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Config target gitlab and corresponding token
    Config(Config),
    /// Actions upon projects/repositories
    Project(Project),
    /// Actions upon branches of selected repositories
    Branch(Branch),

    /// Actions upon merge requests of selcted repositories
    #[structopt(name = "merge-request", alias = "mr")]
    MergeRequest(MergeRequest),
}
#[derive(Debug, StructOpt)]
pub enum Config {
    #[structopt(name = "set", alias = "s", about = "Set config for gitlab")]
    Set {
        /// Target gitlab url
        #[structopt(short, long)]
        url: String,
        /// Personal token
        #[structopt(short, long)]
        token: String,
    },
    #[structopt(name = "display", alias = "d", about = "Display current config")]
    Display,
}

#[derive(Debug, StructOpt)]
pub enum Project {
    #[structopt(name = "list", alias = "l", about = "List all visible projects")]
    List {
        #[structopt(short, long)]
        search_name: Option<String>,
    },
    #[structopt(about = "Open your favorite Project in your favorite Browser!")]
    Open {
        #[structopt(short, long)]
        project_id: i32,
    },
}

#[derive(Debug, StructOpt)]
pub enum Branch {
    #[structopt(
        name = "list",
        alias = "l",
        about = "List all matching branches of target project"
    )]
    List {
        #[structopt(short, long)]
        project_id: i32,
        #[structopt(short, long)]
        search_name: Option<String>,
    },
    #[structopt(name = "create", alias = "c", about = "Create a new branch")]
    Create {
        #[structopt(short, long)]
        project_id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long, about = "Branch off target branch")]
        base_branch: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum MergeRequest {
    #[structopt(name = "list", alias = "l", about = "List open merge requets")]
    List {
        #[structopt(short, long)]
        project_id: i32,
    },
    #[structopt(alias = "o", about = "Open your MRs in browser")]
    Open {
        #[structopt(short, long)]
        project_id: i32,
        #[structopt(short, name = "mr-id")]
        merge_request_iid: i32,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Gitlab Cli", about = "A command line for easy gitlab usage")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
