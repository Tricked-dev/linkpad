use std::fs::read_to_string;

use mlua::{Function, Lua, Table, Value::Nil};
use rust_embed::RustEmbed;

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "command")]
pub mod command;
pub mod env;
#[cfg(feature = "events")]
pub mod event_handler;
#[cfg(feature = "events")]
pub mod event_sender;
pub mod json;
pub mod log;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "notify")]
pub mod notify;
#[cfg(feature = "open")]
pub mod open;
pub mod utils;
pub mod versions;

#[macro_export]
macro_rules! create_body {
    ($lua:expr, $($key:expr => $value:expr),*) => {
        {
            let tb = $lua.create_table()?;
            $(tb.set($key, $value)?;)*
            Ok(tb)
        }
    }
}
#[derive(RustEmbed)]
#[folder = "src/std/"]
struct StdFiles;

impl StdFiles {
    pub fn get_lua_file(name: &str) -> Option<String> {
        std::str::from_utf8(Self::get(&format!("{name}.lua"))?.data.as_ref())
            .map(|x| x.to_string())
            .ok()
    }
}

pub fn search(lua: &Lua, module: String) -> mlua::Result<Table> {
    let loaded_modules: Table = lua.globals().get::<_, Table>("package")?.get("loaded")?;

    if let Ok(table) = loaded_modules.get::<_, Table>(&*module) {
        return Ok(table);
    }
    /* loads the module from the filesystem this needs to be updated when released */
    let load_std = || {
        let code = StdFiles::get_lua_file(&module).ok_or(mlua::Error::RuntimeError(format!(
            "module {module} not found"
        )))?;

        let table: Table = lua.load(&code).set_name(&module)?.call(())?;
        Ok::<_, mlua::Error>(table)
    };

    let load_teal = || {
        let table: Table = lua.load(tl::TEAL_LUA).set_name(&module)?.call(())?;
        loaded_modules.set("tl", table.clone())?;
        Ok::<_, mlua::Error>(table)
    };

    //TODO(everyone): keep this sorted alphabetically
    #[rustfmt::skip]
    let result: Table = match module.as_str() {
#[cfg(feature = "audio")]       "audio" => audio::init(lua)?,
#[cfg(feature = "clipboard")]   "clipboard" => clipboard::init(lua)?,
#[cfg(feature = "command")]     "command" => command::init(lua)?,
/* always-on */                 "env" => env::init(lua)?,
#[cfg(feature = "events")]      "event_handler_internal" => event_handler::init(lua)?,
#[cfg(feature = "events")]      "event_handler" => load_std()?,
#[cfg(feature = "events")]      "event_sender_internal" => event_sender::init(lua)?,
#[cfg(feature = "events")]      "event_sender" => load_std()?,
/* always-on */                 "json" => json::init(lua)?,
/* always-on */                 "fun" => load_std()?,
/* always-on */                 "kv" => load_std()?,
/* always-on */                 "pprint" => load_std()?,
/* always-on */                 "preload" => load_std()?,
/* always-on */                 "log" => log::init(lua)?,
#[cfg(feature = "network")]     "network" => network::init(lua)?,
#[cfg(feature = "notify")]      "notify" => notify::init(lua)?,
#[cfg(feature = "open")]        "open" => open::init(lua)?,
/* always-on */                 "tl" => load_teal()?,
/* always-on */                 "utils_internal" => utils::init(lua)?,
/* always-on */                 "versions" => versions::init(lua)?,
#[cfg(feature = "events")]      "shortcuts" => load_std()?,
/* always-on */                 "utils" => load_std()?,
/* always-on */                 m if m.ends_with(".tl")  => {
                                    let tl = if let Ok(table) = loaded_modules.get::<_, Table>("tl") {
                                        table
                                    } else {
                                        load_teal()?
                                    };
                                    let load = tl.get::<_, Function>("load")?;
                                    let data = read_to_string(m)?;
                                    load.call::<_, Function>((data, m))?.call::<_, Table>(())?
                                }

        _ => {
            /* early return so other modules can be cached */
            return Err(mlua::Error::RuntimeError(format!(
                "module {module} not found"
            )))?;
        }
    };

    loaded_modules.set(module, result.clone())?;
    Ok(result)
}

pub fn search_wrapper(lua: &Lua, module: String) -> mlua::Result<mlua::Value> {
    println!("Searching {module}");
    let result = search(lua, module);

    match result {
        Ok(table) => {
            //TODO: be smarter and find otu how to this better!
            let reg = lua.create_registry_value(table)?;
            let fun = lua.create_function(move |lua, _args: ()| {
                let table = lua.registry_value::<Table>(&reg)?;
                Ok(table.clone())
            })?;
            Ok(mlua::Value::Function(fun))
        }
        Err(_) => Ok(mlua::Value::Nil),
    }
}
