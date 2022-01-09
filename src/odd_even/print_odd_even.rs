use std::sync::mpsc;
use std::thread;

pub fn populate_odd_even(){
    let (tx11, rx11) = mpsc::channel();
    let  handles =  thread::spawn(move || {
        for val in 1..10 {
            if val%2!=0{
                println!("odd: {}",val);
            }else {
                tx11.send((val));
                thread::park();
            }
        }
        std::mem::drop(tx11);
    });
    thread::spawn(move || {
        for val in 1..10 {
            let x = rx11.recv();
            if x.is_ok() {
                println!("even :{}", x.unwrap());
            }
            handles.thread().unpark();
        }
        std::mem::drop(rx11);
    }).join().unwrap();
}