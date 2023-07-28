use crate::mapper;
use crate::prelude::*;
use clap::Args;
use ptree;
use ptree::{print_tree, TreeBuilder};

#[derive(Debug, Args, Clone)]
#[command(aliases=["ls", "l"])]
/// List projects
pub struct CommandList {
    pub folder: Option<String>,
    #[arg(short, long)]
    pub flat: bool,
    #[arg(short = 't', long = "type", id = "TYPE")]
    pub env_type: Option<EnvironmentType>,
    #[arg(short, long)]
    pub max_depth: Option<u32>,
}

impl CommandList {
    pub fn execute(mut self, root: &Path) -> Result<()> {
        self.env_type = self.env_type.or(Some(EnvironmentType::Project));

        let root = self
            .folder
            .clone()
            .and_then(|f| mapper::find_environment(root, &f))
            .map(|e| e.source)
            .unwrap_or(root.to_path_buf());

        let enviroments = mapper::map_directory(&root).context("fails to find environments")?;

        if self.flat {
            self.print_flat(enviroments, 0);
        } else {
            self.print_tree(root, enviroments);
        }
        Ok(())
    }
    fn should_print(&self, env: &Environment, depth: u32) -> bool {
        let ctype = env.details.enviroment_type();
        return (self.max_depth.is_none() || self.max_depth.unwrap() > depth)
            && ctype as u8 <= self.env_type.unwrap() as u8;
    }

    fn print_tree(self, root: PathBuf, enviroments: Vec<Environment>) {
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<root>")
            .to_string();

        let mut tree_builder = ptree::TreeBuilder::new(name);
        for child in enviroments {
            self.build_tree(child, 0, &mut tree_builder);
        }
        let tree = tree_builder.build();
        let _ = print_tree(&tree);
    }

    fn build_tree(&self, env: Environment, depth: u32, tree_builder: &mut TreeBuilder) {
        if self.should_print(&env, depth) {
            let name = if env.details.enviroment_type() == EnvironmentType::SubProject {
                "+ ".to_string() + &env.name
            } else {
                env.name
            };
            tree_builder.begin_child(name);
            for child in env.children {
                self.build_tree(child, depth + 1, tree_builder);
            }
            tree_builder.end_child();
        }
    }

    fn print_flat(&self, environments: Vec<Environment>, depth: u32) {
        for env in environments {
            if self.should_print(&env, depth) {
                println!("{}", env.name);
                self.print_flat(env.children, depth + 1);
            }
        }
    }
}
