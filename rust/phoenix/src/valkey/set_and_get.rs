#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_set_and_get() -> redis::RedisResult<()> {
        use redis::AsyncCommands;

        let client = redis::Client::open("redis://127.0.0.1/")?;

        let mut con = client.get_multiplexed_async_connection().await?;

        let _: () = con.set("key1", b"foo").await?;

        redis::cmd("SET")
            .arg(&["key2", "bar"])
            .exec_async(&mut con)
            .await?;

        let result = redis::cmd("MGET")
            .arg(&["key1", "key2"])
            .query_async(&mut con)
            .await;

        assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

        Ok(())
    }
}
