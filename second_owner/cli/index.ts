import * as web3 from '@solana/web3.js';

import {
  getCounter,
  increase,
  createCounter,
  changeOwnership,
  zeroOutAccount,
} from './functions';
import { Counter } from './types';

const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // write your code here
  let firstOwner = new web3.PublicKey('');

  let secondOwner = new web3.PublicKey('');

  let counterAddress = new web3.PublicKey('');

  await increase(counterAddress, connection);
  console.log(await getCounter(counterAddress, connection));

  await zeroOutAccount(connection, counterAddress);
  await changeOwnership(connection, counterAddress, firstOwner);
  console.log(await getCounter(counterAddress, connection));
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
