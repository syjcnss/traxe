> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Historical ERC20-Token Account Balance by BlockNo

> Retrieves the ERC-20 token balance for a specified address at a given block height.

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>

<Warning>This endpoint is throttled to **2 calls/second** regardless of API Pro tier.</Warning>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="tokenbalancehistory">
  Set to `tokenbalancehistory` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x57d90b64a1a57749b0f932f1a3395792e12e7055">
  Contract address of the ERC-20 token.
</ParamField>

<ParamField query="address" type="string" default="0xe04f27eb70e025b78871a2ad7eabe85e61212761">
  Address to check for balance.
</ParamField>

<ParamField query="blockno" type="integer" default="8000000">
  Block number to check balance for.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=account&action=tokenbalancehistory&contractaddress=0x57d90b64a1a57749b0f932f1a3395792e12e7055&address=0xe04f27eb70e025b78871a2ad7eabe85e61212761&blockno=8000000&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "135499"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).