import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : string };
export interface UserProfile {
  'id' : bigint,
  'status' : UserStatus,
  'owner' : Principal,
  'interests' : Array<string>,
  'avatar_uri' : string,
  'memo' : string,
  'name' : string,
  'biography' : string,
  'wallet_principal' : [] | [Principal],
  'created_at' : bigint,
  'email' : string,
  'avatar_id' : bigint,
  'location' : string,
}
export interface UserRegisterCommand {
  'memo' : string,
  'name' : string,
  'email' : string,
}
export type UserStatus = { 'Enable' : null } |
  { 'Disable' : null };
export interface _SERVICE {
  'auto_register_user' : ActorMethod<[], Result>,
  'get_caller' : ActorMethod<[], string>,
  'greet' : ActorMethod<[string], string>,
  'next_id' : ActorMethod<[], bigint>,
  'now' : ActorMethod<[], bigint>,
  'register_user' : ActorMethod<[UserRegisterCommand], Result_1>,
}
