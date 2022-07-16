use lazy_static::lazy_static;
use moon_lang_node::package::PackageJson;
use moon_logger::{color, debug, warn};
use moon_task::{Target, TargetID, Task, TaskError, TaskID, TaskType};
use moon_utils::{process, regex};
use std::collections::{BTreeMap, HashMap};

// requirements:
//  - use "npm run", "yarn run", and "pnpm run" instead of the shorthand
//  - "post" hooks dont work the same

const TARGET: &str = "moon:node-task";

type TasksMap = BTreeMap<TaskID, Task>;
type ScriptsMap = HashMap<String, String>;

lazy_static! {
    pub static ref WIN_DRIVE: regex::Regex = regex::create_regex(r#"^[A-Z]:"#).unwrap();

    pub static ref ARG_ENV_VAR: regex::Regex = regex::create_regex(r#"^[A-Z0-9_]+="#).unwrap();

    pub static ref ARG_OUTPUT_FLAG: regex::Regex =
        regex::create_regex(r#"^(-o|--(out|output|dist)(-{0,1}(?i:dir|file))?)$"#).unwrap();

    pub static ref INFO_OPTIONS: regex::Regex =
        regex::create_regex(r#"--(help|version)"#)
            .unwrap();

    // This isn't exhaustive but captures very popular tools
    pub static ref DEV_COMMAND: regex::Regex =
        regex::create_regex(r#"(concurrently)|(gatsby (new|dev|develop|serve|repl))|(next (dev|start))|(parcel [^build])|(react-scripts start)|(snowpack dev)|(vite (dev|preview|serve))|(vue-cli-service serve)|(webpack (s|serve|server|w|watch|-))"#)
            .unwrap();

    pub static ref DEV_COMMAND_SOLO: regex::Regex =
            regex::create_regex(r#"^(npx |yarn dlx |pnpm dlx )?(parcel|vite|webpack)$"#)
                .unwrap();

    pub static ref SYSTEM_COMMAND: regex::Regex =
                regex::create_regex(r#"^(bash|cp|echo|find|git|make|mkdir|rm|rsync|svn)$"#)
                    .unwrap();

    // Special package manager handling
    pub static ref PM_RUN_COMMAND: regex::Regex = regex::create_regex(r#"(npm|pnpm|yarn) run"#)
        .unwrap();
    pub static ref PM_LIFE_CYCLES: regex::Regex = regex::create_regex(r#"^(prepare|prepublish|prepublishOnly|publish|prepack|pack|postpack|preinstall|install|postinstall|preversion|version|postversion|dependencies)$"#)
        .unwrap();

    // These patterns are currently not allowed
    pub static ref INVALID_CD: regex::Regex = regex::create_regex(r#"(^|\b|\s)cd "#).unwrap();
    pub static ref INVALID_REDIRECT: regex::Regex = regex::create_regex(r#"\s(<|<<|>>|>)\s"#).unwrap();
    pub static ref INVALID_PIPE: regex::Regex = regex::create_regex(r#"\s\|\s"#).unwrap();
    pub static ref INVALID_OPERATOR: regex::Regex = regex::create_regex(r#"\s(\|\||;;)\s"#).unwrap();

    pub static ref TASK_ID_CHARS: regex::Regex = regex::create_regex(r#"[^a-zA-Z0-9-_]+"#).unwrap();
}

fn is_bash_script(arg: &str) -> bool {
    arg.ends_with(".sh")
}

fn is_node_script(arg: &str) -> bool {
    arg.ends_with(".js") || arg.ends_with(".cjs") || arg.ends_with(".mjs")
}

pub fn should_run_in_ci(script: &str) -> bool {
    if INFO_OPTIONS.is_match(script) {
        return true;
    }

    if script.contains("--watch")
        || DEV_COMMAND.is_match(script)
        || DEV_COMMAND_SOLO.is_match(script)
    {
        return false;
    }

    true
}

fn clean_env_var(pair: &str) -> (String, String) {
    let mut parts = pair.split('=');
    let key = parts.next().unwrap();
    let mut val = parts.next().unwrap_or_default();

    if val.ends_with(';') {
        val = &val[0..(val.len() - 1)];
    }

    (key.to_owned(), val.to_owned())
}

fn clean_output_path(target_id: &str, output: &str) -> Result<String, TaskError> {
    if output.starts_with("..") {
        Err(TaskError::NoParentOutput(
            output.to_owned(),
            target_id.to_owned(),
        ))
    } else if output.starts_with('/') || WIN_DRIVE.is_match(output) {
        Err(TaskError::NoAbsoluteOutput(
            output.to_owned(),
            target_id.to_owned(),
        ))
    } else if output.starts_with("./") || output.starts_with(".\\") {
        Ok(output[2..].to_owned())
    } else {
        Ok(output.to_owned())
    }
}

fn clean_script_name(name: &str) -> String {
    TASK_ID_CHARS.replace_all(name, "-").to_string()
}

fn detect_task_type(command: &str) -> TaskType {
    if SYSTEM_COMMAND.is_match(command) || command == "noop" {
        return TaskType::System;
    }

    TaskType::Node
}

#[track_caller]
pub fn convert_script_to_task(
    target_id: &str,
    script_name: &str,
    script: &str,
) -> Result<Task, TaskError> {
    let script_args = process::split_args(script)?;
    let mut task = Task::new(target_id);
    let mut args = vec![];

    for (index, arg) in script_args.iter().enumerate() {
        // Extract nvironment variables
        if ARG_ENV_VAR.is_match(arg) {
            let (key, val) = clean_env_var(arg);

            task.env.insert(key, val);

            continue;
        }

        // Detect possible outputs
        if ARG_OUTPUT_FLAG.is_match(arg) {
            if let Some(output) = script_args.get(index + 1) {
                task.outputs.push(clean_output_path(target_id, output)?);
            }
        }

        args.push(arg.to_owned());
    }

    if let Some(command) = args.get(0) {
        if is_bash_script(command) {
            task.command = "bash".to_owned();
        } else if is_node_script(command) {
            task.command = "node".to_owned();
        } else {
            task.command = args.remove(0);
        }
    } else {
        task.command = "noop".to_owned();
    }

    task.args = args;
    task.type_of = detect_task_type(&task.command);
    task.options.run_in_ci = should_run_in_ci(script);

    debug!(
        target: &task.log_target,
        "Creating task {} with command {} (from package.json script {})",
        color::target(target_id),
        color::shell(&task.command),
        color::symbol(script_name)
    );

    Ok(task)
}

pub fn create_tasks_from_scripts(
    project_id: &str,
    package_json: &PackageJson,
) -> Result<TasksMap, TaskError> {
    let mut parser = ScriptParser::new(project_id);

    parser.parse(package_json)?;

    Ok(parser.tasks)
}

struct ScriptParser<'a> {
    /// Life cycle events like "prepublishOnly".
    life_cycles: ScriptsMap,

    /// Script names -> task IDs.
    names_to_ids: HashMap<String, String>,

    /// Scripts that started with "post".
    post: ScriptsMap,

    /// Scripts that started with "pre".
    pre: ScriptsMap,

    /// The project being parsed.
    project_id: &'a str,

    /// Scripts that still need to be parsed.
    scripts: ScriptsMap,

    /// Tasks that have been parsed and converted from scripts.
    tasks: TasksMap,
}

impl<'a> ScriptParser<'a> {
    pub fn new(project_id: &'a str) -> Self {
        ScriptParser {
            life_cycles: HashMap::new(),
            names_to_ids: HashMap::new(),
            post: HashMap::new(),
            pre: HashMap::new(),
            project_id,
            scripts: HashMap::new(),
            tasks: BTreeMap::new(),
        }
    }

    pub fn parse(&mut self, package_json: &PackageJson) -> Result<(), TaskError> {
        let scripts = match &package_json.scripts {
            Some(s) => s.clone(),
            None => {
                return Ok(());
            }
        };
        let mut standalone_scripts = HashMap::new();

        // First pass:
        //  - Remove unsupported scripts
        //  - Extract hooks and life cycles
        //  - Convert stand-alone scripts
        //  - Retain && operators
        for (name, script) in &scripts {
            if PM_LIFE_CYCLES.is_match(name) {
                self.life_cycles.insert(name.clone(), script.clone());
                continue;
            }

            if name.starts_with("pre") {
                self.pre
                    .insert(name.strip_prefix("pre").unwrap().to_owned(), script.clone());
                continue;
            }

            if name.starts_with("post") {
                self.post.insert(
                    name.strip_prefix("post").unwrap().to_owned(),
                    script.clone(),
                );
                continue;
            }

            // Do not allow "cd ..."
            if INVALID_CD.is_match(script) {
                warn!(
                    target: TARGET,
                    "Changing directories (cd ...) is not supported by moon, skipping script \"{}\" for project \"{}\".",
                    name,
                    self.project_id,
                );

                continue;
            }

            // Rust commands do not support redirects natively
            if INVALID_REDIRECT.is_match(script) {
                warn!(
                    target: TARGET,
                    "Redirects (<, >, etc) are not supported by moon, skipping script \"{}\" for project \"{}\".",
                    name,
                    self.project_id,
                );

                continue;
            }

            // Rust commands do not support pipes natively
            if INVALID_PIPE.is_match(script) {
                warn!(
                    target: TARGET,
                    "Pipes (|) are not supported by moon, skipping script \"{}\" for project \"{}\". As an alternative, create a executable that does the piping: https://moonrepo.dev/docs/faq#how-to-pipe-tasks",
                    name,
                    self.project_id,
                );

                continue;
            }

            // Rust commands do not support operators natively
            if INVALID_OPERATOR.is_match(script) {
                warn!(
                    target: TARGET,
                    "OR operator (||) is not supported by moon, skipping script \"{}\" for project \"{}\".",
                    name,
                    self.project_id,
                );

                continue;
            }

            // Defer "npm run", "yarn run", and any "&&" usage, etc till the next pass
            if PM_RUN_COMMAND.is_match(script) || script.contains("&&") {
                self.scripts.insert(name.clone(), script.clone());
                continue;
            }

            // Stand-alone script? Hopefully...
            standalone_scripts.insert(name.clone(), script.clone());
        }

        // Second pass:
        //  - Parse stand alone and basic scripts without complexity
        //  - These are typically the base of other scripts
        //  - Take pre and post into account
        for (name, script) in &standalone_scripts {
            self.create_task(name, script)?;
        }

        Ok(())
    }

    pub fn parse_script(&self, script: &str) -> Vec<String> {
        let mut commands = vec![];

        for command in script.split("&&") {
            commands.push(command.trim().to_owned());
        }

        commands
    }

    pub fn create_task<T: AsRef<str>>(
        &mut self,
        name: T,
        value: &str,
    ) -> Result<TaskID, TaskError> {
        let name = name.as_ref();
        let task_id = clean_script_name(name);
        let target_id = Target::format(self.project_id, &task_id)?;

        self.names_to_ids.insert(name.to_owned(), task_id.clone());

        let mut task = convert_script_to_task(&target_id, name, value)?;

        // Convert pre hooks as `deps`
        if self.pre.contains_key(name) {
            task.deps = self.apply_pre_hooks(name)?;
        }

        self.tasks.insert(task_id.clone(), task);

        // Use this target as a `deps` for post hooks
        if self.post.contains_key(name) {
            self.apply_post_hooks(name, &task_id)?;
        }

        Ok(task_id)
    }

    pub fn apply_pre_hooks(&mut self, script_name: &str) -> Result<Vec<TargetID>, TaskError> {
        let mut deps = vec![];
        let script = self.pre.remove(script_name).unwrap();
        let commands = self.parse_script(&script);

        for (index, command) in commands.iter().enumerate() {
            let task_id = self.create_task(format!("{}-pre{}", script_name, index + 1), command)?;

            deps.push(format!("~:{}", task_id));
        }

        Ok(deps)
    }

    pub fn apply_post_hooks(
        &mut self,
        script_name: &str,
        dep_task_id: &str,
    ) -> Result<(), TaskError> {
        let script = self.post.remove(script_name).unwrap();
        let commands = self.parse_script(&script);

        for (index, command) in commands.iter().enumerate() {
            let task_id =
                self.create_task(format!("{}-post{}", script_name, index + 1), command)?;

            if let Some(task) = self.tasks.get_mut(&task_id) {
                task.deps.push(format!("~:{}", dep_task_id));
            }
        }

        Ok(())
    }
}
