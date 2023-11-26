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


<!-- todo -->

ic的storage库和ic-stable-memory是否调用的同样的底层存储?

1.openchat的存储应用案例是stable memory.

2.oc升级罐子的案例是用的ic-rs-agent配合一部分dfx::core的接口直接调用.来升级
还用到了获取本地wasm文件.说明一部分rs代码是跑在开发机的?跑的main.rs
另外类似的安装罐子也用的rs-agent

3.oc如何生成did的?





