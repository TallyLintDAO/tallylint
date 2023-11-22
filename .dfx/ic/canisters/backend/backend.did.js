export const idlFactory = ({ IDL }) => {
  const NeuronAddCommand = IDL.Record({
    'from' : IDL.Text,
    'name' : IDL.Text,
    'address' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const AddRecordCommand = IDL.Record({
    'tag' : IDL.Text,
    'time' : IDL.Nat64,
    't_type' : IDL.Text,
    'comment' : IDL.Text,
    'address' : IDL.Text,
    'manual' : IDL.Bool,
    'principal_id' : IDL.Opt(IDL.Text),
    'price' : IDL.Float64,
    'amount' : IDL.Nat32,
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
  const EditHistoryCommand = IDL.Record({
    'id' : IDL.Nat64,
    'tag' : IDL.Text,
    'time' : IDL.Nat64,
    't_type' : IDL.Text,
    'comment' : IDL.Text,
    'manual' : IDL.Bool,
    'price' : IDL.Float64,
    'amount' : IDL.Nat32,
  });
  const FromUserRecord = IDL.Record({ 'user_id' : IDL.Principal });
  const FromCanisterRecord = IDL.Record({
    'canister_version' : IDL.Opt(IDL.Nat64),
    'canister_id' : IDL.Principal,
  });
  const CanisterChangeOrigin = IDL.Variant({
    'from_user' : FromUserRecord,
    'from_canister' : FromCanisterRecord,
  });
  const CreationRecord = IDL.Record({ 'controllers' : IDL.Vec(IDL.Principal) });
  const CanisterInstallMode = IDL.Variant({
    'reinstall' : IDL.Null,
    'upgrade' : IDL.Null,
    'install' : IDL.Null,
  });
  const CodeDeploymentRecord = IDL.Record({
    'mode' : CanisterInstallMode,
    'module_hash' : IDL.Vec(IDL.Nat8),
  });
  const CanisterChangeDetails = IDL.Variant({
    'creation' : CreationRecord,
    'code_deployment' : CodeDeploymentRecord,
    'controllers_change' : CreationRecord,
    'code_uninstall' : IDL.Null,
  });
  const CanisterChange = IDL.Record({
    'timestamp_nanos' : IDL.Nat64,
    'canister_version' : IDL.Nat64,
    'origin' : CanisterChangeOrigin,
    'details' : CanisterChangeDetails,
  });
  const CanisterInfoResponse = IDL.Record({
    'controllers' : IDL.Vec(IDL.Principal),
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'recent_changes' : IDL.Vec(CanisterChange),
    'total_num_changes' : IDL.Nat64,
  });
  const CanisterStatusType = IDL.Variant({
    'stopped' : IDL.Null,
    'stopping' : IDL.Null,
    'running' : IDL.Null,
  });
  const DefiniteCanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const CanisterStatusResponse = IDL.Record({
    'status' : CanisterStatusType,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettings,
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
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
  const Result_4 = IDL.Variant({ 'Ok' : WalletProfile, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({
    'Ok' : IDL.Vec(WalletProfile),
    'Err' : IDL.Vec(WalletProfile),
  });
  const RecordProfile = IDL.Record({
    'id' : IDL.Nat64,
    'tag' : IDL.Text,
    'time' : IDL.Nat64,
    't_type' : IDL.Text,
    'comment' : IDL.Text,
    'address' : IDL.Text,
    'manual' : IDL.Bool,
    'principal_id' : IDL.Opt(IDL.Text),
    'price' : IDL.Float64,
    'amount' : IDL.Nat32,
  });
  const NeuronUpdateCommand = IDL.Record({
    'id' : IDL.Nat64,
    'from' : IDL.Text,
    'name' : IDL.Text,
  });
  const HistoryQueryCommand = IDL.Record({
    'tag' : IDL.Text,
    'from_time' : IDL.Nat64,
    'to_time' : IDL.Nat64,
    't_type' : IDL.Text,
    'sort_method' : IDL.Text,
    'address' : IDL.Opt(IDL.Text),
  });
  const Result_6 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(RecordProfile))),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'add_neuron_wallet' : IDL.Func([NeuronAddCommand], [Result], []),
    'add_transaction_record' : IDL.Func([AddRecordCommand], [Result_1], []),
    'add_wallet' : IDL.Func([WalletAddCommand], [Result], []),
    'auto_register_user' : IDL.Func([], [Result_2], []),
    'create_and_install' : IDL.Func([], [IDL.Text], []),
    'delete_neuron_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_transaction_record' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'edit_transaction_record' : IDL.Func([EditHistoryCommand], [Result], []),
    'get_balance' : IDL.Func([], [IDL.Nat64], []),
    'get_canister_info' : IDL.Func([IDL.Text], [CanisterInfoResponse], []),
    'get_canister_status' : IDL.Func([IDL.Text], [CanisterStatusResponse], []),
    'get_ledger_id' : IDL.Func([IDL.Principal], [IDL.Nat32], ['query']),
    'get_neuron_info' : IDL.Func([IDL.Nat64], [Result_3], []),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_a_wallet' : IDL.Func([IDL.Nat64], [Result_4], ['query']),
    'query_all_wallets' : IDL.Func([], [Result_5], ['query']),
    'query_neuron_wallet' : IDL.Func([IDL.Nat64], [Result_4], []),
    'sync_transaction_record' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Vec(RecordProfile)))],
        [Result],
        [],
      ),
    'update_neuron_wallet' : IDL.Func([NeuronUpdateCommand], [Result], []),
    'update_wallet' : IDL.Func([NeuronUpdateCommand], [Result], []),
    'user_quantity' : IDL.Func([], [IDL.Nat32], []),
    'wallet_history' : IDL.Func([HistoryQueryCommand], [Result_6], ['query']),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
