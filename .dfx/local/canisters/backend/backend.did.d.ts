import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AddRecordCommand {
  'to' : string,
  'fee' : number,
  'tag' : string,
  'status' : string,
  'cost' : number,
  'from' : string,
  'hash' : string,
  'memo' : string,
  'time' : bigint,
  't_type' : string,
  'coin_type' : string,
  'comment' : string,
  'income' : number,
  'address' : string,
  'profit' : number,
  'manual' : boolean,
  'principal_id' : [] | [string],
  'price' : number,
  'amount' : number,
}
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
export interface EditHistoryCommand {
  'id' : bigint,
  'to' : string,
  'fee' : number,
  'tag' : string,
  'status' : string,
  'cost' : number,
  'from' : string,
  'hash' : string,
  'memo' : string,
  'time' : bigint,
  't_type' : string,
  'coin_type' : string,
  'comment' : string,
  'income' : number,
  'address' : string,
  'profit' : number,
  'manual' : boolean,
  'principal_id' : [] | [string],
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
export type HttpFailureReason = { 'ProxyError' : string } |
  { 'RequestTimeout' : null };
export interface HttpHeader { 'value' : string, 'name' : string }
export type HttpMethod = { 'GET' : null } |
  { 'PUT' : null } |
  { 'DELETE' : null } |
  { 'HEAD' : null } |
  { 'POST' : null };
export type HttpOverWsError = { 'NotHttpOverWsType' : string } |
  { 'ProxyNotFound' : null } |
  { 'NotYetReceived' : null } |
  { 'ConnectionNotAssignedToProxy' : null } |
  { 'RequestIdNotFound' : null } |
  { 'NoProxiesConnected' : null } |
  { 'InvalidHttpMessage' : null } |
  { 'RequestFailed' : HttpFailureReason };
export interface HttpRequest {
  'url' : string,
  'method' : HttpMethod,
  'body' : [] | [Uint8Array | number[]],
  'headers' : Array<HttpHeader>,
}
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export type HttpResult = { 'Success' : HttpResponse } |
  { 'Failure' : HttpFailureReason };
export type InvalidRequest = { 'TooManyHeaders' : null } |
  { 'InvalidTimeout' : null } |
  { 'InvalidUrl' : string };
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
export type ProxyCanisterError = { 'HttpOverWs' : HttpOverWsError } |
  { 'InvalidRequest' : InvalidRequest };
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
export type Result_4 = { 'Ok' : bigint } |
  { 'Err' : ProxyCanisterError };
export type Result_5 = { 'Ok' : NeuronProfile } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : WalletProfile } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : Array<NeuronProfile> } |
  { 'Err' : Array<NeuronProfile> };
export type Result_8 = { 'Ok' : Array<WalletProfile> } |
  { 'Err' : Array<WalletProfile> };
export type Result_9 = { 'Ok' : Array<[string, Array<TransactionB>]> } |
  { 'Err' : string };
export interface TransactionB {
  'id' : bigint,
  'to' : string,
  'fee' : number,
  'tag' : string,
  'status' : string,
  'cost' : number,
  'from' : string,
  'hash' : string,
  'memo' : string,
  't_type' : string,
  'coin_type' : string,
  'comment' : string,
  'income' : number,
  'address' : string,
  'timestamp' : bigint,
  'profit' : number,
  'manual' : boolean,
  'principal_id' : [] | [string],
  'price' : number,
  'amount' : number,
}
export interface TransactionF {
  'hash' : string,
  'walletName' : string,
  't_type' : string,
  'timestamp' : bigint,
  'details' : Details,
}
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
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
  'add_transaction_record' : ActorMethod<[AddRecordCommand], Result_1>,
  'add_wallet' : ActorMethod<[WalletAddCommand], Result>,
  'auto_register_user' : ActorMethod<[], Result_2>,
  'clean_db' : ActorMethod<[], string>,
  'delete_neuron_wallet' : ActorMethod<[bigint], Result>,
  'delete_transaction_record' : ActorMethod<[bigint], Result_1>,
  'delete_wallet' : ActorMethod<[bigint], Result>,
  'do_pre_upgrade_and_print_db' : ActorMethod<[], string>,
  'edit_transaction_record' : ActorMethod<[EditHistoryCommand], Result>,
  'get_balance' : ActorMethod<[], bigint>,
  'get_http_result_by_id' : ActorMethod<[bigint], [] | [HttpResult]>,
  'get_http_results' : ActorMethod<[], Array<[bigint, HttpResult]>>,
  'get_icp_usd_exchange' : ActorMethod<[], string>,
  'get_neuron_info' : ActorMethod<[bigint], Result_3>,
  'get_payload' : ActorMethod<[], string>,
  'get_payload_from_dropbox' : ActorMethod<[string, string], string>,
  'get_payload_from_stable_mem' : ActorMethod<[], string>,
  'http_request_via_proxy' : ActorMethod<
    [HttpRequest, [] | [bigint], boolean],
    Result_4
  >,
  'http_response_callback' : ActorMethod<[bigint, HttpResult], undefined>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_a_neuron_wallet' : ActorMethod<[bigint], Result_5>,
  'query_a_wallet' : ActorMethod<[bigint], Result_6>,
  'query_all_neuron_wallet' : ActorMethod<[], Result_7>,
  'query_all_wallets' : ActorMethod<[], Result_8>,
  'restore_db_from_dropbox' : ActorMethod<[string, string], boolean>,
  'save_payload_to_dropbox' : ActorMethod<[string, number], string>,
  'save_payload_to_dropbox_blocking' : ActorMethod<[], string>,
  'set_payload' : ActorMethod<[], undefined>,
  'set_stable_mem_use_payload' : ActorMethod<[], undefined>,
  'store_paylaod_to_dropbox' : ActorMethod<[], string>,
  'sync_transaction_record' : ActorMethod<
    [Array<[bigint, Array<TransactionF>]>],
    Result
  >,
  'transform' : ActorMethod<[TransformArgs], HttpResponse>,
  'update_neuron_wallet' : ActorMethod<[NeuronUpdateCommand], Result>,
  'update_wallet' : ActorMethod<[WalletUpdateCommand], Result>,
  'user_quantity' : ActorMethod<[], number>,
  'wallet_history' : ActorMethod<[HistoryQueryCommand], Result_9>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
