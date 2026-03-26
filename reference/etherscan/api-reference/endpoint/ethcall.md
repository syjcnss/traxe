> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_call

> Execute a call without creating a transaction.

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

<ParamField query="action" type="string" default="eth_call">
  Set to `eth_call` for this endpoint.
</ParamField>

<ParamField query="to" type="string" default="0xAEEF46DB4855E25702F8237E8f403FddcaF931C0">
  The address to interact with.
</ParamField>

<ParamField query="data" type="string" default="0x70a08231000000000000000000000000e16359506c028e51f16be38986ec5746251e9724">
  Hash of the method signature and encoded parameters.
</ParamField>

<ParamField query="tag" type="string" default="latest">
  Use `latest`, `earliest`, `pending`, or a block number in hex.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_call&to=0xAEEF46DB4855E25702F8237E8f403FddcaF931C0&data=0x70a08231000000000000000000000000e16359506c028e51f16be38986ec5746251e9724&tag=latest&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x00000000000000000000000000000000000000000000000000601d8888141c00"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).