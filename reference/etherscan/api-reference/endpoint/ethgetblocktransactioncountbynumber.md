> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_getBlockTransactionCountByNumber

> Get the number of transactions in a block.

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

<ParamField query="action" type="string" default="eth_getBlockTransactionCountByNumber">
  Set to `eth_getBlockTransactionCountByNumber` for this endpoint.
</ParamField>

<ParamField query="tag" type="string" default="0x10FB78">
  Block number in hex format.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getBlockTransactionCountByNumber&tag=0x10FB78&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x3"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).