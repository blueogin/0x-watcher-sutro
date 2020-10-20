//! Chain instance handling JSON-RPC requests.

use web3::types::CallRequest;

#[derive(Clone, Default, Debug)]
pub struct Server {}

impl Server {
    pub fn chain_id(&self) -> u64 {
        1
    }

    pub fn block_number(&self) -> u64 {
        1
    }

    /// Process transaction, mine new block and return transaction hash.
    // TODO: Async processing.
    fn transact(&mut self, transaction: CallRequest) -> [u8; 32] {
        [5; 32]
    }
}