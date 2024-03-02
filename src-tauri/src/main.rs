// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use base64::{engine::general_purpose, Engine as _};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SendMessages {
    Data { data: Vec<serde_json::Value> },
    Actions { data: Vec<serde_json::Value> },
    IconData { data: Icon },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ReceiveMessages {
    UpsertIcon { data: Icon },
    UpdatePanels { data: Vec<serde_json::Value> },
    Click { data: String },
    LongClick { data: String },
    GetIcon { data: String, id: Uuid },
}

#[derive(Debug, Clone)]
pub struct Base64(Vec<u8>);
impl Serialize for Base64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&general_purpose::STANDARD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = Base64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                general_purpose::STANDARD
                    .decode(v)
                    .map(Base64)
                    .map_err(Error::custom)
            }
        }
        deserializer.deserialize_str(Vis)
    }
}
#[derive(Debug)]
pub struct ButtonType<const V: u8>;

impl<const V: u8> Serialize for ButtonType<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(V)
    }
}

impl<'de, const V: u8> Deserialize<'de> for ButtonType<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if value == V {
            Ok(ButtonType::<V>)
        } else {
            Err(serde::de::Error::custom(color_eyre::eyre::anyhow!(
                "Error!"
            )))
        }
    }
}

fn main() -> color_eyre::Result<()> {
    thread::spawn(rocky::main).join().unwrap();
    // tauri::Builder::default().run(tauri::generate_context!())?;

    Ok(())
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LibraryInfo {
    panels: Vec<serde_json::Value>,
    icons: Vec<Icon>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Icon {
    pub id: Uuid,
    pub data: Base64,
    pub content_type: String,
}

mod rocky {
    use std::fs;

    use actions::IconResult;
    use rocket::{futures::stream::iter, get, launch, routes};
    use serde_json::json;
    use uuid::Uuid;
    use ws::{stream, Message};

    use crate::{Base64, Icon, LibraryInfo, ReceiveMessages, SendMessages};

    #[get("/connect")]
    fn connect(ws: ws::WebSocket) -> ws::Channel<'static> {
        use actions::load_modules;
        use rocket::futures::{SinkExt, StreamExt};

        let app_dir = dirs::config_dir().unwrap().join("linkpad");
        fs::create_dir_all(&app_dir).unwrap();

        let default_lib = LibraryInfo {
            panels: vec![json!({
                "buttons": [
                    [
                        {
                            "action": "mute",
                            "text": "123",
                            "autoSize": true,
                            "textSize": 20,
                            "type": 1,
                            "id": Uuid::new_v4(),
                        }
                    ]
                    ],
                "id": Uuid::new_v4(),
                "name": "test",
            })],
            icons: vec![],
        };

        let mut library = match fs::File::open(app_dir.join("libraries.json")) {
            Ok(file) => serde_json::from_reader(file).unwrap_or(default_lib),
            _ => default_lib,
        };

        ws.channel(move |mut stream| {
            Box::pin(async move {
                fs::create_dir_all(app_dir.join("modules")).unwrap();
                let modules = load_modules(app_dir.join("modules")).unwrap();

                let actions = modules
                    .modules
                    .iter()
                    .map(|m| {
                        json! ({
                            "id": m.0,
                            "name": m.1.name()
                        })
                    })
                    .collect::<Vec<serde_json::Value>>();

                stream
                    .send(Message::Text(
                        serde_json::to_string_pretty(&SendMessages::Data {
                            data: library.panels.clone(),
                        })
                        .unwrap(),
                    ))
                    .await
                    .unwrap();

                stream
                    .send(Message::Text(
                        serde_json::to_string_pretty(&SendMessages::Actions { data: actions })
                            .unwrap(),
                    ))
                    .await
                    .unwrap();
                let mut st = iter(library.icons.clone().into_iter().map(|icon| {
                    Ok(Message::Text(
                        serde_json::to_string_pretty(&SendMessages::IconData { data: icon })
                            .unwrap(),
                    ))
                }));
                stream.send_all(&mut st).await.unwrap();

                while let Some(message) = stream.next().await {
                    if let Ok(t) = message {
                        let msg: ReceiveMessages = serde_json::from_str(t.to_text()?).unwrap();

                        match msg {
                            ReceiveMessages::UpdatePanels { data } => {
                                library.panels = data;
                            }
                            ReceiveMessages::UpsertIcon { data } => {
                                library.icons.retain(|x| x.id != data.id);
                                library.icons.push(data.clone());

                                stream
                                    .send(Message::Text(
                                        serde_json::to_string_pretty(&SendMessages::IconData {
                                            data,
                                        })
                                        .unwrap(),
                                    ))
                                    .await
                                    .unwrap();
                            }
                            ReceiveMessages::Click { data } => {
                                println!("Hello World");
                                modules
                                    .modules
                                    .get(&data)
                                    .unwrap()
                                    .on_click(&modules.lua)
                                    .unwrap();
                            }
                            ReceiveMessages::LongClick { data } => {
                                println!("Hello World Long Click");
                                modules
                                    .modules
                                    .get(&data)
                                    .unwrap()
                                    .on_long_click(&modules.lua)
                                    .unwrap();
                            }
                            ReceiveMessages::GetIcon { data, id } => {
                                let data = modules
                                    .modules
                                    .get(&data)
                                    .unwrap()
                                    .icon(&modules.lua)
                                    .unwrap();

                                let res = match data {
                                    IconResult::None | IconResult::NoChange => None,
                                    IconResult::Url { url } => {
                                        let res = reqwest::get(url).await.unwrap();
                                        let c_type = res
                                            .headers()
                                            .get("content-type")
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_string();
                                        Some((res.bytes().await.unwrap().to_vec(), c_type))
                                    }
                                    IconResult::Base64 { data, content_type } => {
                                        Some((data, content_type))
                                    }
                                    IconResult::Path { path } => todo!(),
                                };
                                if let Some(icon) = res {
                                    stream
                                        .send(Message::Text(
                                            serde_json::to_string_pretty(&SendMessages::IconData {
                                                data: Icon {
                                                    id,
                                                    data: Base64(icon.0),
                                                    content_type: icon.1,
                                                },
                                            })
                                            .unwrap(),
                                        ))
                                        .await
                                        .unwrap();
                                }
                            }
                        }
                    }

                    fs::write(
                        app_dir.join("libraries.json"),
                        serde_json::to_string_pretty(&library).unwrap(),
                    )
                    .unwrap();
                    // dbg!(&library);
                }

                Ok(())
            })
        })
    }
    #[launch]
    pub fn rocket() -> _ {
        rocket::build().mount("/", routes![connect])
    }
}
