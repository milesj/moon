use crate::action::ActionStatus;
use crate::operation_meta::*;
use moon_time::chrono::NaiveDateTime;
use moon_time::now_timestamp;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::mem;
use std::process::Output;
use std::time::{Duration, Instant};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub duration: Option<Duration>,

    pub finished_at: Option<NaiveDateTime>,

    pub meta: OperationMeta,

    pub started_at: NaiveDateTime,

    #[serde(skip)]
    pub start_time: Option<Instant>,

    pub status: ActionStatus,
}

impl Operation {
    pub fn new(meta: OperationMeta) -> Self {
        Operation {
            duration: None,
            finished_at: None,
            meta,
            started_at: now_timestamp(),
            start_time: Some(Instant::now()),
            status: ActionStatus::Running,
        }
    }

    pub fn new_finished(meta: OperationMeta, status: ActionStatus) -> Self {
        let time = now_timestamp();

        Operation {
            duration: None,
            finished_at: Some(time),
            meta,
            started_at: time,
            start_time: None,
            status,
        }
    }

    pub fn get_output(&self) -> Option<&OperationMetaOutput> {
        match &self.meta {
            OperationMeta::OutputHydration(output) | OperationMeta::TaskExecution(output) => {
                Some(output)
            }
            _ => None,
        }
    }

    pub fn get_output_mut(&mut self) -> Option<&mut OperationMetaOutput> {
        match &mut self.meta {
            OperationMeta::OutputHydration(output) | OperationMeta::TaskExecution(output) => {
                Some(output)
            }
            _ => None,
        }
    }

    pub fn finish(&mut self, status: ActionStatus) {
        self.finished_at = Some(now_timestamp());
        self.status = status;

        if let Some(start) = &self.start_time {
            self.duration = Some(start.elapsed());
        }
    }

    pub fn finish_from_output(&mut self, process_output: &mut Output) {
        if let Some(output) = self.get_output_mut() {
            output.exit_code = process_output.status.code();

            output.set_stderr(
                String::from_utf8(mem::take(&mut process_output.stderr)).unwrap_or_default(),
            );

            output.set_stdout(
                String::from_utf8(mem::take(&mut process_output.stdout)).unwrap_or_default(),
            );
        }

        self.finish(if process_output.status.success() {
            ActionStatus::Passed
        } else {
            ActionStatus::Failed
        });
    }

    pub fn has_failed(&self) -> bool {
        matches!(
            &self.status,
            ActionStatus::Failed | ActionStatus::FailedAndAbort
        )
    }

    pub fn has_passed(&self) -> bool {
        matches!(
            &self.status,
            ActionStatus::Cached | ActionStatus::CachedFromRemote | ActionStatus::Passed
        )
    }

    pub fn has_output(&self) -> bool {
        self.get_output().is_some_and(|output| {
            output.stderr.as_ref().is_some_and(|err| !err.is_empty())
                || output.stdout.as_ref().is_some_and(|out| !out.is_empty())
        })
    }

    pub fn is_cached(&self) -> bool {
        matches!(
            &self.status,
            ActionStatus::Cached | ActionStatus::CachedFromRemote
        )
    }

    pub fn track<T, F>(self, func: F) -> miette::Result<Self>
    where
        F: FnOnce() -> miette::Result<T>,
    {
        self.handle_track(func(), |_| true)
    }

    pub fn track_with_check<T, F, C>(self, func: F, checker: C) -> miette::Result<Self>
    where
        F: FnOnce() -> miette::Result<T>,
        C: FnOnce(T) -> bool,
    {
        self.handle_track(func(), checker)
    }

    pub async fn track_async<T, F, Fut>(self, func: F) -> miette::Result<Self>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = miette::Result<T>>,
    {
        self.handle_track(func().await, |_| true)
    }

    pub async fn track_async_with_check<T, F, Fut, C>(
        self,
        func: F,
        checker: C,
    ) -> miette::Result<Self>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = miette::Result<T>>,
        C: FnOnce(T) -> bool,
    {
        self.handle_track(func().await, checker)
    }

    fn handle_track<T>(
        mut self,
        result: miette::Result<T>,
        checker: impl FnOnce(T) -> bool,
    ) -> miette::Result<Self> {
        match result {
            Ok(value) => {
                self.finish(if checker(value) {
                    ActionStatus::Passed
                } else {
                    ActionStatus::Skipped
                });

                Ok(self)
            }
            Err(error) => {
                self.finish(ActionStatus::Failed);

                Err(error)
            }
        }
    }
}
