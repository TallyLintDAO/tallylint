from ic.client import Client
from ic.identity import Identity
from ic.agent import Agent

# Identity and Client are dependencies of Agent

# TODO get pk from env:
iden = Identity(
    privkey="833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42"
)  # create an instance from private key

client = Client(url="https://ic0.app")
agent = Agent(iden, client)


# TODO get local file as string:
# query the name of token canister `gvbup-jyaaa-aaaah-qcdwa-cai`


# transfer 100 token to blackhole address `aaaaa-aa`
params = [
    {"type": Types.Principal, "value": "aaaaa-aa"},
    {"type": Types.Nat, "value": 10000000000},
]
ret = agent.update_raw(
    "gvbup-jyaaa-aaaah-qcdwa-cai", "send_payload_string_to_canister", encode(params)
)

print(ret)
