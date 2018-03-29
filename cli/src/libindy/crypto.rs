use super::{ErrorCode,IndyHandle};


use libc::c_char;
use std::ffi::CString;

use base58::FromBase58;

pub struct Crypto {}


impl Crypto {

    pub fn encrypt(their_key: &str, msg: &str) -> Result<String, ErrorCode>
    {
        let (receiver, command_handle, cb) =  super::callbacks::_closure_to_cb_ec_message58();

        let their_key = CString::new(their_key).unwrap();
        let message = CString::new(msg).unwrap();
        let as_bytes = message.as_bytes();

        let msg_data = as_bytes.as_ptr();
        let msg_len: u32  = as_bytes.len()  as u32;


        let err = unsafe {
            indy_crypto_anon_crypt(command_handle,their_key.as_ptr(),msg_data,msg_len, cb)
        };
        super::results::result_to_string(err, receiver)
    }

    pub fn decrypt(wallet_handle: IndyHandle, my_key: &str, base58msg: &str) ->Result<String, ErrorCode>
    {
        let (receiver, command_handle, cb) =  super::callbacks::_closure_to_cb_ec_message();

        let my_key = CString::new(my_key).unwrap();
        let vec = base58msg.from_base58().unwrap();


        let err = unsafe {
            indy_crypto_anon_decrypt(command_handle,wallet_handle, my_key.as_ptr(),vec.as_ptr(),vec.len() as u32, cb)
        };
        super::results::result_to_string(err, receiver)
    }

}


extern {
    #[no_mangle]
    pub fn indy_crypto_anon_crypt(command_handle: i32,
                                          their_vk: *const c_char,
                                          msg_data: *const u8,
                                          msg_len: u32,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               encrypted_msg: *const u8,
                                                               encrypted_len: u32)>) -> ErrorCode;

    pub fn indy_crypto_anon_decrypt(command_handle: i32,
                                            wallet_handle: i32,
                                            my_vk: *const c_char,
                                            encrypted_msg: *const u8,
                                            encrypted_len: u32,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 msg_data: *const u8,
                                                                 msg_len: u32)>) -> ErrorCode;
}