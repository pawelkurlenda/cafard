use std::future::{ready, Ready};
use actix_web::web;
use background_jobs_core::{Backoff, Job, MaxRetries, BoxError};
use crate::state::CacheState;

pub const DEFAULT_QUEUE: &'static str = "default";

/*#[derive(Clone, Debug)]
pub struct MyState {
    pub app_name: String,
}*/

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MyJob {
}

/*impl MyState {
    pub fn new(app_name: &str) -> Self {
        MyState {
            app_name: app_name.to_owned(),
        }
    }
}*/

impl MyJob {
    pub fn new() -> Self {
        MyJob {
        }
    }
}

impl Job for MyJob {
    type State = web::Data<CacheState>;
    type Error = BoxError;
    type Future = Ready<Result<(), BoxError>>;

    // The name of the job. It is super important that each job has a unique name,
    // because otherwise one job will overwrite another job when they're being
    // registered.
    const NAME: &'static str = "MyJob";

    // The queue that this processor belongs to
    //
    // Workers have the option to subscribe to specific queues, so this is important to
    // determine which worker will call the processor
    //
    // Jobs can optionally override the queue they're spawned on
    const QUEUE: &'static str = DEFAULT_QUEUE;

    // The number of times background-jobs should try to retry a job before giving up
    //
    // This value defaults to MaxRetries::Count(5)
    // Jobs can optionally override this value
    const MAX_RETRIES: MaxRetries = MaxRetries::Count(1);

    // The logic to determine how often to retry this job if it fails
    //
    // This value defaults to Backoff::Exponential(2)
    // Jobs can optionally override this value
    const BACKOFF: Backoff = Backoff::Exponential(2);

    // This is important for allowing the job server to reap processes that were started but never
    // completed.
    //
    // Defaults to 5 seconds
    const HEARTBEAT_INTERVAL: u64 = 5_000;

    fn run(self, state: web::Data<CacheState>) -> Self::Future {
        //println!("state : {:?}", state);
        //println!("self : {:?}", self);

        ready(Ok(()))
    }
}