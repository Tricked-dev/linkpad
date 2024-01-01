// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

use base64::{engine::general_purpose, Engine as _};
use color_eyre::owo_colors::colors::xterm::LightSilverChalice;
use rocket::{futures::TryFutureExt, get, routes};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SendMessages {
    Data { data: Vec<serde_json::Value> },
    IconData { data: Icon },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ReceiveMessages {
    UpsertIcon { data: Icon },
    UpdatePanels { data: Vec<serde_json::Value> },
    Click { data: Uuid },
    LongClick { data: Uuid },
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
    thread::spawn(rocky::main);
    tauri::Builder::default().run(tauri::generate_context!())?;

    Ok(())
}
#[derive(Debug, Clone)]
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
    use color_eyre::owo_colors::OwoColorize;
    use rocket::{futures::stream::iter, get, launch, routes};
    use serde_json::json;
    use uuid::Uuid;
    use ws::{stream, Message};

    use crate::{Base64, LibraryInfo, ReceiveMessages, SendMessages};

    #[get("/connect")]
    fn connect(ws: ws::WebSocket) -> ws::Channel<'static> {
        use rocket::futures::{SinkExt, StreamExt};

        let mut library = LibraryInfo {
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

        ws.channel(move |mut stream| {
            Box::pin(async move {
                stream
                    .send(Message::Text(
                        serde_json::to_string_pretty(&SendMessages::Data {
                            data: library.panels.clone(),
                        })
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
                        dbg!(&t.to_text()?);
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
                        }
                    }

                    dbg!(&library);
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
