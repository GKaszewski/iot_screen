pub mod protocol;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn send_message(app: &str, payload: &str) -> protocol::Message {
    let app: [u8; 32] = match app.as_bytes().try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            let short_app_name = app.chars().take(32).collect::<String>();
            let mut bytes = [0; 32];
            bytes[..short_app_name.len()].copy_from_slice(short_app_name.as_bytes());
            bytes
        },
    };

    let payload = match payload.as_bytes().try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            let short_payload = payload.chars().take(1024).collect::<String>();
            let mut bytes = [0; 1024];
            bytes[..short_payload.len()].copy_from_slice(short_payload.as_bytes());
            bytes
        },
    };

    let message = protocol::Message::new(&app, &payload);
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
