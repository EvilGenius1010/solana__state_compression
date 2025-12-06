use std::time::SystemTime;


struct LoyaltyPointsChangeType{
    change:i64,
    timestamp:SystemTime
}

struct LoyaltyPointsHistory{
    owner_pubkey:u32,
    current_points:u64,

    /// not being stored on chain nor on merkle tree.
    changes:Vec<LoyaltyPointsChangeType>
}


fn main() {
    println!("Hello, world!");
}
