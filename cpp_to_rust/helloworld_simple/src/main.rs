
extern { fn test(a: i32) ->i32 ; }

fn main() {
    println!("Hello, world!");

    unsafe { 
        let x = test(3);
        println!("output: {}", x)
    } 
}

