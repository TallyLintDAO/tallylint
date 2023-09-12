export const idlFactory = ({ IDL }) => {
  const UserProfile = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'wallet_principal' : IDL.Opt(IDL.Principal),
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : UserProfile, 'Err' : IDL.Text });
  const UserRegisterCommand = IDL.Record({ 'name' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  return IDL.Service({
    'auto_register_user' : IDL.Func([], [Result], []),
    'get_caller' : IDL.Func([], [IDL.Text], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'next_id' : IDL.Func([], [IDL.Nat64], ['query']),
    'now' : IDL.Func([], [IDL.Nat64], ['query']),
    'register_user' : IDL.Func([UserRegisterCommand], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
