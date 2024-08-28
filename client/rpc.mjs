import {
    Connection,
    PublicKey,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";

const programId = new PublicKey("3YeTvPaMWJNBnGdPQ3FwYRo8FWty8cvFyBevXmNfJUvc");

// Connect to a solana cluster. Either to your local test validator or to devnet
const connection = new Connection("http://localhost:8899", "confirmed");
//const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// We load the keypair that we created in a previous step
const keyPair = await getKeypairFromFile("~/.config/solana/id.json");

const blockhashInfo = await connection.getLatestBlockhash();
 
// Create a new transaction
const tx = new Transaction({
  ...blockhashInfo,
});
 
// Add our Hello World instruction
tx.add(
  new TransactionInstruction({
    programId: programId,
    keys: [],
    data: Buffer.from([]),
  }),
);
 
// Sign the transaction with your previously created keypair
tx.sign(keyPair);
 
// Send the transaction to the Solana network
const txHash = await connection.sendRawTransaction(tx.serialize());
 
console.log("Transaction sent with hash:", txHash);
 
await connection.confirmTransaction({
  blockhash: blockhashInfo.blockhash,
  lastValidBlockHeight: blockhashInfo.lastValidBlockHeight,
  signature: txHash,
});
 
const parsed = await connection.getParsedTransaction(txHash);
const data = parsed.meta.returnData;

const buf = Buffer.from(data.data[0], 'base64');
const message = buf.toString();

console.log(message);

// Custom return value -- That actually worked. Baller.