use sha2::{Digest, Sha256};
// sha2 = "0.10.8"
// TODO : use git history to gen wasm file and gen sha256 and compare to 
// online prod hash: 0xbad927a539cb743c1d44371c2e841c866e524a803b1db9a4c77035ab15c6d74e
// be careful with 0x start.
// use code to copy every git commit changed backend folder code to gen a new folder and compile to wasm 
// so need 1. control git using rust.  2.compile a rust proj using rust code .
let sha=Sha256::digest(xx.wasm);
