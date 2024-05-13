require("dotenv").config();
const { ethers } = require("ethers");

let providerCache = {};

function getProvider(network, apiKey) {
    const networkName = network === "mainnet" ? "homestead" : network;
    if (!providerCache[network] && (network === "rinkeby" || network === "mainnet")) {
        try {
            providerCache[network] = new ethers.providers.AlchemyProvider(networkName, apiKey);
        } catch (error) {
            console.error(`Failed to create provider for network: ${network} with error: ${error.message}`);
            return null;
        }
    } else if (!providerCache[network]) {
        console.error("Unsupported network: " + network);
        return null;
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
        console.log("Deploying contract...");

        const MyContract = await ethers.getContractFactory("MyContract", wallet);
        const myContract = await MyContract.deploy();
        await myContract.deployed();

        console.log(`Contract deployed to: ${myContract.address}`);
    } catch (error) {
        console.error(`An error occurred during the deployment process: ${error.message}`);
        process.exit(1);
    }
}

main().catch((error) => {
    console.error(`An unexpected error occurred: ${error.message}`);
    process.exit(1);
});