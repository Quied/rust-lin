fn main() {
        let a = 42;
        let d = 42;

    let arr:[i32;4] = [24,42,24,42];
    println!("Array size {}", arr.len());
    let v = &["main", "this", "pointer"];

    let mut iter = arr.iter();
   
    
    let next = iter.next();
    for text in v{ println!("This {}.", text);   }
   
   let mut some: u32 = 42;
   if some == 42 {  println!("Correct"); }

    let mut vec: Vec<i32> = Vec::new();
    
        vec.push(42);
        vec.push(43);
        vec.push(84);

        println!("{}", vec[0]);

        for user in &vec{ println!("{}", user) };
}
