> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Event Logs by Address

> Retrieves event logs from a specific address, with optional block range filtering.

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

<ParamField query="address" type="string" default="0xbd3531da5cf5857e7cfaa92426877b022e612cf8">
  Address to check for logs.
</ParamField>

<ParamField query="fromBlock" type="integer" default="12878196">
  Starting block number to search from.
</ParamField>

<ParamField query="toBlock" type="integer" default="12878196">
  Ending block number to search to.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="1000">
  Number of records per page. Limited to 1000 records per query; use the `page` parameter for subsequent records.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=logs&action=getLogs&address=0xbd3531da5cf5857e7cfaa92426877b022e612cf8&fromBlock=12878196&toBlock=12878196&page=1&offset=1000&apikey=YourApiKeyToken"
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
      },
      {
        "address": "0xbd3531da5cf5857e7cfaa92426877b022e612cf8",
        "topics": [
          "0x645f26e653c951cec836533f8fe0616d301c20a17153debc17d7c3dbe4f32b28",
          "0x00000000000000000000000000000000000000000000000000000000000000b5"
        ],
        "data": "0x",
        "blockNumber": "0xc48174",
        "blockHash": "0x837e109ab8b1b40ec7d1032bff82397325d85e719b97d900fa0d9aa9745b2c27",
        "timeStamp": "0x60f9ce56",
        "gasPrice": "0x2e90edd000",
        "gasUsed": "0x247205",
        "logIndex": "0x1",
        "transactionHash": "0x4ffd22d986913d33927a392fe4319bcd2b62f3afe1c15a2c59f77fc2cc4c20a9",
        "transactionIndex": "0x"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).