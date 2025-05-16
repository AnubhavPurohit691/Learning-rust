fn main(){
    // intolter -> it doesn't borrow it takes ownership of the values.
    let v1=vec![1,2,];
    let v1_iter=v1.into_iter();
    for val in v1_iter{
        println!("{}",val)
    }
}