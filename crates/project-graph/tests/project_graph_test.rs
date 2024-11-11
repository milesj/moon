use moon_common::{path::WorkspaceRelativePathBuf, Id};
use moon_config::{
    DependencyConfig, DependencyScope, DependencySource, TaskDependencyConfig, WorkspaceProjects,
    WorkspaceProjectsConfig,
};
use moon_project::{FileGroup, Project};
use moon_project_graph::*;
use moon_query::build_query;
use moon_task::Target;
use moon_test_utils2::*;
use moon_workspace::{
    ExtendProjectData, ExtendProjectEvent, ExtendProjectGraphData, ExtendProjectGraphEvent,
    WorkspaceProjectsCacheState,
};
use petgraph::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use starbase_events::EventState;
use starbase_sandbox::{assert_snapshot, create_sandbox, Sandbox};
use starbase_utils::{fs, json, string_vec};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

pub fn append_file<P: AsRef<Path>>(path: P, data: &str) {
    let mut file = OpenOptions::new().append(true).open(path.as_ref()).unwrap();

    writeln!(file, "\n\n{data}").unwrap();
}

fn map_ids(ids: Vec<Id>) -> Vec<String> {
    ids.into_iter().map(|id| id.to_string()).collect()
}

fn map_ids_from_target(targets: Vec<Target>) -> Vec<String> {
    targets
        .into_iter()
        .map(|target| target.task_id.to_string())
        .collect()
}

fn get_ids_from_projects(projects: Vec<Arc<Project>>) -> Vec<String> {
    let mut ids = projects
        .iter()
        .map(|p| p.id.to_string())
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

mod project_graph {
    use super::*;

    #[tokio::test]
    async fn gets_by_id() {
        let graph = generate_workspace_graph("dependencies").await;

        assert!(graph.get_project("a").is_ok());
    }

    #[tokio::test]
    #[should_panic(expected = "No project has been configured with the identifier or alias z")]
    async fn errors_unknown_id() {
        let graph = generate_workspace_graph("dependencies").await;

        graph.get_project("z").unwrap();
    }

    #[tokio::test]
    async fn gets_by_path() {
        let sandbox = create_sandbox("dependencies");
        let graph = generate_workspace_graph_from_sandbox(sandbox.path()).await;

        assert_eq!(
            graph
                .get_project_from_path(Some(&sandbox.path().join("c/moon.yml")))
                .unwrap()
                .id,
            "c"
        );
    }

    #[tokio::test]
    #[should_panic(expected = "No project could be located starting from path z/moon.yml")]
    async fn errors_non_matching_path() {
        let sandbox = create_sandbox("dependencies");
        let graph = generate_workspace_graph_from_sandbox(sandbox.path()).await;

        graph
            .get_project_from_path(Some(&sandbox.path().join("z/moon.yml")))
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "A project already exists with the identifier id")]
    async fn errors_duplicate_ids() {
        generate_workspace_graph("dupe-folder-conflict").await;
    }

    mod sources {
        use super::*;

        #[tokio::test]
        async fn globs() {
            let graph = generate_workspace_graph("dependencies").await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "b", "c", "d"]
            );
        }

        #[tokio::test]
        async fn globs_with_root() {
            let sandbox = create_sandbox("dependencies");
            let root = sandbox.path().join("dir");

            // Move files so that we can infer a compatible root project name
            fs::copy_dir_all(sandbox.path(), sandbox.path(), &root).unwrap();

            let mut mock = create_workspace_graph_mocker(&root);

            mock.workspace_config.projects = WorkspaceProjects::Globs(string_vec!["*", "."]);

            let graph = mock.build_workspace_graph().await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "b", "c", "d", "dir"]
            );
        }

        #[tokio::test]
        async fn globs_with_config() {
            let sandbox = create_sandbox("locate-configs");
            let mut mock = create_workspace_graph_mocker(sandbox.path());

            mock.workspace_config.projects = WorkspaceProjects::Globs(string_vec!["*/moon.yml"]);

            let graph = mock.build_workspace_graph().await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "c"]
            );
        }

        #[tokio::test]
        async fn paths() {
            let sandbox = create_sandbox("dependencies");
            let mut mock = create_workspace_graph_mocker(sandbox.path());

            mock.workspace_config.projects = WorkspaceProjects::Sources(FxHashMap::from_iter([
                (Id::raw("c"), "c".into()),
                (Id::raw("b"), "b".into()),
            ]));

            let graph = mock.build_workspace_graph().await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["b", "c"]
            );
        }

        #[tokio::test]
        async fn paths_and_globs() {
            let sandbox = create_sandbox("dependencies");
            let mut mock = create_workspace_graph_mocker(sandbox.path());

            mock.workspace_config.projects = WorkspaceProjects::Both(WorkspaceProjectsConfig {
                globs: string_vec!["{a,c}"],
                sources: FxHashMap::from_iter([
                    (Id::raw("b"), "b".into()),
                    (Id::raw("root"), ".".into()),
                ]),
            });

            let graph = mock.build_workspace_graph().await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "b", "c", "root"]
            );
        }

        #[tokio::test]
        async fn ignores_git_moon_folders() {
            let sandbox = create_sandbox("dependencies");

            sandbox.enable_git();
            sandbox.create_file(".moon/workspace.yml", "projects: ['*']");

            let graph = generate_workspace_graph_from_sandbox(sandbox.path()).await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "b", "c", "d"]
            );
        }

        #[tokio::test]
        async fn filters_dot_folders() {
            let sandbox = create_sandbox("dependencies");
            sandbox.create_file(".foo/moon.yml", "");

            let graph = generate_workspace_graph_from_sandbox(sandbox.path()).await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "b", "c", "d"]
            );
        }

        #[tokio::test]
        async fn filters_using_gitignore() {
            let sandbox = create_sandbox("type-constraints");

            sandbox.enable_git();
            sandbox.create_file(".gitignore", "*-other");

            let mut mock = create_workspace_graph_mocker(sandbox.path());
            mock.with_vcs();

            let graph = mock.build_workspace_graph().await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["app", "library", "tool", "unknown"]
            );
        }

        #[tokio::test]
        async fn supports_id_formats() {
            let graph = generate_workspace_graph("ids").await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                [
                    "Capital",
                    "PascalCase",
                    "With_nums-123",
                    "camelCase",
                    "kebab-case",
                    "snake_case"
                ]
            );
        }
    }

    mod cache {
        use super::*;
        use moon_cache::CacheEngine;
        use moon_workspace::ProjectBuildData;

        const CACHE_PATH: &str = ".moon/cache/states/workspaceGraph.json";
        const STATE_PATH: &str = ".moon/cache/states/projectsBuildData.json";

        async fn do_generate(root: &Path) -> WorkspaceGraph {
            let cache_engine = CacheEngine::new(root).unwrap();

            let mut mock = create_workspace_graph_mocker(root);
            mock.with_vcs();

            mock.build_workspace_graph_with_options(WorkspaceMockOptions {
                cache: Some(cache_engine),
                ..Default::default()
            })
            .await
        }

        async fn generate_cached_project_graph(
            func: impl FnOnce(&Sandbox),
        ) -> (Sandbox, WorkspaceGraph) {
            let sandbox = create_sandbox("dependencies");

            func(&sandbox);

            let graph = do_generate(sandbox.path()).await;

            (sandbox, graph)
        }

        #[tokio::test]
        async fn doesnt_cache_if_no_vcs() {
            let (sandbox, _graph) = generate_cached_project_graph(|_| {}).await;

            assert!(!sandbox.path().join(CACHE_PATH).exists())
        }

        #[tokio::test]
        async fn caches_if_vcs() {
            let (sandbox, _graph) = generate_cached_project_graph(|sandbox| {
                sandbox.enable_git();
            })
            .await;

            assert!(sandbox.path().join(CACHE_PATH).exists());
        }

        #[tokio::test]
        async fn loads_from_cache() {
            let (sandbox, graph) = generate_cached_project_graph(|sandbox| {
                sandbox.enable_git();
            })
            .await;
            let cached_graph = do_generate(sandbox.path()).await;

            assert_eq!(
                graph.projects.get_node_keys(),
                cached_graph.projects.get_node_keys()
            );
        }

        #[tokio::test]
        async fn creates_states_and_manifests() {
            let (sandbox, _graph) = generate_cached_project_graph(|sandbox| {
                sandbox.enable_git();
            })
            .await;

            let state: WorkspaceProjectsCacheState =
                json::read_file(sandbox.path().join(STATE_PATH)).unwrap();

            assert_eq!(
                state.projects,
                FxHashMap::from_iter([
                    (
                        Id::raw("a"),
                        ProjectBuildData {
                            node_index: Some(NodeIndex::from(2)),
                            source: "a".into(),
                            ..Default::default()
                        }
                    ),
                    (
                        Id::raw("b"),
                        ProjectBuildData {
                            node_index: Some(NodeIndex::from(1)),
                            source: "b".into(),
                            ..Default::default()
                        }
                    ),
                    (
                        Id::raw("c"),
                        ProjectBuildData {
                            node_index: Some(NodeIndex::from(0)),
                            source: "c".into(),
                            ..Default::default()
                        }
                    ),
                    (
                        Id::raw("d"),
                        ProjectBuildData {
                            node_index: Some(NodeIndex::from(3)),
                            source: "d".into(),
                            ..Default::default()
                        }
                    ),
                ])
            );

            assert!(sandbox
                .path()
                .join(".moon/cache/hashes")
                .join(format!("{}.json", state.last_hash))
                .exists());
        }

        mod invalidation {
            use super::*;

            async fn test_invalidate(func: impl FnOnce(&Sandbox)) {
                let (sandbox, _graph) = generate_cached_project_graph(|sandbox| {
                    sandbox.enable_git();
                })
                .await;

                let state1: WorkspaceProjectsCacheState =
                    json::read_file(sandbox.path().join(STATE_PATH)).unwrap();

                func(&sandbox);
                do_generate(sandbox.path()).await;

                let state2: WorkspaceProjectsCacheState =
                    json::read_file(sandbox.path().join(STATE_PATH)).unwrap();

                assert_ne!(state1.last_hash, state2.last_hash);
            }

            #[tokio::test]
            async fn with_workspace_changes() {
                test_invalidate(|sandbox| {
                    sandbox.create_file(".moon/workspace.yml", "# Changes");
                })
                .await;
            }

            #[tokio::test]
            async fn with_toolchain_changes() {
                test_invalidate(|sandbox| {
                    sandbox.create_file(".moon/toolchain.yml", "# Changes");
                })
                .await;
            }

            #[tokio::test]
            async fn with_tasks_changes() {
                test_invalidate(|sandbox| {
                    sandbox.create_file(".moon/tasks.yml", "# Changes");
                })
                .await;
            }

            #[tokio::test]
            async fn with_scoped_tasks_changes() {
                test_invalidate(|sandbox| {
                    sandbox.create_file(".moon/tasks/node.yml", "# Changes");
                })
                .await;
            }

            #[tokio::test]
            async fn with_project_config_changes() {
                test_invalidate(|sandbox| {
                    sandbox.create_file("a/moon.yml", "# Changes");
                })
                .await;

                test_invalidate(|sandbox| {
                    sandbox.create_file("b/moon.yml", "# Changes");
                })
                .await;
            }

            #[tokio::test]
            async fn with_new_source_add() {
                test_invalidate(|sandbox| {
                    sandbox.create_file("z/moon.yml", "# Changes");
                })
                .await;
            }
        }
    }

    mod cycles {
        use super::*;

        #[tokio::test]
        async fn can_generate_with_cycles() {
            let graph = generate_workspace_graph("cycle").await;

            assert_eq!(
                get_ids_from_projects(graph.get_all_projects().unwrap()),
                ["a", "b", "c"]
            );

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("a").unwrap())
                ),
                ["b"]
            );

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("b").unwrap())
                ),
                ["c"]
            );

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("c").unwrap())
                ),
                string_vec![]
            );
        }
    }

    mod inheritance {
        use super::*;

        async fn generate_inheritance_project_graph(fixture: &str) -> WorkspaceGraph {
            let sandbox = create_sandbox(fixture);
            let mut mock = create_workspace_graph_mocker(sandbox.path());

            mock.inherited_tasks = mock
                .config_loader
                .load_tasks_manager_from(sandbox.path(), sandbox.path().join(".moon"))
                .unwrap();

            mock.build_workspace_graph().await
        }

        #[tokio::test]
        async fn inherits_scoped_tasks() {
            let graph = generate_inheritance_project_graph("inheritance/scoped").await;

            assert_eq!(
                map_ids_from_target(graph.get_project("node").unwrap().task_targets.clone()),
                ["global", "global-node", "node"]
            );

            assert_eq!(
                map_ids_from_target(
                    graph
                        .get_project("node-library")
                        .unwrap()
                        .task_targets
                        .clone()
                ),
                [
                    "global",
                    "global-node",
                    "global-node-library",
                    "node-library"
                ]
            );

            assert_eq!(
                map_ids_from_target(
                    graph
                        .get_project("system-library")
                        .unwrap()
                        .task_targets
                        .clone()
                ),
                ["global", "system-library"]
            );
        }

        #[tokio::test]
        async fn inherits_tagged_tasks() {
            let graph = generate_inheritance_project_graph("inheritance/tagged").await;

            assert_eq!(
                map_ids_from_target(graph.get_project("mage").unwrap().task_targets.clone()),
                ["mage", "magic"]
            );

            assert_eq!(
                map_ids_from_target(graph.get_project("warrior").unwrap().task_targets.clone()),
                ["warrior", "weapons"]
            );

            assert_eq!(
                map_ids_from_target(graph.get_project("priest").unwrap().task_targets.clone()),
                ["magic", "priest", "weapons"]
            );
        }

        #[tokio::test]
        async fn inherits_file_groups() {
            let graph = generate_inheritance_project_graph("inheritance/file-groups").await;
            let project = graph.get_project("project").unwrap();

            assert_eq!(
                project.file_groups.get("sources").unwrap(),
                &FileGroup::new_with_source(
                    "sources",
                    [WorkspaceRelativePathBuf::from("project/src/**/*")]
                )
                .unwrap()
            );
            assert_eq!(
                project.file_groups.get("tests").unwrap(),
                &FileGroup::new_with_source(
                    "tests",
                    [WorkspaceRelativePathBuf::from("project/tests/**/*")]
                )
                .unwrap()
            );
            assert_eq!(
                project.file_groups.get("configs").unwrap(),
                &FileGroup::new_with_source(
                    "configs",
                    [WorkspaceRelativePathBuf::from("project/config.*")]
                )
                .unwrap()
            );
        }

        #[tokio::test]
        async fn inherits_implicit_deps_inputs() {
            let graph = generate_inheritance_project_graph("inheritance/implicits").await;
            let task = graph.get_task_from_project("project", "example").unwrap();

            assert_eq!(
                task.deps,
                [
                    TaskDependencyConfig::new(Target::parse("project:other").unwrap()),
                    TaskDependencyConfig::new(Target::parse("base:local").unwrap()),
                ]
            );

            assert_eq!(
                task.input_files,
                FxHashSet::from_iter([WorkspaceRelativePathBuf::from("project/local.txt")])
            );

            assert_eq!(
                task.input_globs,
                FxHashSet::from_iter([
                    WorkspaceRelativePathBuf::from(".moon/*.yml"),
                    WorkspaceRelativePathBuf::from("project/global.*")
                ])
            );
        }
    }

    mod expansion {
        use super::*;

        #[tokio::test]
        async fn expands_project() {
            let graph = generate_workspace_graph("expansion").await;
            let project = graph.get_project("project").unwrap();

            assert_eq!(
                project.dependencies,
                vec![DependencyConfig {
                    id: Id::raw("base"),
                    scope: DependencyScope::Development,
                    source: DependencySource::Explicit,
                    ..Default::default()
                }]
            );

            assert!(graph
                .get_task_from_project("project", "build")
                .unwrap()
                .deps
                .is_empty());
        }

        #[tokio::test]
        async fn expands_tasks() {
            let graph = generate_workspace_graph("expansion").await;
            let task = graph.get_task_from_project("tasks", "build").unwrap();

            assert_eq!(task.args, string_vec!["a", "../other.yaml", "b"]);

            assert_eq!(
                task.input_files,
                FxHashSet::from_iter([
                    WorkspaceRelativePathBuf::from("tasks/config.json"),
                    WorkspaceRelativePathBuf::from("other.yaml"),
                ])
            );

            assert_eq!(
                task.input_globs,
                FxHashSet::from_iter([
                    WorkspaceRelativePathBuf::from(".moon/*.yml"),
                    WorkspaceRelativePathBuf::from("tasks/file.*"),
                ])
            );

            assert_eq!(
                task.output_files,
                FxHashSet::from_iter([WorkspaceRelativePathBuf::from("tasks/build")])
            );

            assert_eq!(
                task.deps,
                [TaskDependencyConfig::new(
                    Target::parse("project:build").unwrap()
                )]
            );
        }

        #[tokio::test]
        async fn expands_tag_deps_in_task() {
            let graph = generate_workspace_graph("expansion").await;
            let task = graph.get_task_from_project("tasks", "test-tags").unwrap();

            assert_eq!(
                task.deps,
                [
                    TaskDependencyConfig::new(Target::parse("tag-one:test").unwrap()),
                    TaskDependencyConfig::new(Target::parse("tag-three:test").unwrap()),
                ]
            );
        }
    }

    mod dependencies {
        use super::*;

        #[tokio::test]
        async fn lists_ids_of_dependencies() {
            let graph = generate_workspace_graph("dependencies").await;

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("a").unwrap())
                ),
                ["b"]
            );
            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("b").unwrap())
                ),
                ["c"]
            );
            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("c").unwrap())
                ),
                string_vec![]
            );
            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("d").unwrap())
                ),
                ["c", "b", "a"]
            );
        }

        #[tokio::test]
        async fn lists_ids_of_dependents() {
            let graph = generate_workspace_graph("dependencies").await;

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependents_of(&graph.get_project("a").unwrap())
                ),
                ["d"]
            );
            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependents_of(&graph.get_project("b").unwrap())
                ),
                ["d", "a"]
            );
            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependents_of(&graph.get_project("c").unwrap())
                ),
                ["d", "b"]
            );
            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependents_of(&graph.get_project("d").unwrap())
                ),
                string_vec![]
            );
        }

        mod isolation {
            use super::*;

            #[tokio::test]
            async fn no_depends_on() {
                let sandbox = create_sandbox("dependency-types");
                let mock = create_workspace_graph_mocker(sandbox.path());

                let graph = mock.build_workspace_graph_for(&["no-depends-on"]).await;

                assert_eq!(map_ids(graph.projects.get_node_keys()), ["no-depends-on"]);
            }

            #[tokio::test]
            async fn some_depends_on() {
                let sandbox = create_sandbox("dependency-types");
                let mock = create_workspace_graph_mocker(sandbox.path());

                let graph = mock.build_workspace_graph_for(&["some-depends-on"]).await;

                assert_eq!(
                    map_ids(graph.projects.get_node_keys()),
                    ["a", "c", "some-depends-on"]
                );
            }

            #[tokio::test]
            async fn from_task_deps() {
                let sandbox = create_sandbox("dependency-types");
                let mock = create_workspace_graph_mocker(sandbox.path());

                let graph = mock.build_workspace_graph_for(&["from-task-deps"]).await;

                assert_eq!(
                    map_ids(graph.projects.get_node_keys()),
                    ["b", "c", "from-task-deps"]
                );

                let deps = &graph.get_project("from-task-deps").unwrap().dependencies;

                assert_eq!(deps[0].scope, DependencyScope::Build);
                assert_eq!(deps[1].scope, DependencyScope::Build);
            }

            #[tokio::test]
            async fn from_root_task_deps() {
                let sandbox = create_sandbox("dependency-types");
                let mock = create_workspace_graph_mocker(sandbox.path());

                let graph = mock
                    .build_workspace_graph_for(&["from-root-task-deps"])
                    .await;

                assert_eq!(
                    map_ids(graph.projects.get_node_keys()),
                    ["root", "from-root-task-deps"]
                );

                let deps = &graph
                    .get_project("from-root-task-deps")
                    .unwrap()
                    .dependencies;

                assert_eq!(deps[0].scope, DependencyScope::Root);
            }

            #[tokio::test]
            async fn self_task_deps() {
                let sandbox = create_sandbox("dependency-types");
                let mock = create_workspace_graph_mocker(sandbox.path());

                let graph = mock.build_workspace_graph_for(&["self-task-deps"]).await;

                assert_eq!(map_ids(graph.projects.get_node_keys()), ["self-task-deps"]);
            }
        }
    }

    mod aliases {
        use super::*;

        async fn generate_aliases_project_graph() -> WorkspaceGraph {
            generate_aliases_project_graph_for_fixture("aliases").await
        }

        async fn generate_aliases_project_graph_for_fixture(fixture: &str) -> WorkspaceGraph {
            let sandbox = create_sandbox(fixture);
            let mock = create_workspace_graph_mocker(sandbox.path());
            let context = mock.create_context();

            // Set aliases for projects
            context
                .extend_project_graph
                .on(
                    |event: Arc<ExtendProjectGraphEvent>,
                     data: Arc<RwLock<ExtendProjectGraphData>>| async move {
                        let mut data = data.write().await;

                        for (id, source) in &event.sources {
                            let alias_path = source.join("alias").to_path(&event.workspace_root);

                            if alias_path.exists() {
                                data.aliases.push((
                                    id.to_owned(),
                                    fs::read_file(alias_path).unwrap().trim().to_owned(),
                                ));
                            }
                        }

                        Ok(EventState::Continue)
                    },
                )
                .await;

            // Set implicit deps for projects
            context
                .extend_project
                .on(
                    |event: Arc<ExtendProjectEvent>,
                     data: Arc<RwLock<ExtendProjectData>>| async move {
                        let mut data = data.write().await;

                        if event.project_id == "explicit-and-implicit" || event.project_id == "implicit" {
                            data.dependencies.push(DependencyConfig {
                                id: Id::raw("@three"),
                                scope: DependencyScope::Build,
                                ..Default::default()
                            });
                        }

                        if event.project_id == "implicit" {
                            data.dependencies.push(DependencyConfig {
                                id: Id::raw("@one"),
                                scope: DependencyScope::Peer,
                                ..Default::default()
                            });
                        }

                        Ok(EventState::Continue)
                    },
                )
                .await;

            mock.build_workspace_graph_with_options(WorkspaceMockOptions {
                context: Some(context),
                ..Default::default()
            })
            .await
        }

        #[tokio::test]
        async fn loads_aliases() {
            let graph = generate_aliases_project_graph().await;

            assert_snapshot!(graph.projects.to_dot());

            assert_eq!(
                graph.projects.aliases(),
                FxHashMap::from_iter([
                    ("@one", &Id::raw("alias-one")),
                    ("@two", &Id::raw("alias-two")),
                    ("@three", &Id::raw("alias-three")),
                ])
            );
        }

        #[tokio::test]
        async fn doesnt_set_alias_if_same_as_id() {
            let graph = generate_aliases_project_graph().await;

            assert_eq!(graph.get_project("alias-same-id").unwrap().alias, None);
        }

        #[tokio::test]
        async fn doesnt_set_alias_if_a_project_has_the_id() {
            let graph = generate_aliases_project_graph_for_fixture("aliases-conflict-ids").await;

            assert_eq!(graph.get_project("one").unwrap().alias, None);
            assert_eq!(graph.get_project("two").unwrap().alias, None);
        }

        #[tokio::test]
        async fn can_get_projects_by_alias() {
            let graph = generate_aliases_project_graph().await;

            assert!(graph.get_project("@one").is_ok());
            assert!(graph.get_project("@two").is_ok());
            assert!(graph.get_project("@three").is_ok());

            assert_eq!(
                graph.get_project("@one").unwrap(),
                graph.get_project("alias-one").unwrap()
            );
            assert_eq!(
                graph.get_project("@two").unwrap(),
                graph.get_project("alias-two").unwrap()
            );
            assert_eq!(
                graph.get_project("@three").unwrap(),
                graph.get_project("alias-three").unwrap()
            );
        }

        #[tokio::test]
        async fn can_depends_on_by_alias() {
            let graph = generate_aliases_project_graph().await;

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("explicit").unwrap())
                ),
                ["alias-two", "alias-one"]
            );

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("explicit-and-implicit").unwrap())
                ),
                ["alias-three", "alias-two"]
            );

            assert_eq!(
                map_ids(
                    graph
                        .projects
                        .dependencies_of(&graph.get_project("implicit").unwrap())
                ),
                ["alias-three", "alias-one"]
            );
        }

        #[tokio::test]
        async fn removes_or_flattens_dupes() {
            let graph = generate_aliases_project_graph().await;

            assert_eq!(
                graph.get_project("dupes-depends-on").unwrap().dependencies,
                vec![DependencyConfig {
                    id: Id::raw("alias-two"),
                    scope: DependencyScope::Build,
                    source: DependencySource::Explicit,
                    ..DependencyConfig::default()
                }]
            );

            assert_eq!(
                graph
                    .get_task_from_project("dupes-task-deps", "no-dupes")
                    .unwrap()
                    .deps,
                [TaskDependencyConfig::new(
                    Target::parse("alias-one:global").unwrap()
                )]
            );
        }

        #[tokio::test]
        async fn can_use_aliases_as_task_deps() {
            let graph = generate_aliases_project_graph().await;

            assert_eq!(
                graph
                    .get_task_from_project("tasks", "with-aliases")
                    .unwrap()
                    .deps,
                [
                    TaskDependencyConfig::new(Target::parse("alias-one:global").unwrap()),
                    TaskDependencyConfig::new(Target::parse("alias-three:global").unwrap()),
                    TaskDependencyConfig::new(Target::parse("implicit:global").unwrap()),
                ]
            );
        }

        #[tokio::test]
        #[should_panic(expected = "Project two is already using the alias @test")]
        async fn errors_duplicate_aliases() {
            generate_aliases_project_graph_for_fixture("aliases-conflict").await;
        }

        #[tokio::test]
        async fn ignores_duplicate_aliases_if_ids_match() {
            let sandbox = create_sandbox("aliases-conflict");
            let mock = create_workspace_graph_mocker(sandbox.path());
            let context = mock.create_context();

            context
                .extend_project_graph
                .on(
                    |event: Arc<ExtendProjectGraphEvent>,
                     data: Arc<RwLock<ExtendProjectGraphData>>| async move {
                        let mut data = data.write().await;

                        for (id, _) in &event.sources {
                            // Add dupes
                            data.aliases.push((id.to_owned(), format!("@{id}")));
                            data.aliases.push((id.to_owned(), format!("@{id}")));
                        }

                        Ok(EventState::Continue)
                    },
                )
                .await;

            let graph = mock
                .build_workspace_graph_with_options(WorkspaceMockOptions {
                    context: Some(context),
                    ..Default::default()
                })
                .await;

            assert!(graph.get_project("@one").is_ok());
            assert!(graph.get_project("@two").is_ok());
        }
    }

    mod type_constraints {
        use super::*;

        async fn generate_type_constraints_project_graph(
            func: impl FnOnce(&Sandbox),
        ) -> WorkspaceGraph {
            let sandbox = create_sandbox("type-constraints");

            func(&sandbox);

            let mut mock = create_workspace_graph_mocker(sandbox.path());

            mock.workspace_config
                .constraints
                .enforce_project_type_relationships = true;

            mock.build_workspace_graph().await
        }

        #[tokio::test]
        async fn app_can_use_unknown() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("app/moon.yml"), "dependsOn: [unknown]");
            })
            .await;
        }

        #[tokio::test]
        async fn app_can_use_library() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("app/moon.yml"), "dependsOn: [library]");
            })
            .await;
        }

        #[tokio::test]
        async fn app_can_use_tool() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("app/moon.yml"), "dependsOn: [tool]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid project relationship. Project app of type application")]
        async fn app_cannot_use_app() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("app/moon.yml"),
                    "dependsOn: [app-other]",
                );
            })
            .await;
        }

        #[tokio::test]
        async fn library_can_use_unknown() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("library/moon.yml"),
                    "dependsOn: [unknown]",
                );
            })
            .await;
        }

        #[tokio::test]
        async fn library_can_use_library() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("library/moon.yml"),
                    "dependsOn: [library-other]",
                );
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid project relationship. Project library of type library")]
        async fn library_cannot_use_app() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("library/moon.yml"), "dependsOn: [app]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid project relationship. Project library of type library")]
        async fn library_cannot_use_tool() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("library/moon.yml"), "dependsOn: [tool]");
            })
            .await;
        }

        #[tokio::test]
        async fn tool_can_use_unknown() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("tool/moon.yml"), "dependsOn: [unknown]");
            })
            .await;
        }

        #[tokio::test]
        async fn tool_can_use_library() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("tool/moon.yml"), "dependsOn: [library]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid project relationship. Project tool of type tool")]
        async fn tool_cannot_use_app() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("tool/moon.yml"), "dependsOn: [app]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid project relationship. Project tool of type tool")]
        async fn tool_cannot_use_tool() {
            generate_type_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("tool/moon.yml"),
                    "dependsOn: [tool-other]",
                );
            })
            .await;
        }
    }

    mod tag_constraints {
        use super::*;

        async fn generate_tag_constraints_project_graph(
            func: impl FnOnce(&Sandbox),
        ) -> WorkspaceGraph {
            let sandbox = create_sandbox("tag-constraints");

            func(&sandbox);

            let mut mock = create_workspace_graph_mocker(sandbox.path());

            mock.workspace_config.constraints.tag_relationships.insert(
                Id::raw("warrior"),
                vec![Id::raw("barbarian"), Id::raw("paladin"), Id::raw("druid")],
            );

            mock.workspace_config.constraints.tag_relationships.insert(
                Id::raw("mage"),
                vec![Id::raw("wizard"), Id::raw("sorcerer"), Id::raw("druid")],
            );

            mock.build_workspace_graph().await
        }

        #[tokio::test]
        async fn can_depon_tags_but_self_empty() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("a/moon.yml"), "dependsOn: [b, c]");
                append_file(sandbox.path().join("b/moon.yml"), "tags: [barbarian]");
                append_file(sandbox.path().join("c/moon.yml"), "tags: [druid]");
            })
            .await;
        }

        #[tokio::test]
        async fn ignores_unconfigured_relationships() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(sandbox.path().join("a/moon.yml"), "dependsOn: [b, c]");
                append_file(sandbox.path().join("b/moon.yml"), "tags: [some]");
                append_file(sandbox.path().join("c/moon.yml"), "tags: [value]");
            })
            .await;
        }

        #[tokio::test]
        async fn matches_with_source_tag() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b]\ntags: [warrior]",
                );
                append_file(sandbox.path().join("b/moon.yml"), "tags: [warrior]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid tag relationship. Project a with tag #warrior")]
        async fn errors_for_no_source_tag_match() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b]\ntags: [warrior]",
                );
                append_file(sandbox.path().join("b/moon.yml"), "tags: [other]");
            })
            .await;
        }

        #[tokio::test]
        async fn matches_with_allowed_tag() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b]\ntags: [warrior]",
                );
                append_file(sandbox.path().join("b/moon.yml"), "tags: [barbarian]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid tag relationship. Project a with tag #warrior")]
        async fn errors_for_no_allowed_tag_match() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b]\ntags: [warrior]",
                );
                append_file(sandbox.path().join("b/moon.yml"), "tags: [other]");
            })
            .await;
        }

        #[tokio::test]
        #[should_panic(expected = "Invalid tag relationship. Project a with tag #mage")]
        async fn errors_for_depon_empty_tags() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b]\ntags: [mage]",
                );
            })
            .await;
        }

        #[tokio::test]
        async fn matches_multiple_source_tags_to_a_single_allowed_tag() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b]\ntags: [warrior, mage]",
                );
                append_file(sandbox.path().join("b/moon.yml"), "tags: [druid]");
            })
            .await;
        }

        #[tokio::test]
        async fn matches_single_source_tag_to_a_multiple_allowed_tags() {
            generate_tag_constraints_project_graph(|sandbox| {
                append_file(
                    sandbox.path().join("a/moon.yml"),
                    "dependsOn: [b, c]\ntags: [mage]",
                );
                append_file(sandbox.path().join("b/moon.yml"), "tags: [druid, wizard]");
                append_file(
                    sandbox.path().join("c/moon.yml"),
                    "tags: [wizard, sorcerer, barbarian]",
                );
            })
            .await;
        }
    }

    mod query {
        use super::*;

        #[tokio::test]
        async fn by_language() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("language!=[typescript,python]").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a", "d"]);
        }

        #[tokio::test]
        async fn by_project() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("project~{b,d}").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["b", "d"]);
        }

        #[tokio::test]
        async fn by_project_type() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("projectType!=[library]").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a", "c"]);
        }

        #[tokio::test]
        async fn by_project_source() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("projectSource~a").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a"]);
        }

        #[tokio::test]
        async fn by_tag() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("tag=[three,five]").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["b", "c"]);
        }

        #[tokio::test]
        async fn by_task() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("task=[test,build]").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a", "c", "d"]);
        }

        #[tokio::test]
        async fn by_task_platform() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("taskPlatform=[node]").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a", "b"]);

            let projects = graph
                .query_projects(build_query("taskPlatform=system").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["c", "d"]);
        }

        #[tokio::test]
        async fn by_task_type() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("taskType=run").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a"]);
        }

        #[tokio::test]
        async fn with_and_conditions() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("task=build && taskPlatform=deno").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["d"]);
        }

        #[tokio::test]
        async fn with_or_conditions() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(build_query("language=javascript || language=typescript").unwrap())
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["a", "b"]);
        }

        #[tokio::test]
        async fn with_nested_conditions() {
            let graph = generate_workspace_graph("query").await;

            let projects = graph
                .query_projects(
                    build_query("projectType=library && (taskType=build || tag=three)").unwrap(),
                )
                .unwrap();

            assert_eq!(get_ids_from_projects(projects), vec!["b", "d"]);
        }
    }

    mod to_dot {
        use super::*;

        #[tokio::test]
        async fn renders_full() {
            let graph = generate_workspace_graph("dependencies").await;

            assert_snapshot!(graph.projects.to_dot());
        }

        #[tokio::test]
        async fn renders_partial() {
            let sandbox = create_sandbox("dependencies");
            let mock = create_workspace_graph_mocker(sandbox.path());

            let graph = mock.build_workspace_graph_for(&["b"]).await;

            assert_snapshot!(graph.projects.to_dot());
        }
    }

    mod custom_id {
        use super::*;

        #[tokio::test]
        async fn can_load_by_new_id() {
            let sandbox = create_sandbox("custom-id");
            let graph = generate_workspace_graph_from_sandbox(sandbox.path()).await;

            assert_eq!(graph.get_project("foo").unwrap().id, "foo");
            assert_eq!(graph.get_project("bar-renamed").unwrap().id, "bar-renamed");
            assert_eq!(graph.get_project("baz-renamed").unwrap().id, "baz-renamed");
        }

        #[tokio::test]
        async fn tasks_can_depend_on_new_id() {
            let sandbox = create_sandbox("custom-id");
            let graph = generate_workspace_graph_from_sandbox(sandbox.path()).await;
            let task = graph.get_task_from_project("foo", "noop").unwrap();

            assert_eq!(
                task.deps,
                [
                    TaskDependencyConfig::new(Target::parse("bar-renamed:noop").unwrap()),
                    TaskDependencyConfig::new(Target::parse("baz-renamed:noop").unwrap()),
                ]
            );
        }

        #[tokio::test]
        async fn doesnt_error_for_duplicate_folder_names_if_renamed() {
            let graph = generate_workspace_graph("dupe-folder-ids").await;

            assert!(graph.get_project("one").is_ok());
            assert!(graph.get_project("two").is_ok());
        }

        #[tokio::test]
        #[should_panic(expected = "A project already exists with the identifier foo")]
        async fn errors_duplicate_ids_from_rename() {
            generate_workspace_graph("custom-id-conflict").await;
        }
    }
}
