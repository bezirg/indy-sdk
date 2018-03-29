use command_executor::{ Command, CommandMetadata, CommandContext,  CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;

use libindy::crypto::Crypto;


pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("crypto", "Crypto management commands"));
}

pub mod compose_key{
    use base58::FromBase58;
    use base58::ToBase58;
    use super::*;

    command!(CommandMetadata::build("comp", "compose the key from its two base58 parts")
                .add_required_param("did", "First part")
                .add_required_param("ver", "Second part")
                .add_example("crypto comp did=Th7MpTaRZVRYnPiabds81Y  ver=~7TYfekw4GUagBnBVCqPjiC")
                .finalize()
    );

    fn execute (_ctx: &CommandContext, params: &CommandParams) -> Result<(), ()>
    {
        let did58 = get_str_param("did", params).map_err(error_err!())?;
        let mut ver58 = get_str_param("ver", params).map_err(error_err!())?;

        ver58 =   if ver58.starts_with("~") { &ver58[1..] } else { ver58 } ;

        let mut did = did58.from_base58().unwrap();
        let mut ver = ver58.from_base58().unwrap();

        did.append(&mut ver); // + ver.concat()

        let full_key = did.to_base58();

        Ok(println_succ!("\n{}\n", full_key))


    }
}

pub mod encrypt {
    use super::*;

    command!(CommandMetadata::build("enc", "Encrypt anonymously")
                .add_required_param("key", "Validation key to use")
                .add_required_param("msg", "Message text")
                .add_example("crypto enc  key=VsKV7grR1BUE29mG2Fm2kX msg={ did: XXXXXXXXXXXXX ; nonce: 123456789 } ")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let key = get_str_param("key", params).map_err(error_err!())?;
        let msg = get_str_param("msg", params).map_err(error_err!())?;

        trace!(r#"Crypto::encrypt try: key {}, msg {:?}"#, key, msg);


        let res = Crypto::encrypt(key, msg);

        trace!(r#"Crypto::encrypt return: {:?}"#, res);

        let res = match res {
            Ok(base64msg) => Ok(println_succ!("message encrypted \n\n{}\n", base64msg)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }

}


pub mod decrypt{

    use super::*;

    command!(CommandMetadata::build("dec", "Decrypt anonymously")
                //.add_main_param("name", "The name of the wallet containing key")
                .add_required_param("key", "The validation key corresponding to decryption key")
                .add_required_param("msg", "Message text")
                .add_example("crypto enc  wallet1 key=VsKV7grR1BUE29mG2Fm2kX msg=... ")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        //let wallet_name = get_str_param("name", params).map_err(error_err!())?;

        let key = get_str_param("key", params).map_err(error_err!())?;
        let msg = get_str_param("msg", params).map_err(error_err!())?;

//        let wallet_handle =
//            match Wallet::open_wallet(wallet_name, None, None){
//                Ok(handle) => { handle }
//                Err(err) => {
//                    match err {
//                        ErrorCode::CommonInvalidStructure => Err(println_err!("Invalid wallet config")),
//                        ErrorCode::WalletAlreadyOpenedError => Err(println_err!("Wallet \"{}\" already opened", wallet_name)),
//                        ErrorCode::WalletAccessFailed => Err(println_err!("Cannot open encrypted wallet \"{}\"", wallet_name)),
//                        ErrorCode::CommonIOError => Err(println_err!("Wallet \"{}\" not found or unavailable", wallet_name)),
//                        err => Err(println_err!("Indy SDK error occurred {:?}", err)),
//                    }
//                    return err
//                }
//        };

        let wallet_handle =
            match get_opened_wallet(ctx){
               Some((handle, _)) => handle,
               None => {
                   return Err(println_err!("No wallets opened"))
               }
            };


        trace!(r#"Crypto::decrypt try: key {}, msg {:?}"#, key, msg);

        let res = Crypto::decrypt(wallet_handle, key, msg);

        trace!(r#"Crypto::decrypt return: {:?}"#, res);

        let res = match res {
            Ok(decoded_msg) => Ok(println_succ!("message decrypted \n\n{}\n", decoded_msg)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }

}
