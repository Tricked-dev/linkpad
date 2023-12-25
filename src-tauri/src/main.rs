// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use base64::{engine::general_purpose, Engine as _};
use color_eyre::owo_colors::colors::xterm::LightSilverChalice;
use rocket::{futures::TryFutureExt, get, routes};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ButtonData {
    TextButton {
        text: String,
        auto_size: bool,
        text_size: isize,
        #[serde(rename = "type")]
        b_type: ButtonType<0>,
    },
    // icon is retrieved from seprate ws messages
    IconButton {
        #[serde(rename = "type")]
        b_type: ButtonType<1>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    #[serde(flatten)]
    data: ButtonData,
    action: String,
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PanelData {
    id: Uuid,
    name: String,
    buttons: Vec<Vec<Button>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SendMessages {
    Panels { data: Vec<PanelData> },
    IconData { id: Uuid, data: Base64 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReceiveMessages {
    UpdatePanel { data: PanelData },
    UpdateIcon { id: Uuid, data: Base64 },
    UpdateButton { id: Uuid, data: Button },

    DeleteButton { id: Uuid },
    DeletePanel { id: Uuid },

    CreatePanel { data: PanelData },
    CreateButton { data: Button, row: usize },

    SetActivePanel { id: Uuid },
}

#[derive(Debug)]
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
    thread::spawn(rocky::main);
    tauri::Builder::default().run(tauri::generate_context!())?;

    Ok(())
}

pub struct LibraryInfo {
    panels: Vec<PanelData>,
    icons: Vec<(Uuid, Vec<u8>)>,
}

impl LibraryInfo {
    pub fn get_panel(&mut self, id: Uuid) -> Option<&mut PanelData> {
        self.panels.iter_mut().find(|p| p.id == id)
    }
}

mod rocky {
    use rocket::{get, launch, routes};
    use uuid::Uuid;

    use crate::{Button, LibraryInfo, PanelData, ReceiveMessages};

    #[get("/echo")]
    fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
        use rocket::futures::{SinkExt, StreamExt};

        let mut library = LibraryInfo {
            panels: vec![PanelData {
                buttons: vec![
                    vec![Button {
                        action: "mute".to_owned(),
                        data: crate::ButtonData::TextButton {
                            text: "123".to_owned(),
                            auto_size: true,
                            text_size: 20,
                            b_type: crate::ButtonType,
                        },
                        id: Uuid::new_v4(),
                    }],
                    vec![],
                ],
                id: Uuid::new_v4(),
                name: "test".to_string(),
            }],
            icons: vec![],
        };

        let active_panel = library.panels[0].id;

        ws.channel(move |mut stream| {
            Box::pin(async move {
                while let Some(message) = stream.next().await {
                    if let Ok(t) = message {
                        let msg: ReceiveMessages = serde_json::from_str(t.to_text()?).unwrap();

                        match msg {
                            ReceiveMessages::UpdatePanel { data } => {
                                if let Some(index) =
                                    library.panels.iter().position(|item| item.id == data.id)
                                {
                                    // Replace the item at the found index with the replacement item
                                    library.panels[index] = data;
                                }
                            }
                            ReceiveMessages::UpdateButton { id, data } => {}
                            ReceiveMessages::UpdateIcon { id, data } => {}

                            ReceiveMessages::CreateButton { data, row } => {
                                library.get_panel(active_panel).unwrap().buttons[row].push(data);
                            }
                            ReceiveMessages::CreatePanel { data } => {
                                library.panels.push(data);
                            }
                            _ => {}
                        }
                    }
                }

                Ok(())
            })
        })
    }
    #[launch]
    pub fn rocket() -> _ {
        rocket::build().mount("/", routes![echo_channel])
    }
}
