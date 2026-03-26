> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_getCode

> Get the code stored at an address.

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

<ParamField query="action" type="string" default="eth_getCode">
  Set to `eth_getCode` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0xf75e354c5edc8efed9b59ee9f67a80845ade7d0c">
  The address to query.
</ParamField>

<ParamField query="tag" type="string" default="latest">
  Use `latest`, `earliest`, `pending`, or a block number in hex.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getCode&address=0xf75e354c5edc8efed9b59ee9f67a80845ade7d0c&tag=latest&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x3660008037602060003660003473273930d21e01ee25e4c219b63259d214872220a261235a5a03f21560015760206000f3"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).