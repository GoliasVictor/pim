use std::collections::HashSet;

use crate::mapper;
use crate::prelude::*;
use clap::Args;
use clap::ValueEnum;
use ptree;
use ptree::{print_tree, TreeBuilder};
use serde::Deserialize;
use serde::Serialize;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Style {
    Flat,
    Tree,
    GroupByLanguage,
    GroupByCategory,
}

#[derive(Debug, Args, Clone)]
#[command(aliases=["ls", "l"])]
/// List projects
pub struct CommandList {
    pub folder: Option<String>,
    #[arg(short, long, default_value = "tree")]
    pub style: Style,
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

        let environments = mapper::map_directory(&root).context("fails to find environments")?;
        match self.style {
            Style::Flat => {
                self.print_flat(environments, 0);
            }
            Style::GroupByCategory => {
                self.print_by(environments,"without category".to_owned(), |env| env.categories.clone());
            },
            Style::GroupByLanguage => {
                self.print_by(environments,"undefined languages".to_owned(), |env| {
                    match &env.details {
                        EnvironmentDetails::Project { languages }=> languages.clone(),
                        _ => HashSet::new()
                    }
                })
            }
            _ => {
                self.print_tree(root, environments);
            }
        }
        Ok(())
    }
    fn should_print(&self, env: &Environment, depth: u32) -> bool {
        let ctype = env.details.enviroment_type();
        return (self.max_depth.is_none() || self.max_depth.unwrap() > depth)
            && ctype as u8 <= self.env_type.unwrap() as u8;
    }

    fn print_tree(self, root: PathBuf, environments: Vec<Environment>) {
        fn build_tree(
            command: &CommandList,
            env: Environment,
            depth: u32,
            tree_builder: &mut TreeBuilder,
        ) {
            if command.should_print(&env, depth) {
                let name = if env.details.enviroment_type() == EnvironmentType::SubProject {
                    "+ ".to_string() + &env.name
                } else {
                    env.name
                };
                tree_builder.begin_child(name);
                for child in env.children {
                    build_tree(command, child, depth + 1, tree_builder);
                }
                tree_builder.end_child();
            }
        }
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<root>")
            .to_string();

        let mut tree_builder = ptree::TreeBuilder::new(name);
        for child in environments {
            build_tree(&self, child, 0, &mut tree_builder);
        }
        let tree = tree_builder.build();
        let _ = print_tree(&tree);
    }

    fn all_envs<'a>(&self, environments: &'a [Environment], depth: u32) -> Vec<&'a Environment> {
        environments
            .iter()
            .filter(|env| self.should_print(env, depth))
            .flat_map(|env| {
                [env]
                    .into_iter()
                    .chain(self.all_envs(&env.children, depth + 1))
            })
            .collect()
    }
    
    fn print_by(&self, environments: Vec<Environment>, empty_name :  String, get_vec : fn(&Environment) -> HashSet<String>) {
        let envs = self.all_envs(&environments, 0);
        let mut hash = HashMap::new();
        let mut emptys  = vec![];
        for env in  envs  {
            let vec = get_vec(env);
            if vec.is_empty(){
                emptys.push(env);
            } else {
                for category in vec.iter() {
                    hash.entry(category.to_lowercase().to_owned()).or_insert_with(Vec::new).push(env)
                }
            }
        }
        for (category, envs) in hash  {
            println!("{}:", category);
            for env in envs {
                println!("  {}", env.name);
            }
        }
        println!("{}: ", empty_name);
        for env in emptys {
            println!("  {}", env.name);
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
