import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CustomWalletInfo {
  'wallet_register_time' : bigint,
  'wallet_addr' : Principal,
  'wallet_name' : string,
  'wallet_type' : string,
  'wallet_id' : string,
}
export type Result = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : boolean } |
  { 'Err' : string };
export interface UserProfile {
  'custom_wallet_info' : [] | [CustomWalletInfo],
  'owner' : Principal,
  'name' : string,
  'created_at' : bigint,
}
export interface _SERVICE {
  'auto_register_user' : ActorMethod<[], Result>,
  'delete_wallet' : ActorMethod<[string], Result_1>,
  'get_caller' : ActorMethod<[], string>,
  'greet' : ActorMethod<[string], string>,
  'next_id' : ActorMethod<[], bigint>,
  'now' : ActorMethod<[], bigint>,
  'update_wallet' : ActorMethod<[CustomWalletInfo], Result_1>,
}
