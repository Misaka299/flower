fn main() {




    println!("{}", 2 | 4);
    println!("{}",2 & 6);

    let t = 8;

    println!("{}",1 == t || t > 1 && (1 & t) == 1);
    println!("{}",2 == t || t > 2 && (2 & t) == 2);
    println!("{}",4 == t || t > 4 && (4 & t) == 4);
    println!("{}",8 == t || t > 8 && (8 & t) == 8);



}