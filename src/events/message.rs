use std::borrow::Cow;

use minicbor::{Decode, Encode};

use crate::events::common::Unsigned;

#[derive(Encode, Decode)]
pub(crate) struct Event<'a, Content> {
    #[b(0)]
    pub(crate) event_type: EventType,
    #[b(1)]
    pub(crate) content: Content,
    #[b(2)]
    pub(crate) room_id: Cow<'a, str>,
    #[b(3)]
    pub(crate) event_id: Cow<'a, str>,
    #[b(4)]
    pub(crate) sender: Cow<'a, str>,
    #[b(5)]
    pub(crate) origin_server_ts: u64,
    #[b(6)]
    pub(crate) unsigned: Unsigned<'a, Content>,
    #[b(7)]
    pub(crate) state_key: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode)]
pub(crate) struct TextContent<'a> {
    #[b(0)]
    pub(crate) msg_type: MessageType,
    #[b(1)]
    pub(crate) body: Cow<'a, str>,
    #[b(2)]
    pub(crate) format: Option<FormatType>,
    #[b(3)]
    pub(crate) formatted_body: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode)]
pub(crate) enum FormatType {
    #[n(0)]
    Html,
}

#[derive(Encode, Decode)]
pub(crate) enum MessageType {
    #[n(0)]
    Text,
}

#[derive(Encode, Decode)]
pub(crate) enum EventType {
    #[n(0)]
    RoomMessage,
}
