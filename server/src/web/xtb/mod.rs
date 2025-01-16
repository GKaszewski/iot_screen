use sqlx::SqlitePool;
use tokio::{sync::mpsc, time::interval};
use xtb_client::{
    schema::{StreamGetKeepAliveSubscribe, StreamGetProfitSubscribe},
    StreamApi, XtbClient, XtbClientBuilder,
};

use crate::{
    db::{get_xtb_credentials, XtbCredentials},
    tcp::StateMessage,
};

pub async fn initialize_xtb_websocket(
    db: SqlitePool,
    sender: mpsc::Sender<StateMessage>,
) -> anyhow::Result<()> {
    let mut interval = interval(tokio::time::Duration::from_secs(30));
    let mut is_connected = false;
    let mut xtb_client: Option<XtbClient> = None;

    loop {
        interval.tick().await;

        if !is_connected {
            let xtb_credentials: Option<XtbCredentials> = get_xtb_credentials(&db).await?;

            if xtb_credentials.is_none() {
                println!("No XTB credentials found in the database");
                continue;
            }

            let xtb_credentials = xtb_credentials.unwrap();

            let real_builder = XtbClientBuilder::new_real();
            match real_builder
                .build(&xtb_credentials.user_id, &xtb_credentials.password)
                .await
            {
                Ok(client) => {
                    println!("Successfully connected to XTB");
                    is_connected = true;
                    xtb_client = Some(client);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Ok(());
                }
            };
        }

        if let Some(ref mut client) = xtb_client {
            let mut profits_listener = match client
                .subscribe_profits(StreamGetProfitSubscribe::default())
                .await
            {
                Ok(listener) => listener,
                Err(e) => {
                    println!("Error subscribing to profits: {:?}", e);
                    is_connected = false;
                    xtb_client = None;
                    continue;
                }
            };

            let mut keep_alive_listener = match client
                .subscribe_keep_alive(StreamGetKeepAliveSubscribe::default())
                .await
            {
                Ok(listener) => listener,
                Err(e) => {
                    println!("Error subscribing to keep-alive: {:?}", e);
                    is_connected = false;
                    xtb_client = None;
                    continue;
                }
            };

            let sender_clone = sender.clone();
            tokio::spawn(async move {
                while let Ok(Some(item)) = profits_listener.next().await {
                    let payload = format!("Profit: {}", item.profit);
                    if let Err(e) = sender_clone.send(StateMessage::XtbData(payload)).await {
                        println!("Error sending state message: {:?}", e);
                    }
                }
                println!("Profit listener task terminated");
            });

            tokio::spawn(async move {
                println!("Listening for keep-alive...");
                while let Ok(Some(_)) = keep_alive_listener.next().await {}
                println!("Keep-alive listener task terminated");
            });
        }
    }
}
