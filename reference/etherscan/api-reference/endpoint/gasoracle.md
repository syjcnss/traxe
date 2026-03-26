> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Gas Oracle

> Get current gas price recommendations.

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

<ParamField query="action" type="string" default="gasoracle">
  Set to `gasoracle` for this endpoint.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=gastracker&action=gasoracle&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "LastBlock": "23467872",
      "SafeGasPrice": "0.496839934",
      "ProposeGasPrice": "0.496840168",
      "FastGasPrice": "0.55411917",
      "suggestBaseFee": "0.496839934",
      "gasUsedRatio": "0.405942555555556,0.784013777777778,0.502624148542588,0.719479644444444,0.45673524947105"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).