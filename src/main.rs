use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;
use rand::Rng;

///Simulates a task that lasts between 1 and 500 milliseconds with a probability of 4/5 success.
fn an_amazing_task() -> Result<(), &'static str>{
    let mut rng = rand::thread_rng();
    thread::sleep(Duration::from_millis(rng.gen_range(1..500)));
    if rng.gen_ratio(4, 5) {
        Ok(())
    } else {
        Err("Task error.")
    }
}

//Counts ops by second using a mpsc Channel.
fn main() {

    let (tx, rx) = channel();
    let (timer_tx, timer_rx) = channel();

    //Thread to update counters.
    let _receiver_handle = thread::spawn(move || {
        let mut counter = 0;
        let mut err_counter = 0;
        let start = Instant::now();
        let mut past_seconds = 0;
        while let Ok(task_result) = rx.recv() {
            match task_result {
                Ok(_) => counter += 1,
                Err(_) => err_counter += 1
            }
            let duration = start.elapsed();

            if duration.as_secs() > past_seconds {
                past_seconds += 1;
                let _ = timer_tx.send((counter, err_counter, duration.as_secs()));
            }
        }
    });

    //This thread sleeps once a second to print counters.
    let timer_handle = thread::spawn(move || {
        loop {
            while let Ok(counters) = timer_rx.recv() {
                let success = counters.0;
                let errors = counters.1;
                let secs = counters.2;
                    println!("{} ok ops/sec | OK: {} Err: {} Elapsed Time: {}", success / secs, success, errors, secs);
            }
        }
    });


    //Creates five threads to simulate tasks running concurrently.
    for _ in 0..5 {
        let tx_c = tx.clone();
        let _worker = thread::spawn(move || {
            loop {
                let _ = tx_c.send(an_amazing_task());
            }
        });
    }

    println!("Press CTRL+C to terminate.\n");

    let _res = timer_handle.join();
}
