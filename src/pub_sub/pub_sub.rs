use std::sync::mpsc;
use std::{thread,time};
use std::sync::mpsc::{Receiver, Sender};


 fn Publisher(sender :Sender<i32>){
    let  publish =  thread::spawn(move || {
        for val in 1..10 {
             println!("Sending value .....{}",val);
             sender.send(val).unwrap();
        }
    });
}

 fn Subscriber(receiver :Receiver<i32>){
    let  subscribe =  thread::spawn(move || {
        for val in receiver {
            println!("Receiving value .....{}",val);
        }
    }).join();
}

pub fn pub_sub(){
    let (tx11, rx11) = mpsc::channel();
    Publisher(tx11);
    Subscriber(rx11);
}