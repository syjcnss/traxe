> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Estimation of Confirmation Time

> Estimate confirmation time based on a provided gas price.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="gastracker">
  Set to `gastracker` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="gasestimate">
  Set to `gasestimate` for this endpoint.
</ParamField>

<ParamField query="gasprice" type="string" default="2000000000">
  Gas price paid per unit of gas, in wei.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=gastracker&action=gasestimate&gasprice=2000000000&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "45"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).