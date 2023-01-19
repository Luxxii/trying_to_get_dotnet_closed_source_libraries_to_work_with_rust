// use netcorehost::{nethost, pdcstr};



fn main() {

    extern { fn FancyName(a: i32) -> i32; }

        unsafe {
            let x = FancyName(3);
        }
    
    println!("Hello, world!");



    



    // // Lets call a thremo function!
    // let some_thermo_function = fn_loader
    // .get_function_with_unmanaged_callers_only::<fn(f: unsafe extern "system" fn(*const u16, i32) -> *mut c_char)>(
    //     pdcstr!("ThermoFisher.CommonCore.RawFileReader.DataModel.WrappedRunHeader, ThermoFisher"),
    //     pdcstr!("SpectraCount"),
    // )
    // .unwrap();

    // let some_thermo_function = fn_loader
    // .get_function(type_name, method_name, delegate_type_name)


    // P:ThermoFisher.CommonCore.RawFileReader.DataModel.WrappedRunHeader.SpectraCount



    // Peaks get neares
    // match get_nearest_peak([1.0,2.0,3.0,4.0,5.0].to_vec(), [10.0,20.0,30.0,40.0,50.0].to_vec(), 3.5, 1.0) {
    //     Some((x, y)) => println!("Nearest peak: {} {}", x, y),
    //     None => println!("No peak near..."),
    // }
    
    
}   

// Just random stuff to get with RUST going
// fn get_nearest_peak(mz: Vec<f64>, intens: Vec<f64>, mz_target: f64, mz_tolerance: f64) -> Option<(f64, f64)>{
//     // we assume the vec of mz is in same order as the intens-values (mz1, i1), (mz2, i2), etc ...
//     assert_eq!(mz.len(), intens.len(), "Vectors are not of same length");
    
//     let mut min_mz: f64 = f64::MAX;
//     let mut min_intens: f64 = f64::MAX;
//     let mut min_diff: f64 = f64::MAX;

//     // iterate both at once (similar to pythons zip)
//     for (imz, i) in mz.iter().zip(intens.iter()) {
//         let diff = (imz - mz_target).abs();
//         if diff <  min_diff {
//             min_mz = *imz;
//             min_intens = *i;
//             min_diff = diff;
//         }
//     }

//     // Either return result or None
//     if (min_mz - mz_target).abs() < mz_tolerance{
//         Some((min_mz, min_intens))
//     } else {
//         None
//     }

// }

