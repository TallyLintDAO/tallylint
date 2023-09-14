export const idlFactory = ({ IDL }) => {
  const WalletInfo = IDL.Record({
    'from' : IDL.Text,
    'name' : IDL.Text,
    'address' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const UserInfo = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : UserInfo, 'Err' : IDL.Text });
  const FullWalletInfo = IDL.Record({
    'id' : IDL.Text,
    'create_time' : IDL.Nat64,
    'wallet_info' : WalletInfo,
  });
  const UserProfile = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
    'full_wallet_info_array' : IDL.Vec(FullWalletInfo),
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(FullWalletInfo),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'add_wallet' : IDL.Func([WalletInfo], [Result], []),
    'auto_register_user' : IDL.Func([], [Result_1], []),
    'delete_wallet' : IDL.Func([IDL.Text], [Result], []),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_wallet_array' : IDL.Func([], [Result_2], []),
    'user_quantity' : IDL.Func([], [IDL.Nat32], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
