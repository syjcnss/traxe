> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Event Logs by Topics

> Retrieves event logs within a specified block range, filtered by topics.

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

<ParamField query="fromBlock" type="integer" default="12878196">
  Starting block number to search from.
</ParamField>

<ParamField query="toBlock" type="integer" default="12879196">
  Ending block number to search to.
</ParamField>

<ParamField query="topic0" type="string" default="0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef">
  First topic to filter by, such as an event signature.
</ParamField>

<ParamField query="topic0_1_opr" type="string" default="and">
  Topic operator between `topic0` and `topic1`, either `and` or `or`.
</ParamField>

<ParamField query="topic1" type="string" default="0x0000000000000000000000000000000000000000000000000000000000000000">
  Second topic to filter by.
</ParamField>

<ParamField query="topic1_2_opr" type="string" default="and">
  Topic operator between `topic1` and `topic2`, either `and` or `or`.
</ParamField>

<ParamField query="topic2" type="string" default="0x000000000000000000000000c45a4b3b698f21f88687548e7f5a80df8b99d93d">
  Third topic to filter by.
</ParamField>

<ParamField query="topic2_3_opr" type="string" default="and">
  Topic operator between `topic2` and `topic3`, either `and` or `or`.
</ParamField>

<ParamField query="topic3" type="string" default="0x00000000000000000000000000000000000000000000000000000000000000b5">
  Fourth topic to filter by.
</ParamField>

<ParamField query="topic0_2_opr" type="string" default="and">
  Topic operator between `topic0` and `topic2`, either `and` or `or`.
</ParamField>

<ParamField query="topic0_3_opr" type="string" default="and">
  Topic operator between `topic0` and `topic3`, either `and` or `or`.
</ParamField>

<ParamField query="topic1_3_opr" type="string" default="and">
  Topic operator between `topic1` and `topic3`, either `and` or `or`.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1000">
  Number of records per page. Limited to 1000 records per query; use the `page` parameter for subsequent records.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=logs&action=getLogs&fromBlock=12878196&toBlock=12879196&topic0=0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef&topic0_1_opr=and&topic1=0x0000000000000000000000000000000000000000000000000000000000000000&topic1_2_opr=and&topic2=0x000000000000000000000000c45a4b3b698f21f88687548e7f5a80df8b99d93d&topic2_3_opr=and&topic3=0x00000000000000000000000000000000000000000000000000000000000000b5&topic0_2_opr=and&topic0_3_opr=and&topic1_3_opr=and&page=1&offset=1000&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "address": "0xbd3531da5cf5857e7cfaa92426877b022e612cf8",
        "topics": [
          "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
          "0x0000000000000000000000000000000000000000000000000000000000000000",
          "0x000000000000000000000000c45a4b3b698f21f88687548e7f5a80df8b99d93d",
          "0x00000000000000000000000000000000000000000000000000000000000000b5"
        ],
        "data": "0x",
        "blockNumber": "0xc48174",
        "blockHash": "0x837e109ab8b1b40ec7d1032bff82397325d85e719b97d900fa0d9aa9745b2c27",
        "timeStamp": "0x60f9ce56",
        "gasPrice": "0x2e90edd000",
        "gasUsed": "0x247205",
        "logIndex": "0x",
        "transactionHash": "0x4ffd22d986913d33927a392fe4319bcd2b62f3afe1c15a2c59f77fc2cc4c20a9",
        "transactionIndex": "0x"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).