#!/bin/bash
set -ex

echo "开始设置 RUSTFLAGS..."
export RUSTFLAGS=$RUSTFLAGS' -C target-feature=+simd128'
echo "RUSTFLAGS 设置完成: $RUSTFLAGS"

echo "开始构建项目..."
cargo build --release --target=wasm32-unknown-unknown -p backend --locked
if [ $? -ne 0 ]; then
    echo "Cargo 构建失败！"
    exit 1
else
    echo "Cargo 构建成功。"
fi

echo "开始生成 did 文件..."
candid-extractor ./target/wasm32-unknown-unknown/release/backend.wasm > ./backend/canisters/backend/backend.did
if [ $? -ne 0 ]; then
    echo "did 生成失败！"
    exit 1
else
    echo "did 生成成功。"
fi

# 检查 backend.did 文件内容
if ! grep -q "backup_data" ./backend/canisters/backend/backend.did; then
    echo "backend.did 文件中缺少 backup_data 方法！"
    exit 1
fi

if ! grep -q "restore_data" ./backend/canisters/backend/backend.did; then
    echo "backend.did 文件中缺少 restore_data 方法！"
    exit 1
fi

echo "脚本执行完成！"