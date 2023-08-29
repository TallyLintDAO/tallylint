为dfx.json文件写的注释.
.json文件里面不允许注释?

assets为前端文件
vote1_backend 为后端文件

这是默认的前端格式:
```json
    {
  "canisters": {
    "nns_helper_backend": {
      "candid": "src/nns_helper_backend/nns_helper_backend.did",
      "package": "nns_helper_backend",
      "type": "rust"
    },
    "nns_helper_frontend": {
      "dependencies": [
        "nns_helper_backend" // 这里我们的项目目前没有绑定过去
      ],
      "frontend": {
        "entrypoint": "src/nns_helper_frontend/src/index.html"
      },
      "source": [
        "src/nns_helper_frontend/assets",
        "dist/nns_helper_frontend/"
      ],
      "type": "assets"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "output_env_file": ".env",
  "version": 1
}
```