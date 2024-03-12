import os
import subprocess
import sys

#!step1: gen did
cmd = "cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/canisters/backend/backend.did"
subprocess.run(cmd, shell=True, stdout=subprocess.PIPE)
base_dir = subprocess.run("git rev-parse --show-toplevel",shell=True, stdout=subprocess.PIPE)

#! step2 : deploy
passwd = os.getenv("DFXPASS_BTWLZ")
deploy_local = f"dfx deploy backend "
deploy_ic = f"dfx deploy backend --network ic"

deploy_cmd='none'
if len(sys.argv) > 1:
    if sys.argv[1] == 'ic':
        deploy_cmd = deploy_ic
    elif sys.argv[1] == 'local':
        deploy_cmd = deploy_local
    else:
        print("Invalid argument. Please use 'deploy_local' or 'deploy_ic'.")
        sys.exit(1)
else:
    print("No argument provided. Please use 'deploy_local' or 'deploy_ic'.")
    sys.exit(1)

# TODO goto basedir and then give passwd but not immediately.  
    # maybe wait for 1 sec to simulate ?  dont have to like expect script language 
    
proc = subprocess.Popen(deploy_cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
proc.communicate(input=f"{passwd}\n".encode())


# TODO
#!step3 change did name
# run those:
# cd $base_dir
# cd ./.dfx/ic/canisters/backend
# mv service.did backend.did
# mv service.did.d.ts backend.did.d.ts
# mv service.did.js backend.did.js

# # mv会覆盖写入已存在的同名文件
# cd $base_dir
# cd ./.dfx/local/canisters/backend
# mv service.did backend.did
# mv service.did.d.ts backend.did.d.ts
# mv service.did.js backend.did.js

# cd $base_dir
# git add -f ./.dfx/ic/canisters/backend/backend.did 
# git add -f ./.dfx/ic/canisters/backend/backend.did.d.ts  
# git add -f ./.dfx/ic/canisters/backend/backend.did.js 
# git add -f ./.dfx/ic/canisters/backend/index.js 

# cd $base_dir
# git add -f ./.dfx/local/canisters/backend/backend.did
# git add -f ./.dfx/local/canisters/backend/backend.did.d.ts  
# git add -f ./.dfx/local/canisters/backend/backend.did.js
# git add -f ./.dfx/local/canisters/backend/index.js