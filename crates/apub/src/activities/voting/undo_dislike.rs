use crate::activities::{
  verify_activity,
  verify_person_in_community,
  voting::{dislike::DislikePostOrComment, receive_undo_like_or_dislike},
};
use activitystreams::activity::kind::UndoType;
use lemmy_apub_lib::{values::PublicUrl, verify_urls_match, ActivityCommonFields, ActivityHandler};
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use url::Url;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoDislikePostOrComment {
  to: PublicUrl,
  object: DislikePostOrComment,
  cc: [Url; 1],
  #[serde(rename = "type")]
  kind: UndoType,
  #[serde(flatten)]
  common: ActivityCommonFields,
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for UndoDislikePostOrComment {
  async fn verify(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    verify_activity(self.common())?;
    verify_person_in_community(&self.common.actor, &self.cc[0], context, request_counter).await?;
    verify_urls_match(&self.common.actor, &self.object.common().actor)?;
    self.object.verify(context, request_counter).await?;
    Ok(())
  }

  async fn receive(
    &self,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError> {
    receive_undo_like_or_dislike(
      &self.common.actor,
      &self.object.object,
      context,
      request_counter,
    )
    .await
  }

  fn common(&self) -> &ActivityCommonFields {
    &self.common
  }
}
