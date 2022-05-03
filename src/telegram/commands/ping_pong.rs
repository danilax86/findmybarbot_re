use telexide::api::types::SendMessage;
use telexide::client::Context;
use telexide::framework::CommandResult;
use telexide::model::Message;
use telexide::prelude::command;

#[command(description = "just a ping-pong command")]
async fn ping(context: Context, message: Message) -> CommandResult {
    context
        .api
        .send_message(SendMessage::new(message.chat.get_id(), "pong"))
        .await?;
    Ok(())
}