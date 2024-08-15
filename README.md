## Tally Lint : Your IC coin tracker

Try it here: 🧭
https://vresg-vyaaa-aaaag-qcj2q-cai.icp0.io/

## Key Features⚓

### Manage Asset Info 📜

Are you troubled by how to check your assets? Still worried about how to track your profits?

TallyLint counts the profits of transaction for you.

### Auto Calculate Tax Result🛩️

Our main goal is to provide users in the IC ecosystem with a convenient tax management tool to help them move away from tedious manual statistics to easier automated statistics, thereby improving their tax payment experience and reducing error rates.

## Open Source activities is Welcome ! 😊

We value and encourage collaboration, so whether you're a developer, designer, or simply someone interested in improving our software, your participation is greatly appreciated!  
So everyone is free to request for new feature, report a bug, submit a pull request and give feedback ,etc.  
Together, let's make tallylint even more easy to use, efficient, and powerful for everyone. Join us on this exciting journey of open-source development~

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

backend: by running the script to download tallylint source code and prepare backend development enviroment and for you.

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

## Frontend install

Because [DAB.js](https://github.com/bitfinity-network/DAB-js) is used in order to support NFT, you need to perform the following steps when installing the dependency to confirm completion:

### Interaction guide

To pull and install from [@Psychedelic](https://github.com/psychedelic) via the NPM CLI, you'll need:

- A Github account
- A Github personal access token (you can create a personal acess token [here](https://github.com/settings/tokens))
- The personal access token with the correct scopes, **repo** and **read:packages** to be granted access to the [GitHub Package Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-npm-registry#authenticating-to-github-packages).
- Authentication via `npm login`, using your Github email for the **username** and the **personal access token** as your **password**:

Once you have those ready, run:

```
npm login --registry=https://npm.pkg.github.com --scope=@psychedelic
```

> **Note:** You only need to configure this once to install the package!

    On npm login provide your Github email as your username and the Personal access token as the password.

You can also setup your npm global settings to fetch from the Github registry everytime it finds a **@Psychdelic** package, find the instructions [here](https://docs.npmjs.com/configuring-your-registry-settings-as-an-npm-enterprise-user).

⚠️ Alternatively, a token could be passed to the `.npmrc` as `//npm.pkg.github.com/:_authToken=xxxxxx` but we'd like to keep it clean and tokenless.

---

then: run `npm -i` before run `dfx deploy assets`
