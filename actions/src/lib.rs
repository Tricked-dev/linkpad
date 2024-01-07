pub(crate) mod assets;
pub(crate) mod modules;
pub(crate) mod private;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use mlua::{Function, Lua, RegistryKey};
pub use modules::require;

pub use modules::*;
use walkdir::WalkDir;

pub struct LoadContext {
    pub lua: Lua,
    pub modules: HashMap<String, Module>,
}

pub struct Module {
    name: String,
    path: PathBuf,
    on_click: RegistryKey,
    on_long_click: RegistryKey,
}

impl Module {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn on_click<'a>(&'a self, lua: &'a Lua) -> color_eyre::Result<()> {
        lua.registry_value::<Function>(&self.on_click)
            .unwrap()
            .call(())?;
        Ok(())
    }

    pub fn on_long_click<'a>(&'a self, lua: &'a Lua) -> color_eyre::Result<()> {
        lua.registry_value::<Function>(&self.on_long_click)
            .unwrap()
            .call(())?;
        Ok(())
    }
}

pub fn load_modules(folder: PathBuf) -> color_eyre::Result<LoadContext> {
    let lua = Lua::new();

    let globals = lua.globals();
    let lua_require = lua.create_function(require)?;
    globals.set("require_ref", globals.get::<_, mlua::Function>("require")?)?;
    globals.set("require", lua_require)?;
    globals.set("__INTERNAL_LOADED_MODULES", lua.create_table()?)?;

    fn lua_runnable(name: &OsStr) -> bool {
        let str_name = name.to_str().unwrap();

        str_name.ends_with(".lua") || str_name.ends_with(".tl")
    }

    let mut modules = HashMap::<String, Module>::new();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().as_os_str().to_string_lossy().to_string();

        let id = match entry.depth() {
            1 => {
                if lua_runnable(entry.file_name()) {
                    Some(entry.file_name().to_string_lossy().to_string())
                } else {
                    None
                }
            }
            _ => {
                if entry.file_name() == OsStr::new("main.lua")
                    || entry.file_name() == OsStr::new("main.tl")
                {
                    Some(
                        entry
                            .path()
                            .parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                    )
                } else {
                    None
                }
            }
        };

        if let Some(id) = id {
            let module = require(&lua, path)?;
            let name: String = module.get("name")?;
            let on_click: Function = module.get("on_click")?;
            let on_long_click: Function = module.get("on_click")?;
            let data = Module {
                name,
                path: entry.path().to_path_buf(),
                on_click: lua.create_registry_value(on_click)?,
                on_long_click: lua.create_registry_value(on_long_click)?,
            };
            modules.insert(id, data);
        }
    }

    //we need to drop globals so that lua can be passed to the struct

    drop(globals);

    Ok(LoadContext { lua, modules })
}
