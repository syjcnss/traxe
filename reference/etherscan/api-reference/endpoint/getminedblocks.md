> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Blocks Validated by Address

> Retrieves the list of blocks validated by a specified address.

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

<ParamField query="action" type="string" default="getminedblocks">
  Set to `getminedblocks` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x9dd134d14d1e65f84b706d6f205cd5b1cd03a46b">
  Validator The address to query, like `0xfefefefefefefefefefefefefefefefefefefefe`.
</ParamField>

<ParamField query="blocktype" type="string" default="blocks">
  Use `blocks` for full blocks, optionally `uncles` for [pre-Merge](https://ethereum.org/en/roadmap/merge/) blocks.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1">
  Number of records per page.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "blockNumber": "20226140",
        "timeStamp": "1720011623",
        "blockReward": "33424103001400554"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).