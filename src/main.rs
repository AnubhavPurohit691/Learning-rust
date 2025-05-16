use std::{iter, vec};

fn main(){
    let  v1 = vec![12,34,34];
   consuming_adapter( &v1);
   iterator_adapter(&v1);  // this both change vec to iterator 
   
  let vecfromiter= iter_to_vec(&v1);
  println!("vecfrom iter{:?}",vecfromiter)
}


fn consuming_adapter(v1:& Vec<i32>){
     // consuming adapters
     
     let v1_iter=v1.iter();
     let sumsed:i32 = v1_iter.sum();
     println!("{}",sumsed);
 
     // but this take ownership so you can't use this v1_iter so u can't use it again further.
}

fn iterator_adapter(v1 :& Vec<i32>){
// iterator adapter.
let iter= v1.iter();
// let iter2=iter.map(|x|x+1);
let iter2=iter.filter(|x| *x %2 ==0); 
// here |x | is input and *x is reference of reference value
for v in iter2{
    println!("{}",v);
}
// this map iterator return another iterator which we can use to iterate and it doesn't take ownership .
}

fn iter_to_vec(v1 :& Vec<i32>)->Vec<i32>{
let itervec = v1.iter().map(|x| *x +1);
let vec_iter:Vec<i32>= itervec.collect();
return vec_iter
}