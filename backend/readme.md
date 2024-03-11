REMAINDER:
in CN work time : AM9-PM5 corresponding below 
Switzerland  -> PM4-12AM(midnight00:00 )
Singapore -> same time zone 
USA-LA  -> 1AM-9AM
USA-Washington DC  -> 10PM-6AM
TODO list:
now do:
fixed, 1. query_all_neuron_wallet err.

middle level

1. did classify. not alphabet. maybe ref openchat.
2. TODO critical for production: update ic-fs wihout fail. middle level complexity. need divide and conquer.
3. How to do product time logging ? need to learn about wasm running on ic ?

testing ic-fs :
ic fs testing .

1. add a fs data structure. is fs data update OK?
2. update a fs data structure. is fs data update OK?
   its not diffcult to do. but just fussy

efficentcy:
done, https://linuxhandbook.com/sudo-without-password/
steps in my github repo: linux_useful_scripts

done, give vm 18 cores, for compile

how to edit a file in vscode with root permit when i login vscode in normal user ?
for example:
Failed to save 'sources.list': Unable to write file 'vscode-remote://ssh-remote+ubuntu_vmware_btwl/etc/apt/sources.list' (NoPermissions (FileSystemError): Error: EACCES: permission denied, open '/etc/apt/sources.list')

## backend ic address

backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=v7g7o-oiaaa-aaaag-qcj3q-cai

TODO all dfx calls can be made by ic-agent-rs code
openchat even directly use dfx::core lib of rs.(to manage canister on replica)

## add cycles to can:

dfx canister --network ic --wallet vwfus-yaaaa-aaaag-qcj2a-cai deposit-cycles 5000000000000 assets
5 000 000 000 000 5TC

## generate rust backend canister did file :

https://internetcomputer.org/docs/current/developer-docs/backend/rust/candid

## lines of code

```
find . -name "*.rs" -print | xargs wc -l
```

## check canister cycles balance:

goal: update can without losing data and incompability when datastructure modifyied cause restore_fs fail.
1.openchat 的存储应用案例是 stable memory.
threadlocal with ic-stable memory manager with memory(0,1,2...)
memory(x) is Virtuaized Mem in a canister mem.into 16 VMems.
now taxlint use: ic 的 storage 库和 ic-stable-memory 是否调用的同样的底层存储?

goal:run rs code instead of dfx-CLI to install and upgrade canister and inspect canister.(canistergeek can do this?)
2.oc 升级罐子的案例是用的 ic-rs-agent 配合一部分 dfx::core 的接口直接调用.来升级
还用到了获取本地 wasm 文件.说明一部分 rs 代码是跑在开发机的?跑的 main.rs
另外类似的安装罐子也用的 rs-agent

goal: generate grouped, usage categoriesed did file . instead of alphabet sort api
3.oc 如何生成 did 的?
使用 library/candid_gen/的 rs 代码
如 community 的罐子,在 mian.rs 使用 generate_candid_method macro 生成了 did 文件?
part of oc rs code have main.rs .whats for ? run local to gen did in dev workspace?
以及 candid 库的: export_service()
也许 cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/backend.did
这个 candid-extractor 也会用到 candid::export_service() ?
并没有:用到的是:impl Parse for SystemAPI
export_service() 生成的文件在哪?

## goal:run UTest and ITest and Log??

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

logging: keywords: info! , canister_logger::init_with_logs(data.test_mode, logs, traces);
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
IMPORTANT . pocket-ic is a binary file that run directly. dfx is another binary file .
They both provide a env for canister .but different env.

rs code with pocketIC: #[macro_export]
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
// go pub fn create_channel() may have some clue.

// Updates
generate_update_call!(add_reaction);
generate_update_call!(block_user);



Commit: Refactor existing utility functions in state_machine.rs, adding new functionalities such as stable memory management and a safe upgrading mechanism to ensure backward data compatibility. Enhance Error Handling:
Commit: Implement more descriptive error messages and structured logging to facilitate debugging and traceability.


Serialization process:
rust instance -> json ,xml,ymal,...
rust instance ->  CBOR(Concise Binary Object Representation) , do this for good storage space maybe ?




Learn from deserailization procedure:

1. you can ignore unknown field when deserialization.

TODO:
Q: if rust instance is struct S(A:int,B:int) then do serialize.  then the struct S becomde(A:string,B:int) 
how to do err handling when deserializing ?



#[derive(Debug, Clone, CandidType,Serialize,  Deserialize)]
pub struct NeuronProfile {
  pub owner: Principal, // 用户 Principal
  pub name: String,
  pub id: u64,
  pub address: String,
  pub create_time: u64,

}
#[derive(Debug, Clone, CandidType,Serialize,  Deserialize)]
pub struct NeuronProfile {
  pub owner: Principal, // 用户 Principal
  pub name: String,
  pub id: u64,
  pub address: String,
  pub create_time: u64,

  // newlly add . update with DB deserilize err. the odd bin DB file fail to find this field . shoud ignore it .
  pub update_time: u64,
}
 how to let serde deserialize this with new data structure added when using the old data structure serialized missing update_time field. 


Senerios: how to handle all those serde issue ? not language specific. a convert problem. 


rs: A(B:int,C:string) -> serialized data(json ,CBOR,...):  -> deserialize: A(B:int,C:string)  [OK]


rs: A(B:int,C:string) -> serialized data(json ,CBOR,...):  -> deserialize as new rs data structure: A(B:int,C:int)  [?] [rs code field type changed ],...
so modify the middle json file. edit the C field.


rs: A(B:int,C:string) -> serialized data(json ,CBOR,...):  -> deserialize as new rs data structure: A(B:int,C:string,D:int)  [?] rs code field added ,...
let rust auto give default field value for D . 

rs: A(B:int,C:string) -> serialized data(json ,CBOR,...):  -> deserialize as new rs data structure: A(B:int)  [?] rs code field removed 
let rust ignore that when no match. this doing by default automatic.




#[serde(deny_unknown_fields)] ?


Now can you generic these two approach into rust code of serialize and deserialize as a whole file ,like materialized mode(batch mode). or using a kind of File stream . to make it like volcano process(or pipeline iterator) ?
Is pipeline mode better than batch mode if the file is huge when deal with serde?




How to get transaction history of a sns instance coin :

user input: 
  sns coin name,
  sns wallet addr,

ouput:
  transaction history .

How to get coin history price?
input: 
  sns coin name,
  transaction time,
 
 output:
  coin price at transaction time.



TODO 线上数据库备份方式:
1. 开一个新的git分支,分支为上次部署ic的后端代码版本.并切换到这个分支 **如何找到这个时间点分支还不知道**
    使用` dfx canister status backend --network ic ` 没有看到版本信息.
    TODO 最好有一个手动的tag指示当前代码版本和日期. 此处可以参考openchat.

    现在测试这个branch是否可用: 
    `git log -S 'do_pre_upgrade_and_print_db' -- /home/btwl/code/ic/tax_lint/backend/canisters/backend/src/common/api.rs` 
    `git log -S 'do_post_upgrade' -- /home/btwl/code/ic/tax_lint/backend/canisters/backend/src/common/api.rs` 
    `git checkout -b prod_db_backup 797e08350bdbf4cb129ac9bfc3945df52442c930`

2. 加入现有写好的dropbox接口代码.
3. 升级ic线上版本.
4. 执行备份stable data的操作到dropbox去

5. 切换回到main分支
6. 升级ic上的后端代码.并清空ic上的stable data
7. 调用dropbox api来复原线上数据.



TODO: 逻辑上死亡卡住了在当前线上版本的preupgrade环节.  可能尝试management canister 等api 来获取当前 ic 上面的罐子的stable mem.
https://forum.dfinity.org/t/any-possibility-to-check-the-latest-wasm-code-install-time-on-main-ic-net/27682
TODO: 了解uninstall_code 的api是否会导致stable mem 被删除. 如果不会. 则选择force uninstall 然后install最新版本代码. **尤其注意install这个地方一定不要有对stable mem的任何写入**!
如果文档或者QA没有明写.可以自行开个新的测试项目.
  rust层: 做一些简单的stable_save 和 stable_restore接口. 
  rust或者dfx层: uninstall 和install 接口

# 备份线上数据库和还原
目前的线上代码执行过一次pre_upgrade. 那个pre_upgrade代码就是上上次的代码,即下面的commit,检查了是ok的.也必定是执行成功才能部署到目前的线上代码.
这个分支是距离最近线上代码的上一次代码. 表示这个代码的pre_upgrade是执行完成了的. 这个commit是从dfx命令查询canister_info得到的. 这个pre_upgrade正确得存储了paylaod到stable里面.
只执行过一次. 那就是最近一次成功部署后端的时候. 因此目前线上的stable数据是ok的.
git checkout -b prod_db_backup_3_dec_22 56b89a70eb70ad51ed2ba2b2d46f6d4886e5911e

1. 现在手动选择skip_pre_install来install当前最新后端代码.
```bash
# skip_pre_upgrade flag on the install_code method, it will skip the pre_upgrade method on the canister. It only works to save the stable-memory. Any global variables in the main canister memory (wasm heap) will be lost.
dfx canister \
--ic \ 
call aaaaa-aa canister_info\
 '(record {
  canister_id = principal "v7g7o-oiaaa-aaaag-qcj3q-cai"; 
  num_requested_changes = opt 5 : opt nat64
  })'\
  --wallet $(dfx identity --ic get-wallet) \
  --candid /home/btwl/code/ic/tax_lint/backend/canisters/backend/my_tests/test_in_cmd/dfx_calls/manage_can.did

```
2. 再使用dropbox的 to dropbox api 和from dropbox api 来尝试复原heap数据.
```bash
# 

# 0: collect_running_payload.  1: use stable mem
dfx canister call  backend save_payload_to_dropbox '("sl.BxPmJ_Y5qKXvWPtfPwon2tAIuGG-mkXQ0BT_c-13SAcN2Fv7jZOBpKKodcBHdULtrtC0OU7b1SUFQ5J0n-NcKOHNqa_D_Xoa-w2qwfq7U04c9rlqaPi_pzUpTQ2dy-3CL8RFB5KnKlr1-5cWxz0PddM", 0 )' --ic

dfx canister call  backend set_payload_using_dropbox --ic

dfx canister call  backend set_stable_mem_use_payload_simple --ic
```

3. 如果都执行成功.再升级一次代码启用 pre 和 post的2个hook





  
