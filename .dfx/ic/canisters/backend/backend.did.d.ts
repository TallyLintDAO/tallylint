import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export interface UserProfile {
  'owner' : Principal,
  'name' : string,
  'wallet_principal' : [] | [Principal],
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
}
