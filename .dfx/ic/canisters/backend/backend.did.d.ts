import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : UserProfile } |
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
}
export interface WalletProfile {
  'id' : bigint,
  'from' : string,
  'name' : string,
  'create_time' : bigint,
  'address' : string,
  'holder' : Principal,
}
export interface _SERVICE {
  'add_wallet' : ActorMethod<[WalletAddCommand], Result>,
  'auto_register_user' : ActorMethod<[], Result_1>,
  'delete_wallet' : ActorMethod<[bigint], Result>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_all_wallets' : ActorMethod<[], Array<WalletProfile>>,
  'user_quantity' : ActorMethod<[], number>,
}
