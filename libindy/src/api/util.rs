extern crate libc;

use std::ffi::{CStr,CString};
use self::libc::c_char;

use utils::environment::{set_env_var,get_env_var};
use utils::environment::INDY_HOME_PATH;



// this is new func to modify static new PATH
#[no_mangle]
pub extern fn set_indy_home(path : *const c_char ){

    let indy_home_path : String = unsafe { CStr::from_ptr(path).to_str().unwrap().to_string() };
    if  !indy_home_path.is_empty() {
            set_env_var(INDY_HOME_PATH, indy_home_path);
    }

}


#[no_mangle]
pub extern fn get_indy_home() -> *mut c_char{
    let s = CString::new(get_env_var(INDY_HOME_PATH)).unwrap();
    s.into_raw()
}


#[no_mangle]
pub extern fn free_raw_string(path: *mut c_char)
{
    unsafe {
            if path.is_null() { return }
            CString::from_raw(path);
    }
}
