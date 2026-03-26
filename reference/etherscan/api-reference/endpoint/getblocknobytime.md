> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Block Number by Timestamp

> Retrieves the block number mined at a specific timestamp.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID you query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="block">
  Set to `block` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getblocknobytime">
  Set to `getblocknobytime` for this endpoint.
</ParamField>

<ParamField query="timestamp" type="integer" default="1578638524">
  Unix timestamp in seconds.
</ParamField>

<ParamField query="closest" type="string" default="before">
  Closest available block to the provided timestamp, either `before` or `after`.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=block&action=getblocknobytime&timestamp=1578638524&closest=before&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "9251482"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).