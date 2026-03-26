> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Historical Native Balance for an Address

> Retrieves the balance of a specified address at a given block height.

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

<ParamField query="action" type="string" default="balancehistory">
  Set to `balancehistory` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae">
  The address to query, like `0xfefefefefefefefefefefefefefefefefefefefe`.
</ParamField>

<ParamField query="blockno" type="integer" default="8000000">
  Block number to check balance at, all the way up to the genesis block `0`.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "610538078574759898951277"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).