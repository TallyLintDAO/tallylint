export const idlFactory = ({ IDL }) => {
  const NeuronAddCommand = IDL.Record({
    'from' : IDL.Text,
    'name' : IDL.Text,
    'address' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const Currency = IDL.Record({ 'decimals' : IDL.Nat64, 'symbol' : IDL.Text });
  const Details = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Float64,
    'status' : IDL.Text,
    'ledgerCanisterId' : IDL.Text,
    'value' : IDL.Float64,
    'cost' : IDL.Float64,
    'from' : IDL.Text,
    'currency' : Currency,
    'profit' : IDL.Float64,
    'price' : IDL.Float64,
    'amount' : IDL.Float64,
  });
  const TransactionF = IDL.Record({
    'hash' : IDL.Text,
    'walletName' : IDL.Text,
    't_type' : IDL.Text,
    'timestamp' : IDL.Float64,
    'details' : Details,
  });
  const TransactionB = IDL.Record({
    'id' : IDL.Nat64,
    'tag' : IDL.Text,
    'memo' : IDL.Text,
    'comment' : IDL.Text,
    'income' : IDL.Float64,
    'address' : IDL.Text,
    'manual' : IDL.Bool,
    'principal_id' : IDL.Opt(IDL.Text),
    'transaction_f' : TransactionF,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text });
  const WalletAddCommand = IDL.Record({
    'from' : IDL.Text,
    'name' : IDL.Text,
    'address' : IDL.Text,
    'principal_id' : IDL.Opt(IDL.Text),
  });
  const UserProfile = IDL.Record({
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
  });
  const Result_2 = IDL.Variant({ 'Ok' : UserProfile, 'Err' : IDL.Text });
  const NeuronId = IDL.Record({ 'id' : IDL.Vec(IDL.Nat8) });
  const BallotInfo = IDL.Record({
    'vote' : IDL.Int32,
    'proposal_id' : IDL.Opt(NeuronId),
  });
  const KnownNeuronData = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Opt(IDL.Text),
  });
  const NeuronInfo = IDL.Record({
    'dissolve_delay_seconds' : IDL.Nat64,
    'recent_ballots' : IDL.Vec(BallotInfo),
    'created_timestamp_seconds' : IDL.Nat64,
    'state' : IDL.Int32,
    'stake_e8s' : IDL.Nat64,
    'joined_community_fund_timestamp_seconds' : IDL.Opt(IDL.Nat64),
    'retrieved_at_timestamp_seconds' : IDL.Nat64,
    'known_neuron_data' : IDL.Opt(KnownNeuronData),
    'voting_power' : IDL.Nat64,
    'age_seconds' : IDL.Nat64,
  });
  const GovernanceError = IDL.Record({
    'error_message' : IDL.Text,
    'error_type' : IDL.Int32,
  });
  const CustomResult1 = IDL.Variant({
    'Ok' : NeuronInfo,
    'Err' : GovernanceError,
  });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Tuple(CustomResult1),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const NeuronProfile = IDL.Record({
    'id' : IDL.Nat64,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'update_time' : IDL.Nat64,
    'create_time' : IDL.Nat64,
    'address' : IDL.Text,
  });
  const Result_4 = IDL.Variant({ 'Ok' : NeuronProfile, 'Err' : IDL.Text });
  const WalletProfile = IDL.Record({
    'id' : IDL.Nat64,
    'last_transaction_time' : IDL.Nat64,
    'last_sync_time' : IDL.Nat64,
    'from' : IDL.Text,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
    'address' : IDL.Text,
    'principal_id' : IDL.Opt(IDL.Text),
    'holder' : IDL.Principal,
    'transactions' : IDL.Nat64,
  });
  const Result_5 = IDL.Variant({ 'Ok' : WalletProfile, 'Err' : IDL.Text });
  const Result_6 = IDL.Variant({
    'Ok' : IDL.Vec(NeuronProfile),
    'Err' : IDL.Vec(NeuronProfile),
  });
  const Result_7 = IDL.Variant({
    'Ok' : IDL.Vec(WalletProfile),
    'Err' : IDL.Vec(WalletProfile),
  });
  const HistoryQueryCommand = IDL.Record({
    'tag' : IDL.Text,
    'from_time' : IDL.Nat64,
    'to_time' : IDL.Nat64,
    't_type' : IDL.Text,
    'sort_method' : IDL.Text,
    'address' : IDL.Opt(IDL.Text),
  });
  const Result_8 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(TransactionB))),
    'Err' : IDL.Text,
  });
  const SyncTransactionCommand = IDL.Record({
    'history' : IDL.Vec(TransactionF),
    'walletId' : IDL.Nat64,
  });
  const NeuronUpdateCommand = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
  });
  const WalletUpdateCommand = IDL.Record({
    'id' : IDL.Nat64,
    'from' : IDL.Text,
    'name' : IDL.Text,
  });
  return IDL.Service({
    'add_neuron_wallet' : IDL.Func([NeuronAddCommand], [Result], []),
    'add_transaction' : IDL.Func([TransactionB], [Result_1], []),
    'add_wallet' : IDL.Func([WalletAddCommand], [Result], []),
    'auto_register_user' : IDL.Func([], [Result_2], []),
    'clean_db' : IDL.Func([], [IDL.Bool], []),
    'collect_running_payload' : IDL.Func([], [IDL.Text], ['query']),
    'delete_neuron_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_transaction' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'do_pre_upgrade_and_print_db' : IDL.Func([], [IDL.Text], ['query']),
    'get_balance' : IDL.Func([], [IDL.Nat64], []),
    'get_neuron_info' : IDL.Func([IDL.Nat64], [Result_3], []),
    'get_payload_from_dropbox' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'get_payload_from_stable_mem' : IDL.Func([], [IDL.Text], ['query']),
    'get_payload_from_stable_mem_simple' : IDL.Func([], [IDL.Text], ['query']),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_a_neuron_wallet' : IDL.Func([IDL.Nat64], [Result_4], ['query']),
    'query_a_wallet' : IDL.Func([IDL.Nat64], [Result_5], ['query']),
    'query_all_neuron_wallet' : IDL.Func([], [Result_6], ['query']),
    'query_all_wallets' : IDL.Func([], [Result_7], ['query']),
    'query_wallet_transactions' : IDL.Func(
        [HistoryQueryCommand],
        [Result_8],
        ['query'],
      ),
    'save_payload_to_dropbox' : IDL.Func(
        [IDL.Text, IDL.Nat32, IDL.Nat],
        [IDL.Text],
        [],
      ),
    'set_payload_using_dropbox' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Bool],
        [],
      ),
    'set_payload_using_stable_mem' : IDL.Func([], [], []),
    'set_stable_mem_use_payload' : IDL.Func([], [], []),
    'set_stable_mem_use_payload_simple' : IDL.Func([], [], []),
    'sync_transaction_record' : IDL.Func(
        [IDL.Vec(SyncTransactionCommand)],
        [Result],
        [],
      ),
    'update_neuron_wallet' : IDL.Func([NeuronUpdateCommand], [Result], []),
    'update_transaction' : IDL.Func([TransactionB], [Result], []),
    'update_wallet' : IDL.Func([WalletUpdateCommand], [Result], []),
    'user_quantity' : IDL.Func([], [IDL.Nat32], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
