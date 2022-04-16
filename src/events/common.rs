use minicbor::{Decode, Encode};
use std::borrow::Cow;

#[derive(Encode, Decode)]
pub(crate) struct Unsigned<'a, Content> {
    #[b(0)]
    pub(crate) age: u64,
    #[b(1)]
    pub(crate) transaction_id: Cow<'a, str>,
    #[b(2)]
    pub(crate) prev_content: Option<Content>,
    #[b(3)]
    pub(crate) redacted_because: Option<Cow<'a, str>>,
}
