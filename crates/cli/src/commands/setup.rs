use crate::helpers::{create_progress_bar, load_workspace};
use moon_platform::Runtime;
use moon_project_graph::project_graph::ProjectGraph;
use moon_runner::{DepGraph, Runner};
use moon_utils::is_test_env;

pub async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let done = create_progress_bar("Downloading and installing tools...");

    let workspace = load_workspace().await?;
    let project_graph = ProjectGraph::generate(&workspace).await?;
    let mut dep_graph = DepGraph::generate(&project_graph);

    if let Some(node) = &workspace.config.node {
        let runtime = Runtime::Node(node.version.to_owned());

        dep_graph.setup_tool(&runtime);

        if !is_test_env() {
            dep_graph.install_deps(&runtime)?;
        }
    }

    Runner::new().run(workspace, dep_graph, None).await?;

    done("Setup complete", true);

    Ok(())
}
