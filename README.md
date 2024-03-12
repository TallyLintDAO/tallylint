## Tax Lint : Your personal cryptocurrency tax helper
Try it here:  🧭
https://vresg-vyaaa-aaaag-qcj2q-cai.icp0.io/

## Key Features⚓
### Manage Tax Info 📜
TaxLint is designed to help users who use ICP more easily count and manage their tax information. With taxlint, users will be able to record, track and calculate tax information related to their transactions and assets in the IC ecosystem.
### Auto Calculate Tax Result🛩️
Our main goal is to provide users in the IC ecosystem with a convenient tax management tool to help them move away from tedious manual statistics to easier automated statistics, thereby improving their tax payment experience and reducing error rates.
## Open Source activities is Welcome ! 😊
We value and encourage collaboration, so whether you're a developer, designer, or simply someone interested in improving our software, your participation is greatly appreciated!  
So everyone is free to request for new feature, report a bug, submit a pull request and give feedback ,etc.  
Together, let's make TaxLint even more easy to use, efficient, and powerful for everyone. Join us on this exciting journey of open-source development~
## Code transparency on mainnet :💎 We seriously respect that (Thanks to consensus protocol)
We make sure all the code running on ic mainnet is what you see , what you get.  
For user to check code transparency ,
just one command will run all stuff and check for you :
```sh
# assume you already have docker installed in your machine~
docker run  btwl/taxlint:v1
```
### Reproducible steps:  
We use docker container to remain all codes and its dependencies and environment.  
Here is the Dockerfile:
[Link to Dockerfile](./Reproducible/Dockerfile)
Current running canister code Module hash:  
running this will show module hash : 
```sh
dfx canister status --network ic --wallet vwfus-yaaaa-aaaag-qcj2a-cai backend | grep "Module hash" | awk '{ print $3 }'
```
frontend hash:  
`0x1286960c50eb7a773cfb5fdd77cc238588f39e21f189cc3eb0f35199a99b9c7e`  
backend hash:  
`0x17ea3f0714b9df0316c73609dbad90827bb2d8359a683591fcc79844cab2530b`  

## Developer
backend: by running the script  to download taxlint source code and prepare backend development enviroment and  for you.
```bash
wget  https://github.com/TaxLintDAO/taxlint/raw/master/reproducible/prep_backend_dev_env.sh 
wget  https://github.com/TaxLintDAO/taxlint/raw/master/reproducible/continue_prep_env.sh 
chmod 644 ./prep_backend_dev_env.sh 
chmod 644 ./continue_prep_env.sh 
./prep_backend_dev_env.sh
```
(if you dont have `wget` in your terminal yet. please install it .if you use ubuntu would be `apt update && apt install wget -y `) 

### deploy backend
`./backend/scripts/deploy_backend <mode>`  
`mode` input local(run `dfx start` before) or ic 


frontend:
run `npm -i` before run `dfx deploy assets` TODO: maybe this can auto exec before deploy assets in dfx.json config file 


