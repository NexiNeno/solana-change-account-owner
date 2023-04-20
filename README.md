# solana-change-account-owner
It is a common misconception that once a Solana account is created and its ownership assigned to a program, the owner cannot be changed again.

This repo was created to prove that ownership can be assigned indefinitely.

The only limitation to a new assignment is that the data field of the account MUST be zeroed out first.

## Usage
Pre-Requirements:
* solana cli
* nodeJS

### From the root folder:
1 - cd first_owner/cli && npm install && cd ../../second_owner/cli && npm install  
2 - solana-test-validator -r (This will start a local validator)  

### In another terminal:
3 - `cd first_owner/ && cargo build-sbf && solana program deploy target/deploy/first_owner.so --url localhost && cd ..`  
4 - Paste the returned programId into the PROGRAM_ID field in first_owner/cli/constants.ts  
5 - `cd second_owner/ && cargo build-sbf && solana program deploy target/deploy/second_owner.so --url localhost && cd ..`  
6 - Paste the returned programId into the PROGRAM_ID field in second_owner/cli/constants.ts  
7 - In both first_owner/cli/constant.ts, insert your test private key. Try: $cat ~/.config/solana/id.json (If you don't have one yet, run `solana-keygen new` first)  
8 - Update in both cli/index.ts the programId from firstOwner and secondOwner.   
9 - Open first_owner/cli/index.ts. You'll find function calls to send txs to the deployed contracts.  
10 - Execute first `createCounter()` from first_owner.   
11 - Comment and uncomment fns in one contract and the other to have a taste of what you can and cannot do changing account ownership.  
