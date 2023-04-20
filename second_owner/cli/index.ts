import * as web3 from '@solana/web3.js';

import { getCounter, increase, createCounter, changeOwnership, zeroOutAccount } from './functions';
import { Counter } from './types';

const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // write your code here
  let firstOwner = new web3.PublicKey(
    'EbqooCAkrkKqSXDvDb7AzCb6tKbzZMaav3DQpnqA7GWn'
  );

  let secondOwner = new web3.PublicKey(
    'JDe4d7PftELw9mG6qWTGrijDDC8KxvvCcMP2K2U9Dhyx'
  );

  let counterAddress = new web3.PublicKey("8bDKnSNM3HPzfPgXu5nUxksSjBgGJjTGVkQH9ExRjFrr")

  // await increase(counterAddress, connection);
  // console.log(await getCounter(counterAddress, connection));
  
  await zeroOutAccount(connection, counterAddress);
  await changeOwnership(connection, counterAddress, firstOwner);
  console.log(await getCounter(counterAddress, connection));

  // await increase(counterAddress, connection);
  // await increase(counterAddress, connection);
  // await increase(counterAddress, connection);
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
