> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Address ERC721 Token Holding

> Retrieves the ERC-721 tokens and their quantities owned by a specific address.

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

<ParamField query="action" type="string" default="addresstokennftbalance">
  Set to `addresstokennftbalance` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x6b52e83941eb10f9c613c395a834457559a80114">
  Address to check for NFT holdings.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="100">
  Number of records per page.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=account&action=addresstokennftbalance&address=0x6b52e83941eb10f9c613c395a834457559a80114&page=1&offset=100&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "0",
    "message": "NOTOK",
    "result": "Maximum rate limit reached"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).