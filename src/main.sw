contract;

use std::storage::StorageVec;
use std::option::Option;

abi OCR {
    #[storage(read, write)]
    fn init();
    #[storage(read)]
    fn output() -> u64;
}

storage {
    // inputs: [u64; 3] = [120, 510, 210],
    // weights: [u64; 3] = [310, 210, 870],
    inputs: StorageVec<u64> = StorageVec {},
    weights: StorageVec<u64> = StorageVec {},
    bias: u64 = 3,
}

impl OCR for Contract {
    #[storage(read, write)]
    fn init() {
        storage.inputs.push(120);
        storage.inputs.push(510);
        storage.inputs.push(210);

        storage.weights.push(310);
        storage.weights.push(210);
        storage.weights.push(870);
    }

    #[storage(read)]
    fn output() -> u64 {
        ((storage.inputs.get(0).unwrap()*storage.weights.get(0).unwrap())/10000) +
        ((storage.inputs.get(1).unwrap()*storage.weights.get(1).unwrap())/10000) + 
        ((storage.inputs.get(2).unwrap()*storage.weights.get(2).unwrap())/10000) + 
        storage.bias
    }
}
