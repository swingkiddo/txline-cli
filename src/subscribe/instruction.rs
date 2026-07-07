use solana_sdk::instruction::{AccountMeta, Instruction};

use super::accounts;
use crate::config::Network;
use crate::types::SubscribeAccounts;

const SUBSCRIBE_DISCRIMINATOR: [u8; 8] = [254, 28, 191, 138, 156, 179, 183, 53];

pub fn build_subscribe_instruction(
    accounts: &SubscribeAccounts,
    network: &Network,
    service_level_id: u32,
    weeks: u32,
) -> Instruction {
    let pid = accounts::program_id(network);

    let mut data = Vec::with_capacity(11);
    data.extend_from_slice(&SUBSCRIBE_DISCRIMINATOR);
    data.extend_from_slice(&(service_level_id as u16).to_le_bytes());
    data.push(weeks as u8);

    Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new(accounts.user, true),
            AccountMeta::new(accounts.pricing_matrix, false),
            AccountMeta::new_readonly(accounts.token_mint, false),
            AccountMeta::new(accounts.user_token_account, false),
            AccountMeta::new(accounts.token_treasury_vault, false),
            AccountMeta::new(accounts.token_treasury_pda, false),
            AccountMeta::new_readonly(accounts.token_program, false),
            AccountMeta::new_readonly(accounts.system_program, false),
            AccountMeta::new_readonly(accounts.associated_token_program, false),
        ],
        data,
    }
}
