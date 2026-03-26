> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Internal Transactions by Block Range

> Returns internal transactions within a specified block range, with optional pagination.

export const chain = '1';

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

<ParamField query="action" type="string" default="txlistinternal">
  Set to `txlistinternal` for this endpoint.
</ParamField>

<ParamField query="startblock" type="integer" default="13481773">
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
        "blockNumber": "13481773",
        "timeStamp": "1635100060",
        "hash": "0x8b440f5ec0e986589517b131c5b75e921f2768b9912f028d2cdb009d759036ab",
        "from": "0x3909336de913344701c6f096502d26208210b39f",
        "to": "0xff62dfadca3b5643d0b283571fe154d886580c0c",
        "value": "1159078546481168231",
        "contractAddress": "",
        "input": "",
        "type": "call",
        "gas": "101300",
        "gasUsed": "13898",
        "traceId": "3",
        "isError": "0",
        "errCode": ""
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).