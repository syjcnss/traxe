> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_getStorageAt

> Get the value at a storage position.

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

<ParamField query="action" type="string" default="eth_getStorageAt">
  Set to `eth_getStorageAt` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x6e03d9cce9d60f3e9f2597e13cd4c54c55330cfd">
  The address to query.
</ParamField>

<ParamField query="position" type="string" default="0x0">
  Storage position in hex.
</ParamField>

<ParamField query="tag" type="string" default="latest">
  Use `latest`, `earliest`, `pending`, or a block number in hex.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getStorageAt&address=0x6e03d9cce9d60f3e9f2597e13cd4c54c55330cfd&position=0x0&tag=latest&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x0000000000000000000000000000000000000000000000000000000000000000"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).