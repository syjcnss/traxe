> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Internal Transactions by Address

> Retrieves the internal transaction history of a specified address, with optional pagination.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint
</ParamField>

<ParamField query="action" type="string" default="txlistinternal">
  Set to `txlistinternal` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x2c1ba59d6f58433fb1eaee7d20b26ed83bda51a3">
  The address to query, like `0xfefefefefefefefefefefefefefefefefefefefe`.
</ParamField>

<ParamField query="startblock" type="integer" default="0">
  Starting block number to search from.
</ParamField>

<ParamField query="endblock" type="integer" default="9999999999">
  Ending block number to search to.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1">
  Number of transactions per page.
</ParamField>

<ParamField query="sort" type="string" default="desc">
  Sort order either `desc` for the latest transactions first or `asc` for the oldest transactions first.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "blockNumber": "2535368",
        "timeStamp": "1477837690",
        "hash": "0x8a1a9989bda84f80143181a68bc137ecefa64d0d4ebde45dd94fc0cf49e70cb6",
        "from": "0x20d42f2e99a421147acf198d775395cac2e8b03d",
        "to": "",
        "value": "0",
        "contractAddress": "0x2c1ba59d6f58433fb1eaee7d20b26ed83bda51a3",
        "input": "",
        "type": "create",
        "gas": "254791",
        "gasUsed": "46750",
        "traceId": "0",
        "isError": "0",
        "errCode": ""
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).