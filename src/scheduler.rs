use std::{
    time::Duration,
    thread::sleep,
    sync::{Arc, Mutex},
};

use job_scheduler::{
    JobScheduler,
    Job,
};

use teloxide::prelude2::*;

use crate::catmate::CatMateMessage;

static SLEEP_DURATION_SECONDS: u64 = 1;

pub fn init(
    bot: &'static AutoSend<Bot>,
    catmate_message: Arc<Mutex<CatMateMessage>>,
) {
    println!("Initializing scheduler...");

    tokio::spawn(async move {
        let mut cron = JobScheduler::new();

        let job_catmate_message = catmate_message.clone();

        cron.add(Job::new("1/10 * * * * *".parse().unwrap(), move || {
            let mut locked_job_catmate_message = job_catmate_message.lock().unwrap();
            *locked_job_catmate_message = locked_job_catmate_message.get_next();
        }));

        loop {
            let mut locked_catmate_message = *catmate_message.lock().unwrap();

            tokio::spawn(async move {
                if locked_catmate_message.not_sent_yet() {
                    locked_catmate_message.send(bot).await;
                    locked_catmate_message.mark_as_sent();
                }
            });

            //panic!("Catmate done!"); // TODO

            sleep(Duration::from_millis(SLEEP_DURATION_SECONDS * 1000));

            cron.tick();
        }
    });
}
