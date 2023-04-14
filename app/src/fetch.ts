import { Connection, PublicKey } from '@solana/web3.js';
import * as dotenv from 'dotenv';

import { PROGRAM_ID } from 'config';
import { getProgram, getProvider, loadWallet } from 'utils';

dotenv.config();

const main = async () => {
  const connection = new Connection(process.env.ALCHEMY_SOL_DEV_HTTPS!);
  const wallet = loadWallet();

  const provider = getProvider(connection, wallet);
  const program = getProgram(provider, PROGRAM_ID);

  const RESTAURANT = 'restaurant 2';

  // deterministically get PDA of review
  const [REVIEW_PDA] = PublicKey.findProgramAddressSync(
    [Buffer.from(RESTAURANT), wallet.publicKey.toBuffer()],
    program.programId
  );
  console.log(REVIEW_PDA);

  // fetch review PDA
  const data = await program.account.review.fetch(REVIEW_PDA);
  console.log(data);
};

main();
