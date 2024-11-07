use std::time::Duration;
use std::future::Future;
use std::pin::Pin;
use trpl::{Either, Html};

pub fn message_futures() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("future"),
                String::from("message"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("Received a message ('{value}')");
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    });
}

pub fn web_scraper() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

pub fn future_spawn_join() {
    trpl::run(async {
        let fut1 = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi from first async {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        let fut2 = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi from second async {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        trpl::join(fut1, fut2).await;
    });
}

pub fn future_vec() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async move {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        //Requires that all the futures have the same output type
        trpl::join_all(futures).await;
        // Permits to create a tuple of results, such a way that output type can mismatch
        // (res1, res2, res3) = trpl::join!(fut1, fut2, fut3)
    });
}
