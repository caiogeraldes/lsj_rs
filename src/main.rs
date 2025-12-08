pub(crate) mod error;
pub(crate) mod parser;
use crate::error::Error;
use crate::parser::Asset;

use include_assets::EnumArchive;

fn main() -> Result<(), Error> {
    let archive = EnumArchive::<Asset>::load();
    let a_asset = &archive[Asset::A];
    match str::from_utf8(a_asset) {
        Ok(a) => {
            // let b: Part = serde_xml_rs::from_str(a).unwrap();
            dbg!(a);
            Ok(())
        }
        Err(e) => {
            let err_string = e.to_string().clone();
            Err(Error::ParsingError(err_string))
        }
    }
}
