use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
use redis::{Script,Value,RedisResult,aio::MultiplexedConnection,Client};
use spl_concurrent_merkle_tree::concurrent_merkle_tree::ConcurrentMerkleTree;
use std::io::{Error, ErrorKind, Result};
use std::rc::Rc;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::u8;
static MAX_BUFFER_SIZE: usize = 2;
static MAX_DEPTH_SIZE: usize = 20;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BorshSystemTime(pub SystemTime);

impl BorshSerialize for BorshSystemTime {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let duration = self
            .0
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        let _ = duration.as_secs().serialize(writer); //u64
        let _ = duration.subsec_nanos().serialize(writer); //u32
        Ok(())
    }
}

impl BorshDeserialize for BorshSystemTime {
    fn try_from_slice(v: &[u8]) -> std::io::Result<Self> {
        //length of 64(time in secs)+32(time in nanosecs) bits
        let mut v_mut = v;
        Self::deserialize(&mut v_mut)
    }
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        let secs = u64::deserialize(buf)?;
        let nanos = u32::deserialize(buf)?;

        let duration = Duration::new(secs, nanos);
        let time = UNIX_EPOCH.checked_add(duration).ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Overflow deserializing SystemTime")
        })?;

        Ok(BorshSystemTime(time))
    }
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let secs = u64::deserialize_reader(reader)?;
        let nanos = u32::deserialize_reader(reader)?;
        let duration = Duration::new(secs, nanos);
        let time = UNIX_EPOCH.checked_add(duration).ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Overflow deserializing SystemTime")
        })?;

        Ok(BorshSystemTime(time))
    }
    /// Deprecated in the borsh crate, use deserialize_reader() instead.
    fn try_from_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Self::deserialize_reader(reader)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
struct LoyaltyPointsChange {
    change: i64,
    timestamp: BorshSystemTime,
    owner_pubkey: u32,
    tx_signature: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct LoyaltyPoints {
    owner_pubkey: u32,
    current_points: u64,
}

/// Handles on chain successful verification and updates database and merkle tree.
fn handle_update_from_chain() {}

/// Handles requests from chain for requests for sibling nodes.
fn handle_sibling_root_req(
    index: u64,
    cmt: Rc<ConcurrentMerkleTree<MAX_DEPTH_SIZE, MAX_BUFFER_SIZE>>,
) {
}

fn insert_into_redis_sync() -> redis::RedisResult<()> {
    // 1. Synchronous Client
    let client = redis::Client::open("redis://127.0.0.1/")?;

    // 2. Synchronous Connection (Blocks here)
    let mut connection = client.get_connection()?;

    let script = redis::Script::new(include_str!("../lua/register_user.lua"));

    // 3. "invoke" instead of "invoke_async" (Blocks here)
    let index: u64 = script
        .key("tree:1:users")
        .key("tree:1:next_index")
        .arg("5Q5fe...")
        .invoke(&mut connection)?; // <--- No .await needed

    Ok(())
}

/// creates a connection
async fn connect_to_redis() -> redis::RedisResult<MultiplexedConnection> {
    // Create a Redis client from a URL string
    let client = Client::open("redis://127.0.0.1/")?;

    // Get a synchronous connection
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_multiplexed_async_connection().await?;

    Ok(con)
}

/// If user doesn't exist in redis, add them.
async fn register_new_user(pubkey: u32) -> RedisResult<()> {
    let mut connection = connect_to_redis().await?;
    let check_pubkey_script = Script::new(
        r#"
local v = redis.call("HGET", KEYS[1], ARGV[1])
if v == false then
  return 0
end
return v
"#,
    );

    let result = check_pubkey_script
        .key("lpm:pubkeys")
        .arg(pubkey)
        .invoke_async(&mut connection)
        .await?;
    match result {
        Value::Int(0) => {
            println!("User does NOT exist");
        }
        Value::BulkString(bytes) => {
            let index = String::from_utf8(bytes).unwrap();
            println!("User exists, index = {}", index);
        }
        _ => unreachable!("Unexpected Redis return"),
    }

    Ok(())

    // Fetch the latest index from
}

fn hash_all_nodes(index: u64){
    
}


/// Update redis data in the case of change of data.
// async fn update_redis()->redis::RedisResult<()>{
//     let mut connection = connect_to_redis().await?;

// }

/// inserts values into redis along with intermediate nodes calculated.
// TODO: Pipeline for batch insertion.
async fn initialize_redis() -> redis::RedisResult<()> {
    let mut connection = connect_to_redis().await?;
    let script = redis::Script::new(include_str!("../lua/initialize_tree.lua"));
    let index: String = script
        .key("lpm")
        .arg(20)
        .invoke_async(&mut connection)
        .await?;
    println!("{index}");
    Ok(())
}

/// called after changes made by end-user ie. before points change is calculated.
fn handle_points_change(points: LoyaltyPoints, change: LoyaltyPointsChange) {}

#[tokio::main]
async fn main() {
    let cmt: Rc<ConcurrentMerkleTree<MAX_DEPTH_SIZE, MAX_BUFFER_SIZE>> =
        Rc::new(ConcurrentMerkleTree::new());

    // connect to redis
    // if let Err(e) = connect_to_redis() {
    //     eprintln!("Failed to connect or interact with Redis: {}", e);
    // }

    initialize_redis().await.unwrap();
    register_new_user(83012830).await.unwrap();
}
