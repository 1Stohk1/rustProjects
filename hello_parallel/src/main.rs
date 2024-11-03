mod hello_async;
mod hello_thread;

use crate::hello_async::async_channel;
use crate::hello_async::future_join;
use crate::hello_async::multi_future;
use crate::hello_async::webScraper;
use crate::hello_async::cst_await;

use crate::hello_thread::ctrl_thread;


fn main() {
    async_channel::message_futures();
    // future_join::future_spawn_join();
    // multi_future::run();
    // webScraper::return_title(); // Needs args
    // cst_await::_await();

    ctrl_thread::_spawn();
}