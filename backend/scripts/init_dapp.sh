#!/bin/bash
# 使用前提:有dfx和npm 两个工具, 脚本不帮助检查这两个
# 使用方法:# ./init.sh "vote1" 则初始化一个dapp名为vote1并启动

name=$1
# name = $1  如果有空格,会认为是在执行name指令 , 流汗黄豆..
dfx --version
dfx new --type=rust $name

cd $name
dfx start --clean --background
npm install
dfx deploy

# ./init.sh "vote1"
# ./init.sh vote1
