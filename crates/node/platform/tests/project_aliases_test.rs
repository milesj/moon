use moon_cache::CacheEngine;
use moon_config::{
    GlobalProjectConfig, NodeConfig, NodeProjectAliasFormat, ToolchainConfig, WorkspaceConfig,
    WorkspaceProjects,
};
use moon_node_platform::NodePlatform;
use moon_platform::Platformable;
use moon_project_graph::ProjectGraph;
use moon_test_utils::{assert_snapshot, create_sandbox_with_config, Sandbox};
use rustc_hash::FxHashMap;

async fn get_aliases_graph(node_config: NodeConfig) -> (ProjectGraph, Sandbox) {
    let workspace_config = WorkspaceConfig {
        projects: WorkspaceProjects::Sources(FxHashMap::from_iter([
            ("noLang".to_owned(), "no-lang".to_owned()),
            ("nodeNameOnly".to_owned(), "node-name-only".to_owned()),
            ("nodeNameScope".to_owned(), "node-name-scope".to_owned()),
        ])),
        ..WorkspaceConfig::default()
    };
    let toolchain_config = ToolchainConfig {
        node: Some(node_config),
        ..ToolchainConfig::default()
    };

    let sandbox = create_sandbox_with_config(
        "project-graph/aliases",
        Some(&workspace_config),
        Some(&toolchain_config),
        None,
    );

    let mut graph = ProjectGraph::generate(
        sandbox.path(),
        &workspace_config,
        &toolchain_config,
        GlobalProjectConfig::default(),
        &CacheEngine::load(sandbox.path()).await.unwrap(),
    )
    .await
    .unwrap();

    graph
        .register_platform(Box::new(NodePlatform::default()))
        .unwrap();

    (graph, sandbox)
}

#[tokio::test]
async fn loads_node_aliases_name_only() {
    let (graph, _sandbox) = get_aliases_graph(NodeConfig {
        alias_package_names: Some(NodeProjectAliasFormat::NameOnly),
        ..NodeConfig::default()
    })
    .await;

    assert_eq!(
        graph.aliases_map,
        FxHashMap::from_iter([
            ("pkg-bar".to_owned(), "nodeNameOnly".to_owned()),
            ("pkg-foo".to_owned(), "nodeNameScope".to_owned())
        ])
    );
}

#[tokio::test]
async fn loads_node_aliases_name_scopes() {
    let (graph, _sandbox) = get_aliases_graph(NodeConfig {
        alias_package_names: Some(NodeProjectAliasFormat::NameAndScope),
        ..NodeConfig::default()
    })
    .await;

    assert_eq!(
        graph.aliases_map,
        FxHashMap::from_iter([
            ("pkg-bar".to_owned(), "nodeNameOnly".to_owned()),
            ("@scope/pkg-foo".to_owned(), "nodeNameScope".to_owned())
        ])
    );
}

#[tokio::test]
async fn returns_project_using_alias() {
    let (graph, _sandbox) = get_aliases_graph(NodeConfig {
        alias_package_names: Some(NodeProjectAliasFormat::NameAndScope),
        ..NodeConfig::default()
    })
    .await;

    assert_eq!(
        graph.load("@scope/pkg-foo").unwrap().id,
        "nodeNameScope".to_owned()
    );
}

#[tokio::test]
async fn graph_uses_id_for_nodes() {
    let (graph, _sandbox) = get_aliases_graph(NodeConfig {
        alias_package_names: Some(NodeProjectAliasFormat::NameAndScope),
        ..NodeConfig::default()
    })
    .await;

    graph.load("pkg-bar").unwrap();
    graph.load("@scope/pkg-foo").unwrap();

    assert_snapshot!(graph.to_dot());
}
