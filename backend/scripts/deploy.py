import os
import subprocess
import sys
from dotenv import load_dotenv
load_dotenv()
current_dir = os.getcwd()
print("Current working directory:", current_dir)


#!step1: gen did
base_dir_cmd = "sh ./backend/scripts/get_proj_base_dir.sh"
result = subprocess.run(base_dir_cmd, shell=True, stdout=subprocess.PIPE)
proj_base_dir = result.stdout.decode().strip()
os.chdir(proj_base_dir)#cd $base_dir (in bash)

#!step2: deploy 
# Get the passphrase from the environment variable
passwd = os.getenv("DFXPASS_BTWL0")
value = os.getenv("MY_VARIABLE")
value2 = os.getenv("DFX_VERSION")

# passwd = os.getenv("PATH")

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

proc = subprocess.Popen(deploy_cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
proc.communicate(input=f"{passwd}\n".encode())

#!step3 change did name 
change_name_cmd = "sh ./change_name.sh"
subprocess.run(change_name_cmd)


#!step4 upload did file 
if not len(sys.argv) > 2:
    # argv[2] exists
    print("argv[2] does not exist. provice uplaod did info")
    sys.exit(1)
# '''  means can include ' or " in string of python syntax 
upload_cmd = f'''./backend/scripts/sync_remote.sh "did uploading: {sys.argv[2]}"''' 
subprocess.run(upload_cmd)

