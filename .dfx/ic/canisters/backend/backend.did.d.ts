import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddRecordCommand {
  'tag' : string,
  'time' : bigint,
  't_type' : string,
  'comment' : string,
  'address' : string,
  'manual' : boolean,
  'principal_id' : [] | [string],
  'price' : number,
  'amount' : number,
}
export interface BallotInfo { 'vote' : number, 'proposal_id' : [] | [NeuronId] }
export type CustomResult1 = { 'Ok' : NeuronInfo } |
  { 'Err' : GovernanceError };
export interface EditHistoryCommand {
  'id' : bigint,
  'tag' : string,
  'time' : bigint,
  't_type' : string,
  'comment' : string,
  'manual' : boolean,
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
  'create_time' : bigint,
  'address' : string,
}
export interface NeuronUpdateCommand { 'id' : bigint, 'name' : string }
export interface RecordProfile {
  'id' : bigint,
  'tag' : string,
  'time' : bigint,
  't_type' : string,
  'comment' : string,
  'address' : string,
  'manual' : boolean,
  'principal_id' : [] | [string],
  'price' : number,
  'amount' : number,
}
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
export type Result_8 = { 'Ok' : Array<[string, Array<RecordProfile>]> } |
  { 'Err' : string };
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
  'add_transaction_record' : ActorMethod<[AddRecordCommand], Result_1>,
  'add_wallet' : ActorMethod<[WalletAddCommand], Result>,
  'auto_register_user' : ActorMethod<[], Result_2>,
  'delete_neuron_wallet' : ActorMethod<[bigint], Result>,
  'delete_transaction_record' : ActorMethod<[bigint], Result_1>,
  'delete_wallet' : ActorMethod<[bigint], Result>,
  'edit_transaction_record' : ActorMethod<[EditHistoryCommand], Result>,
  'get_balance' : ActorMethod<[], bigint>,
  'get_neuron_info' : ActorMethod<[bigint], Result_3>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_a_neuron_wallet' : ActorMethod<[bigint], Result_4>,
  'query_a_wallet' : ActorMethod<[bigint], Result_5>,
  'query_all_neuron_wallet' : ActorMethod<[], Result_6>,
  'query_all_wallets' : ActorMethod<[], Result_7>,
  'sync_transaction_record' : ActorMethod<
    [Array<[bigint, Array<RecordProfile>]>],
    Result
  >,
  'update_neuron_wallet' : ActorMethod<[NeuronUpdateCommand], Result>,
  'update_wallet' : ActorMethod<[WalletUpdateCommand], Result>,
  'user_quantity' : ActorMethod<[], number>,
  'wallet_history' : ActorMethod<[HistoryQueryCommand], Result_8>,
}
