use git2::Repository;
use sha2::{Digest, Sha256};

// TODO : use git history to gen wasm file and gen sha256 and compare to
// online prod hash:
// 0xbad927a539cb743c1d44371c2e841c866e524a803b1db9a4c77035ab15c6d74e be careful
// with 0x start. use code to copy every git commit changed backend folder code
// to gen a new folder and compile to wasm so need 1. control git using rust.
// 2.compile a rust proj using rust code .
fn main() {
  let repo_path = String::from(
    "/home/btwl/code/ic/tax_lint/backend/lib/sha256_versions/src/testings",
  );

  let repo = download_repo();

  let versions = get_versions(&repo);

  for version in versions {
    switch_version(version, &repo);
    let wasm = compile(repo_path.clone());
    let _ = compute_hash_and_diff(&wasm);
  }
}

fn download_repo() -> Repository {
  let repo_url = "https://github.com/TaxLintDAO/taxlint";
  let repo_path = String::from(
    "/home/btwl/code/ic/tax_lint/backend/lib/sha256_versions/src/testings",
  );
  // Clone the repository to a specific path
  let repo = Repository::clone(repo_url, repo_path.clone())
    .expect("Failed to clone repository");
  return repo;
}

use std::{fs::File, process::Command};
fn compile(repo_path: String) -> Vec<u8> {
  let output = Command::new("cargo")
    .current_dir(repo_path.clone())
    .args(&[
      "build",
      "--release",
      "--target",
      "wasm32-unknown-unknown",
      "--package",
      "backend",
    ])
    .output()
    .expect("Failed to execute command");

  if output.status.success() {
    println!("Command executed successfully");
  } else {
    eprintln!("Command execution failed");
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
  }
  let wasm_path = format!(
    "{}/target/wasm32-unknown-unknown/release/backend.wasm",
    repo_path
  );
  let ret = std::fs::read(&wasm_path);
  if ret.is_err() {
    panic!("read file not exsit");
  }
  let wasm = ret.unwrap();
  return wasm;
}

use std::io::Write;
fn compute_hash_and_diff(wasm: &Vec<u8>) -> std::io::Result<()> {
  let sha = Sha256::digest(&wasm);
  let sha_string = format!("{:x}", sha);

  let latest_on_ic_version = String::from(
    "0xbad927a539cb743c1d44371c2e841c866e524a803b1db9a4c77035ab15c6d74e",
  );

  let comparison_result = if sha_string == latest_on_ic_version {
    "MATCH FIND!!! wohoo!!!"
  } else {
    "not match"
  };

  let dst = "/home/btwl/code/ic/tax_lint/backend/lib/sha256_versions/src/testings/result.txt";
  let mut file = match File::create(&dst) {
    Err(why) => panic!("couldn't create file, {}", why),
    Ok(file) => file,
  };
  writeln!(
    file,
    "Computed Hash: {}    Latest Version on IC: {}    Comparison Result: {}\n",
    sha_string, latest_on_ic_version, comparison_result
  )?;

  Ok(())
}

fn get_versions(repo: &Repository) -> Vec<String> {
  let mut revwalk = match repo.revwalk() {
    Ok(rw) => rw,
    Err(e) => {
      println!("Failed to get revwalk: {}", e);
      return Vec::new();
    }
  };

  if let Err(e) = revwalk.push_head() {
    println!("Failed to push head: {}", e);
    return Vec::new();
  }

  let mut versions = Vec::new();
  for id in revwalk {
    match id {
      Ok(oid) => versions.push(oid.to_string()),
      Err(e) => println!("Failed to get commit: {}", e),
    }
  }
  versions
}

// Checkout the specific commit
fn switch_version(commit_hash: String, repo: &Repository) {
  let object = repo
    .revparse_single(&commit_hash)
    .expect("Failed to parse hash");
  repo
    .checkout_tree(&object, None)
    .expect("Failed to checkout tree");
  repo
    .set_head_detached(object.id())
    .expect("Failed to detach head");
}
