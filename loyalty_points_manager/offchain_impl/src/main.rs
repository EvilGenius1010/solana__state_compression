use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
use redis::Client;
use redis::Commands;
use spl_concurrent_merkle_tree::concurrent_merkle_tree::ConcurrentMerkleTree;
use spl_concurrent_merkle_tree::error::ConcurrentMerkleTreeError;
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
struct LoyaltyPointsHistory {
    owner_pubkey: u32,
    current_points: u64
}

fn connect_to_redis() -> redis::RedisResult<()> {
    // Create a Redis client from a URL string
    let client = Client::open("redis://127.0.0.1/")?;

    // Get a synchronous connection
    let mut con = client.get_connection()?;

    Ok(())
}

/// Handles on chain successful verification and updates database and merkle tree.
fn handle_update_from_chain() {}

/// Handles requests from chain for requests for sibling nodes.
fn handle_sibling_root_req(
    index: u64,
    cmt: Rc<ConcurrentMerkleTree<MAX_DEPTH_SIZE, MAX_BUFFER_SIZE>>,
) {

    
}

fn main() {
    let cmt: Rc<ConcurrentMerkleTree<MAX_DEPTH_SIZE, MAX_BUFFER_SIZE>> =
        Rc::new(ConcurrentMerkleTree::new());

    // connect to redis
    if let Err(e) = connect_to_redis() {
        eprintln!("Failed to connect or interact with Redis: {}", e);
    }
}
