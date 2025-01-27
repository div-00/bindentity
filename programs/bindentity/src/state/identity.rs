use anchor_lang::{prelude::*, solana_program::hash::hashv};

/// An Identity is an account that proves the association of the given data to its owner's wallet account.
/// An Identity is considered void if the corresponding Link account is closed.
#[account]
pub struct Identity {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// The hash of any data, such as phone number or email. (32)
    pub data: [u8; 32],

    /// Owner of this identity. (32)
    pub owner: Pubkey,

    /// The authority who issued this identity. (32)
    pub provider: Pubkey,

    /// Unix timestamp. Besides from acting as a seed, useful to get the identities' creation order. (4)
    pub timestamp: u64,
}

impl Identity {
    pub fn len() -> usize {
        8 + 1 + 32 + 32 + 32 + 8
    }

    pub fn data_hash(provider_name: &String, id: &Vec<u8>) -> [u8; 32] {
        hashv(&[provider_name.as_bytes(), ":".as_bytes(), id.as_ref()])
            .to_bytes()
            .try_into()
            .unwrap()
    }
}
