> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get ERC1155 Token Transfers by Address

> Retrieves a list of ERC-1155 tokens transferred by a specific address, with optional filtering by token contract.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="token1155tx">
  Set to `token1155tx` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x76be3b62873462d2142405439777e971754e8e77">
  The ERC1155 token contract address to filter transfers by, eg `0x495f947276749ce646f68ac8c248420045cb7b5e` for [Opensea Shared Storefront](https://etherscan.io/token/0x495f947276749ce646f68ac8c248420045cb7b5e).
</ParamField>

<ParamField query="address" type="string" default="0x83f564d180b58ad9a02a449105568189ee7de8cb">
  The address to query, like `0xfefefefefefefefefefefefefefefefefefefefe`
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1">
  Number of transactions per page.
</ParamField>

<ParamField query="startblock" type="integer" default="0">
  Starting block number to search from.
</ParamField>

<ParamField query="endblock" type="integer" default="9999999999">
  Ending block number to search to.
</ParamField>

<ParamField query="sort" type="string" default="desc">
  Sort order either `desc` for the latest transactions first or `asc` for the oldest transactions first.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "blockNumber": "13472395",
        "timeStamp": "1634973285",
        "hash": "0x643b15f3ffaad5d38e33e5872b4ebaa7a643eda8b50ffd5331f682934ee65d4d",
        "nonce": "41",
        "blockHash": "0xa5da536dfbe8125eb146114e2ee0d0bdef2b20483aacbf30fed6b60f092059e6",
        "transactionIndex": "100",
        "gas": "140000",
        "gasPrice": "52898577246",
        "gasUsed": "105030",
        "cumulativeGasUsed": "11739203",
        "input": "deprecated",
        "methodId": "0x3e6b214b",
        "functionName": "",
        "contractAddress": "0x76be3b62873462d2142405439777e971754e8e77",
        "from": "0x1e63326a84d2fa207bdfa856da9278a93deba418",
        "to": "0x83f564d180b58ad9a02a449105568189ee7de8cb",
        "tokenID": "10371",
        "tokenValue": "1",
        "tokenName": "parallel",
        "tokenSymbol": "LL",
        "confirmations": "9995266"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).