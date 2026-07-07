use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;

use crate::config::Network;
use crate::types::SubscribeAccounts;

const TOKEN_2022: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const ASSOCIATED_TOKEN: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xr25e9TcAaCnjTXHp1";
const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";

pub fn program_id(network: &Network) -> Pubkey {
    match network {
        Network::Mainnet => {
            Pubkey::from_str("9ExbZjAapQww1vfcisDmrngPinHTEfpjYRWMunJgcKaA").unwrap()
        }
        Network::Devnet => {
            Pubkey::from_str("6pW64gN1s2uqjHkn1unFeEjAwJkPGHoppGvS715wyP2J").unwrap()
        }
    }
}

pub fn token_mint(network: &Network) -> Pubkey {
    match network {
        Network::Mainnet => {
            Pubkey::from_str("Zhw9TVKp68a1QrftncMSd6ELXKDtpVMNuMGr1jNwdeL").unwrap()
        }
        Network::Devnet => {
            Pubkey::from_str("4Zao8ocPhmMgq7PdsYWyxvqySMGx7xb9cMftPMkEokRG").unwrap()
        }
    }
}

fn find_associated_token_address(
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    associated_token_program: &Pubkey,
) -> Pubkey {
    let seeds = &[owner.as_ref(), token_program.as_ref(), mint.as_ref()];
    Pubkey::find_program_address(seeds, associated_token_program).0
}

pub fn derive_subscribe_accounts(
    user_pubkey: &Pubkey,
    network: &Network,
) -> SubscribeAccounts {
    let pid = program_id(network);
    let mint = token_mint(network);
    let token_prog = Pubkey::from_str(TOKEN_2022).unwrap();
    let atp = Pubkey::from_str(ASSOCIATED_TOKEN).unwrap();

    let (token_treasury_pda, _) =
        Pubkey::find_program_address(&[b"token_treasury_v2"], &pid);
    let token_treasury_vault =
        find_associated_token_address(&token_treasury_pda, &mint, &token_prog, &atp);
    let (pricing_matrix, _) =
        Pubkey::find_program_address(&[b"pricing_matrix"], &pid);
    let user_token_account =
        find_associated_token_address(user_pubkey, &mint, &token_prog, &atp);

    SubscribeAccounts {
        user: *user_pubkey,
        pricing_matrix,
        token_mint: mint,
        user_token_account,
        token_treasury_vault,
        token_treasury_pda,
        token_program: token_prog,
        system_program: Pubkey::from_str(SYSTEM_PROGRAM).unwrap(),
        associated_token_program: atp,
    }
}
