#[cfg(test)]
mod tests {
    use bs58;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };
    use std::io::{self, stdin, BufRead};

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

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58: ");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is: ");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array: ");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is: ");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
