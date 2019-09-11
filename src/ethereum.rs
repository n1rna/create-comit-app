use testcontainers::{self, Docker};
use web3::{
    api::Web3,
    futures::Future,
    transports::Http,
    types::{TransactionRequest, U256},
};

pub fn start_ethereum_node() -> u32 {
    let docker = testcontainers::clients::Cli::default();
    let container = docker.run(testcontainers::images::parity_parity::ParityEthereum::default());

    let port = container.get_host_port(8545).unwrap();
    // let endpoint = format!("http://localhost:{}", port);

    // let (_event_loop, transport) = Http::new(&endpoint).unwrap();
    // let client = Web3::new(transport);

    // // Fund taker address

    // let parity_dev_account: web3::types::Address =
    //     "00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap();

    // // FIXME: Derive address from seed
    // let taker_address: web3::types::Address =
    //     "458968726a444a90fda1edc082129c661d39c7ff".parse().unwrap();
    // let wei_amount = U256::from_dec_str("200000000000000000000").unwrap();

    // client
    //     .personal()
    //     .send_transaction(
    //         TransactionRequest {
    //             from: parity_dev_account,
    //             to: Some(taker_address),
    //             gas: None,
    //             gas_price: None,
    //             value: Some(wei_amount),
    //             data: None,
    //             nonce: None,
    //             condition: None,
    //         },
    //         "",
    //     )
    //     .wait()
    //     .unwrap();

    port
}

#[cfg(test)]
mod tests {
    use super::*;
    use web3::types::{BlockId, BlockNumber, U128};

    #[test]
    fn got_port() {
        let port = start_ethereum_node();
        let endpoint = format!("http://localhost:{}", port);

        let (_event_loop, transport) = Http::new(&endpoint).unwrap();
        let client = Web3::new(transport);

        let _ = client
            .eth()
            .block(BlockId::Number(BlockNumber::from(0)))
            .map(|block| assert_eq!(block.unwrap().number, Some(U128::from(0))))
            .map_err(|_| panic!());
    }
}
