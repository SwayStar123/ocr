use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/ocr-abi.json");

async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(Some(1), Some(1), Some(1_000_000)),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/ocr.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/ocr-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContractBuilder::new(id.to_string(), wallet).build();

    (instance, id.into())
}

#[tokio::test]
async fn can_get_output() {
    let (instance, _id) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
    instance.init().call().await.unwrap();

    let res = instance.output().call().await.unwrap().value;
    println!("{}", res);

    assert_eq!(res, 5);
}
