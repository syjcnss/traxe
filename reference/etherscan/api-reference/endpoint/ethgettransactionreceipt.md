> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_getTransactionReceipt

> Get the receipt of a transaction by hash.

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

<ParamField query="action" type="string" default="eth_getTransactionReceipt">
  Set to `eth_getTransactionReceipt` for this endpoint.
</ParamField>

<ParamField query="txhash" type="string" default="0xadb8aec59e80db99811ac4a0235efa3e45da32928bcff557998552250fa672eb">
  The transaction hash to query.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getTransactionReceipt&txhash=0xadb8aec59e80db99811ac4a0235efa3e45da32928bcff557998552250fa672eb&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
      "blockHash": "0x07c17710dbb7514e92341c9f83b4aab700c5dba7c4fb98caadd7926a32e47799",
      "blockNumber": "0xcf2427",
      "contractAddress": null,
      "cumulativeGasUsed": "0xeb67d5",
      "effectiveGasPrice": "0x1a96b24c26",
      "from": "0x292f04a44506c2fd49bac032e1ca148c35a478c8",
      "gasUsed": "0xb41d",
      "logs": [
        {
          "address": "0xdac17f958d2ee523a2206206994597c13d831ec7",
          "topics": [
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            "0x000000000000000000000000292f04a44506c2fd49bac032e1ca148c35a478c8",
            "0x000000000000000000000000ab6960a6511ff18ed8b8c012cb91c7f637947fc0"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000000000013f81a6",
          "blockNumber": "0xcf2427",
          "transactionHash": "0xadb8aec59e80db99811ac4a0235efa3e45da32928bcff557998552250fa672eb",
          "transactionIndex": "0x122",
          "blockHash": "0x07c17710dbb7514e92341c9f83b4aab700c5dba7c4fb98caadd7926a32e47799",
          "logIndex": "0xdb",
          "removed": false
        }
      ],
      "logsBloom": "0x0000000000000000000000000000000000000000000000000000000000000400000000000400000000000000000001000000000000000000000000000000000000000000000000000000000800000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000001000000000110000000000000000000000000000000000000000000000000000000200100000000000000000000000000080000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "status": "0x1",
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "transactionHash": "0xadb8aec59e80db99811ac4a0235efa3e45da32928bcff557998552250fa672eb",
      "transactionIndex": "0x122",
      "type": "0x2"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).