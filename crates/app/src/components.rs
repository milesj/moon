use crate::app::Commands;
use crate::session::CliSession;
use moon_action::Action;
use moon_action_context::ActionContext;
use moon_action_graph::ActionGraph;
use moon_action_pipeline2::ActionPipeline as ExpActionPipeline;
use moon_platform::PlatformManager;
use moon_project_graph::{
    ExtendProjectData, ExtendProjectEvent, ExtendProjectGraphData, ExtendProjectGraphEvent,
    ProjectGraphBuilderContext,
};
use starbase_events::{Emitter, EventState};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn run_action_pipeline(
    session: &CliSession,
    action_graph: ActionGraph,
    action_context: Option<ActionContext>,
) -> miette::Result<Vec<Action>> {
    let project_graph = session.get_project_graph().await?;

    let mut pipeline = ExpActionPipeline::new(session.get_app_context()?, project_graph);

    if let Some(concurrency) = &session.cli.concurrency {
        pipeline.concurrency = *concurrency;
    }

    match &session.cli.command {
        Commands::Check(cmd) => {
            pipeline.bail = true;
            pipeline.summarize = cmd.summary;
        }
        Commands::Ci(_) => {
            pipeline.report_name = "ciReport.json".into();
            pipeline.summarize = true;
        }
        Commands::Run(cmd) => {
            pipeline.bail = true;
            pipeline.summarize = cmd.summary;
        }
        _ => {}
    };

    let results = match action_context {
        Some(ctx) => pipeline.run_with_context(action_graph, ctx).await?,
        None => pipeline.run(action_graph).await?,
    };

    Ok(results)
}

pub async fn create_project_graph_context(
    session: &CliSession,
) -> miette::Result<ProjectGraphBuilderContext> {
    let context = ProjectGraphBuilderContext {
        extend_project: Emitter::<ExtendProjectEvent>::new(),
        extend_project_graph: Emitter::<ExtendProjectGraphEvent>::new(),
        inherited_tasks: &session.tasks_config,
        toolchain_config: &session.toolchain_config,
        vcs: Some(session.get_vcs_adapter()?),
        working_dir: &session.working_dir,
        workspace_config: &session.workspace_config,
        workspace_root: &session.workspace_root,
    };

    context
        .extend_project
        .on(
            |event: Arc<ExtendProjectEvent>, data: Arc<RwLock<ExtendProjectData>>| async move {
                let mut data = data.write().await;

                for platform in PlatformManager::read().list() {
                    data.dependencies
                        .extend(platform.load_project_implicit_dependencies(
                            &event.project_id,
                            event.project_source.as_str(),
                        )?);

                    data.tasks.extend(
                        platform
                            .load_project_tasks(&event.project_id, event.project_source.as_str())?,
                    );
                }

                Ok(EventState::Continue)
            },
        )
        .await;

    context
        .extend_project_graph
        .on(|event: Arc<ExtendProjectGraphEvent>, data: Arc<RwLock<ExtendProjectGraphData>>| async move {
            let mut data = data.write().await;


            for platform in PlatformManager::write().list_mut() {
                platform.load_project_graph_aliases(&event.sources, &mut data.aliases)?;
            }

            Ok(EventState::Continue)
        })
        .await;

    Ok(context)
}
