from pocket_ic import PocketIC

pic = PocketIC()
canister_id = pic.create_canister()

# use PocketIC to interact with your canisters
pic.add_cycles(canister_id, 1_000_000)
response = pic.update_call(canister_id, method="greeting")
assert(response == 'Hello, PocketIC!')
