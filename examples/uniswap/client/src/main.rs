#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::{address, Address};
use alloy_sol_macro::sol;
use bincode;
use sp1_cc_client_executor::{io::EVMStateSketch, ClientExecutor, ContractInput};

sol! {
    /// Simplified interface of the IUniswapV3PoolState interface.
    interface IUniswapV3PoolState {
        function slot0(
        ) external view returns (uint160 sqrtPriceX96, int24 tick, uint16 observationIndex, uint16 observationCardinality, uint16 observationCardinalityNext, uint8 feeProtocol, bool unlocked);
    }
}

/// Address of Uniswap V3 pool.
const CONTRACT: Address = address!("1d42064Fc4Beb5F8aAF85F4617AE8b3b5B8Bd801");

/// Address of the caller.
const CALLER: Address = address!("0000000000000000000000000000000000000000");

pub fn main() {
    // Read the state sketch from stdin. We'll use this during the execution in order to
    // access Ethereum state.
    let state_sketch_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let state_sketch = bincode::deserialize::<EVMStateSketch>(&state_sketch_bytes).unwrap();

    // Commit the sketch's state root.
    sp1_zkvm::io::commit(&state_sketch.state_root());

    // Initialize the client executor with the state sketch.
    // This step also validates all of the storage against the provided state root.
    let executor = ClientExecutor::new(state_sketch).unwrap();

    // Execute the slot0 call using the client executor.
    let slot0_call = IUniswapV3PoolState::slot0Call {};
    let price_x96 = executor
        .execute(ContractInput {
            contract_address: CONTRACT,
            caller_address: CALLER,
            calldata: slot0_call,
        })
        .unwrap()
        .sqrtPriceX96;

    // Commit the result.
    sp1_zkvm::io::commit(&price_x96);
}