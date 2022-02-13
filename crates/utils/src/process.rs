use crate::fs::get_home_dir;
use moon_error::{map_io_to_process_error, MoonError};
use moon_logger::{color, logging_enabled, trace};
use std::env;
use std::ffi::OsStr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::task;

pub use std::process::{Output, Stdio};

fn log_command_info(command: &Command) {
    // Avoid all this overhead if we're not logging
    if !logging_enabled() {
        return;
    }

    let cmd = command.as_std();
    let bin_name = cmd.get_program().to_str().unwrap_or("<unknown>");
    let args_list = cmd
        .get_args()
        .into_iter()
        .map(|a| a.to_str().unwrap())
        .collect::<Vec<_>>();
    let command_line = format!("{} {}", bin_name, args_list.join(" "))
        .replace(get_home_dir().unwrap_or_default().to_str().unwrap(), "~");

    if let Some(cwd) = cmd.get_current_dir() {
        trace!(
            target: "moon:utils",
            "Running command {} (in {})",
            color::shell(&command_line),
            color::file_path(cwd),
        );
    } else {
        trace!(
            target: "moon:utils",
            "Running command {} ",
            color::shell(&command_line),
        );
    }
}

pub fn create_command<S: AsRef<OsStr>>(bin: S) -> Command {
    Command::new(bin)
}

pub async fn exec_command(command: &mut Command) -> Result<Output, MoonError> {
    log_command_info(command);

    let output = command.output();

    Ok(output.await.map_err(|e| {
        map_io_to_process_error(e, command.as_std().get_program().to_str().unwrap())
    })?)
}

pub async fn exec_command_capture_stdout(command: &mut Command) -> Result<String, MoonError> {
    let output = exec_command(command).await?;

    Ok(output_to_string(&output.stdout))
}

pub async fn spawn_command(command: &mut Command) -> Result<Output, MoonError> {
    log_command_info(command);

    let mut child = command
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        // Inherit ANSI colors since they're stripped from pipes
        .env("FORCE_COLOR", env::var("FORCE_COLOR").unwrap_or_default())
        .env("TERM", env::var("TERM").unwrap_or_default())
        .spawn()
        .unwrap();

    // https://stackoverflow.com/a/49063262
    let err = BufReader::new(child.stderr.take().unwrap());
    let out = BufReader::new(child.stdout.take().unwrap());

    // Spawn additional threads for logging the buffer
    let mut handles = vec![];
    let stderr = Arc::new(RwLock::new(vec![]));
    let stderr_clone = Arc::clone(&stderr);
    let stdout = Arc::new(RwLock::new(vec![]));
    let stdout_clone = Arc::clone(&stdout);

    handles.push(task::spawn(async move {
        let mut lines = err.lines();
        let mut stderr_write = stderr_clone.write().await;

        while let Some(line) = lines.next_line().await.unwrap() {
            eprintln!("{}", line);
            stderr_write.push(line);
        }
    }));

    handles.push(task::spawn(async move {
        let mut lines = out.lines();
        let mut stdout_write = stdout_clone.write().await;

        while let Some(line) = lines.next_line().await.unwrap() {
            println!("{}", line);
            stdout_write.push(line);
        }
    }));

    for handle in handles {
        handle.await.unwrap();
    }

    // Attempt to capture the child output
    let mut output = child.wait_with_output().await.map_err(|e| {
        map_io_to_process_error(e, command.as_std().get_program().to_str().unwrap())
    })?;

    if output.stderr.is_empty() {
        output.stderr = stderr.read().await.join("").into_bytes();
    }

    if output.stdout.is_empty() {
        output.stdout = stdout.read().await.join("").into_bytes();
    }

    Ok(output)
}

pub fn output_to_string(data: &[u8]) -> String {
    String::from_utf8(data.to_vec()).unwrap_or_default()
}
