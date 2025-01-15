use flatbuffers::FlatBufferBuilder;
use protocol::{Message, MessageArgs};

pub mod protocol;

#[derive(Debug)]
pub enum MessageError {
    AppNameTooLong,
    PayloadTooLong,
}

// Spotify -> Imagine Dragons - Believer
pub fn send_message(app: &str, payload: &str) -> Result<Vec<u8>, MessageError> {
    if app.len() > 32 {
        return Err(MessageError::AppNameTooLong);
    }

    if payload.len() > 1024 {
        return Err(MessageError::PayloadTooLong);
    }
    let mut builder = FlatBufferBuilder::with_capacity(1056);

    let app_offset = builder.create_string(app);
    let payload_offset = builder.create_string(payload);

    let message = Message::create(&mut builder, &MessageArgs { app: Some(app_offset), payload: Some(payload_offset) });
    builder.finish(message, None);

    Ok(builder.finished_data().to_vec())
}