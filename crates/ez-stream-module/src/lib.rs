mod module_handler;
mod structs;

use structs::*;
use std::sync::{Arc, Mutex};
use log::*;

const MODAPI_LOG_TARGET: &'static str = "EZStreamModuleAPI";

lazy_static::lazy_static! {
    static ref LOADED_MODULE: Arc<Mutex<Option<ModuleHandler>>> = Arc::new(
        Mutex::new(None)
    );
}

pub struct ModuleAPI;

impl ModuleAPI {

    /// Checks if there is a module currently loaded, returns
    /// true if there is
    pub fn is_module_loaded() -> bool {
        trace!(target: MODAPI_LOG_TARGET, "Called is_module_loaded()");
        let loaded_mod = LOADED_MODULE.lock().unwrap();
        trace!(target: MODAPI_LOG_TARGET, "module_loaded: {}", loaded_mod.is_some());
        loaded_mod.is_some()
    }

    pub fn load_null_module() {
        trace!(target: MODAPI_LOG_TARGET, "Called load_null_module()");
        let mut loaded_mod = LOADED_MODULE.lock().unwrap();
        trace!(target: MODAPI_LOG_TARGET, "Setting module to None...");
        *loaded_mod = None;
    }

    /// Loads a module into memory, replacing whatever module
    /// may have been there before. `path` is the path to the
    /// module's base folder, in which there should be a
    /// description file called `module_info.yaml`.
    /// 
    /// Returns an error if something went wrong during module load
    pub fn load_module<P>(path: P) -> anyhow::Result<()> where P: AsRef<std::path::Path> {
        trace!(target: MODAPI_LOG_TARGET, "Called load_module(\"{}\")", path.as_ref().display());
        let module = ModuleHandler::new(path)?;
        let mut loaded_mod = LOADED_MODULE.lock().unwrap();
        *loaded_mod = Some(module);
        Ok(())
    }

    /// Loads the default module, usually called during startup.
    /// This is either (in order of preference):
    ///   - A module specified in the program settings as the default
    ///   - The NULL Module, that has no characters
    /// 
    /// Returns an error if something went wrong during the module
    /// load
    pub fn load_default_module() -> anyhow::Result<()> {
        todo!()
    }

    /// Returns a vector of all characters in the currently loaded
    /// module. If there is no module loaded, return None.
    /// 
    /// @TODO: See if Result<> is required for this type, are there
    /// any errors that could possibly be thrown in this function?
    pub fn get_list_of_characters() -> Option<Vec<CharacterData>> {
        todo!()
    }

    /// Returns the number of characters in the currently loaded
    /// module. If there is no loaded module, returns `None`.
    pub fn get_number_of_characters() -> Option<usize> {
        trace!(target: MODAPI_LOG_TARGET, "Called get_number_of_characters()");
        let loaded_mod = LOADED_MODULE.lock().unwrap();
        match &*loaded_mod {
            Some(m) => {
                trace!(target: MODAPI_LOG_TARGET, "Number of characters: {}", m.characters.len());
                Some(m.characters.len())
            },
            None => {
                trace!(target: MODAPI_LOG_TARGET, "No module loaded!");
                None
            }
        }
    }

    /// Returns the currently loaded module's name, if there is
    /// a module loaded. If not, returns `None`.
    pub fn get_loaded_module_name() -> Option<String> {
        trace!(target: MODAPI_LOG_TARGET, "Called get_loaded_module_name()");
        let loaded_mod = LOADED_MODULE.lock().unwrap();
        match &(*loaded_mod) {
            Some(m) => {
                trace!(target: MODAPI_LOG_TARGET, "Loaded module name: {}", m.current_module_name.clone());
                Some(m.current_module_name.clone())
            },
            None => {
                trace!(target: MODAPI_LOG_TARGET, "No module loaded!");
                None
            }
        }
    }

    // @TODO What should the user query this with to get
    // a character? The display name? Some ID?
    //
    // /// Returns character data from the loaded module
    // /// corresponding to {insert parameters here}. 
    // pub fn get_character_from_module() {
    //     todo!()
    // }
}