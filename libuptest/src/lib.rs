#![feature(error_in_core)]

mod chains;
//mod edgeware;
pub mod codec;
mod connect;
mod error;
pub mod jsonrpseeclient;
pub mod types;
pub mod ws_mod;

#[cfg(feature = "metadatadecode")]
pub mod decode_extrinsic;

/*
pub struct PalletTest {
    pallet_name: String,
    pallet_method: String,

}

impl PalletTest {
    fn new() -> PalletTest {
        PalletTest {
            pallet_name: "test".to_string(),
            pallet_method: "test".to_string()
        }
    }
}

pub fn test() -> PalletTest  {
    PalletTest::new()
}

*/
