use serenity::builder::{
    CreateInteractionResponseData, CreateInteractionResponseFollowup, CreateMessage,
};

/// A trait that enables shorthand embed syntax
/// 
/// This trait defines a method report_status that can be used on various Create... contexts to quickly send an embed
/// 
/// # Example
/// ```
/// |resp: &mut CreateMessage| {
///     resp.report_status(
///         "Failure",
///         "Something went wrong!"
///     )
/// }
/// ```
pub trait EmbedResponse<S1, S2>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    fn report_status(self, title: S1, body: S2) -> Self;
}

#[doc(hidden)]
macro_rules! impl_embed_response {
    ($t:ident) => {
        impl<'a, 'b, S1, S2> EmbedResponse<S1, S2> for &'a mut $t<'b>
        where
            S1: AsRef<str>,
            S2: AsRef<str>,
        {
            fn report_status(self, title: S1, body: S2) -> Self {
                self.embed(|e| e.title(title.as_ref()).description(body.as_ref()))
            }
        }
    };
}

impl_embed_response! { CreateMessage }
impl_embed_response! { CreateInteractionResponseData }
impl_embed_response! { CreateInteractionResponseFollowup }
