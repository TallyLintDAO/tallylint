.dfx 文件夹下的部署文件还未生成.dont know if the gitignore is right yet.


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
