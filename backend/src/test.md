```
dfx canister  call backend  register_user  { memo : text; name : text; email : text }
dfx canister  call backend  register_user  '("text":memo1, zzzz:text, zzzz:text)'
dfx canister  call backend  register_user  '("memo1":text, "name1":text, "email1":text)'
dfx canister  call backend  greet btwl  
 dfx canister install  --mode upgrade backend
```