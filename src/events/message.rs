use minicbor::{Decode, Encode};
use std::borrow::Cow;

#[derive(Encode, Decode, Debug)]
pub(crate) enum RoomMessageContent<'a> {
    #[b(0)]
    Text(#[b(0)] TextContent<'a>),
    #[b(1)]
    Audio(#[b(0)] AudioContent<'a>),
    #[b(2)]
    File(#[b(0)] FileContent<'a>),
    #[b(3)]
    Image(#[b(0)] ImageContent<'a>),
    #[b(4)]
    Video(#[b(0)] VideoContent<'a>),
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
#[cbor(index_only)]
pub(crate) enum FormatType {
    #[b(0)]
    Html,
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
pub(crate) struct FileContent<'a> {
    #[b(0)]
    pub(crate) body: Cow<'a, str>,
    #[b(1)]
    pub(crate) filename: Cow<'a, str>,
    #[b(2)]
    pub(crate) url: Cow<'a, str>,
    #[b(3)]
    pub(crate) info: Option<FileInfo<'a>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct FileInfo<'a> {
    #[b(0)]
    pub(crate) mime_type: Cow<'a, str>,
    #[b(1)]
    pub(crate) size: u64,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct ImageContent<'a> {
    #[b(0)]
    pub(crate) body: Cow<'a, str>,
    #[b(1)]
    pub(crate) url: Cow<'a, str>,
    #[b(2)]
    pub(crate) info: Option<ImageInfo<'a>>,
    #[b(3)]
    pub(crate) file: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct ImageInfo<'a> {
    #[b(0)]
    pub(crate) mime_type: Cow<'a, str>,
    #[b(1)]
    pub(crate) size: u64,
    #[b(2)]
    pub(crate) h: Option<u64>,
    #[b(3)]
    pub(crate) w: Option<u64>,
    #[b(4)]
    pub(crate) thumbnail_file: Option<Cow<'a, str>>,
    #[b(5)]
    pub(crate) thumbnail_info: Option<ThumbnailInfo<'a>>,
    #[b(6)]
    pub(crate) thumbnail_url: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct ThumbnailInfo<'a> {
    #[b(0)]
    pub(crate) mime_type: Cow<'a, str>,
    #[b(1)]
    pub(crate) size: u64,
    #[b(2)]
    pub(crate) h: Option<u64>,
    #[b(3)]
    pub(crate) w: Option<u64>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct VideoContent<'a> {
    #[b(0)]
    pub(crate) body: Cow<'a, str>,
    #[b(1)]
    pub(crate) url: Cow<'a, str>,
    #[b(2)]
    pub(crate) info: Option<VideoInfo<'a>>,
    #[b(3)]
    pub(crate) file: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct VideoInfo<'a> {
    #[b(0)]
    pub(crate) duration: u64,
    #[b(1)]
    pub(crate) mime_type: Cow<'a, str>,
    #[b(2)]
    pub(crate) size: u64,
    #[b(3)]
    pub(crate) h: Option<u64>,
    #[b(4)]
    pub(crate) w: Option<u64>,
    #[b(5)]
    pub(crate) thumbnail_file: Option<Cow<'a, str>>,
    #[b(6)]
    pub(crate) thumbnail_info: Option<ThumbnailInfo<'a>>,
    #[b(7)]
    pub(crate) thumbnail_url: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct LocationContent<'a> {
    #[b(0)]
    pub(crate) body: Cow<'a, str>,
    #[b(1)]
    pub(crate) geo_uri: Cow<'a, str>,
    #[b(2)]
    pub(crate) info: Option<LocationInfo<'a>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct LocationInfo<'a> {
    #[b(0)]
    pub(crate) thumbnail_file: Option<Cow<'a, str>>,
    #[b(1)]
    pub(crate) thumbnail_info: Option<ThumbnailInfo<'a>>,
    #[b(2)]
    pub(crate) thumbnail_url: Option<Cow<'a, str>>,
}

#[derive(Encode, Decode, Debug)]
pub(crate) struct ServerNotice<'a> {
    #[b(0)]
    pub(crate) admin_contact: Cow<'a, str>,
    #[b(1)]
    pub(crate) body: Cow<'a, str>,
    #[b(2)]
    pub(crate) server_notice_type: ServerNoticeType<'a>,
    #[b(3)]
    pub(crate) limit_type: Option<LimitType>,
}

#[derive(Encode, Decode, Debug)]
#[cbor(index_only)]
pub(crate) enum LimitType {
    #[b(0)]
    MontlyActiveUser,
}

#[derive(Encode, Decode, Debug)]
pub(crate) enum ServerNoticeType<'a> {
    #[b(0)]
    UsageLimitExceeded,
    #[b(1)]
    Other(#[b(0)] Cow<'a, str>),
}
