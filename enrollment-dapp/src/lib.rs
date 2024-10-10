#[cfg(test)]
mod tests {
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana Wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
        //Dev wallet - J5KD3VXQouUnuQwDJ7K7xzcyeYpWXTDqbdE69CyiHCu
    }

    #[test]
    fn airdrop() {}

    #[test]
    fn transfer_sol() {}
}
