use serenity::builder::{
    CreateInteractionResponseData, CreateInteractionResponseFollowup, CreateMessage,
};

pub trait EmbedResponse<S1, S2>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    fn report_status(self, title: S1, body: S2) -> Self;
}

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
