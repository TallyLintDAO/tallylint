export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  return IDL.Service({
    'get_caller' : IDL.Func([], [IDL.Text], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'next_id' : IDL.Func([], [IDL.Nat64], ['query']),
    'now' : IDL.Func([], [IDL.Nat64], ['query']),
    'register_user' : IDL.Func([], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
