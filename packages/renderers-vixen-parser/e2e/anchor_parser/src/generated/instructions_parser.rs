//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use codama_renderers_rust_e2e_anchor::instructions::{
    CreateGuard as CreateGuardIxAccounts, CreateGuardInstructionArgs as CreateGuardIxData,
    Execute as ExecuteIxAccounts, ExecuteInstructionArgs as ExecuteIxData,
    Initialize as InitializeIxAccounts, UpdateGuard as UpdateGuardIxAccounts,
    UpdateGuardInstructionArgs as UpdateGuardIxData,
};
use codama_renderers_rust_e2e_anchor::ID;

/// WenTransferGuard Instructions
#[derive(Debug)]
pub enum WenTransferGuardProgramIx {
    CreateGuard(CreateGuardIxAccounts, CreateGuardIxData),
    Execute(ExecuteIxAccounts, ExecuteIxData),
    Initialize(InitializeIxAccounts),
    UpdateGuard(UpdateGuardIxAccounts, UpdateGuardIxData),
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = WenTransferGuardProgramIx;

    fn id(&self) -> std::borrow::Cow<str> {
        "WenTransferGuard::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        if ix_update.program.equals_ref(ID) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl yellowstone_vixen_core::ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        ID.to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<WenTransferGuardProgramIx> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..8].try_into()?;
        let mut ix_data = &ix.data[8..];
        match ix_discriminator {
            [251, 254, 17, 198, 219, 218, 154, 99] => {
                check_min_accounts_req(accounts_len, 8)?;
                let ix_accounts = CreateGuardIxAccounts {
                    guard: ix.accounts[0].0.into(),
                    mint: ix.accounts[1].0.into(),
                    mint_token_account: ix.accounts[2].0.into(),
                    guard_authority: ix.accounts[3].0.into(),
                    payer: ix.accounts[4].0.into(),
                    associated_token_program: ix.accounts[5].0.into(),
                    token_program: ix.accounts[6].0.into(),
                    system_program: ix.accounts[7].0.into(),
                };
                let de_ix_data: CreateGuardIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(WenTransferGuardProgramIx::CreateGuard(
                    ix_accounts,
                    de_ix_data,
                ))
            }
            [105, 37, 101, 197, 75, 251, 102, 26] => {
                check_min_accounts_req(accounts_len, 7)?;
                let ix_accounts = ExecuteIxAccounts {
                    source_account: ix.accounts[0].0.into(),
                    mint: ix.accounts[1].0.into(),
                    destination_account: ix.accounts[2].0.into(),
                    owner_delegate: ix.accounts[3].0.into(),
                    extra_metas_account: ix.accounts[4].0.into(),
                    guard: ix.accounts[5].0.into(),
                    instruction_sysvar_account: ix.accounts[6].0.into(),
                };
                let de_ix_data: ExecuteIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(WenTransferGuardProgramIx::Execute(ix_accounts, de_ix_data))
            }
            [43, 34, 13, 49, 167, 88, 235, 235] => {
                check_min_accounts_req(accounts_len, 6)?;
                let ix_accounts = InitializeIxAccounts {
                    extra_metas_account: ix.accounts[0].0.into(),
                    guard: ix.accounts[1].0.into(),
                    mint: ix.accounts[2].0.into(),
                    transfer_hook_authority: ix.accounts[3].0.into(),
                    system_program: ix.accounts[4].0.into(),
                    payer: ix.accounts[5].0.into(),
                };
                Ok(WenTransferGuardProgramIx::Initialize(ix_accounts))
            }
            [51, 38, 175, 180, 25, 249, 39, 24] => {
                check_min_accounts_req(accounts_len, 6)?;
                let ix_accounts = UpdateGuardIxAccounts {
                    guard: ix.accounts[0].0.into(),
                    mint: ix.accounts[1].0.into(),
                    token_account: ix.accounts[2].0.into(),
                    guard_authority: ix.accounts[3].0.into(),
                    token_program: ix.accounts[4].0.into(),
                    system_program: ix.accounts[5].0.into(),
                };
                let de_ix_data: UpdateGuardIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(WenTransferGuardProgramIx::UpdateGuard(
                    ix_accounts,
                    de_ix_data,
                ))
            }
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        }
    }
}

pub fn check_min_accounts_req(
    actual: usize,
    expected: usize,
) -> yellowstone_vixen_core::ParseResult<()> {
    if actual < expected {
        Err(yellowstone_vixen_core::ParseError::from(format!(
            "Too few accounts provided: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}
