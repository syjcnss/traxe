> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Total Nodes Count

> Retrieves the total count of discoverable Ethereum nodes.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="stats">
  Set to `stats` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="nodecount">
  Set to `nodecount` for this endpoint.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=stats&action=nodecount&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "UTCDate": "2025-09-25",
      "TotalNodeCount": "11786"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).