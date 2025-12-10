use std::path::PathBuf;
use super::structs::*;

const MODULE_INFO_FILENAME: &'static str = "module_info.yaml";

impl ModuleHandler {

    pub fn new<P>(path: P) -> anyhow::Result<Self> where P: AsRef<std::path::Path> {
        let mut new_module = Self {
            current_module_name: "Something".to_string(),
            base_directory_path: PathBuf::from(path.as_ref()),
            characters: vec![],
            default_character: CharacterData {
                display_name: "Nil".to_string(),
                path_in_folder: "NIL".to_string(),
                num_costumes: 0,
                aliases: vec![]
            }
        };

        new_module.parse_module_information()?;

        Ok(new_module)
    }

    fn parse_module_information(&mut self) -> anyhow::Result<()> {

        let path = self.base_directory_path.clone();
        let module_info_path = path.join(MODULE_INFO_FILENAME);
        let module_info_string = match std::fs::read_to_string(module_info_path) {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!("Error opening {}: {}", MODULE_INFO_FILENAME, e));
            }
        };

        let module_info = match serde_saphyr::from_str::<ModuleData>(&module_info_string) {
            Ok(d) => d,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to parse {}: {}", MODULE_INFO_FILENAME, e));
            }
        };

        self.current_module_name = module_info.module_name;

        for character in &module_info.character_data {
            
            // @TODO Add some verification code here that ensures the
            // data in the YAML is correct as according to the filesystem
            // ...at current, it just assumes the YAML is infallible.

            self.characters.push(character.clone());
            if character.display_name == module_info.default_character {
                self.default_character = character.clone();
            }
        }

        if self.default_character.display_name == "Nil" {
            return Err(anyhow::anyhow!("The {} provided has an invalid default character!", MODULE_INFO_FILENAME));
        }

        Ok(())
    }

}
