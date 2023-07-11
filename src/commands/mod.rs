use crate::mapper;
use crate::prelude::*;
use ptree::{print_tree, TreeBuilder};

pub fn command_list(
    root: &Path,
    folder: Option<String>,
    flat: bool,
    envtype: Option<EnvironmentType>,
    max_depth: Option<i32>,
) {
    let envtype = envtype.unwrap_or(EnvironmentType::Project);

    let root = folder
        .and_then(|f| mapper::find_environment(root, &f))
        .map(|e| e.source)
        .unwrap_or(root.to_path_buf());

    let mut enviroments = mapper::map_directory(&root);

    if flat {
        while let Some(env) = enviroments.pop() {
            println!("{}", env.name);
            enviroments.extend(env.children);
        }
    } else {
        let name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<root>")
            .to_string();

        let mut tree_builder = ptree::TreeBuilder::new(name);
        for child in enviroments {
            build_tree(child, flat, envtype, max_depth, &mut tree_builder);
        }
        let tree = tree_builder.build();
        let _ = print_tree(&tree);
    }
}

fn build_tree(
    env: Environment,
    flat: bool,
    envtype: EnvironmentType,
    max_depth: Option<i32>,
    tree_builder: &mut TreeBuilder,
) {
    let ctype = env.details.enviroment_type();
    if (max_depth.is_none() || max_depth.is_some_and(|m| m > 0)) && ctype as u8 <= envtype as u8 {
        let name = if ctype == EnvironmentType::SubProject {
            "+ ".to_string() + &env.name
        } else {
            env.name
        };
        tree_builder.begin_child(name);
        for child in env.children {
            build_tree(child, flat, envtype, max_depth.map(|m| m - 1), tree_builder);
        }
        tree_builder.end_child();
    }
}
