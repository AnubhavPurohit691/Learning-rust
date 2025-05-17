fn main (){
    // slice
    let  str=String::from("anubhav purohit");
    let word = &str[0..8];
    println!("{}",word);
    let ans = first_word(&str);
    println!("{}",ans);
}
fn first_word(str :&String)->&str{
let mut spaceindex=0;
for i in str.chars(){
    if i == ' '{
         break;
}
spaceindex=spaceindex+1;
};
return &str[0..spaceindex];
}