// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct InitArgs { pub ledger_id: Principal }

pub type TxId = candid::Nat;
#[derive(CandidType, Deserialize)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct GetAccountTransactionsArgs {
  pub max_results: candid::Nat,
  pub start: Option<TxId>,
  pub account: Account,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionBurnInner {
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionMintInner {
  pub to: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionApproveInner {
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<u64>,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionTransferInner {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
  pub burn: Option<TransactionBurnInner>,
  pub kind: String,
  pub mint: Option<TransactionMintInner>,
  pub approve: Option<TransactionApproveInner>,
  pub timestamp: u64,
  pub transfer: Option<TransactionTransferInner>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionWithId { pub id: TxId, pub transaction: Transaction }

#[derive(CandidType, Deserialize)]
pub struct GetTransactions {
  pub transactions: Vec<TransactionWithId>,
  pub oldest_tx_id: Option<TxId>,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsErr { pub message: String }

#[derive(CandidType, Deserialize)]
pub enum GetTransactionsResult { Ok(GetTransactions), Err(GetTransactionsErr) }

pub type SubAccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct ListSubaccountsArgs {
  pub owner: Principal,
  pub start: Option<SubAccount>,
}

pub struct Service(pub Principal);
impl Service {
  pub async fn get_account_transactions(
    &self,
    arg0: GetAccountTransactionsArgs,
  ) -> Result<(GetTransactionsResult,)> {
    ic_cdk::call(self.0, "get_account_transactions", (arg0,)).await
  }
  pub async fn ledger_id(&self) -> Result<(Principal,)> {
    ic_cdk::call(self.0, "ledger_id", ()).await
  }
  pub async fn list_subaccounts(&self, arg0: ListSubaccountsArgs) -> Result<
    (Vec<SubAccount>,)
  > { ic_cdk::call(self.0, "list_subaccounts", (arg0,)).await }
}

use ic_cdk::{query, update};

#[update]
async fn get_account_transactions() -> Result<GetTransactionsResult> {
    let oc_ledger = Principal::from_text("2awyi-oyaaa-aaaaq-aaanq-cai").unwrap();
    let pid = oc_ledger;
    let service = Service(pid);
    let arg0 = GetAccountTransactionsArgs {
        max_results: candid::Nat::from(10),
        start: None,
        account: Account {
            owner: Principal::from_text("xhwxj-taaaa-aaaak-aeh7a-cai").unwrap(),
            subaccount: None,
        },
    };
    let ret = service.get_account_transactions(arg0).await;
    return ret.map(|(result,)| result);
}

