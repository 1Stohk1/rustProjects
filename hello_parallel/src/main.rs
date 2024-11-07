extern crate trpl;

mod hello_async;
mod hello_thread;

use std::{thread, time::Duration};

use crate::hello_async::futures;

use crate::hello_thread::ctrl_thread;


fn main() {
    // futures::message_futures();
    // futures::future_spawn_join();
    // futures::future_vec();
    // futures::web_scraper(); // Needs args
    // cst_await::_await();
    // streams::run_stream();
    // hello_async::streams::web_socket();

    // ctrl_thread::_spawn();
    // hello_thread::streams::web_socket();

    both_worlds();
}

fn both_worlds() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}