use {
    crate::instruction::try_from_slice_checked,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey},
};

pub const PREFIX: &str = "vault";
// struct Items {
//     IERC20 token;
//     address withdrawer;
//     uint256 amount;
//     uint256 unlockTimestamp;
//     bool withdrawn;
//     bool deposited;
// }

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub enum VaultState {
    Activate,
    Inactive,
}

//  trait Address  {
//     type EthAdress  ;
//     type SolAdress ;
// }

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd)]
pub enum Address {
    EthAddress(String), //0x erc20 => solana => solang
    SolAddress(Pubkey), //solang erc20 -> solana
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd)]

pub struct ERC20 {
    pub EthERC: Address, //string 0x
    pub SolERC: Address, // pubkey => adress
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd)]

pub struct Owner {
    pub Ethowner: Address, // metamask
    pub SolOwner: Address, //pubkey
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize)]

pub struct Vault {
    pub Erc20: ERC20,
    pub withdrawer: Owner,
    pub amount: i32,
    pub lock: bool,
    pub limit: i32,
    pub id: i32,
}

impl Vault {
    pub fn from_account_info(a: &AccountInfo) -> Result<Vault, ProgramError> {
        let vt: Vault = try_from_slice_checked(&a.data.borrow_mut(), 1)?;

        Ok(vt)
    }

    pub fn get_token_type_count(a: &AccountInfo) -> u8 {
        return a.data.borrow()[194];
    }
}
