//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
#[derive(Debug)]
pub struct IncreaseOracleLength {
    pub oracle: solana_program::pubkey::Pubkey,

    pub funder: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl IncreaseOracleLength {
    pub fn instruction(
        &self,
        args: IncreaseOracleLengthInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: IncreaseOracleLengthInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.oracle,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = IncreaseOracleLengthInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseOracleLengthInstructionData {
    discriminator: [u8; 8],
}

impl IncreaseOracleLengthInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [190, 61, 125, 87, 103, 79, 158, 173],
        }
    }
}

impl Default for IncreaseOracleLengthInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseOracleLengthInstructionArgs {
    pub length_to_add: u64,
}

/// Instruction builder for `IncreaseOracleLength`.
///
/// ### Accounts:
///
///   0. `[writable]` oracle
///   1. `[writable, signer]` funder
///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   3. `[]` event_authority
///   4. `[]` program
#[derive(Clone, Debug, Default)]
pub struct IncreaseOracleLengthBuilder {
    oracle: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    length_to_add: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl IncreaseOracleLengthBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn length_to_add(&mut self, length_to_add: u64) -> &mut Self {
        self.length_to_add = Some(length_to_add);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = IncreaseOracleLength {
            oracle: self.oracle.expect("oracle is not set"),
            funder: self.funder.expect("funder is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = IncreaseOracleLengthInstructionArgs {
            length_to_add: self
                .length_to_add
                .clone()
                .expect("length_to_add is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `increase_oracle_length` CPI accounts.
pub struct IncreaseOracleLengthCpiAccounts<'a, 'b> {
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `increase_oracle_length` CPI instruction.
pub struct IncreaseOracleLengthCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: IncreaseOracleLengthInstructionArgs,
}

impl<'a, 'b> IncreaseOracleLengthCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: IncreaseOracleLengthCpiAccounts<'a, 'b>,
        args: IncreaseOracleLengthInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            oracle: accounts.oracle,
            funder: accounts.funder,
            system_program: accounts.system_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.oracle.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = IncreaseOracleLengthInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `IncreaseOracleLength` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` oracle
///   1. `[writable, signer]` funder
///   2. `[]` system_program
///   3. `[]` event_authority
///   4. `[]` program
#[derive(Clone, Debug)]
pub struct IncreaseOracleLengthCpiBuilder<'a, 'b> {
    instruction: Box<IncreaseOracleLengthCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> IncreaseOracleLengthCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(IncreaseOracleLengthCpiBuilderInstruction {
            __program: program,
            oracle: None,
            funder: None,
            system_program: None,
            event_authority: None,
            program: None,
            length_to_add: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }
    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn length_to_add(&mut self, length_to_add: u64) -> &mut Self {
        self.instruction.length_to_add = Some(length_to_add);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = IncreaseOracleLengthInstructionArgs {
            length_to_add: self
                .instruction
                .length_to_add
                .clone()
                .expect("length_to_add is not set"),
        };
        let instruction = IncreaseOracleLengthCpi {
            __program: self.instruction.__program,

            oracle: self.instruction.oracle.expect("oracle is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct IncreaseOracleLengthCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    length_to_add: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
