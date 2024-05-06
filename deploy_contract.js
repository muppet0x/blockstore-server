require("dotenv").config();
const { ethers } = require("hardhat");

const providerCache = {};

function getProvider(network, apiKey) {
    if (!providerCache[network]) {
        try {
            if (network === "rinkeby") {
                providerCache[network] = new ethers.providers.AlchemyProvider("rinkeby", apiKey);
            } else if (network === "mainnet") {
                providerCache[network] = new ethers.providers.AlchemyProvider("homestead", apiKey);
            } else {
                console.error("Unsupported network: " + network);
                return null;
            }
        } catch (error) {
            console.error("Failed to create provider for network: " + network + " with error: " + error.message);
            return null;
        }
    }
    return providerCache[network];
}

async function main() {
    const privateKey = process.env.PRIVATE_KEY;
    const alchemyApiKey = process.env.ALCHEMY_API_KEY;
    const network = process.env.NETWORK;

    if (!privateKey || !alchemyApiKey || !network) {
        console.error("Environment variables are not set correctly. PRIVATE_KEY, ALCHEMY_API_KEY, and NETWORK must be defined.");
        process.exit(1);
    }

    const provider = getProvider(network, alchemyApiKey);
    if (!provider) {
        console.error("Cannot continue without a provider. Exiting...");
        process.exit(1);
    }

    const wallet = new ethers.Wallet(privateKey, provider);

    try {
        console.log("Compiling the contract...");
        await hre.run("compile");

        const MyContract = await ethers.getContractFactory("MyContract");
        console.log("Deploying MyContract...");

        const myContract = await MyContract.connect(wallet).deploy();
        await myContract.deployed();
        console.log(`MyContract deployed to: ${myContract.address}`);
    } catch (error) {
        console.error("An error occurred during the contract deployment process: " + error.message);
        process.exit(1);
    }
}

main().catch((error) => {
    console.error("An unexpected error occurred: " + error.message);
    process.exit(1);
});