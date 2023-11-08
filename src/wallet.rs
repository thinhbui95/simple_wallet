use std::fs::File;
use std::str::FromStr;
use std::fs;
use rand::{Rng, thread_rng};
use std::io::Write;
use ethereum_private_key_to_address::PrivateKey;
use hex;

pub struct wallet{}
impl wallet  {
    pub fn create_wallet() ->String{

        let mut rng = thread_rng();
        let sk_bytes: [u8; 32] = rng.gen();
        let private_key_bytes = PrivateKey::from(&sk_bytes);
        let private_key_hex = hex::encode(sk_bytes);
        let address = private_key_bytes.address();
        let output = File::create(address.clone());
        let _ = output.expect("Fail").write(&private_key_hex.as_bytes());
        return address;
    }

    pub fn import_wallet(sercret : String) {

        let private_key = PrivateKey::from_str(&sercret).unwrap();
        let address = private_key.address();

        let  output = File::create(address);
        let _ = output.expect("Fail").write(sercret.as_bytes());
    }

    pub fn load_key(path:String) ->String{
        let  data_file: Result<String, std::io::Error> = fs::read_to_string(path);
         match data_file {
            Ok(file) => return file,
            Err(error) => panic!("Problem opening the data file: {:?}", error),
        };
    }

}


