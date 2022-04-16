pub(crate) mod common;
pub(crate) mod message;

use std::borrow::Cow;

use minicbor::{Decode, Encode};

use crate::events::{common::Unsigned, message::RoomMessageContent};

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
pub(crate) enum Content<'a> {
    #[b(0)]
    RoomMessage(#[b(0)] RoomMessageContent<'a>),
}
