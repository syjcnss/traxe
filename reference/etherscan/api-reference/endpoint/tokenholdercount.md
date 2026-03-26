> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Token Holder Count by Contract Address

> Retrieves a simple count of ERC-20 token holders.

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="token">
  Set to `token` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="tokenholdercount">
  Set to `tokenholdercount` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0xaaaebe6fe48e54f431b0c390cfaf0b017d09d42d">
  Contract address of the ERC-20 token.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=token&action=tokenholdercount&contractaddress=0xaaaebe6fe48e54f431b0c390cfaf0b017d09d42d&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "30506"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).