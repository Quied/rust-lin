fn main() {
        let a = 42;
        let d = 42;

    let arr:[i32;4] = [24,42,24,42];
    println!("Array size {}", arr.len());
    let v = &["main", "this", "pointer"];

    let mut iter = arr.iter();
   
    
    let next = iter.next();
    for text in v{
    println!("This {}.", text);
    
    }
   
   
}
