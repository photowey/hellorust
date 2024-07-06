use redis::AsyncCommands;

use super::*;

#[test]
fn test_say_hi() {
    let words = say_hi();

    assert_eq!(words, "say hello to: sharkchili");
}

// $ cargo test -- --show-output

#[test]
fn test_add_two() {
    assert_eq!(4, add_two(2));
    assert_eq!(4, inner_add_two(2, 2));
}

#[test]
fn test_capitalize_first() {
    let x = capitalize_first("hello");
    assert_eq!("Hello", x);
}

#[test]
fn test_env_run() {
    env::run();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_tokio_async() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://:root@192.168.19.250:6379/5").unwrap();
    let mut con = client.get_multiplexed_async_connection().await?;

    con.set("key1", b"foo").await?;

    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut con)
        .await?;

    let result: Vec<Option<Vec<u8>>> = redis::cmd("MGET")
        .arg(&["key1", "key2"])
        .query_async(&mut con)
        .await?;

    assert_eq!(result, vec![Some(b"foo".to_vec()), Some(b"bar".to_vec())]);

    Ok(())
}

#[test]
fn test_fib_fn() {
    //macros::fib_fn();
    macros::fib_macro();
}
