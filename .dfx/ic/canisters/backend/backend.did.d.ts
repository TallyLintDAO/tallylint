import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CustomWalletInfo {
  'wallet_addr' : Principal,
  'wallet_name' : string,
  'wallet_type' : string,
}
export type Result = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : boolean } |
  { 'Err' : string };
export interface UserProfile {
  'custom_wallet_info' : [] | [CustomWalletInfo],
  'owner' : Principal,
  'name' : string,
  'created_at' : bigint,
}
export interface UserRegisterCommand { 'name' : string }
export interface _SERVICE {
  'auto_register_user' : ActorMethod<[], Result>,
  'get_caller' : ActorMethod<[], string>,
  'greet' : ActorMethod<[string], string>,
  'next_id' : ActorMethod<[], bigint>,
  'now' : ActorMethod<[], bigint>,
  'register_user' : ActorMethod<[UserRegisterCommand], Result_1>,
  'update_wallet' : ActorMethod<[CustomWalletInfo], Result_2>,
}
