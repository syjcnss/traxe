> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Withdrawal Transactions by Address

> Retrieves all withdrawal transactions made by a specified address.

export const chain = '1';

### Query Parameters

<Callout icon="globe" iconType="regular">This endpoint is only available for the Arbitrum and Optimism stack chains.</Callout>

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="10">
  Chain ID to query, eg `10` for Optimism or `42161` for Arbitrum from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getwithdrawaltxs">
  Set to `getwithdrawaltxs` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x80f3950a4d371c43360f292a4170624abd9eed03">
  Address to check for cross-chain withdrawals from L2 to Ethereum.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="10">
  Number of records per page.
</ParamField>

<ParamField query="sort" type="string" default="desc">
  Sort order either `desc` for the latest transactions first or `asc` for the oldest transactions first.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=10&module=account&action=getwithdrawaltxs&address=0x80f3950a4d371c43360f292a4170624abd9eed03&page=1&offset=10&sort=desc&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "0",
    "message": "No transactions found",
    "result": []
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).