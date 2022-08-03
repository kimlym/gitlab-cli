use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Config target gitlab and corresponding token
    Config(Config),
    /// Actions upon projects/repositories
    Project(Project),
    /// Actions upon branches of selected repositories
    Branch(Branch),
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

#[derive(StructOpt, Debug)]
#[structopt(name = "config", alias = "c", about = "Config target gitlab")]
pub struct ConfigCommand {
    pub url: String,
    pub token: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Gitlab Cli", about = "A command line for easy gitlab usage")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
