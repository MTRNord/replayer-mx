use std::borrow::Cow;

use minicbor::{Decode, Encode};

use crate::events::common::Unsigned;

#[derive(Encode, Decode, Debug)]
pub(crate) struct Event<'a> {
    #[b(0)]
    pub(crate) content: Content<'a>,
    #[b(1)]
    pub(crate) room_id: Cow<'a, str>,
    #[b(2)]
    pub(crate) event_id: Cow<'a, str>,
    #[b(3)]
    pub(crate) sender: Cow<'a, str>,
    #[b(4)]
    pub(crate) origin_server_ts: u64,
    #[b(5)]
    pub(crate) unsigned: Unsigned<'a, Content<'a>>,
    #[b(6)]
    pub(crate) state_key: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) enum RoomMessageContent<'a> {
    #[b(0)]
    Text(#[b(0)] TextContent<'a>),
    #[b(1)]
    Audio(#[b(0)] AudioContent<'a>),
}

/// Use for text, emotes and notices
#[derive(Encode, Decode, Debug)]
pub(crate) struct TextContent<'a> {
    #[b(0)]
    pub(crate) body: Cow<'a, str>,
    #[b(1)]
    pub(crate) format: Option<FormatType>,
    #[b(2)]
    pub(crate) formatted_body: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct AudioContent<'a> {
    #[b(0)]
    pub(crate) body: Cow<'a, str>,
    #[b(1)]
    pub(crate) url: Cow<'a, str>,
    #[b(2)]
    pub(crate) info: Option<AudioInfo<'a>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct AudioInfo<'a> {
    #[b(0)]
    pub(crate) duration: u64,
    #[b(1)]
    pub(crate) mime_type: Cow<'a, str>,
    #[b(2)]
    pub(crate) size: u64,
}

#[derive(Encode, Decode, Debug)]
pub(crate) enum FormatType {
    #[b(0)]
    Html,
}

#[derive(Encode, Decode, Debug)]
pub(crate) enum Content<'a> {
    #[b(0)]
    RoomMessage(#[b(0)] RoomMessageContent<'a>),
}
