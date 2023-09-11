export const idlFactory = ({ IDL }) => {
  const UserStatus = IDL.Variant({ 'Enable' : IDL.Null, 'Disable' : IDL.Null });
  const UserProfile = IDL.Record({
    'id' : IDL.Nat64,
    'status' : UserStatus,
    'owner' : IDL.Principal,
    'interests' : IDL.Vec(IDL.Text),
    'avatar_uri' : IDL.Text,
    'memo' : IDL.Text,
    'name' : IDL.Text,
    'biography' : IDL.Text,
    'wallet_principal' : IDL.Opt(IDL.Principal),
    'created_at' : IDL.Nat64,
    'email' : IDL.Text,
    'avatar_id' : IDL.Nat64,
    'location' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : UserProfile, 'Err' : IDL.Text });
  const UserRegisterCommand = IDL.Record({
    'memo' : IDL.Text,
    'name' : IDL.Text,
    'email' : IDL.Text,
  });
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
