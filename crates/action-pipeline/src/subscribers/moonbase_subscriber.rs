use crate::event_emitter::{Event, Subscriber};
use async_trait::async_trait;
use moon_action::ActionPipelineStatus;
use moon_api::Moonbase;
use std::sync::Arc;
use tracing::debug;

pub struct MoonbaseSubscriber {
    session: Arc<Moonbase>,
}

impl MoonbaseSubscriber {
    pub fn new(session: Arc<Moonbase>) -> Self {
        MoonbaseSubscriber { session }
    }
}

#[async_trait]
impl Subscriber for MoonbaseSubscriber {
    async fn on_emit<'data>(&mut self, event: &Event<'data>) -> miette::Result<()> {
        if matches!(
            event,
            Event::PipelineCompleted {
                status: ActionPipelineStatus::Completed,
                ..
            }
        ) {
            debug!("Waiting for in-flight moonbase requests to finish");

            self.session.wait_for_requests().await;
        }

        Ok(())
    }
}
