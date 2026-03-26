> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Daily Block Count and Rewards

> Retrieves the daily count of mined blocks along with the corresponding block rewards.

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID you query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="stats">
  Set to `stats` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="dailyblkcount">
  Set to `dailyblkcount` for this endpoint.
</ParamField>

<ParamField query="startdate" type="string" default="2019-02-01">
  Starting date in `yyyy-MM-dd` format.
</ParamField>

<ParamField query="enddate" type="string" default="2019-02-28">
  Ending date in `yyyy-MM-dd` format.
</ParamField>

<ParamField query="sort" type="string" default="desc">
  Sort order, either `asc` or `desc`.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=stats&action=dailyblkcount&startdate=2019-02-01&enddate=2019-02-28&sort=desc&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "UTCDate": "2019-02-01",
        "unixTimeStamp": "1548979200",
        "blockCount": 4848,
        "blockRewards_Eth": "14929.464690870590355682"
      },
      {
        "UTCDate": "2019-02-02",
        "unixTimeStamp": "1549065600",
        "blockCount": 4935,
        "blockRewards_Eth": "15120.386084685869906669"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).