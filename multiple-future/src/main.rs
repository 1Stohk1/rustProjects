use std::time::Duration;
use std::future::Future;
use std::pin::Pin;

fn main() {
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
