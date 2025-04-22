use futures_util::StreamExt;
use redis::Commands;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("Starting valkey pubsub pong test...");

    let valkey_url = std::env::var("VALKEY_URL").unwrap_or("redis://admin:71684edf744a7a804bb68b9136fd4b7c0da7c5334f7694d7bdb304feb7e85fea@127.0.0.1:6379/".to_owned());

    let mut client = redis::Client::open(valkey_url)?;
    let (mut sink, mut stream) = client.get_async_pubsub().await?.split();
    sink.psubscribe("ping:*").await?;

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        loop {
            let msg = stream.next().await.unwrap();
            let channel = msg.get_channel_name().to_owned();
            let payload: u32 = msg.get_payload().unwrap();
            println!("received from '{}': {}", channel, payload);
            if (tx.send((channel, payload)).await).is_err() {
                println!("receiver dropped");
                return;
            }
        }
    });

    while let Some((channel, payload)) = rx.recv().await {
        let id = channel.strip_prefix("ping:").unwrap();
        let response = payload + 1;
        let _: () = client.publish(format!("pong:{}", id), response)?;
    }

    Ok(())
}
