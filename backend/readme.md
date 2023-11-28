## backend ic address    
backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=v7g7o-oiaaa-aaaag-qcj3q-cai

## add cycles to can:
dfx canister --network ic --wallet vwfus-yaaaa-aaaag-qcj2a-cai deposit-cycles 5000000000000 assets
5 000 000 000 000 5TC

## generate rust backend canister did file :
https://internetcomputer.org/docs/current/developer-docs/backend/rust/candid

## line of code
```
find . -name "*.rs" -print | xargs wc -l
```

## check canister cycles balance:


<!-- TODO -->




goal: update can without losing data and incompability when datastructure modifyied cause restore_fs fail.
1.openchat的存储应用案例是stable memory.
threadlocal with ic-stable memory manager with memory(0,1,2...)
memory(x)  is Virtuaized Mem in a canister mem.into 16 VMems.
now taxlint use: ic的storage库和ic-stable-memory是否调用的同样的底层存储?



goal:run rs code instead of dfx-CLI to install and upgrade canister and inspect canister.(canistergeek can do this?)
2.oc升级罐子的案例是用的ic-rs-agent配合一部分dfx::core的接口直接调用.来升级
还用到了获取本地wasm文件.说明一部分rs代码是跑在开发机的?跑的main.rs
另外类似的安装罐子也用的rs-agent



goal: generate grouped, usage categoriesed did file . instead of alphabet sort api 
3.oc如何生成did的?
使用library/candid_gen/的rs代码
如community的罐子,在mian.rs使用 generate_candid_method macro 生成了did文件?
part of oc rs code have main.rs .whats for ? run local to gen did in dev workspace?
以及candid库的: export_service() 
也许cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/backend.did
这个candid-extractor也会用到  candid::export_service() ?
并没有:用到的是:impl Parse for SystemAPI
export_service() 生成的文件在哪?


## goal:run UTest and ITest  and Log??
ref:openchat
canister_ids.json use like this :
  "group_index": {
    "ic": "4ijyc-kiaaa-aaaaf-aaaja-cai",
    "ic_test": "7kifq-3yaaa-aaaaf-ab2cq-cai"
  },  

 "ic_test": {
  "providers": [
    "https://ic0.app/"
  ],
  "type": "persistent"
},

in rs code:  
// const NETWORK = "ic_test";  

logging: keywords: info!  ,     canister_logger::init_with_logs(data.test_mode, logs, traces);
where the log file store? ic-fs? or local ? 
what about running in production time logging ? 


src/utils lib . any useful stuff for taxlint?
### Itest : use "pocketIC" as env to provide canister running platform.
pocketIC , good name ,explain how ez to provide function of a running platform of IC.
compare with dfx: 
dfx : 
(hiding stuff is ic-env and caller )
dfx call backend greet "alex"  
TODO: maybe use dfx::core in rs code can also do this ? need with a env running ?

rs code with pocketIC:
#[macro_export]
macro_rules! generate_update_call {
    ($method_name:ident) => {
        #[allow(dead_code)]
        pub fn $method_name(
            env: &mut pocket_ic::PocketIc,
            sender: candid::Principal,
            canister_id: candid::Principal,
            args: &$method_name::Args,
        ) -> $method_name::Response {
            let method_name = stringify!($method_name);

            $crate::client::execute_update(env, sender, canister_id, method_name, args)
        }
    };
}
backend/integration_tests/src/client/community.rs

// Queries
generate_query_call!(channel_summary);
// after generate calls. when exec? where is return and assert ?
// go  pub fn create_channel()  may have some clue.

// Updates
generate_update_call!(add_reaction);
generate_update_call!(block_user);








