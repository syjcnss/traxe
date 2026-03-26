> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Ethereum Nodes Size

> Retrieves the total size of the Ethereum blockchain, in bytes, within a specified date range.

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

<ParamField query="action" type="string" default="chainsize">
  Set to `chainsize` for this endpoint.
</ParamField>

<ParamField query="startdate" type="string" default="2019-02-01">
  Starting date in `yyyy-MM-dd` format.
</ParamField>

<ParamField query="enddate" type="string" default="2019-02-28">
  Ending date in `yyyy-MM-dd` format.
</ParamField>

<ParamField query="clienttype" type="string" default="geth">
  Node client to query, either `geth` or `parity`.
</ParamField>

<ParamField query="syncmode" type="string" default="default">
  Node type to run, either `default` or `archive`.
</ParamField>

<ParamField query="sort" type="string" default="desc">
  Sort order either `desc` for the latest results first or `asc` for the oldest results first.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=stats&action=chainsize&startdate=2019-02-01&enddate=2019-02-28&clienttype=geth&syncmode=default&sort=desc&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "blockNumber": "7156164",
        "chainTimeStamp": "2019-02-01",
        "chainSize": "184726421279",
        "clientType": "Geth",
        "syncMode": "Default"
      },
      {
        "blockNumber": "7161012",
        "chainTimeStamp": "2019-02-02",
        "chainSize": "184726654448",
        "clientType": "Geth",
        "syncMode": "Default"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).