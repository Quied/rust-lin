fn main(){

limit();
println! ("Main ");

}


fn limit(){

    let st = MyStruct{

    v: 1, s: Box::new( Some (MyStruct {v: 2, s: Box::new(None), })),
    };

    println!("End");
    }

    struct MyStuct{
    v: i32;
    s: Box<Option<MyStruct>>,
    }


    impl Drop for MyStruct{
        fn drop(&mut self){
            println!("Clean {}", self.v)
        }
    }



