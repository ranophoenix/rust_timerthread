use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::{RwLock, Arc};
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

    //Written by the receiver thread
    let counter = Arc::new(RwLock::new(0));
    //Read by the timer thread
    let counter_ro = counter.clone();

    //Same as above
    let err_counter = Arc::new(RwLock::new(0));
    let err_counter_ro = err_counter.clone();

    //Thread to update counters.
    let _receiver_handle = thread::spawn(move || {
        while let Ok(task_result) = rx.recv() {
            match task_result {
                Ok(_) => *counter.write().unwrap() += 1,
                Err(_) => *err_counter.write().unwrap() += 1
            }
        }
    });

    //This thread sleeps once a second to print counters.
    let timer_handle = thread::spawn(move || {
        let mut secs = 1;
        loop {
            thread::sleep(Duration::from_secs(1));
            let success = *counter_ro.read().unwrap();
            let errors =  *err_counter_ro.read().unwrap();
            println!("{} ok ops/sec | OK: {} Err: {}", success / secs, success, errors);
            secs += 1;
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
