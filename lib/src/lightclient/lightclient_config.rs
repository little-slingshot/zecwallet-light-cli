use std::{
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};

use log::{error, info, LevelFilter};
use log4rs::{
    append::rolling_file::{
        policy::compound::{roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy},
        RollingFileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use tokio::runtime::Runtime;
use zcash_primitives::{
    consensus::Network,
    constants::{mainnet, regtest, testnet},
};

use crate::{grpc_connector::GrpcConnector, lightclient::checkpoints};

pub const DEFAULT_SERVER: &str = "https://lwdv3.zecwallet.co";
pub const WALLET_NAME: &str = "zecwallet-light-wallet.dat";
pub const LOGFILE_NAME: &str = "zecwallet-light-wallet.debug.log";
pub const ANCHOR_OFFSET: [u32; 5] = [4, 0, 0, 0, 0];
pub const MAX_REORG: usize = 100;
pub const GAP_RULE_UNUSED_ADDRESSES: usize = if cfg!(any(target_os = "ios", target_os = "android")) {
    0
} else {
    5
};


#[derive(Clone, Debug)]
pub struct LightClientConfig {
    pub chain_name                  : String,
    pub sapling_activation_height   : u64,
    pub consensus_branch_id         : String,
    pub anchor_offset               : u32,
    pub data_dir                    : Option<String>
}

impl LightClientConfig {

    // Create an unconnected (to any server) config to test for local wallet etc...
    pub fn create_unconnected(chain_name: String, dir: Option<String>) -> LightClientConfig {
        LightClientConfig {
            chain_name                  : chain_name,
            sapling_activation_height   : 0,
            consensus_branch_id         : "".to_string(),
            anchor_offset               : ANCHOR_OFFSET,
            data_dir                    : dir,
        }
    }

    pub async fn create() -> io::Result<(LightClientConfig, u64)> {
        // Do a getinfo first, before opening the wallet
        let info = grpcconnector::get_info().await
            .map_err(|e| std::io::Error::new(ErrorKind::ConnectionRefused, e))?;

        // Create a Light Client Config
        let config = LightClientConfig {
            chain_name                  : info.chainName,
            sapling_activation_height   : info.saplingActivationHeight,
            consensus_branch_id         : info.consensusBranchId,
            anchor_offset               : ANCHOR_OFFSET,
            data_dir                    : None,
        };

        Ok((config, info.blockHeight))
    }


    pub fn get_initial_state(&self, height: u64) -> Option<(u64, &str, &str)> {
        checkpoints::get_closest_checkpoint(&self.chain_name, height)
    }

    pub fn get_coin_type(&self) -> u32 {
        match &self.chain_name[..] {
            "main"    => mainnet::COIN_TYPE,
            "test"    => testnet::COIN_TYPE,
            "regtest" => regtest::COIN_TYPE,
            c         => panic!("Unknown chain {}", c)
        }
    }

    pub fn hrp_sapling_address(&self) -> &str {
        match &self.chain_name[..] {
            "main"    => mainnet::HRP_SAPLING_PAYMENT_ADDRESS,
            "test"    => testnet::HRP_SAPLING_PAYMENT_ADDRESS,
            "regtest" => regtest::HRP_SAPLING_PAYMENT_ADDRESS,
            c         => panic!("Unknown chain {}", c)
        }
    }

    pub fn hrp_sapling_private_key(&self) -> &str {
        match &self.chain_name[..] {
            "main"    => mainnet::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            "test"    => testnet::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            "regtest" => regtest::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            c         => panic!("Unknown chain {}", c)
        }
    }

    pub fn base58_pubkey_address(&self) -> [u8; 2] {
        match &self.chain_name[..] {
            "main"    => mainnet::B58_PUBKEY_ADDRESS_PREFIX,
            "test"    => testnet::B58_PUBKEY_ADDRESS_PREFIX,
            "regtest" => regtest::B58_PUBKEY_ADDRESS_PREFIX,
            c         => panic!("Unknown chain {}", c)
        }
    }


    pub fn base58_script_address(&self) -> [u8; 2] {
        match &self.chain_name[..] {
            "main"    => mainnet::B58_SCRIPT_ADDRESS_PREFIX,
            "test"    => testnet::B58_SCRIPT_ADDRESS_PREFIX,
            "regtest" => regtest::B58_SCRIPT_ADDRESS_PREFIX,
            c         => panic!("Unknown chain {}", c)
        }
    }

    pub fn base58_secretkey_prefix(&self) -> [u8; 1] {
        match &self.chain_name[..] {
            "main"    => [0x80],
            "test"    => [0xEF],
            "regtest" => [0xEF],
            c         => panic!("Unknown chain {}", c)
        }
    }
}
