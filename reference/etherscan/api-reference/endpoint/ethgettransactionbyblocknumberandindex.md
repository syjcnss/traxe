> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_getTransactionByBlockNumberAndIndex

> Get transaction details by block number and index.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="proxy">
  Set to `proxy` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="eth_getTransactionByBlockNumberAndIndex">
  Set to `eth_getTransactionByBlockNumberAndIndex` for this endpoint.
</ParamField>

<ParamField query="tag" type="string" default="0xC6331D">
  Block number in hex format.
</ParamField>

<ParamField query="index" type="string" default="0x11A">
  Transaction index position in the block, in hex.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getTransactionByBlockNumberAndIndex&tag=0xC6331D&index=0x11A&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
      "blockHash": "0x13c5311a78b9a8790c58f1d0d0660226352a498cc98b06e1861f1967d3a9d1b4",
      "blockNumber": "0xc6331d",
      "from": "0xd1af036d589b6ebfbaa184b339a30d32ef611708",
      "gas": "0x11170",
      "gasPrice": "0x5b7e259a7",
      "maxFeePerGas": "0x5b7e259a7",
      "maxPriorityFeePerGas": "0x5b7e259a7",
      "hash": "0xc7ef51f0bfe85eefbb1d4d88f5a39e82fbfc94987d8cbcb515f74d80b6e44902",
      "input": "0x2d2da806000000000000000000000000d1af036d589b6ebfbaa184b339a30d32ef611708",
      "nonce": "0x0",
      "to": "0xabea9132b05a70803a4e85094fd0e1800777fbef",
      "transactionIndex": "0x11a",
      "value": "0x57930b5db6a000",
      "type": "0x2",
      "accessList": [],
      "chainId": "0x1",
      "v": "0x0",
      "r": "0x595619be17755f7427d07626275e4c89b8f9d5a0a668515165ea000d30f26325",
      "s": "0x33d4b22c7c779fc9f04c81788cb4268d2c3f8dc9e219245570bdd384c6ca6690",
      "yParity": "0x0"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).