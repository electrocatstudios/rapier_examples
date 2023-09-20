use std::fs::File;
use std::io::Read;
use serde::*;
use std::path::Path;
use std::vec;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockList {
    pub blocks: vec::Vec::<LocationScale>,
    pub users: vec::Vec::<UserMove>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationScale{
    #[serde(default)]
    pub name: String,
    pub location: BLVec2,
    pub scale: BLVec2,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BLVec2 {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserMove {
    pub location: UserLoc,
    pub rotation: f32,
    pub power: f32,
    pub color: UserColor
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserLoc {
    pub x: f32, 
    pub y: f32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserColor {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub fn get_level_from_file(filename: String) -> Result<BlockList, String> {
    let path = Path::new(&filename);
    if !path.exists() {
        return Err(format!("File [{}] does not exist", filename));
    }

    let mut file = match File::open(filename.to_string()) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };

    let mut buff = String::new();
    match file.read_to_string(&mut buff) {
        Ok(_) => {},
        Err(err) => return Err(err.to_string())
    }; 

    let block_list: BlockList = match serde_json::from_str(&buff) {
        Ok(lj) => lj,
        Err(err) => return Err(err.to_string())
    };
    
    Ok(block_list)
}
