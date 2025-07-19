use serde::Serialize;
use sha2::{Sha256, Digest};
use bincode;


#[derive(Debug, Clone, Serialize)]
struct PoHEntry{
    hash: Vec<u8>,
    counter: u64,
    transaction:Option<String>,
}
#[derive(Clone)]
struct PoofOfHistory{
    entries: Vec<PoHEntry>,
}

impl PoofOfHistory {
    fn new() -> Self {
        let mut hasher=Sha256::new();
         hasher.update(b"genesis"); 
         let initial_hash= hasher.finalize().to_vec();
        PoofOfHistory {
            entries: vec![PoHEntry {
                hash: initial_hash,
                counter: 0,
                transaction: None,
            }],
        }
    }
    
    fn append(&mut self, transaction: Option<String>) {
      let last_entry= self.entries.last().unwrap();
        let mut hasher = Sha256::new();
       hasher.update(&last_entry.hash);
       if let Some(tx)= &transaction {
            hasher.update(tx.as_bytes());
       }

       let new_hash = hasher.finalize().to_vec();
       self.entries.push(PoHEntry {
            hash: new_hash,
            counter: last_entry.counter + 1,
            transaction,
        });
    }

    fn verify(&self) -> bool{
        for i in 1..self.entries.len(){
            let prev_entry=&self.entries[i-1];
            let current_entry=&self.entries[i];
            let mut hasher=Sha256::new();
             let serialized = bincode::serialize(&prev_entry).unwrap();
             hasher.update(&serialized);


            if let Some(tx)=&current_entry.transaction{
                hasher.update(tx.as_bytes());
            }

            let computed_hash=hasher.finalize().to_vec();

            if computed_hash != current_entry.hash {
                return false;
            }
        }
        true
    }

    fn display(&self){
         for entry in &self.entries{
            let hash_hex = entry.hash.iter().map(|b| format!("{:02x}",b)).collect::<String>();
             let tx = entry.transaction.as_ref().map_or("None".to_string(), |t| t.clone());
            println!("Counter: {}, Hash: {}, Transaction: {}", entry.counter, &hash_hex[..8], tx);
         }
    }
}
fn main() {
    let mut poh=PoofOfHistory::new();

    poh.append(None);
    poh.append(Some("Transfer 1 SOL Alice -> Bob".to_string()));
     poh.append(None); 
    poh.append(Some("Mint NFT #123".to_string()));
    poh.append(None); 

     println!("PoH Chain:");
    poh.display();

    // Simulate tampering (to demonstrate verification failure)
    let mut tampered_poh = poh.clone();
    tampered_poh.entries[2].hash[0] ^= 0xff; 
    println!("\nTampered PoH Chain Verification:");
    let is_tampered_valid = tampered_poh.verify();
    println!("Verification result: {}", if is_tampered_valid { "Valid" } else { "Invalid" });
}
