use crate::errors::DepGraphError;
use moon_action::ActionNode;
use moon_logger::{color, debug, map_list, trace};
use moon_platform::Runtime;
use moon_project::Project;
use moon_project_graph::ProjectGraph;
use moon_task::{Target, TargetError, TargetProjectScope, Task, TouchedFilePaths};
use petgraph::algo::toposort;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use rustc_hash::{FxHashMap, FxHashSet};

pub use petgraph::graph::NodeIndex;

const LOG_TARGET: &str = "moon:dep-graph";

pub type DepGraphType = DiGraph<ActionNode, ()>;
pub type BatchedTopoSort = Vec<Vec<NodeIndex>>;

/// A directed acyclic graph (DAG) for the work that needs to be processed, based on a
/// project or task's dependency chain. This is also known as a "task graph" (not to
/// be confused with our tasks) or a "dependency graph".
pub struct DepGraph {
    pub graph: DepGraphType,

    indices: FxHashMap<ActionNode, NodeIndex>,
}

impl DepGraph {
    pub fn default() -> Self {
        debug!(target: LOG_TARGET, "Creating dependency graph",);

        DepGraph {
            graph: Graph::new(),
            indices: FxHashMap::default(),
        }
    }

    pub fn get_index_from_node(&self, node: &ActionNode) -> Option<&NodeIndex> {
        self.indices.get(node)
    }

    pub fn get_node_from_index(&self, index: &NodeIndex) -> Option<&ActionNode> {
        self.graph.node_weight(*index)
    }

    // Projects support overriding the the version of their language (tool),
    // so we need to account for this via the runtime. However, some actions require
    // the workspace version of the language, so we must extract 2 runtimes here.
    pub fn get_runtimes_from_project(
        &self,
        project: &Project,
        project_graph: &ProjectGraph,
    ) -> (Runtime, Runtime) {
        let mut project_runtime = None;
        let mut workspace_runtime = None;

        for platform in project_graph.platforms.list() {
            if platform.matches(&project.config, None) {
                project_runtime = platform.get_runtime_from_config(
                    Some(&project.config),
                    &project_graph.workspace_config,
                );

                workspace_runtime =
                    platform.get_runtime_from_config(None, &project_graph.workspace_config);

                break;
            }
        }

        (
            project_runtime.unwrap_or(Runtime::System),
            workspace_runtime.unwrap_or(Runtime::System),
        )
    }

    pub fn install_project_deps(&mut self, runtime: &Runtime, project_id: &str) -> NodeIndex {
        let node = ActionNode::InstallProjectDeps(runtime.clone(), project_id.to_owned());

        if let Some(index) = self.get_index_from_node(&node) {
            return *index;
        }

        trace!(
            target: LOG_TARGET,
            "Adding install {} dependencies (in project {}) node to graph",
            runtime.label(),
            color::id(project_id)
        );

        // Before we install deps, we must ensure the language has been installed
        let index = self.insert_node(&node);
        let setup_tool_index = self.setup_tool(runtime);

        self.graph.add_edge(index, setup_tool_index, ());

        index
    }

    pub fn install_workspace_deps(&mut self, runtime: &Runtime) -> NodeIndex {
        let node = ActionNode::InstallDeps(runtime.clone());

        if let Some(index) = self.get_index_from_node(&node) {
            return *index;
        }

        trace!(
            target: LOG_TARGET,
            "Adding install {} dependencies (in workspace) node to graph",
            runtime.label()
        );

        // Before we install deps, we must ensure the language has been installed
        let index = self.insert_node(&node);
        let setup_tool_index = self.setup_tool(runtime);

        self.graph.add_edge(index, setup_tool_index, ());

        index
    }

    pub fn run_dependents_for_target<T: AsRef<Target>>(
        &mut self,
        target: T,
        project_graph: &ProjectGraph,
    ) -> Result<(), DepGraphError> {
        let target = target.as_ref();

        trace!(
            target: LOG_TARGET,
            "Adding dependents to run for target {}",
            color::target(&target.id),
        );

        let (project_id, task_id) = target.ids()?;
        let dependents = project_graph.get_dependents_of(&project_id)?;

        for dependent_id in dependents {
            let dependent = project_graph.load(&dependent_id)?;

            if dependent.tasks.contains_key(&task_id) {
                self.run_target(Target::new(&dependent_id, &task_id)?, project_graph, &None)?;
            }
        }

        Ok(())
    }

    pub fn run_target<T: AsRef<Target>>(
        &mut self,
        target: T,
        project_graph: &ProjectGraph,
        touched_files: &Option<TouchedFilePaths>,
    ) -> Result<Vec<Target>, DepGraphError> {
        let target = target.as_ref();
        let mut inserted_targets = vec![];

        match &target.project {
            // :task
            TargetProjectScope::All => {
                for project_id in project_graph.ids() {
                    let project = project_graph.load(&project_id)?;

                    if project.tasks.contains_key(&target.task_id) {
                        let all_target = Target::new(&project.id, &target.task_id)?;

                        if self
                            .run_target_by_project(
                                &all_target,
                                &project,
                                project_graph,
                                touched_files,
                            )?
                            .is_some()
                        {
                            inserted_targets.push(all_target);
                        }
                    }
                }
            }
            // ^:task
            TargetProjectScope::Deps => {
                target.fail_with(TargetError::NoProjectDepsInRunContext)?;
            }
            // project:task
            TargetProjectScope::Id(project_id) => {
                let project = project_graph.load(project_id)?;
                let own_target = Target::new(&project.id, &target.task_id)?;

                if self
                    .run_target_by_project(&own_target, &project, project_graph, touched_files)?
                    .is_some()
                {
                    inserted_targets.push(own_target);
                }
            }
            // ~:task
            TargetProjectScope::OwnSelf => {
                target.fail_with(TargetError::NoProjectSelfInRunContext)?;
            }
        };

        Ok(inserted_targets)
    }

    pub fn run_target_by_project<T: AsRef<Target>>(
        &mut self,
        target: T,
        project: &Project,
        project_graph: &ProjectGraph,
        touched_files: &Option<TouchedFilePaths>,
    ) -> Result<Option<NodeIndex>, DepGraphError> {
        let target = target.as_ref();
        let node = ActionNode::RunTarget(target.id.to_owned());

        if let Some(index) = self.get_index_from_node(&node) {
            return Ok(Some(*index));
        }

        // Compare against touched files if provided
        if let Some(touched) = touched_files {
            if !project.get_task(&target.task_id)?.is_affected(touched)? {
                trace!(
                    target: LOG_TARGET,
                    "Target {} not affected based on touched files, skipping",
                    color::target(&target.id),
                );

                return Ok(None);
            }
        }

        trace!(
            target: LOG_TARGET,
            "Adding run target {} node to graph",
            color::target(&target.id),
        );

        let (project_runtime, workspace_runtime) =
            self.get_runtimes_from_project(project, project_graph);

        // We should install deps & sync projects *before* running targets
        let index = self.insert_node(&node);
        let sync_project_index =
            self.sync_project(&workspace_runtime, &project.id, project_graph)?;
        let install_deps_index = if project_runtime == workspace_runtime {
            self.install_workspace_deps(&workspace_runtime)
        } else {
            self.install_project_deps(&project_runtime, &project.id)
        };

        self.graph.add_edge(index, install_deps_index, ());
        self.graph.add_edge(index, sync_project_index, ());

        // And we also need to wait on all dependent targets
        let task = project.get_task(&target.task_id)?;

        if !task.deps.is_empty() {
            trace!(
                target: LOG_TARGET,
                "Adding dependencies {} to target {}",
                map_list(&task.deps, |f| color::symbol(f)),
                color::target(&target.id),
            );

            for dep_index in
                self.run_target_task_dependencies(task, project_graph, touched_files)?
            {
                self.graph.add_edge(index, dep_index, ());
            }
        }

        Ok(Some(index))
    }

    pub fn run_target_task_dependencies(
        &mut self,
        task: &Task,
        project_graph: &ProjectGraph,
        touched_files: &Option<TouchedFilePaths>,
    ) -> Result<Vec<NodeIndex>, DepGraphError> {
        let parallel = task.options.run_deps_in_parallel;
        let mut indexes = vec![];
        let mut previous_target_index = None;

        for dep_target_id in &task.deps {
            let dep_target = Target::parse(dep_target_id)?;
            let dep_project = match &dep_target.project_id {
                Some(id) => project_graph.load(id)?,
                None => {
                    continue;
                }
            };

            if let Some(dep_index) =
                self.run_target_by_project(&dep_target, &dep_project, project_graph, touched_files)?
            {
                // When parallel, parent depends on child
                if parallel {
                    indexes.push(dep_index);

                    // When serial, next child depends on previous child
                } else if let Some(prev) = previous_target_index {
                    self.graph.add_edge(dep_index, prev, ());
                }

                previous_target_index = Some(dep_index);
            }
        }

        if !parallel {
            indexes.push(previous_target_index.unwrap());
        }

        Ok(indexes)
    }

    pub fn run_targets_by_id(
        &mut self,
        target_ids: &[String],
        project_graph: &ProjectGraph,
        touched_files: &Option<TouchedFilePaths>,
    ) -> Result<Vec<Target>, DepGraphError> {
        let mut qualified_targets = vec![];

        for target_id in target_ids {
            qualified_targets.extend(self.run_target(
                Target::parse(target_id)?,
                project_graph,
                touched_files,
            )?);
        }

        Ok(qualified_targets)
    }

    pub fn setup_tool(&mut self, runtime: &Runtime) -> NodeIndex {
        let node = ActionNode::SetupTool(runtime.clone());

        if let Some(index) = self.get_index_from_node(&node) {
            return *index;
        }

        trace!(
            target: LOG_TARGET,
            "Adding setup {} tool node to graph",
            runtime.label()
        );

        self.insert_node(&node)
    }

    pub fn sync_project(
        &mut self,
        runtime: &Runtime,
        project_id: &str,
        project_graph: &ProjectGraph,
    ) -> Result<NodeIndex, DepGraphError> {
        let node = ActionNode::SyncProject(runtime.clone(), project_id.to_owned());

        if let Some(index) = self.get_index_from_node(&node) {
            return Ok(*index);
        }

        trace!(
            target: LOG_TARGET,
            "Adding sync project {} node to graph",
            color::id(project_id),
        );

        // Syncing depends on the language's tool to be installed
        let index = self.insert_node(&node);
        let setup_tool_index = self.setup_tool(runtime);

        self.graph.add_edge(index, setup_tool_index, ());

        // And we should also depend on other projects
        for dep_project_id in project_graph.get_dependencies_of(project_id)? {
            let dep_index = self.sync_project(runtime, &dep_project_id, project_graph)?;

            self.graph.add_edge(index, dep_index, ());
        }

        Ok(index)
    }

    pub fn sort_topological(&self) -> Result<Vec<NodeIndex>, DepGraphError> {
        let list = match toposort(&self.graph, None) {
            Ok(nodes) => nodes,
            Err(error) => {
                return Err(DepGraphError::CycleDetected(
                    self.get_node_from_index(&error.node_id()).unwrap().label(),
                ));
            }
        };

        Ok(list.into_iter().rev().collect())
    }

    pub fn sort_batched_topological(&self) -> Result<BatchedTopoSort, DepGraphError> {
        let mut batches: BatchedTopoSort = vec![];

        // Count how many times an index is referenced across nodes and edges
        let mut node_counts = FxHashMap::<NodeIndex, u32>::default();

        for ix in self.graph.node_indices() {
            node_counts.entry(ix).and_modify(|e| *e += 1).or_insert(0);

            for dep_ix in self.graph.neighbors(ix) {
                node_counts
                    .entry(dep_ix)
                    .and_modify(|e| *e += 1)
                    .or_insert(0);
            }
        }

        // Gather root nodes (count of 0)
        let mut root_nodes = FxHashSet::<NodeIndex>::default();

        for (ix, count) in &node_counts {
            if *count == 0 {
                root_nodes.insert(*ix);
            }
        }

        // If no root nodes are found, but nodes exist, then we have a cycle
        if root_nodes.is_empty() && !node_counts.is_empty() {
            self.detect_cycle()?;
        }

        while !root_nodes.is_empty() {
            // Push this batch onto the list
            batches.push(root_nodes.clone().into_iter().collect());

            // Reset the root nodes and find new ones after decrementing
            let mut next_root_nodes = FxHashSet::<NodeIndex>::default();

            for ix in &root_nodes {
                for dep_ix in self.graph.neighbors(*ix) {
                    let count = node_counts
                        .entry(dep_ix)
                        .and_modify(|e| *e -= 1)
                        .or_insert(0);

                    if *count == 0 {
                        next_root_nodes.insert(dep_ix);
                    }
                }
            }

            root_nodes = next_root_nodes;
        }

        Ok(batches.into_iter().rev().collect())
    }

    pub fn to_dot(&self) -> String {
        let graph = self.graph.map(|_, n| n.label(), |_, e| e);

        let dot = Dot::with_attr_getters(
            &graph,
            &[Config::EdgeNoLabel, Config::NodeNoLabel],
            &|_, e| {
                if e.source().index() == 0 {
                    String::from("arrowhead=none")
                } else {
                    String::from("arrowhead=box, arrowtail=box")
                }
            },
            &|_, n| {
                let id = n.1;

                format!(
                    "label=\"{}\" style=filled, shape=oval, fillcolor=gray, fontcolor=black",
                    id
                )
            },
        );

        format!("{:?}", dot)
    }

    // PRIVATE

    #[track_caller]
    fn detect_cycle(&self) -> Result<(), DepGraphError> {
        use petgraph::algo::kosaraju_scc;

        let scc = kosaraju_scc(&self.graph);
        let cycle = scc
            .last()
            .unwrap()
            .iter()
            .map(|i| self.get_node_from_index(i).unwrap().label())
            .collect::<Vec<String>>()
            .join(" → ");

        Err(DepGraphError::CycleDetected(cycle))
    }

    fn insert_node(&mut self, node: &ActionNode) -> NodeIndex {
        let index = self.graph.add_node(node.to_owned());

        self.indices.insert(node.to_owned(), index);

        index
    }
}
