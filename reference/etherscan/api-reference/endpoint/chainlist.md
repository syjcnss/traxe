> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Chainlist

> Returns the list of all supported Etherscan mainnet and testnet chains.

### Query Parameters

No parameters required.

<ResponseExample>
  ```json Response theme={null}
  {
    "comments": "List of API endpoints maintained by Etherscan EAAS. Available Status codes are (0)=Offline, (1)=Ok, (2)=Degraded",
    "totalcount": 67,
    "result": [
      {
        "chainname": "Ethereum Mainnet",
        "chainid": "1",
        "blockexplorer": "https://etherscan.io/",
        "apiurl": "https://api.etherscan.io/v2/api?chainid=1",
        "status": 1,
        "comment": ""
      },
      {
        "chainname": "Sepolia Testnet",
        "chainid": "11155111",
        "blockexplorer": "https://sepolia.etherscan.io/",
        "apiurl": "https://api.etherscan.io/v2/api?chainid=11155111",
        "status": 1,
        "comment": ""
      },
      {
        "chainname": "Base Mainnet",
        "chainid": "8453",
        "blockexplorer": "https://basescan.org/",
        "apiurl": "https://api.etherscan.io/v2/api?chainid=8453",
        "status": 1,
        "comment": ""
      },
      {
        "chainname": "Polygon Mainnet",
        "chainid": "137",
        "blockexplorer": "https://polygonscan.com/",
        "apiurl": "https://api.etherscan.io/v2/api?chainid=137",
        "status": 1,
        "comment": ""
      },
      {
        "chainname": "Arbitrum One Mainnet",
        "chainid": "42161",
        "blockexplorer": "https://arbiscan.io/",
        "apiurl": "https://api.etherscan.io/v2/api?chainid=42161",
        "status": 1,
        "comment": ""
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).