import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : string } |
  { 'Err' : string };
export interface _SERVICE {
  'get_caller' : ActorMethod<[], string>,
  'greet' : ActorMethod<[string], string>,
  'next_id' : ActorMethod<[], bigint>,
  'now' : ActorMethod<[], bigint>,
  'register_user' : ActorMethod<[], Result>,
}
