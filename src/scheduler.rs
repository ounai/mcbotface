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

pub fn init(
    bot: &'static AutoSend<Bot>,
    catmate_message: Arc<Mutex<CatMateMessage>>,
) {
    println!("Initializing scheduler...");

    tokio::spawn(async move {
        let mut cron = JobScheduler::new();

        let job_catmate_message = catmate_message.clone();

        cron.add(Job::new("1/10 * * * * *".parse().unwrap(), move || {
            *job_catmate_message.lock().unwrap() = job_catmate_message.lock().unwrap().get_next();
        }));

        loop {
            cron.tick();

            let mut locked_catmate_message = *catmate_message.lock().unwrap();

            if locked_catmate_message.not_sent_yet() {
                locked_catmate_message.mark_as_sent();

                tokio::spawn(async move {
                    locked_catmate_message.send(bot).await;
                });

                panic!("Catmate done!"); // TODO
            }

            sleep(Duration::from_millis(500));
        }
    });
}
