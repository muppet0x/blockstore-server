from web3 import Web3
import os
from dotenv import load_dotenv
from functools import lru_cache

load_dotenv()

INFURA_URL = os.getenv('INFURA_URL')
CONTRACT_ADDRESS = os.getenv('CONTRACT_ADDRESS')
PRIVATE_KEY = os.getenv('PRIVATE_KEY')
ACCOUNT_ADDRESS = os.getenv('ACCOUNT_ADDRESS')

w3 = Web3(Web3.HTTPProvider(INFURA_URL))
if not w3.isConnected():
    raise Exception("Failed to connect to Ethereum blockchain")

with open('ContractABI.json', 'r') as file:
    contract_abi = file.read()

contract = w3.eth.contract(address=CONTRACT_ADDRESS, abi=contract_abi)

def upload_file_hash(file_hash, file_info):
    nonce = w3.eth.getTransactionCount(ACCOUNT_ADDRESS)
    tx = contract.functions.uploadFileHash(file_hash, file_info).buildTransaction({
        'chainId': 1,
        'gas': 2000000,
        'gasPrice': w3.toWei('50', 'gwei'),
        'nonce': nonce,
    })
    signed_tx = w3.eth.account.signTransaction(tx, PRIVATE_KEY)
    tx_hash = w3.eth.sendRawTransaction(signed_tx.rawTransaction)
    receipt = w3.eth.waitForTransactionReceipt(tx_hash)

    fetch_file_info.cache_clear()
    
    return receipt

@lru_cache(maxsize=None)
def fetch_file_info(file_hash):
    return contract.functions.getFileInformation(file_hash).call()

def verify_file_integrity(file_hash, original_info):
    blockchain_info = fetch_file_info(file_hash)
    return blockchain_info == original_info

if __name__ == "__main__":
    test_hash = "0x1234"
    test_info = "Some info about the file"
    
    receipt = upload_file_hash(test_hash, test_info)
    print("File hash uploaded:", receipt)
    
    fetched_info = fetch_file_info(test_hash)
    print("Fetched file info:", fetched_info)
    
    integrity_verified = verify_file_integrity(test_hash, test_info)
    print("File integrity verified:", integrity_verified)