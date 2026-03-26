> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_estimateGas

> Estimate the gas required for a transaction.

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

<ParamField query="action" type="string" default="eth_estimateGas">
  Set to `eth_estimateGas` for this endpoint.
</ParamField>

<ParamField query="data" type="string" default="0x4e71d92d">
  Hash of the method signature and encoded parameters.
</ParamField>

<ParamField query="to" type="string" default="0xf0160428a8552ac9bb7e050d90eeade4ddd52843">
  The address to interact with.
</ParamField>

<ParamField query="value" type="string" default="0xff22">
  Value sent in this transaction, in hex.
</ParamField>

<ParamField query="gasPrice" type="string" default="0x51da038cc">
  Gas price in wei.
</ParamField>

<ParamField query="gas" type="string" default="0x5f5e0ff">
  Gas limit provided for the transaction, in hex.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_estimateGas&data=0x4e71d92d&to=0xf0160428a8552ac9bb7e050d90eeade4ddd52843&value=0xff22&gasPrice=0x51da038cc&gas=0x5f5e0ff&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0x66ac"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).