> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Beacon Chain Withdrawals by Address

> Retrieves beacon chain withdrawal transactions made to a specified address.

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

<ParamField query="action" type="string" default="txsBeaconWithdrawal">
  Set to `txsBeaconWithdrawal` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0xB9D7934878B5FB9610B3fE8A5e441e8fad7E293f">
  Address to check for beacon withdrawals.
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
  Number of records per page.
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
        "withdrawalIndex": "13",
        "validatorIndex": "117823",
        "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
        "amount": "3402931175",
        "blockNumber": "17034877",
        "timestamp": "1681338599"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).