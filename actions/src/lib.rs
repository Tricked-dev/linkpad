pub(crate) mod assets;
pub(crate) mod modules;
pub(crate) mod private;

use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use mlua::{Function, Lua, RegistryKey, Table};

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

    let require: Function = globals.get("require")?;

    let package: mlua::Table = globals.get("package")?;
    package
        .get::<_, mlua::Table>("searchers")?
        .push(lua.create_function(search_wrapper)?)?;

    fn add_to_path(globals: &Table, path: &Path) -> color_eyre::Result<()> {
        let package: mlua::Table = globals.get("package")?;
        package.set(
            "path",
            format!(
                "{};{p}/?.lua;{p}/?/init.lua;{p}/?",
                package.get::<_, String>("path")?,
                p = path.to_string_lossy()
            ),
        )?;
        Ok(())
    }

    add_to_path(&globals, &folder)?;

    fn lua_runnable(name: &OsStr) -> bool {
        let str_name = name.to_str().unwrap();

        str_name.ends_with(".lua") || str_name.ends_with(".tl")
    }

    let mut modules = HashMap::<String, Module>::new();

    let mut search_folders: HashSet<PathBuf> = HashSet::new();

    for entry in WalkDir::new(&folder).into_iter().filter_map(|e| e.ok()) {
        if let Some(parent) = entry.path().parent() {
            if !search_folders.contains(parent) {
                add_to_path(&globals, parent)?;
                search_folders.insert(parent.to_path_buf());
            }
        }

        if entry.depth() == 0 {
            continue;
        }

        let path = entry.path().to_string_lossy()[folder.to_string_lossy().len() + 1..]
            .to_string()
            .replace(".lua", "");

        println!("Searching {path}");

        let id = match entry.depth() {
            1 => {
                if lua_runnable(entry.file_name()) {
                    Some(entry.file_name().to_string_lossy().to_string())
                } else {
                    None
                }
            }
            _ => {
                if entry.file_name() == OsStr::new("lua.lua")
                    || entry.file_name() == OsStr::new("lua.tl")
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
            println!("LOading {path}");

            let module: Table = require.call(path)?;
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

    drop(require);
    drop(globals);
    drop(package);

    Ok(LoadContext { lua, modules })
}
