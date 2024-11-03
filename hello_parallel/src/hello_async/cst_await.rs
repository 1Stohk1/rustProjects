use std::future::Future;
use std::time::Duration;
use trpl::Either;

pub fn _await() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished!"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Our future succeed with message '{message}'"),
            Err(duration) => println!("Failed after {} seconds...", duration.as_secs()),
        }
    });
}

async fn timeout<F: Future> (
    future: F,
     max_time: Duration
    ) -> Result<F::Output, Duration> {
    match trpl::race(future, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),        
    }
} 