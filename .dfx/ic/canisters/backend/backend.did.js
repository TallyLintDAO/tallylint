export const idlFactory = ({ IDL }) => {
  const FrontEndWalletInfo = IDL.Record({
    'addr' : IDL.Text,
    'name' : IDL.Text,
    'w_type' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const CustomWalletInfo = IDL.Record({
    'id' : IDL.Text,
    'register_time' : IDL.Nat64,
    'front_end_wallet_info' : FrontEndWalletInfo,
  });
  const UserProfile = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'custom_wallet_info_array' : IDL.Vec(CustomWalletInfo),
  });
  const Result_1 = IDL.Variant({ 'Ok' : UserProfile, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(CustomWalletInfo),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'add_wallet' : IDL.Func([FrontEndWalletInfo], [Result], []),
    'auto_register_user' : IDL.Func([], [Result_1], []),
    'delete_wallet' : IDL.Func([IDL.Text], [Result], []),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_wallet_array' : IDL.Func([], [Result_2], []),
    'user_quantity' : IDL.Func([], [IDL.Nat32], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
