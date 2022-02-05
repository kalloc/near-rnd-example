use std::convert::TryInto;
use std::io::Bytes;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn rng(&self) -> u32 {
        let seed: [u8; 32] = env::random_seed().try_into().unwrap();
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        rng.gen_range(1, 10000)
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    use super::Contract;

    #[test]
    fn test_random() {
        let mut context = VMContextBuilder::new();
        context.block_index(1337);
        context.block_timestamp(1234567890);
        context.random_seed(vec![0u8; 32]);
        testing_env!(context.build());
        let contract = Contract {};
        println!("test: {}", contract.rng());
    }
}
