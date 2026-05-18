use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    username: String
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            username: username.clone()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://api.dicebear.com/9.x/adventurer-neutral/svg?seed={}",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen">
                <div class="flex-none w-56 h-screen" style="background: white; border-right: 3px solid black;">
                    <div style="font-size: 1.2rem; font-weight: 900; padding: 16px; border-bottom: 3px solid black; background: #FFD93D; color: black;">{"👥 Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-white rounded-lg p-2">
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-sm justify-between">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-sm text-gray-400">
                                            {"Hi there!"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div style="width: 100%; height: 56px; border-bottom: 3px solid black; background: #4DC9E6; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;">
                        <div style="font-size: 1.2rem; font-weight: 900; color: black;">{"💬 Live Chat"}</div>
                            <div style="font-size: 0.85rem; font-weight: 700; background: white; border: 2px solid black; border-radius: 20px; padding: 4px 12px;">
                                {format!("👤 Logged in as: {}", self.username)}
                            </div>
                        </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                let is_own = m.from == self.username;
                                html!{
                                    <div class="flex w-full px-8 py-2" style={if is_own { "justify-content: flex-end;" } else { "justify-content: flex-start;" }}>
                                        <div class="flex items-end" style={if is_own {
                                            "background: #FF6B35; border: 3px solid black; border-radius: 16px 0 16px 16px; box-shadow: 4px 4px 0px black; max-width: 50%;"
                                        } else {
                                            "background: white; border: 3px solid black; border-radius: 0 16px 16px 16px; box-shadow: 4px 4px 0px black; max-width: 50%;"
                                        }}>
                                            if !is_own {
                                                <img class="w-8 h-8 rounded-full m-3" style="border: 2px solid black;" src={user.avatar.clone()} alt="avatar"/>
                                            }
                                            <div class="p-3">
                                                if !is_own {
                                                    <div class="text-sm font-bold">{m.from.clone()}</div>
                                                }
                                                <div class="text-sm" style={if is_own { "color: white;" } else { "color: #555;" }}>
                                                    if m.message.ends_with(".gif") || m.message.ends_with(".jpg") || m.message.ends_with(".jpeg") || m.message.ends_with(".png") {
                                                        <img class="mt-3" style="max-width: 200px; border-radius: 8px; border: 2px solid black;" src={m.message.clone()}/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                            if is_own {
                                                <img class="w-8 h-8 rounded-full m-3" style="border: 2px solid black;" src={user.avatar.clone()} alt="avatar"/>
                                            }
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }

                    </div>
                    <div class="w-full h-16 flex px-3 items-center" style="border-top: 3px solid black; background: white;">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" style="display: block; width: 100%; padding: 10px 20px; margin: 0 12px; border: 3px solid black; border-radius: 50px; outline: none; background: #f5f5f5; font-size: 0.95rem;" name="message" required=true />
                        <button onclick={submit} style="border: 3px solid black; background: #FF6B35; width: 44px; height: 44px; border-radius: 50%; display: flex; justify-content: center; align-items: center; cursor: pointer; box-shadow: 3px 3px 0px black; flex-shrink: 0;">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" style="fill: white; width: 20px; height: 20px;">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
