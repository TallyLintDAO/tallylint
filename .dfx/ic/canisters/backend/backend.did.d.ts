import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CustomWalletInfo {
  'id' : string,
  'register_time' : bigint,
  'front_end_wallet_info' : FrontEndWalletInfo,
}
export interface FrontEndWalletInfo {
  'addr' : string,
  'name' : string,
  'w_type' : string,
}
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Array<CustomWalletInfo> } |
  { 'Err' : string };
export interface UserProfile {
  'owner' : Principal,
  'name' : string,
  'created_at' : bigint,
  'custom_wallet_info_array' : Array<CustomWalletInfo>,
}
export interface _SERVICE {
  'add_wallet' : ActorMethod<[FrontEndWalletInfo], Result>,
  'auto_register_user' : ActorMethod<[], Result_1>,
  'delete_wallet' : ActorMethod<[string], Result>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_wallet_array' : ActorMethod<[], Result_2>,
  'user_quantity' : ActorMethod<[], number>,
}
