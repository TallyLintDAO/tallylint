deploy on ic-chain doc :  
https://internetcomputer.org/docs/current/tutorials/deploy_sample_app#what-this-does
```bash
# 第一次部署到主网收费 1TC
dfx deploy --network ic --with-cycles 1000000000000
```




以后修改了代码要升级不用重新deploy指令,  
使用 upgrade指令, 要收费   
升级罐子指南:  
https://internetcomputer.org/docs/current/developer-docs/backend/motoko/upgrading  

cycles的收费规则:  
https://internetcomputer.org/docs/current/developer-docs/gas-cost

```bash
# 另外如果只执行 
dfx deploy
# 隐藏默认参数 --netwrok local . 就 不管有没有部署到ic主网上,都可以输入这个本地测试.

```



```
1 000 000 000 000
```

第一次部署到主网的前后端地址:
  Frontend canister via browser
    assets: https://x63j6-hqaaa-aaaal-acyoa-cai.icp0.io/
  Backend canister via Candid interface:
    vote1_backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=xz2pk-kiaaa-aaaal-acyoq-cai


```
# 生成后端文件供罐子使用: 
dfx deploy vote1_backend
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
        "vote1_backend"
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
