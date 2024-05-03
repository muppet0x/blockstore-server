require("dotenv").config();
const { ethers } = require("hardhat");

const providerCache = {};

function getProvider(network, apiKey) {
    if (!providerCache[network]) {
        if (network === "rinkeby") {
            providerCache[network] = new ethers.providers.AlchemyProvider("rinkeby", apiKey);
        } else if (network === "mainnet") {
            providerCache[network] = new ethers.providers.AlchemyProvider("homestead", apiKey);
        } else {
            console.error("Unsupported network");
            return null;
        }
    }
    return providerCache[network];
}

async function main() {
    const privateKey = process.env.PRIVATE_KEY;
    const alchemyApiKey = process.env.ALCHEMY_API_KEY;
    const network = process.env.NETWORK;

    const provider = getProvider(network, alchemyApiKey);
    if (!provider) {
        return;
    }

    const wallet = new ethers.Wallet(privateKey, provider);

    console.log("Compiling the contract...");
    await hre.run("compile");

    const MyContract = await ethers.getContractFactory("MyContract");
    console.log("Deploying MyContract...");

    const myContract = await MyContract.connect(wallet).deploy();
    await myContract.deployed();
    console.log(`MyContract deployed to: ${myContract.address}`);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});