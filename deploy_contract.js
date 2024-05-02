require("dotenv").config();
const { ethers } = require("hardhat");

async function main() {
    const privateKey = process.env.PRIVATE_KEY;
    const alchemyApiKey = process.env.ALCHEMY_API_KEY;
    const network = process.env.NETWORK;

    let provider;
    if (network === "rinkeby") {
        provider = new ethers.providers.AlchemyProvider("rinkeby", alchemyApiKey);
    } else if (network === "mainnet") {
        provider = new ethers.providers.AlchemyProvider("homestead", alchemyApiKey);
    } else {
        console.error("Unsupported network");
        return;
    }

    const wallet = new ethers.Wallet(privateKey, provider);

    console.log("Compiling the contract...");
    await hre.run("compile");

    const MyContract = await ethers.getContractFactory("MyContract");
    console.log("Deploying MyContract...");

    const myContract = await MyContract.connect(wallet).deploy(
        
    );

    await myContract.deployed();
    console.log(`MyContract deployed to: ${myContract.address}`);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});