use std::time::Duration;

use cached::proc_macro::cached;
use tokio::time::sleep;

#[cached(time=2)]
async fn fetchdata(id:i32)->String{
    println!("first call");
sleep(Duration::from_secs(2)).await;
format!("data:{}",id)
}


#[tokio::main]
 async fn main(){
let string=fetchdata(2).await;
println!("recieved:{}",string);
let string2=fetchdata(2).await;
println!("recieved:{}",string2);
sleep(Duration::from_secs(2)).await;
let string3=fetchdata(2).await;
println!("string3{}",string3)
}
