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
  const TransactionB = IDL.Record({
    'id' : IDL.Nat64,
    'tag' : IDL.Vec(IDL.Text),
    'hash' : IDL.Text,
    'memo' : IDL.Text,
    'walletName' : IDL.Text,
    't_type' : IDL.Text,
    'comment' : IDL.Text,
    'address' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'details' : Details,
    'manual' : IDL.Bool,
    'principal_id' : IDL.Opt(IDL.Text),
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
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text });
  const TransactionF = IDL.Record({
    'hash' : IDL.Text,
    'walletName' : IDL.Text,
    't_type' : IDL.Text,
    'timestamp' : IDL.Float64,
    'details' : Details,
  });
  const TransactionService = IDL.Record({
    'transactions' : IDL.Vec(IDL.Tuple(IDL.Nat64, TransactionF)),
  });
  const WalletRecordService = IDL.Record({
    'records' : IDL.Vec(IDL.Tuple(IDL.Nat64, TransactionB)),
  });
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
  const WalletService = IDL.Record({
    'wallets' : IDL.Vec(IDL.Tuple(IDL.Nat64, WalletProfile)),
  });
  const NeuronProfile = IDL.Record({
    'id' : IDL.Nat64,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'update_time' : IDL.Nat64,
    'create_time' : IDL.Nat64,
    'address' : IDL.Text,
  });
  const NeuronService = IDL.Record({
    'neurons' : IDL.Vec(IDL.Tuple(IDL.Text, NeuronProfile)),
  });
  const UserService = IDL.Record({
    'users' : IDL.Vec(IDL.Tuple(IDL.Principal, UserProfile)),
  });
  const CanisterContext = IDL.Record({
    'id' : IDL.Nat64,
    'trans_f_srv' : TransactionService,
    'wallet_transc_srv' : WalletRecordService,
    'wallet_service' : WalletService,
    'neuron_service' : NeuronService,
    'user_service' : UserService,
  });
  const Result_5 = IDL.Variant({ 'Ok' : NeuronProfile, 'Err' : IDL.Text });
  const Result_6 = IDL.Variant({ 'Ok' : WalletProfile, 'Err' : IDL.Text });
  const Result_7 = IDL.Variant({
    'Ok' : IDL.Vec(NeuronProfile),
    'Err' : IDL.Vec(NeuronProfile),
  });
  const Result_8 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Nat64, TransactionB)),
    'Err' : IDL.Text,
  });
  const Result_9 = IDL.Variant({
    'Ok' : IDL.Vec(WalletProfile),
    'Err' : IDL.Vec(WalletProfile),
  });
  const Result_10 = IDL.Variant({ 'Ok' : TransactionB, 'Err' : IDL.Text });
  const HistoryQueryCommand = IDL.Record({
    'from_time' : IDL.Nat64,
    'to_time' : IDL.Nat64,
    'sort_method' : IDL.Opt(IDL.Text),
    'address' : IDL.Vec(IDL.Text),
  });
  const SimpleTransaction = IDL.Record({
    'id' : IDL.Nat64,
    'tag' : IDL.Vec(IDL.Text),
    'hash' : IDL.Text,
    't_type' : IDL.Text,
    'comment' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'details' : Details,
    'manual' : IDL.Bool,
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
    'collect_running_payload_simple' : IDL.Func([], [IDL.Text], ['query']),
    'delete_neuron_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_transaction' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'get_balance' : IDL.Func([], [IDL.Nat64], []),
    'get_neuron_info' : IDL.Func([IDL.Nat64], [Result_3], []),
    'get_payload_from_dropbox' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'get_payload_from_dropbox_u8' : IDL.Func(
        [IDL.Text, IDL.Text],
        [Result_4],
        [],
      ),
    'get_payload_from_my_server' : IDL.Func([], [IDL.Text], []),
    'get_payload_from_my_server_raw_ip' : IDL.Func([], [IDL.Text], []),
    'get_payload_from_stable_mem_simple' : IDL.Func([], [IDL.Text], ['query']),
    'get_payload_from_stable_mem_simple_raw' : IDL.Func(
        [],
        [CanisterContext],
        ['query'],
      ),
    'get_user_config' : IDL.Func([], [IDL.Vec(UserProfile)], ['query']),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_a_neuron_wallet' : IDL.Func([IDL.Nat64], [Result_5], ['query']),
    'query_a_wallet' : IDL.Func([IDL.Nat64], [Result_6], ['query']),
    'query_all_neuron_wallet' : IDL.Func([], [Result_7], ['query']),
    'query_all_transactions' : IDL.Func([], [Result_8], ['query']),
    'query_all_wallets' : IDL.Func([], [Result_9], ['query']),
    'query_one_transaction' : IDL.Func([IDL.Nat64], [Result_10], ['query']),
    'query_wallet_transactions' : IDL.Func(
        [HistoryQueryCommand],
        [IDL.Vec(SimpleTransaction)],
        ['query'],
      ),
    'save_payload_to_dropbox' : IDL.Func([IDL.Text, IDL.Nat32], [IDL.Text], []),
    'send_payload_string_to_canister' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_payload_using_dropbox' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'set_payload_using_dropbox_simple' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'set_payload_using_dropbox_simple_raw' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'set_payload_using_stable_mem' : IDL.Func([], [IDL.Text], []),
    'set_payload_using_stable_mem_simple' : IDL.Func([], [IDL.Text], []),
    'set_payload_using_stable_mem_simple_raw' : IDL.Func([], [], []),
    'set_stable_mem_using_payload' : IDL.Func([], [], []),
    'set_stable_mem_using_payload_simple' : IDL.Func([], [], []),
    'set_stable_mem_using_payload_simple_raw' : IDL.Func([], [], []),
    'set_user_config' : IDL.Func([], [IDL.Vec(UserProfile)], []),
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
