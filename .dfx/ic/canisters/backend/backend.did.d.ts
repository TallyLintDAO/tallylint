import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface FullWalletInfo {
  'id' : string,
  'create_time' : bigint,
  'wallet_info' : WalletInfo,
}
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : UserInfo } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Array<FullWalletInfo> } |
  { 'Err' : string };
export interface UserInfo {
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
}
export interface UserProfile {
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
  'full_wallet_info_array' : Array<FullWalletInfo>,
}
export interface WalletInfo {
  'from' : string,
  'name' : string,
  'address' : string,
}
export interface _SERVICE {
  'add_wallet' : ActorMethod<[WalletInfo], Result>,
  'auto_register_user' : ActorMethod<[], Result_1>,
  'delete_wallet' : ActorMethod<[string], Result>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_wallet_array' : ActorMethod<[], Result_2>,
  'user_quantity' : ActorMethod<[], number>,
}
