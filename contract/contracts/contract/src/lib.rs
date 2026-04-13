#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, String, Vec};

#[contract]
pub struct TravelNFTContract;

#[contractimpl]
impl TravelNFTContract {
    // Mint a Travel NFT
    pub fn mint_nft(env: Env, owner: String, location: String, metadata: String) {
        let key = symbol_short!("NFT");

        let mut nfts: Vec<(String, String, String)> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        nfts.push_back((owner, location, metadata));

        env.storage().instance().set(&key, &nfts);
    }

    // Get all NFTs
    pub fn get_nfts(env: Env) -> Vec<(String, String, String)> {
        let key = symbol_short!("NFT");

        env.storage().instance().get(&key).unwrap_or(Vec::new(&env))
    }
}