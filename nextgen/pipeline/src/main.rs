use moon_pipeline::{IsolatedStep, Job, Pipeline};
use starbase::App;
use starbase_events::EventState;
use std::time::Duration;
use tokio::time::sleep;

// fn create_batch(id: String) -> JobBatch {
//     let mut batch = JobBatch::new(id.clone());

//     for i in 1..=10 {
//         let job_id = format!("{id}{i}");

//         batch.add_job(Job::new(job_id.clone(), async move {
//             sleep(Duration::from_secs(i)).await;
//             println!("{}", job_id);
//         }));
//     }

//     batch
// }

#[derive(Debug)]
struct TestResult {}

#[tokio::main]
async fn main() {
    App::setup_diagnostics();
    App::setup_tracing();

    let mut pipeline = Pipeline::<TestResult>::new();

    pipeline
        .on_job_state_change
        .on(|e, _| async move {
            dbg!("STATE CHANGE", &e);
            Ok(EventState::Continue)
        })
        .await;

    pipeline
        .on_job_progress
        .on(|e, _| async move {
            dbg!("PROGRESS", &e);
            Ok(EventState::Continue)
        })
        .await;

    // pipeline.pipe(create_batch("a".into()));

    pipeline.add_step(IsolatedStep::new("a".into(), async {
        sleep(Duration::from_secs(1)).await;
        println!("a");
        Ok(TestResult {})
    }));

    let mut b = Job::new("b".into(), async {
        sleep(Duration::from_secs(2)).await;
        println!("b");
        Ok(TestResult {})
    });
    b.timeout = Some(1);

    pipeline.add_step(IsolatedStep::from(b));

    // pipeline.pipe(create_batch("c".into()));

    pipeline.add_step(IsolatedStep::new("c".into(), async {
        sleep(Duration::from_secs(1)).await;
        println!("c");
        Ok(TestResult {})
    }));

    pipeline.add_step(IsolatedStep::new("d".into(), async {
        sleep(Duration::from_secs(1)).await;
        println!("d");
        Ok(TestResult {})
    }));

    pipeline.add_step(IsolatedStep::new("e".into(), async {
        sleep(Duration::from_secs(1)).await;
        println!("e");
        Ok(TestResult {})
    }));

    let results = pipeline.run().await.unwrap();

    dbg!(results);
}
