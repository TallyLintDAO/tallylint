Todos: 
0. 把github仓库弄好,以及建立一个Github Organization. 一个DAO.，
1. 网址弄好，
2. 就可以交grant了
3. dfx.json 前端有个dependecies选项.不知道要不要关联上.默认创建项目是关联的.  
待测试


## Canister deploy on ic-chain and update
deploy on ic-chain doc :  
https://internetcomputer.org/docs/current/tutorials/deploy_sample_app#what-this-does
```bash
# 第一次部署到主网收费 1TC 前后端2个罐子一共花费1TC
dfx deploy --network ic --with-cycles 1000000000000
# 查询自己还有多少 cycles
dfx wallet --network ic balance
#查询自己正在用的是哪个钱包:todo 

```



### update canister code : 
以后修改了代码要升级不用重新deploy,而是:  
使用 upgrade指令, 要收费   
1. Making your build reproducible: 简单说就是上网了的罐子代码都有一个SHA256. 用户可以随时校验. 自治.和对用户代码安全的一种功能.  **allowing for users to determine if a canister's contents have been edited or changed.** 
(另外,这里的reproducible指的是任何一个人来下载你的代码然后上链校验hash,都能得到你公布的那个hash.就是reproducible的.) 
这个投票权(determine)很好.很有去中心的概念.  
操作指南: https://internetcomputer.org/docs/current/developer-docs/backend/reproducible-builds
建议的是用docker或者Nix搭建持续集成(CI),使得这个上链的code是reproducible的.->目的是让用户自治和可信
2. Once a canister has been deployed to the mainnet, the only way for new versions of the canister's code to be shipped is through planned upgrades.
升级罐子指南:  
https://internetcomputer.org/docs/current/developer-docs/backend/motoko/upgrading  



### update 实操
prepare: 
#### 1. Which WebAssembly (wasm) code is being executed for a canister?  
```bash
# check dfx.json include : "output_env_file": ".env",
cat ./.env 
dfx canister --network ic info CANISTER_ID
```
output:  
```bash
# Controllers: bnz7o-iuaaa-aaaaa-qaaaa-cai w4q6h-2cwaj-ir5pl-pelo5-tnylh-s5qrw-63hm5-m23jy-spsak-4oesk-dae
# Module hash: 0x4433b72a9b1723d2d394b70ea5d4d80c72a54a1c4e3c51f0f8aab46b0ffe5abd
```
Controllers can update the canister, so hash will also change atfer upgrade code.


1.1 things provide to users:   
1. source code   
2. build env  
3. offer Instructions on how to repeat the process of building the Wasm from the source code.  
 result to :  .dfx, node_modules, and target directories that could contain pre-built files.

##### step 2 and 3 good approach is using docker .
before writing Dockerfile :  
WARNING: make sure the docker is running in x86_64 architecure machine.
otherwise: see that: https://github.com/lima-vm/lima/blob/master/docs/multi-arch.md
```Dockerfile
# OS , 
# dfx_version

# frontend
# node.js -v ,vue -v  etc.. 

# backend
# cargo -v etc....

# if got os_env_variables :

```
```Dockerfile 
# offcial ic example : how a Rust build environment can be standardized using Docker.
FROM ubuntu:22.04

ENV NVM_DIR=/root/.nvm
ENV NVM_VERSION=v0.39.1
ENV NODE_VERSION=18.1.0

ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV RUST_VERSION=1.62.0

ENV DFX_VERSION=0.14.1

# Install a basic environment needed for our build tools
RUN apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake rsync

# Install Node.js using nvm
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin:${PATH}"
RUN curl --fail -sSf https://raw.githubusercontent.com/creationix/nvm/${NVM_VERSION}/install.sh | bash
RUN . "${NVM_DIR}/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "${NVM_DIR}/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "${NVM_DIR}/nvm.sh" && nvm alias default v${NODE_VERSION}

# Install Rust and Cargo
ENV PATH=/opt/cargo/bin:${PATH}
RUN curl --fail https://sh.rustup.rs -sSf \
        | sh -s -- -y --default-toolchain ${RUST_VERSION}-x86_64-unknown-linux-gnu --no-modify-path && \
    rustup default ${RUST_VERSION}-x86_64-unknown-linux-gnu && \
    rustup target add wasm32-unknown-unknown &&\
    cargo install ic-wasm

# Install dfx
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

COPY . /canister
WORKDIR /canister
```
build the dockerfile into a image and run it as a container:
```bash
docker build -t mycanister .
docker run -it --rm mycanister
```
###### abstract the above whole into a runnable script: cool~




#### 2. The canisters are normally written in a higher-level language, such as Motoko or Rust, and not directly in Wasm. The second question is then: is the Wasm that’s running really the result of compiling the purported source code?







IC上下文中的专指名称:Metrics: 
1. gain insight into a wide range of information regarding your canister's production services  
2. learn about your canister's statistics and productivity.  



cycles的收费规则:  
https://internetcomputer.org/docs/current/developer-docs/gas-cost

```bash
# 另外如果只执行 
dfx deploy
# **隐藏默认参数 --netwrok local . 就 不管有没有部署到ic主网上,都可以输入这个本地测试**.

```



```
1 000 000 000 000
```

第一次部署到主网的前后端地址:
  Frontend canister via browser
    assets: https://x63j6-hqaaa-aaaal-acyoa-cai.icp0.io/
  Backend canister via Candid interface:
    backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=xz2pk-kiaaa-aaaal-acyoq-cai


```
# 生成后端文件供罐子使用: 
dfx deploy backend
# frontend
dfx deploy assets

#maybe used
npm install
npm run build

dfx start --clean 
```

todo:可能的依赖项: dfx.json  

```
 "vote1_frontend": {
      "dependencies": [
        "backend"
      ],
      "frontend": {
        "entrypoint": "src/vote1_frontend/src/index.html"
      },
      "source": [
        "src/vote1_frontend/assets",
        "dist/vote1_frontend/"
      ],
      "type": "assets"
    }
```

前端:  
依赖项管理:
package-lock.json 文件为 build自动生成.会覆盖package.json 的config  
ic-agent-js:
https://www.npmjs.com/search?q=dfinity

 如果要让前端更新过后的 依赖项生效,可能需要删除 lock文件,他自己会再生成  

 然后:  
 ```bash
 npm list
 ```


 ## 网络问题的排查(按步骤):     
 工具有多种可以选,比如wireshark .像是network的IDE.复杂强大  
 或者直接命令行小工具,简洁  
 1. DNS 是否被篡改和污染. 给予了错误的IP地址
```bash
nslookup raw.githubusercontent.com
Server:         172.23.0.1
Address:        172.23.0.1#53

Non-authoritative answer:
Name:   raw.githubusercontent.com
Address: 0.0.0.0
Name:   raw.githubusercontent.com
Address: ::
# 直接瞬间反应.且给了错误的地址!
```
修复DNS:  
```bash
# WSL的方法:其他的linux可能类似,有些许不同.
cat /etc/resolv.conf
# output:
# This file was automatically generated by WSL. To stop automatic generation of this file, add the following entry to /etc/wsl.conf:
# [network]
# generateResolvConf = false
# nameserver 172.23.0.1  
# 上面这个默认一般是用的宿主机的DNS服务器
# <<<<修改 DNS server>>>> 即这里的nameserver 域名服务
# 可选google 8888 cloudfare 1111 等...


```
 2. 得到了正确的IP 是否被禁止访问

 ```bash
 ping x.x.x.x
#   The ping command sends ICMP echo request packets directly to the specified IP address and 
# *does not use the HTTP or HTTPS protocols !*
# 
curl -v -o test.json https://sdk.dfinity.org/manifest.json
 
 ```
## Todo:   LICENSE 的选择

 ![Alt text](/assets_for_doc/image.png)


 ## Ref
 use ic-ledger-api demo:
 https://github.com/ielashi/icp-accountant

 fee:
 https://wiki.internetcomputer.org/wiki/Exchange_rate_canister

discus on tax report: 
 https://forum.dfinity.org/t/tax-reports-on-internet-computer/22356/6

 leger账本简介:  
 https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/  
 Ledger canister  
 https://internetcomputer.org/docs/current/references/ledger  
 dfx ledger命令  
 https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/interact-with-ledger  
 本地leger模拟:  
 https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/ledger-local-setup  

Crypto tax reports in under 20 minutes:  
https://koinly.io/  
Koinly calculates your cryptocurrency taxes and helps you reduce them for next year. Simple & Reliable.  

PwC Annual 
Global Crypto 
Tax Report 2022
https://www.pwc.com/gx/en/financial-services/pdf/global-crypto-tax-report-2022.pdf  

How to Report Crypto on Your Taxes (Step-By-Step)
https://coinledger.io/blog/how-to-report-cryptocurrency-on-taxes


Tax reporting in the age of cryptocurrency
Getting ready for cryptocurrency tax regulation
https://www2.deloitte.com/us/en/pages/tax/articles/cryptocurrency-tax-reporting.html

tax rule of IRS
![Alt text](/assets_for_doc/image2.png)



## ic-rust-dev-doc


### IC-rust中的: Globally mutable states
抽象就是Java中的ThreadLocal. 不同services如加购物车,下订单都可以来操作这个变量.
如把用户的认证和姓名存进去TL中,各个services可以随意CRUD.

"Rust's design makes it difficult to global mutable variables."
可能是因为线程并发安全等考虑.maybe
反正就是语法丑一点,也许可以优化,todo~

### Most of the code that depends on the System API should go into the main file. ?
Canister code should be target-independent, so Most of the code that depends on the System API should go into the main file.   
 but why ? 

 good approach of using ic-stable (its OS persistance)
 ![Alt text](/assets_for_doc/image_ic_stable.png)


### Using variant types to indicate error cases

```rust
type CreateEntityResult = variant {
  Ok  : record { entity_id : EntityId; };
  Err : opt variant {
    EntityAlreadyExists;
    NoSpaceLeftInThisShard;
  }
};
service : {
  create_entity : (EntityParams) -> (CreateEntityResult);
}
```

### DB on chain of canisters
Stable variables vs flexible variables  
Stable variables are global variables that the system **preserves across upgrades**. For example, a user database should probably be stable.  

Flexible variables are global variables that the system **discards on code upgrade**. For example, it is reasonable to make a cache flexible if keeping this cache hot is not critical for your product.  

#### ic-DB is not use traditional SQL and specific DBMS, it use direct rust code libs

good practice : 
1. "Putting all global variables in one place"    
maybe just like putting all data to one DB is fine .  
It is best practice to store all global variables privately in a single file; **the canister main file.**(recall the document says OS syscall use in main is better)  
2. It is also recommended that you add comments that within your code that specify which variables are stable, 

#### ic-DB toturial : good article!
https://mmapped.blog/posts/14-stable-structures.html

https://github.com/dfinity/stable-structures#readme

## Candid Tips: 
1. Making the .did file the canister's source of truth
Your Candid file should be the main source of documentation for people who want to interact with your canister,
frontend candid also vise-versa.  
candid ref doc : https://internetcomputer.org/docs/current/references/candid-ref  






