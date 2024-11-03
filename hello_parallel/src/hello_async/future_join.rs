use std::time::Duration;

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