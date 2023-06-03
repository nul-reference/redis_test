use redis::Commands;

fn write_to_redis(connection: &mut redis::Connection) -> redis::RedisResult<()> {
    connection.set("test", 42)
}

fn read_u8_from_redis(connection: &mut redis::Connection) -> redis::RedisResult<u8> {
    connection.get("test")
}

fn main() -> anyhow::Result<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_connection()?;

    write_to_redis(&mut connection)?;
    let result = read_u8_from_redis(&mut connection)?;

    println!("Sent 42 to key \"test\", got back {result}");

    Ok(())
}
