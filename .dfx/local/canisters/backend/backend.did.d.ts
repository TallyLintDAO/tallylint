import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface BallotInfo { 'vote' : number, 'proposal_id' : [] | [NeuronId] }
export interface Currency { 'decimals' : bigint, 'symbol' : string }
export type CustomResult1 = { 'Ok' : NeuronInfo } |
  { 'Err' : GovernanceError };
export interface Details {
  'to' : string,
  'fee' : number,
  'status' : string,
  'ledgerCanisterId' : string,
  'value' : number,
  'cost' : number,
  'from' : string,
  'currency' : Currency,
  'profit' : number,
  'price' : number,
  'amount' : number,
}
export interface GovernanceError {
  'error_message' : string,
  'error_type' : number,
}
export interface HistoryQueryCommand {
  'tag' : string,
  'from_time' : bigint,
  'to_time' : bigint,
  't_type' : string,
  'sort_method' : string,
  'address' : [] | [string],
}
export interface KnownNeuronData {
  'name' : string,
  'description' : [] | [string],
}
export interface NeuronAddCommand {
  'from' : string,
  'name' : string,
  'address' : string,
}
export interface NeuronId { 'id' : Uint8Array | number[] }
export interface NeuronInfo {
  'dissolve_delay_seconds' : bigint,
  'recent_ballots' : Array<BallotInfo>,
  'created_timestamp_seconds' : bigint,
  'state' : number,
  'stake_e8s' : bigint,
  'joined_community_fund_timestamp_seconds' : [] | [bigint],
  'retrieved_at_timestamp_seconds' : bigint,
  'known_neuron_data' : [] | [KnownNeuronData],
  'voting_power' : bigint,
  'age_seconds' : bigint,
}
export interface NeuronProfile {
  'id' : bigint,
  'owner' : Principal,
  'name' : string,
  'update_time' : bigint,
  'create_time' : bigint,
  'address' : string,
}
export interface NeuronUpdateCommand { 'id' : bigint, 'name' : string }
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : [CustomResult1] } |
  { 'Err' : [RejectionCode, string] };
export type Result_4 = { 'Ok' : NeuronProfile } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : WalletProfile } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<NeuronProfile> } |
  { 'Err' : Array<NeuronProfile> };
export type Result_7 = { 'Ok' : Array<WalletProfile> } |
  { 'Err' : Array<WalletProfile> };
export type Result_8 = { 'Ok' : Array<[string, Array<TransactionB>]> } |
  { 'Err' : string };
export interface SyncTransactionCommand {
  'history' : Array<TransactionF>,
  'walletId' : bigint,
}
export interface TransactionB {
  'id' : bigint,
  'tag' : string,
  'memo' : string,
  'comment' : string,
  'income' : number,
  'address' : string,
  'manual' : boolean,
  'principal_id' : [] | [string],
  'transaction_f' : TransactionF,
}
export interface TransactionF {
  'hash' : string,
  'walletName' : string,
  't_type' : string,
  'timestamp' : number,
  'details' : Details,
}
export interface UserProfile {
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
}
export interface WalletAddCommand {
  'from' : string,
  'name' : string,
  'address' : string,
  'principal_id' : [] | [string],
}
export interface WalletProfile {
  'id' : bigint,
  'last_transaction_time' : bigint,
  'last_sync_time' : bigint,
  'from' : string,
  'name' : string,
  'create_time' : bigint,
  'address' : string,
  'principal_id' : [] | [string],
  'holder' : Principal,
  'transactions' : bigint,
}
export interface WalletUpdateCommand {
  'id' : bigint,
  'from' : string,
  'name' : string,
}
export interface _SERVICE {
  'add_neuron_wallet' : ActorMethod<[NeuronAddCommand], Result>,
  'add_transaction' : ActorMethod<[TransactionB], Result_1>,
  'add_wallet' : ActorMethod<[WalletAddCommand], Result>,
  'auto_register_user' : ActorMethod<[], Result_2>,
  'clean_db' : ActorMethod<[], boolean>,
  'collect_running_payload' : ActorMethod<[], string>,
  'delete_neuron_wallet' : ActorMethod<[bigint], Result>,
  'delete_transaction' : ActorMethod<[bigint], Result_1>,
  'delete_wallet' : ActorMethod<[bigint], Result>,
  'do_pre_upgrade_and_print_db' : ActorMethod<[], string>,
  'get_balance' : ActorMethod<[], bigint>,
  'get_neuron_info' : ActorMethod<[bigint], Result_3>,
  'get_payload_from_dropbox' : ActorMethod<[string, string], string>,
  'get_payload_from_stable_mem' : ActorMethod<[], string>,
  'get_payload_from_stable_mem_simple' : ActorMethod<[], string>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_a_neuron_wallet' : ActorMethod<[bigint], Result_4>,
  'query_a_wallet' : ActorMethod<[bigint], Result_5>,
  'query_all_neuron_wallet' : ActorMethod<[], Result_6>,
  'query_all_wallets' : ActorMethod<[], Result_7>,
  'save_payload_to_dropbox' : ActorMethod<[string, number, bigint], string>,
  'set_payload_using_dropbox' : ActorMethod<[string, string], boolean>,
  'set_payload_using_stable_mem' : ActorMethod<[], undefined>,
  'set_stable_mem_use_payload' : ActorMethod<[], undefined>,
  'set_stable_mem_use_payload_simple' : ActorMethod<[], undefined>,
  'sync_transaction_record' : ActorMethod<
    [Array<SyncTransactionCommand>],
    Result
  >,
  'update_neuron_wallet' : ActorMethod<[NeuronUpdateCommand], Result>,
  'update_transaction' : ActorMethod<[TransactionB], Result>,
  'update_wallet' : ActorMethod<[WalletUpdateCommand], Result>,
  'user_quantity' : ActorMethod<[], number>,
  'wallet_history' : ActorMethod<[HistoryQueryCommand], Result_8>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
