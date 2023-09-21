use std::fs::File;
use std::io::Read;
use serde::*;
use std::path::Path;
use std::vec;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDescriptor {
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

pub fn get_level_from_file(filename: String) -> Result<LevelDescriptor, String> {
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

    // let level_descriptor: LevelDescriptor = 
    match serde_json::from_str(&buff) {
        Ok(level) => Ok(level),
        Err(err) => Err(err.to_string())
    }
}


#[cfg(test)]
mod tests {
    use crate::file_loader::*;

    #[test]
    fn test_err_on_missing_file(){
        match get_level_from_file("not_a_real_file_name".to_string()) {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }   


    #[test]
    fn test_err_on_badly_formatted_file(){
        match get_level_from_file("tests/fixtures/empty.json".to_string()) {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_get_empty_valid_json_file() {
        let level = match get_level_from_file("tests/fixtures/minimal.json".to_string()) {
            Ok(level) => level,
            Err(_) => panic!("Test failed to load valid minimal level")
        };
        assert!(level.blocks.len() == 0);
        assert!(level.users.len() == 0);
    }

    #[test]
    fn test_get_valid_simple_json_file() {
        let level = match get_level_from_file("tests/fixtures/simple.json".to_string()) {
            Ok(level) => level,
            Err(_) => panic!("Test failed to load valid minimal level")
        };
        assert!(level.blocks.len() == 1);
        let block = level.blocks.get(0).unwrap();
        assert!(block.location.x == 0.0);
        assert!(block.location.y == 10.0);

        assert!(block.scale.x == 20.0);
        assert!(block.scale.y == 1.0);
        
        assert!(level.users.len() == 1);
        let user = level.users.get(0).unwrap();
        assert!(user.location.x == 0.0);
        assert!(user.location.y == 0.0);
        
        assert!(user.rotation == 3.141);
        assert!(user.power == 0.5);

        assert!(user.color.r == 0);
        assert!(user.color.g == 128);
        assert!(user.color.b == 255);
        
    }
}