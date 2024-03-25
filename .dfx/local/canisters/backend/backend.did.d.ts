import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface BallotInfo { 'vote' : number, 'proposal_id' : [] | [NeuronId] }
export interface CanisterContext {
  'id' : bigint,
  'trans_f_srv' : TransactionService,
  'wallet_transc_srv' : WalletRecordService,
  'wallet_service' : WalletService,
  'neuron_service' : NeuronService,
  'user_service' : UserService,
}
export interface Currency { 'decimals' : number, 'symbol' : string }
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
  'from_time' : bigint,
  'to_time' : bigint,
  'sort_method' : [] | [string],
  'address' : Array<string>,
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
export interface NeuronService { 'neurons' : Array<[string, NeuronProfile]> }
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
export type Result_7 = { 'Ok' : Array<[bigint, TransactionB]> } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : Array<WalletProfile> } |
  { 'Err' : Array<WalletProfile> };
export type Result_9 = { 'Ok' : TransactionB } |
  { 'Err' : string };
export interface SimpleTransaction {
  'id' : bigint,
  'tag' : Array<string>,
  'hash' : string,
  't_type' : string,
  'comment' : string,
  'timestamp' : bigint,
  'details' : Details,
  'manual' : boolean,
}
export interface SyncTransactionCommand {
  'history' : Array<TransactionF>,
  'walletId' : bigint,
}
export interface TransactionB {
  'id' : bigint,
  'tag' : Array<string>,
  'hash' : string,
  'memo' : string,
  't_type' : string,
  'comment' : string,
  'address' : string,
  'timestamp' : bigint,
  'details' : Details,
  'manual' : boolean,
}
export interface TransactionF {
  'hash' : string,
  't_type' : string,
  'timestamp' : number,
  'details' : Details,
}
export interface TransactionService {
  'transactions' : Array<[bigint, TransactionF]>,
}
export interface UserConfig { 'tax_method' : string }
export interface UserProfile {
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
}
export interface UserService {
  'configs' : Array<[string, UserConfig]>,
  'users' : Array<[Principal, UserProfile]>,
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
export interface WalletRecordService {
  'records' : Array<[bigint, TransactionB]>,
}
export interface WalletService { 'wallets' : Array<[bigint, WalletProfile]> }
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
  'calculate_tax' : ActorMethod<[Array<string>, string, Array<string>], string>,
  'clean_db' : ActorMethod<[], boolean>,
  'collect_running_payload_simple' : ActorMethod<[], string>,
  'collect_running_payload_simple_raw' : ActorMethod<[], string>,
  'delete_neuron_wallet' : ActorMethod<[bigint], Result>,
  'delete_transaction' : ActorMethod<[bigint], Result_1>,
  'delete_wallet' : ActorMethod<[bigint], Result>,
  'get_balance' : ActorMethod<[], bigint>,
  'get_neuron_info' : ActorMethod<[bigint], Result_3>,
  'get_payload_from_stable_mem_simple' : ActorMethod<[], string>,
  'get_payload_from_stable_mem_simple_raw' : ActorMethod<[], CanisterContext>,
  'get_user_config' : ActorMethod<[], UserConfig>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_a_neuron_wallet' : ActorMethod<[bigint], Result_4>,
  'query_a_wallet' : ActorMethod<[bigint], Result_5>,
  'query_all_neuron_wallet' : ActorMethod<[], Result_6>,
  'query_all_transactions' : ActorMethod<[], Result_7>,
  'query_all_wallets' : ActorMethod<[], Result_8>,
  'query_one_transaction' : ActorMethod<[bigint], Result_9>,
  'query_wallets_synced_transactions' : ActorMethod<
    [HistoryQueryCommand],
    Array<SimpleTransaction>
  >,
  'set_payload_using_dev_machine_file' : ActorMethod<[string], string>,
  'set_payload_using_stable_mem_simple' : ActorMethod<[], string>,
  'set_payload_using_stable_mem_simple_raw' : ActorMethod<[], undefined>,
  'set_stable_mem_using_payload_simple' : ActorMethod<[], undefined>,
  'set_stable_mem_using_payload_simple_raw' : ActorMethod<[], undefined>,
  'set_user_config' : ActorMethod<[UserConfig], UserConfig>,
  'sync_transaction_record' : ActorMethod<
    [Array<SyncTransactionCommand>],
    Result
  >,
  'update_neuron_wallet' : ActorMethod<[NeuronUpdateCommand], Result>,
  'update_transaction' : ActorMethod<[TransactionB], Result>,
  'update_wallet' : ActorMethod<[WalletUpdateCommand], Result>,
  'user_quantity' : ActorMethod<[], number>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
