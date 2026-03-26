> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get ERC721 Token Transfers by Address

> Retrieves the list of ERC-721 token transfers made by a specified address, with optional filtering by token contract.

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

<ParamField query="action" type="string" default="tokennfttx">
  Set to `tokennfttx` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x06012c8cf97bead5deae237070f9587f8e7a266d">
  The ERC721 token contract address to filter transfers by, eg `0xbd3531da5cf5857e7cfaa92426877b022e612cf8` for [Pudgy Penguins](https://etherscan.io/token/0xbd3531da5cf5857e7cfaa92426877b022e612cf8).
</ParamField>

<ParamField query="address" type="string" default="0x6975be450864c02b4613023c2152ee0743572325">
  Address to filter token transfers by.
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
        "blockNumber": "4708120",
        "timeStamp": "1512907118",
        "hash": "0x031e6968a8de362e4328d60dcc7f72f0d6fc84284c452f63176632177146de66",
        "nonce": "0",
        "blockHash": "0x4be19c278bfaead5cb0bc9476fa632e2447f6e6259e0303af210302d22779a24",
        "from": "0xb1690c08e213a35ed9bab7b318de14420fb57d8c",
        "contractAddress": "0x06012c8cf97bead5deae237070f9587f8e7a266d",
        "to": "0x6975be450864c02b4613023c2152ee0743572325",
        "tokenID": "202106",
        "tokenName": "CryptoKitties",
        "tokenSymbol": "CK",
        "tokenDecimal": "0",
        "transactionIndex": "81",
        "gas": "158820",
        "gasPrice": "40000000000",
        "gasUsed": "60508",
        "cumulativeGasUsed": "4880352",
        "input": "deprecated",
        "methodId": "0x454a2ab3",
        "functionName": "bid(uint256 _tokenId)",
        "confirmations": "18759540"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).