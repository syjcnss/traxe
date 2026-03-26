> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Event Logs by Address and Topics

> Retrieves event logs from a specified address, filtered by topics and block range.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="logs">
  Set to `logs` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getLogs">
  Set to `getLogs` for this endpoint.
</ParamField>

<ParamField query="fromBlock" type="integer" default="15073139">
  Starting block number to search from.
</ParamField>

<ParamField query="toBlock" type="integer" default="15074139">
  Ending block number to search to.
</ParamField>

<ParamField query="address" type="string" default="0x59728544b08ab483533076417fbbb2fd0b17ce3a">
  Address to check for logs.
</ParamField>

<ParamField query="topic0" type="string" default="0x27c4f0403323142b599832f26acd21c74a9e5b809f2215726e244a4ac588cd7d">
  First topic to filter by, such as an event signature.
</ParamField>

<ParamField query="topic0_1_opr" type="string" default="and">
  Topic operator between `topic0` and `topic1`, either `and` or `or`.
</ParamField>

<ParamField query="topic1" type="string" default="0x00000000000000000000000023581767a106ae21c074b2276d25e5c3e136a68b">
  Second topic to filter by.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1000">
  Number of records per page. Limited to 1000 records per query; use the `page` parameter for subsequent records.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=logs&action=getLogs&fromBlock=15073139&toBlock=15074139&address=0x59728544b08ab483533076417fbbb2fd0b17ce3a&topic0=0x27c4f0403323142b599832f26acd21c74a9e5b809f2215726e244a4ac588cd7d&topic0_1_opr=and&topic1=0x00000000000000000000000023581767a106ae21c074b2276d25e5c3e136a68b&page=1&offset=1000&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "address": "0x59728544b08ab483533076417fbbb2fd0b17ce3a",
        "topics": [
          "0x27c4f0403323142b599832f26acd21c74a9e5b809f2215726e244a4ac588cd7d",
          "0x00000000000000000000000023581767a106ae21c074b2276d25e5c3e136a68b",
          "0x000000000000000000000000000000000000000000000000000000000000236d",
          "0x000000000000000000000000c8a5592031f93debea5d9e67a396944ee01bb2ca"
        ],
        "data": "0x000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000f207539952d0000",
        "blockNumber": "0xe60262",
        "blockHash": "0xb40d77b4ffba5ae2a38cbc87a65a6c9b56f9af5d8bf320aa1f1b6af00b850778",
        "timeStamp": "0x62c26caf",
        "gasPrice": "0x5e2d742c9",
        "gasUsed": "0xfb7f8",
        "logIndex": "0x4b",
        "transactionHash": "0x26fe1a0a403fd44ef11ee72f3b4ceff590b6ea533684cb279cb4242be463304c",
        "transactionIndex": "0x39"
      },
      {
        "address": "0x59728544b08ab483533076417fbbb2fd0b17ce3a",
        "topics": [
          "0x27c4f0403323142b599832f26acd21c74a9e5b809f2215726e244a4ac588cd7d",
          "0x00000000000000000000000023581767a106ae21c074b2276d25e5c3e136a68b",
          "0x0000000000000000000000000000000000000000000000000000000000002261",
          "0x000000000000000000000000c8a5592031f93debea5d9e67a396944ee01bb2ca"
        ],
        "data": "0x000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000de0b6b3a7640000",
        "blockNumber": "0xe6035b",
        "blockHash": "0x5a46aeca5eaf8af1fbf56439b12dfea8fb27d18ca31020cc723271e119cffc04",
        "timeStamp": "0x62c27ab1",
        "gasPrice": "0x27e523173",
        "gasUsed": "0x3b86e",
        "logIndex": "0x1d7",
        "transactionHash": "0x3a299413cf2c91e376e542efcf3fc308c562da79af6e992401217cc6208c7f74",
        "transactionIndex": "0x92"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).