use std::{sync::mpsc, thread::spawn};

fn main(){
    let (tx,rx)=mpsc::channel();
    for i in 0..10{
        let producer=tx.clone();
        spawn(move||{
            let mut sum:i64=0;
            for j in i*10000000..(i+1*10000000)-1{
                sum=sum+j;
            }
            producer.send(sum).unwrap()
        });
    }
    drop(tx); //after thread done its work it will stop this is required other wise consumer think it still sending tx sending the value
    let mut finalsum:i64=0;
    for val in rx{
        finalsum=finalsum+val
    }
    println!("{}",finalsum)

}