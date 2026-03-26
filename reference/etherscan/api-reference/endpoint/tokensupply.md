> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get ERC20-Token TotalSupply by ContractAddress

> Returns the amount of an ERC-20 token in circulation.

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

<ParamField query="action" type="string" default="tokensupply">
  Set to `tokensupply` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x57d90b64a1a57749b0f932f1a3395792e12e7055">
  Contract address of the ERC-20 token.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=stats&action=tokensupply&contractaddress=0x57d90b64a1a57749b0f932f1a3395792e12e7055&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "21265524714464"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).