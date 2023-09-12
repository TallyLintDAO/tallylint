export const idlFactory = ({ IDL }) => {
  const CustomWalletInfo = IDL.Record({
    'wallet_addr' : IDL.Principal,
    'wallet_name' : IDL.Text,
    'wallet_type' : IDL.Text,
  });
  const UserProfile = IDL.Record({
    'custom_wallet_info' : IDL.Opt(CustomWalletInfo),
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : UserProfile, 'Err' : IDL.Text });
  const UserRegisterCommand = IDL.Record({ 'name' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  return IDL.Service({
    'auto_register_user' : IDL.Func([], [Result], []),
    'get_caller' : IDL.Func([], [IDL.Text], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'next_id' : IDL.Func([], [IDL.Nat64], ['query']),
    'now' : IDL.Func([], [IDL.Nat64], ['query']),
    'register_user' : IDL.Func([UserRegisterCommand], [Result_1], []),
    'update_wallet' : IDL.Func([CustomWalletInfo], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
