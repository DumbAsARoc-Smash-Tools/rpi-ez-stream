use std::path::PathBuf;
use super::structs::*;
use super::MODAPI_LOG_TARGET;
use log::*;

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

        trace!(target: MODAPI_LOG_TARGET, "Searching path \"{}\" for module data...", module_info_path.display());
        let module_info_string = match std::fs::read_to_string(module_info_path.clone()) {
            Ok(s) => s,
            Err(e) => {
                warn!(target: MODAPI_LOG_TARGET, "Error opening {}: {}", MODULE_INFO_FILENAME, e);
                return Err(anyhow::anyhow!("Error opening {}: {}", MODULE_INFO_FILENAME, e));
            }
        };

        trace!(target: MODAPI_LOG_TARGET, "Parsing {}", module_info_path.display());
        let module_info = match serde_saphyr::from_str::<ModuleData>(&module_info_string) {
            Ok(d) => d,
            Err(e) => {
                error!(target: MODAPI_LOG_TARGET, "Failed to parse {}: {}", MODULE_INFO_FILENAME, e);
                return Err(anyhow::anyhow!("Failed to parse {}: {}", MODULE_INFO_FILENAME, e));
            }
        };

        trace!(target: MODAPI_LOG_TARGET, "Parsed Module data:");
        trace!(target: MODAPI_LOG_TARGET, " - Name: \"{}\"", module_info.module_name.clone());
        self.current_module_name = module_info.module_name;

        trace!(target: MODAPI_LOG_TARGET, " - Finding Characters...");
        for character in &module_info.character_data {
            
            // @TODO Add some verification code here that ensures the
            // data in the YAML is correct as according to the filesystem
            // ...at current, it just assumes the YAML is infallible.

            trace!(target: MODAPI_LOG_TARGET, "   - Found Character: \"{}\" (Costumes: {})",
                character.display_name.clone(),
                character.num_costumes
            );

            self.characters.push(character.clone());
            if character.display_name == module_info.default_character {
                self.default_character = character.clone();
            }
        }

        trace!(target: MODAPI_LOG_TARGET, " - Total Characters: {}", self.characters.len());

        if self.default_character.display_name == "Nil" {
            error!(target: MODAPI_LOG_TARGET, "The {} provided has an invalid default character!", MODULE_INFO_FILENAME);
            return Err(anyhow::anyhow!("The {} provided has an invalid default character!", MODULE_INFO_FILENAME));
        }

        trace!(target: MODAPI_LOG_TARGET, " - Default Character: \"{}\"", self.default_character.display_name.clone());

        Ok(())
    }

}
