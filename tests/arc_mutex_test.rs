use tokio;
use tokio::time::{sleep, Duration};
use std::sync::{Mutex, Arc};


#[tokio::test]
async fn arc_mutex_test() {
    let start = Arc::new(Mutex::new(0));
    let end = 120;

    let start_clone = Arc::clone(&start);
    let task_1  = tokio::spawn(async move {
        loop {
            {
                let mut num = start_clone.lock().unwrap();
                *num += 1;
                let num_copy = *num;
                drop(num);
                println!("Current value: {}", num_copy);
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    let start_clone = Arc::clone(&start);
    let task_2 = tokio::spawn(async move {
        loop {
            {
                let mut num = start_clone.lock().unwrap();
                *num += 10;
                let num_copy = *num;
                drop(num);
                println!("Current value: {}", num_copy);
            }
            sleep(Duration::from_secs(7)).await;
        }
    });

    let start_clone = Arc::clone(&start);
    loop {
        {
            let num = start_clone.lock().unwrap();
            if *num >= end {
                println!("Finished");
                task_2.abort();
                task_1.abort();
                break;
            }
        }
        sleep(Duration::from_millis(500)).await;
    }
}
