
extern {
// fn setDomainName(a: String);
// fn createObject();
// fn setRawFile(a: String);
// fn openFile();
// fn get_values(a: i32,b: String) -> i32;
fn get_info();
}

fn main() {
    // println!("cargo:rustc-link-lib=dylib=/usr/lib");
    // println!("cargo:rustc-link-lib=dylib=/usr/include/mono-2.0");

    println!("Hello, world!");



    unsafe {
        get_info();
        // setDomainName("rawrrRcpp".to_string());
    }
}
