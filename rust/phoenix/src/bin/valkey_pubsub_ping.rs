use futures_util::StreamExt;
use rand::Rng;
use redis::Commands;
use uuid::Uuid;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("Starting valkey pubsub ping test...");

    let valkey_url = std::env::var("VALKEY_URL").unwrap_or("redis://admin:71684edf744a7a804bb68b9136fd4b7c0da7c5334f7694d7bdb304feb7e85fea@127.0.0.1:6379/".to_owned());

    let client = redis::Client::open(valkey_url.clone())?;
    let (mut sink, mut stream) = client.get_async_pubsub().await?.split();
    sink.psubscribe("pong:*").await?;

    tokio::spawn(async {
        let mut client = redis::Client::open(valkey_url).unwrap();

        let mut rng = rand::rng();

        for _ in 0..u16::MAX {
            let id = Uuid::new_v4();
            let channel = format!("ping:{}", id);
            let payload = rng.random_range(..u32::MAX);
            let _: () = client.publish(channel, format!("{}", payload)).unwrap();
        }
    });

    loop {
        let msg = stream.next().await.unwrap();
        let channel = msg.get_channel_name().to_owned();
        let payload: u32 = msg.get_payload().unwrap();
        println!("received from '{}': {}", channel, payload);
    }
}
