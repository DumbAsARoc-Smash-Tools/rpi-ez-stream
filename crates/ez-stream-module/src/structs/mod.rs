use std::path::PathBuf;
use serde::Deserialize;

pub struct ModuleHandler {
    pub current_module_name: String,
    pub base_directory_path: PathBuf,
    pub characters: Vec<CharacterData>,
    pub default_character: CharacterData,
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
pub struct CharacterData {
    pub display_name: String,
    pub path_in_folder: String,
    pub num_costumes: u32,
    pub aliases: Vec<String>
}

#[derive(Deserialize)]
pub struct ModuleData {
    pub module_name: String,
    pub default_character: String,
    pub character_data: Vec<CharacterData>
}