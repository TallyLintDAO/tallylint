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
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Tuple(CustomResult1),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const WalletProfile = IDL.Record({
    'id' : IDL.Nat64,
    'from' : IDL.Text,
    'name' : IDL.Text,
    'create_time' : IDL.Nat64,
    'address' : IDL.Text,
    'holder' : IDL.Principal,
  });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Vec(WalletProfile),
    'Err' : IDL.Vec(WalletProfile),
  });
  return IDL.Service({
    'add_wallet' : IDL.Func([WalletAddCommand], [Result], []),
    'auto_register_user' : IDL.Func([], [Result_1], []),
    'create_and_install' : IDL.Func([], [IDL.Text], []),
    'delete_wallet' : IDL.Func([IDL.Nat64], [Result], []),
    'get_canister_info' : IDL.Func([IDL.Text], [CanisterInfoResponse], []),
    'get_canister_status' : IDL.Func([IDL.Text], [CanisterStatusResponse], []),
    'get_neuron_info' : IDL.Func([IDL.Nat64], [Result_2], []),
    'list_all_user' : IDL.Func([], [IDL.Vec(UserProfile)], []),
    'query_all_wallets' : IDL.Func([], [Result_3], ['query']),
    'user_quantity' : IDL.Func([], [IDL.Nat32], []),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
