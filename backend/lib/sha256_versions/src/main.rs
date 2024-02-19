use git2::Repository;
use sha2::{Digest, Sha256};

fn main() {
  let local_repo_path = String::from("/home/btwl/code/hash_find/testings/repo");
  let result_path =
    String::from("/home/btwl/code/hash_find/testings/result.txt");
  let repo = open_repo(&local_repo_path);
  let versions = get_versions_with_date(&repo);
  for version in versions {
    switch_version(version.0, &repo);
    let wasm = compile(local_repo_path.clone());
    let _ = compute_hash_and_diff(&wasm, result_path.clone(), version.1);
  }
}

use std::env;

fn open_repo(repo_path: &str) -> Repository {
  let repo_url = "https://github.com/TaxLintDAO/taxlint";

  // Set the proxy environment variables
  env::set_var("http_proxy", "http://127.0.0.1:25526");
  env::set_var("https_proxy", "http://127.0.0.1:25526");

  // Check if a repository already exists at the given path
  match Repository::open(repo_path) {
    Ok(repo) => repo, // If repository exists, return it
    Err(_) => {
      // If repository does not exist, clone it
      match Repository::clone(repo_url, repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repository: {}", e), /* If cloning fails, panic */
      }
    }
  }
}

use std::fs::OpenOptions;
use std::process::Command;
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
    return Vec::new();
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
fn compute_hash_and_diff(
  wasm: &Vec<u8>,
  result_location: String,
  commit_time: String,
) -> std::io::Result<()> {
  let sha = Sha256::digest(&wasm);
  let sha_string = format!("{:x}", sha);

  let latest_on_ic_version = String::from(
    "bad927a539cb743c1d44371c2e841c866e524a803b1db9a4c77035ab15c6d74e",
  );

  let comparison_result = if sha_string == latest_on_ic_version {
    "MATCH FIND!!! wohoo!!!"
  } else {
    "not match"
  };
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(result_location)
    .unwrap();

  writeln!(
    file,
    "Commit Time: {}    Computed Hash: {}    Latest Version on IC: {}    Comparison Result: {}\n",
    commit_time, sha_string, latest_on_ic_version, comparison_result
  )?;
  file.flush()?; // Ensure data is written immediately

  Ok(())
}

use chrono::{DateTime, Utc};
#[allow(deprecated)]
fn get_versions_with_date(repo: &Repository) -> Vec<(String, String)> {
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
      Ok(cid) => {
        let commit = repo.find_commit(cid).unwrap();
        let time = DateTime::<Utc>::from_utc(
          chrono::NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
          Utc,
        );
        let time_str = time.format("%Y-%m-%d %H:%M").to_string();
        versions.push((cid.to_string(), time_str));
      }
      Err(e) => println!("Failed to get commit: {}", e),
    }
  }
  versions
}
// Checkout the specific commit,this will change local file
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

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn test_sha_string() {
    let sha_string = String::from(
      "bad927a539cb743c1d44371c2e841c866e524a803b1db9a4c77035ab15c6d74e",
    );
    let latest_on_ic_version = String::from(
      "bad927a539cb743c1d44371c2e841c866e524a803b1db9a4c77035ab15c6d74e",
    );
    let comparison_result = if sha_string == latest_on_ic_version {
      "MATCH FIND!!! wohoo!!!"
    } else {
      "not match"
    };
    eprint!("{}", comparison_result);
    assert_eq!(
      sha_string, latest_on_ic_version,
      "The SHA strings do not match!"
    );
  }
}
