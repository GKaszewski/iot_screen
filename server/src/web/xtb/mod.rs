use xtb_client::{
    schema::{StreamGetKeepAliveSubscribe, StreamGetProfitSubscribe},
    StreamApi, XtbClientBuilder,
};

pub async fn initialize_xtb_websocket() -> anyhow::Result<()> {
    let user_id = "";
    let password = "";

    println!("Connecting to XTB");

    let real_builder = XtbClientBuilder::new_real();
    let mut client = match real_builder.build(user_id, password).await {
        Ok(client) => client,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let mut listener = client
        .subscribe_keep_alive(StreamGetKeepAliveSubscribe::default())
        .await?;
    let mut profits_listener = client
        .subscribe_profits(StreamGetProfitSubscribe::default())
        .await?;

    tokio::spawn(async move {
        println!("Listening for XTB data in the background");
        while let Some(item) = profits_listener.next().await.unwrap() {
            println!("Profit received: {:?}", item.profit);
        }
    });

    while let Some(_item) = listener.next().await.unwrap() {}

    println!("Closed connection");

    Ok(())
}
