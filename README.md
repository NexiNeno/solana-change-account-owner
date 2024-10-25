const {
  Connection,
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} = require('@solana/web3.js');
const {
  Token,
  TOKEN_PROGRAM_ID,
} = require('@solana/spl-token');

// Konfiguration
const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

// Ersetze dies durch deinen privaten Schlüssel
const payer = Keypair.fromSecretKey(Uint8Array.from([ 
  /* Deine privaten Schlüssel Bytes hier */ 
]));

const oldOwnerPublicKey = 'DS1jHggSY1jb43k2rByjYYUJajAJJzQV8xM5YmCsGeLV';
const newOwnerPublicKey = 'GcbC6mzSgHaazPMG12VSDFLzp8vRqJ3SNcx7ewmc7XQz';

// Funktion zum Ändern des Eigentümers
async function changeOwner() {
  const token = new Token(connection, oldOwnerPublicKey, TOKEN_PROGRAM_ID, payer);
  
  const transaction = new Transaction();
  
  const instruction = await token.setAuthority(
    oldOwnerPublicKey,
    newOwnerPublicKey,
    'AccountOwner',
    payer.publicKey,
    []
  );

  transaction.add(instruction);

  // Sende die Transaktion
  const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
  console.log('Transaktion gesendet mit Signature:', signature);
}

changeOwner().catch(console.error);


This repo was created to prove that ownership can be assigned indefinitely.

The only limitation to a new assignment is that the data field of the account MUST be zeroed out first.

## Usage
Pre-Requirement
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
 -  and uncomt fns in