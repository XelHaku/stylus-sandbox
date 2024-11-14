import { JsonRpcProvider, Wallet, Contract } from "ethers";

// Define the RPC URL of your Ethereum node (local or hosted)
const rpcUrl = "http://127.0.0.1:8547"; // Replace with your node's URL if different
const provider = new JsonRpcProvider(rpcUrl);

// Your private key (make sure to keep it secure and never share it publicly)
const privateKey =
  "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"; // Replace with your actual private key

// Create a wallet instance with the private key and connect it to the provider
const signer = new Wallet(privateKey, provider);

// Now you can use this `signer` to interact with your contract
const COUNTER_ADDRESS = "0xe1080224b632a93951a7cfa33eeea9fd81558b5e";
const counterABI = [
  "function number() view returns (uint256)",
  "function setNumber(uint256 new_number)",
  "function mulNumber(uint256 new_number)",
  "function addNumber(uint256 new_number)",
  "function callIncrement()",
];

// Create a contract instance with the signer
const counter = new Contract(COUNTER_ADDRESS, counterABI, signer);

// Example function to check the current number
async function checkNumber() {
  const currentNumber = await counter.number();
  console.log("Current number:", currentNumber.toString());
}

async function callIncrement() {
  const tx = await counter.callIncrement();
  console.log("Transaction hash:", tx.hash);
}
await checkNumber().catch(console.error);
await callIncrement().catch(console.error);

await checkNumber().catch(console.error);
