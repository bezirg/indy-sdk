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

// Diffie-Helman based encryption. Uses the key derived from composition of local private and remote public keys
pub mod encrypt_dh {
    use super::*;

    command!(CommandMetadata::build("encdh", "Encrypt using DH common secret algorithm")
                .add_required_param("mykey", "Local validation key to use")
                .add_required_param("theirkey", "Remote validation key to use")
                .add_required_param("msg", "Message text")
                .add_example("crypto enc  mykey=...fullkey...   theirkey=...fullkey...    msg=...msgtext... ")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let mykey = get_str_param("mykey", params).map_err(error_err!())?;
        let theirkey = get_str_param("theirkey", params).map_err(error_err!())?;
        let msg = get_str_param("msg", params).map_err(error_err!())?;

        let wallet_handle =
            match get_opened_wallet(ctx){
                Some((handle, _)) => handle,
                None => {
                    return Err(println_err!("No wallets opened"))
                }
            };

        trace!(r#"Crypto::encrypt_dh try: wallet {} mykey {}, theirkey {}, msg {:?}"#, wallet_handle, mykey, theirkey, msg);

        let res = Crypto::encrypt_dh(wallet_handle, mykey, theirkey, msg);

        trace!(r#"Crypto::encrypt_dh return: {:?}"#, res);

        let res = match res {
            Ok(base58msg) => Ok(println_succ!("message encrypted \n\n{}\n", base58msg)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }

}

pub mod decrypt_dh
{
    use super::*;

    command!(CommandMetadata::build("decdh", "Decrypt using DH common secret algorithm")
                .add_required_param("key", "Local validation key to use")
                .add_required_param("msg", "Cipher text")
                .add_example("crypto enc  mykey=...fullkey...   theirkey=...fullkey...    msg=...cipher... ")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let key = get_str_param("key", params).map_err(error_err!())?;
        let msg = get_str_param("msg", params).map_err(error_err!())?;


        let wallet_handle =
            match get_opened_wallet(ctx){
                Some((handle, _)) => handle,
                None => {
                    return Err(println_err!("No wallets opened"))
                }
            };


        trace!(r#"Crypto::decrypt_dh try: key {}, msg {:?}"#, key, msg);

        let res = Crypto::decrypt_dh(wallet_handle, key, msg);

        trace!(r#"Crypto::decrypt_dh return: {:?}"#, res);

        let res = match res {
            Ok((decoded_msg, their_key)) => Ok(println_succ!("message decrypted \n\n{}\nremote key used {}\n", decoded_msg, their_key)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }


}

pub mod encrypt {
    use super::*;

    command!(CommandMetadata::build("enc", "Encrypt anonymously")
                .add_required_param("key", "Validation key to encrypt with")
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
                .add_required_param("key", "The validation key to fetch the secret")
                .add_required_param("msg", "Cipher text")
                .add_example("crypto enc  wallet1 key=VsKV7grR1BUE29mG2Fm2kX msg=... ")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        //let wallet_name = get_str_param("name", params).map_err(error_err!())?;

        let key = get_str_param("key", params).map_err(error_err!())?;
        let msg = get_str_param("msg", params).map_err(error_err!())?;


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
