use std::collections::HashMap;
use std::env::var;
use std::{mem, thread};
use std::ops::Deref;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::time::Duration;
use threadPool::ThreadPool;

fn main() {
    let dataIn:Vec<i32> = (1..9).collect();

    let mut resultMap: HashMap<i32, i32> = HashMap::new();

    let (resultSender, resultReceiver) = mpsc::channel();

    let pool = ThreadPool::new(4);
    for data in dataIn {

        let senderClone = resultSender.clone();
        pool.execute(move || {
            process(data,senderClone);
        });
    }

    println!("Shutting down.");

    // drop the original sender, else the channel will remain open, causing the receiver to infinitely wait
    mem::drop(resultSender);

    for received in resultReceiver {
        for (key, value) in received {
            println!("received key: {} value: {}", key, value);
            *resultMap.entry(key).or_default() = value;
        }

    }

    for val in resultMap.values() {
        println!("resultMap: {}", val);
    }
}

fn process(data: i32,resultSender: Sender<HashMap<i32, i32>>) {
    println!("Processing data: {}", data);

    let resultData = data * 2;
    thread::sleep(Duration::from_secs(1));
    let mut map: HashMap<i32, i32> = HashMap::new();
    *map.entry(data).or_default()=resultData;

    resultSender.send(map).unwrap();

}
