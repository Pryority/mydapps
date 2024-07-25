// client/init.rs

use crate::{error::Result, AppError};
use helios::client::{Client as HeliosClient, FileDB};
use helios::config::networks::Network;
use helios::{
    client::ClientBuilder,
    errors::{BlockNotFoundError, RpcError, SlotNotFoundError},
};
use std::path::PathBuf;
use tracing::info;

pub(super) fn init_client() -> Result<HeliosClient<FileDB>> {
    let untrusted_rpc_url = std::env::var("EXECUTION")
        .expect("Please set the 'EXECUTION' environment variable with the RPC URL");
    info!("Using untrusted RPC URL [REDACTED]");

    let consensus_rpc = "https://www.lightclientdata.org";
    info!("Using consensus RPC URL: {}", consensus_rpc);

    let client = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc(consensus_rpc)
        .execution_rpc(&untrusted_rpc_url)
        .load_external_fallback()
        .data_dir(PathBuf::from("/tmp/helios"))
        .build()
        .map_err(|err| {
            eprintln!("Error building Helios client: {}", err);
            if err.is::<BlockNotFoundError>() {
                AppError::BlockNotFound(err.downcast::<BlockNotFoundError>().unwrap())
            } else if err.is::<SlotNotFoundError>() {
                AppError::SlotNotFound(err.downcast::<SlotNotFoundError>().unwrap())
            } else if err.is::<RpcError<String>>() {
                AppError::RpcError(err.to_string())
            } else {
                AppError::Unknown(err.to_string())
            }
        })?;

    Ok(client)
}
