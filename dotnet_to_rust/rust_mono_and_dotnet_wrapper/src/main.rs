// use wrapped_mono::*;


use netcorehost::{
    hostfxr::{AssemblyDelegateLoader, ManagedFunction},
    nethost, pdcstr,
    pdcstring::PdCStr,
};

use std::{
    ffi::{CStr, CString},
    mem::{self, MaybeUninit},
    os::raw::c_char,
    str::Utf8Error,
    string::FromUtf16Error,
};


fn main(){
    println!("Hello, world!");

    // This is then the code for "dotnetcorehost something"
    let hostfxr = nethost::load_hostfxr().unwrap();
let context = hostfxr.initialize_for_runtime_config(pdcstr!("dlls/dnne-gen.runtimeconfig.json")).unwrap();
let fn_loader = context.get_delegate_loader_for_assembly(pdcstr!("dlls/ThermoFisher.CommonCore.RawFileReader.dll")).unwrap();
// let hello = fn_loader.get_function_with_default_signature(
//     pdcstr!("Test.Program, Test"),
//     pdcstr!("Hello"),
// ).unwrap();
// let result = unsafe { hello(std::ptr::null(), 0) };
// assert_eq!(result, 42);

let example_function = fn_loader
.get_function_with_unmanaged_callers_only::<fn(f: unsafe extern "system" fn(*mut c_char) -> *mut c_char)>(
    pdcstr!("ThermoFisher.CommonCore.RawFileReader, RawFileReaderAdapter"),
    pdcstr!("FileFactory"),
)
.unwrap();





        // This is then the code from mono
//     // Initialize the runtime with default version(`None`), and root domian named "main_domain"
//     let domain = jit::init("main_domain",Some("2.0")); 
//     // Load assembly "SomeAssembly.dll" 
//     // let assembly = domain.assembly_open("dlls/ThermoFisher.CommonCore.Data.dll").expect("Could not load assembly!"); 
//     let assembly = domain.assembly_open("dlls/ThermoFisher.CommonCore.RawFileReader.dll").expect("Could not load assembly!"); 
//     // Get the image, the part of assembly containing executable code(classes,methods, etc.)
//     let image = assembly.get_image(); 
//     // // Get class named SomeClass in SomeNamespace
//     let class = Class::from_name(&image,"ThermoFisher.CommonCore.RawFileReader","RawFileReaderAdapter").expect("Could not find SomeClass!"); 
//     println!("{:?}", class.get_fields().pop().unwrap().get_name());   
    println!("Hello, world!");

}