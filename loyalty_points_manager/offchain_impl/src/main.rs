use redis::Client;
use redis::Commands;
use std::io::{Result, Error, ErrorKind};
use std::time::SystemTime;
use std::time::Duration;
use std::time::UNIX_EPOCH;
use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BorshSystemTime(pub SystemTime);

impl BorshSerialize for BorshSystemTime{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let duration = self.0.duration_since(UNIX_EPOCH)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        let _ = duration.as_secs().serialize(writer); //u64
        let _ = duration.subsec_nanos().serialize(writer); //u32
        Ok(())
    }
}

impl BorshDeserialize for BorshSystemTime{
    fn try_from_slice(v: &[u8]) -> std::io::Result<Self> {
        //length of 64(time in secs)+32(time in nanosecs) bits
        let mut v_mut = v;
        Self::deserialize(&mut v_mut)
        
    }
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        let secs = u64::deserialize(buf)?;
        let nanos = u32::deserialize(buf)?;

        let duration = Duration::new(secs,nanos);
         let time = UNIX_EPOCH.checked_add(duration)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Overflow deserializing SystemTime"))?;
            
        Ok(BorshSystemTime(time))
        
    }
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
       let secs = u64::deserialize_reader( reader)?;
       let nanos = u32::deserialize_reader(reader)?;
       let duration = Duration::new(secs,nanos);
         let time = UNIX_EPOCH.checked_add(duration)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Overflow deserializing SystemTime"))?;
            
        Ok(BorshSystemTime(time))

    }
    fn try_from_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Self::deserialize_reader(reader)
    }
}


#[derive(BorshSerialize,BorshDeserialize)]
struct LoyaltyPointsChangeType {
    change: i64,
    timestamp: BorshSystemTime,
}

#[derive(BorshSerialize,BorshDeserialize)]
struct LoyaltyPointsHistory {
    owner_pubkey: u32,
    current_points: u64,

    /// not being stored on chain nor on merkle tree.
    changes: Vec<LoyaltyPointsChangeType>,
}

fn connect_to_redis() -> redis::RedisResult<()> {
    // Create a Redis client from a URL string
    let client = Client::open("redis://127.0.0.1/")?;

    // Get a synchronous connection
    let mut con = client.get_connection()?;

    // Now you can run commands
    let _: () = con.set("my_key", 42)?;
    let value: isize = con.get("my_key")?;
    println!("Value retrieved: {}", value);

    Ok(())
}
fn main() {

    println!("Hello, world!");
    // connect to redis
    if let Err(e) = connect_to_redis() {
        eprintln!("Failed to connect or interact with Redis: {}", e);
    }
}
