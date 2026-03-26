> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get ERC20 Token Transfers by Address

> Retrieves the list of ERC-20 token transfers made by a specified address, with optional filtering by token contract.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint
</ParamField>

<ParamField query="action" type="string" default="tokentx">
  Set to `tokentx` for this endpoint
</ParamField>

<ParamField query="contractaddress" type="string" default="0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2">
  The ERC20 token contract address to filter transfers by, eg `0xdac17f958d2ee523a2206206994597c13d831ec7` for USDT.
</ParamField>

<ParamField query="address" type="string" default="0x4e83362442b8d1bec281594cea3050c8eb01311c">
  The address to query, like `0xfefefefefefefefefefefefefefefefefefefefe`
</ParamField>

<ParamField query="startblock" type="integer" default="0">
  Starting block number to search from.
</ParamField>

<ParamField query="endblock" type="integer" default="9999999999">
  Ending block number to search to.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1">
  Number of transactions per page.
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
        "blockNumber": "4730207",
        "timeStamp": "1513240363",
        "hash": "0xe8c208398bd5ae8e4c237658580db56a2a94dfa0ca382c99b776fa6e7d31d5b4",
        "nonce": "406",
        "blockHash": "0x022c5e6a3d2487a8ccf8946a2ffb74938bf8e5c8a3f6d91b41c56378a96b5c37",
        "from": "0x642ae78fafbb8032da552d619ad43f1d81e4dd7c",
        "contractAddress": "0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2",
        "to": "0x4e83362442b8d1bec281594cea3050c8eb01311c",
        "value": "5901522149285533025181",
        "tokenName": "Maker",
        "tokenSymbol": "MKR",
        "tokenDecimal": "18",
        "transactionIndex": "81",
        "gas": "940000",
        "gasPrice": "32010000000",
        "gasUsed": "77759",
        "cumulativeGasUsed": "2523379",
        "input": "deprecated",
        "methodId": "0xbe040fb0",
        "functionName": "redeem()",
        "confirmations": "18737452"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).