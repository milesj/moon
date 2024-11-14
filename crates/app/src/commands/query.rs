pub use crate::queries::hash::query_hash;
pub use crate::queries::hash_diff::query_hash_diff;
pub use crate::queries::projects::*;
pub use crate::queries::tasks::*;
pub use crate::queries::touched_files::*;
use crate::session::CliSession;
use clap::{Args, Subcommand};
use moon_affected::{AffectedTracker, DownstreamScope, UpstreamScope};
use moon_vcs::TouchedStatus;
use starbase::AppResult;
use starbase_styles::color;
use starbase_utils::json;
use std::collections::BTreeMap;
use tracing::{instrument, warn};

const HEADING_AFFECTED: &str = "Affected by";
const HEADING_FILTERS: &str = "Filters";

#[derive(Clone, Debug, Subcommand)]
pub enum QueryCommands {
    #[command(
        name = "hash",
        about = "Inspect the contents of a generated hash.",
        long_about = "Inspect the contents of a generated hash, and display all sources and inputs that were used to generate it."
    )]
    Hash(QueryHashArgs),

    #[command(
        name = "hash-diff",
        about = "Query the difference between two hashes.",
        long_about = "Query the difference between two hashes. The left differences will be printed in green, while the right in red, and equal lines in white."
    )]
    HashDiff(QueryHashDiffArgs),

    #[command(
        name = "projects",
        about = "Query for projects within the project graph.",
        long_about = "Query for projects within the project graph. All options support regex patterns."
    )]
    Projects(QueryProjectsArgs),

    #[command(
        name = "tasks",
        about = "List all available tasks, grouped by project."
    )]
    Tasks(QueryTasksArgs),

    #[command(
        name = "touched-files",
        about = "Query for touched files between revisions."
    )]
    TouchedFiles(QueryTouchedFilesArgs),
}

#[derive(Args, Clone, Debug)]
pub struct QueryHashArgs {
    #[arg(required = true, help = "Hash to inspect")]
    hash: String,

    #[arg(long, help = "Print the manifest in JSON format")]
    json: bool,
}

#[instrument(skip_all)]
pub async fn hash(session: CliSession, args: QueryHashArgs) -> AppResult {
    let console = &session.console;
    let cache_engine = session.get_cache_engine()?;
    let result = query_hash(&cache_engine, &args.hash).await?;

    if !args.json {
        console
            .out
            .write_line(format!("Hash: {}", color::hash(result.0)))?;
        console.out.write_newline()?;
    }

    console.out.write_line(result.1)?;

    Ok(None)
}

#[derive(Args, Clone, Debug)]
pub struct QueryHashDiffArgs {
    #[arg(required = true, help = "Base hash to compare against")]
    left: String,

    #[arg(required = true, help = "Other hash to compare with")]
    right: String,

    #[arg(long, help = "Print the diff in JSON format")]
    json: bool,
}

#[instrument(skip_all)]
pub async fn hash_diff(session: CliSession, args: QueryHashDiffArgs) -> AppResult {
    let console = &session.console;
    let cache_engine = session.get_cache_engine()?;
    let mut result = query_hash_diff(&cache_engine, &args.left, &args.right).await?;

    if args.json {
        for diff in diff::lines(&result.left, &result.right) {
            match diff {
                diff::Result::Left(l) => result.left_diffs.push(l.trim().to_owned()),
                diff::Result::Right(r) => result.right_diffs.push(r.trim().to_owned()),
                _ => {}
            };
        }

        console.out.write_line(json::format(&result, true)?)?;
    } else {
        console
            .out
            .write_line(format!("Left:  {}", color::hash(&result.left_hash)))?;
        console
            .out
            .write_line(format!("Right: {}", color::hash(&result.right_hash)))?;
        console.out.write_newline()?;

        let is_tty = console.out.is_terminal();

        for diff in diff::lines(&result.left, &result.right) {
            match diff {
                diff::Result::Left(l) => {
                    if is_tty {
                        console.out.write_line(color::success(l))?
                    } else {
                        console.out.write_line(format!("+{}", l))?
                    }
                }
                diff::Result::Both(l, _) => {
                    if is_tty {
                        console.out.write_line(l)?
                    } else {
                        console.out.write_line(format!(" {}", l))?
                    }
                }
                diff::Result::Right(r) => {
                    if is_tty {
                        console.out.write_line(color::failure(r))?
                    } else {
                        console.out.write_line(format!("-{}", r))?
                    }
                }
            };
        }
    }

    Ok(None)
}

#[derive(Args, Clone, Debug)]
pub struct QueryProjectsArgs {
    #[arg(help = "Filter projects using a query (takes precedence over options)")]
    query: Option<String>,

    #[arg(long, help = "Filter projects that match this alias", help_heading = HEADING_FILTERS)]
    alias: Option<String>,

    #[arg(
        long,
        help = "Filter projects that are affected based on touched files",
        help_heading = HEADING_AFFECTED,
        group = "affected-args"
    )]
    affected: bool,

    #[deprecated]
    #[arg(
        long,
        hide = true,
        help = "Include direct dependents of queried projects",
        help_heading = HEADING_AFFECTED,
        requires = "affected-args",
    )]
    dependents: bool,

    #[arg(
        long,
        default_value_t,
        help = "Include downstream dependents of queried projects",
        help_heading = HEADING_AFFECTED,
        requires = "affected-args",
    )]
    downstream: DownstreamScope,

    #[arg(long, help = "Filter projects that match this ID", help_heading = HEADING_FILTERS)]
    id: Option<String>,

    #[arg(long, help = "Print the projects in JSON format")]
    json: bool,

    #[arg(long, help = "Filter projects of this programming language", help_heading = HEADING_FILTERS)]
    language: Option<String>,

    #[arg(long, help = "Filter projects that match this source path", help_heading = HEADING_FILTERS)]
    stack: Option<String>,

    #[arg(long, help = "Filter projects of this tech stack", help_heading = HEADING_FILTERS)]
    source: Option<String>,

    #[arg(long, help = "Filter projects that have the following tags", help_heading = HEADING_FILTERS)]
    tags: Option<String>,

    #[arg(long, help = "Filter projects that have the following tasks", help_heading = HEADING_FILTERS)]
    tasks: Option<String>,

    #[arg(long = "type", help = "Filter projects of this type", help_heading = HEADING_FILTERS)]
    type_of: Option<String>,

    #[arg(
        long,
        default_value_t,
        help = "Include upstream dependencies of queried projects",
        help_heading = HEADING_AFFECTED,
        requires = "affected-args",
    )]
    upstream: UpstreamScope,
}

#[instrument(skip_all)]
pub async fn projects(session: CliSession, args: QueryProjectsArgs) -> AppResult {
    let console = &session.console;
    let workspace_graph = session.get_workspace_graph().await?;

    let mut options = QueryProjectsOptions {
        alias: args.alias,
        affected: None,
        id: args.id,
        json: args.json,
        language: args.language,
        query: args.query,
        stack: args.stack,
        source: args.source,
        tags: args.tags,
        tasks: args.tasks,
        type_of: args.type_of,
    };

    // Filter down to affected projects only
    if args.affected {
        let vcs = session.get_vcs_adapter()?;
        let touched_files = load_touched_files(&vcs).await?;
        let mut affected_tracker = AffectedTracker::new(&workspace_graph, &touched_files);

        #[allow(deprecated)]
        if args.dependents {
            if !args.json {
                warn!("The --dependents option is deprecated, use --downstream instead!");
            }

            affected_tracker.with_project_scopes(UpstreamScope::Deep, DownstreamScope::Direct);
        } else {
            affected_tracker.with_project_scopes(args.upstream, args.downstream);
        }

        affected_tracker.track_projects()?;

        options.affected = Some(affected_tracker.build());
    }

    let mut projects = query_projects(&workspace_graph, &options).await?;
    projects.sort_by(|a, d| a.id.cmp(&d.id));

    // Write to stdout directly to avoid broken pipe panics
    if args.json {
        let result = QueryProjectsResult { projects, options };

        console.out.write_line(json::format(&result, true)?)?;
    } else if !projects.is_empty() {
        console.out.write_line(
            projects
                .iter()
                .map(|project| {
                    format!(
                        "{} | {} | {} | {} | {} | {}",
                        project.id,
                        project.source,
                        project.stack,
                        project.type_of,
                        project.language,
                        project
                            .config
                            .project
                            .as_ref()
                            .map(|cfg| cfg.description.as_str())
                            .unwrap_or("...")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )?;
    }

    Ok(None)
}

#[derive(Args, Clone, Debug)]
pub struct QueryTasksArgs {
    #[arg(help = "Filter tasks using a query (takes precedence over options)")]
    query: Option<String>,

    // Affected
    #[arg(
        long,
        help = "Filter tasks that are affected based on touched files",
        help_heading = HEADING_AFFECTED,
        group = "affected-args"
    )]
    affected: bool,

    #[arg(
        long,
        default_value_t,
        help = "Include downstream dependents of queried tasks",
        help_heading = HEADING_AFFECTED,
        requires = "affected-args",
    )]
    downstream: DownstreamScope,

    #[arg(
        long,
        default_value_t,
        help = "Include upstream dependencies of queried tasks",
        help_heading = HEADING_AFFECTED,
        requires = "affected-args",
    )]
    upstream: UpstreamScope,

    // Filters
    #[arg(long, help = "Filter tasks that match this ID", help_heading = HEADING_FILTERS)]
    id: Option<String>,

    #[arg(long, help = "Print the tasks in JSON format")]
    json: bool,

    #[arg(long, help = "Filter tasks with the provided command", help_heading = HEADING_FILTERS)]
    command: Option<String>,

    #[arg(long, help = "Filter tasks that belong to a platform", help_heading = HEADING_FILTERS)]
    platform: Option<String>,

    #[arg(long, help = "Filter tasks that belong to a project", help_heading = HEADING_FILTERS)]
    project: Option<String>,

    #[arg(long, help = "Filter tasks with the provided script", help_heading = HEADING_FILTERS)]
    script: Option<String>,

    #[arg(long = "type", help = "Filter projects of this type", help_heading = HEADING_FILTERS)]
    type_of: Option<String>,
}

#[instrument(skip_all)]
pub async fn tasks(session: CliSession, args: QueryTasksArgs) -> AppResult {
    let console = &session.console;
    let workspace_graph = session.get_workspace_graph().await?;

    let mut options = QueryTasksOptions {
        id: args.id,
        json: args.json,
        command: args.command,
        query: args.query,
        platform: args.platform,
        project: args.project,
        script: args.script,
        type_of: args.type_of,
        ..QueryTasksOptions::default()
    };

    // Filter down to affected tasks only
    if args.affected {
        let vcs = session.get_vcs_adapter()?;
        let touched_files = load_touched_files(&vcs).await?;

        let mut affected_tracker = AffectedTracker::new(&workspace_graph, &touched_files);
        affected_tracker.with_task_scopes(args.upstream, args.downstream);
        affected_tracker.track_tasks()?;

        options.affected = Some(affected_tracker.build());
    }

    // Query for tasks that match the filters
    let tasks = query_tasks(&workspace_graph, &options).await?;

    // Group tasks by project
    let mut grouped_tasks = BTreeMap::default();

    for task in tasks {
        let Some(project_id) = task.target.get_project_id() else {
            continue;
        };

        grouped_tasks
            .entry(project_id.to_owned())
            .or_insert(BTreeMap::default())
            .insert(task.id.clone(), task);
    }

    // Write to stdout directly to avoid broken pipe panics
    if options.json {
        console.out.write_line(json::format(
            &QueryTasksResult {
                tasks: grouped_tasks,
                options,
            },
            true,
        )?)?;
    } else if !grouped_tasks.is_empty() {
        for (project_id, tasks) in grouped_tasks {
            if tasks.is_empty() {
                continue;
            }

            console.out.write_line(project_id.as_str())?;

            for (task_id, task) in tasks {
                console.out.write_line(format!(
                    "\t{} | {} | {} | {} | {}",
                    task_id,
                    task.command,
                    task.type_of,
                    task.platform,
                    task.description.as_deref().unwrap_or("...")
                ))?;
            }
        }
    }

    Ok(None)
}

#[derive(Args, Clone, Debug)]
pub struct QueryTouchedFilesArgs {
    #[arg(long, help = "Base branch, commit, or revision to compare against")]
    base: Option<String>,

    #[arg(
        long = "defaultBranch",
        help = "When on the default branch, compare against the previous revision"
    )]
    default_branch: bool,

    #[arg(long, help = "Current branch, commit, or revision to compare with")]
    head: Option<String>,

    #[arg(long, help = "Print the files in JSON format")]
    json: bool,

    #[arg(long, help = "Gather files from you local state instead of the remote")]
    local: bool,

    #[arg(long, help = "Filter files based on a touched status")]
    status: Vec<TouchedStatus>,
}

#[instrument(skip_all)]
pub async fn touched_files(session: CliSession, args: QueryTouchedFilesArgs) -> AppResult {
    let console = &session.console;
    let vcs = session.get_vcs_adapter()?;

    let options = QueryTouchedFilesOptions {
        base: args.base,
        default_branch: args.default_branch,
        head: args.head,
        json: args.json,
        local: args.local,
        status: args.status,
    };

    let result = query_touched_files(&vcs, &options).await?;

    // Write to stdout directly to avoid broken pipe panics
    if args.json {
        console.out.write_line(json::format(&result, true)?)?;
    } else if !result.files.is_empty() {
        let mut files = result
            .files
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        files.sort();

        console.out.write_line(files.join("\n"))?;
    }

    Ok(None)
}
