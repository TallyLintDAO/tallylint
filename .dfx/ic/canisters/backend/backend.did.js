export const idlFactory = ({ IDL }) => {
  const WalletAddCommand = IDL.Record({
    'from' : IDL.Text,
    'name' : IDL.Text,
    'address' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const UserProfile = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : UserProfile, 'Err' : IDL.Text });
  const WalletProfile = IDL.Record({
    'id' : IDL.Nat64,
    'from' : IDL.Text,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
    'address' : IDL.Text,
    'holder' : IDL.Principal,
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(WalletProfile),
    'Err' : IDL.Vec(WalletProfile),
  });
  return IDL.Service({
    'add_wallet' : IDL.Func([WalletAddCommand], [Result], []),
    'auto_register_user' : IDL.Func([], [Result_1], []),
    'delete_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_all_wallets' : IDL.Func([], [Result_2], ['query']),
    'user_quantity' : IDL.Func([], [IDL.Nat32], []),
  });
};
export const init = ({ IDL }) => { return []; };
