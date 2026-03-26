> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Internal Transactions by Transaction Hash

> Retrieves the list of internal transactions executed within a specific transaction.

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

<ParamField query="txhash" type="string" default="0x40eb908387324f2b575b4879cd9d7188f69c8fc9d87c901b9e2daaea4b442170">
  Transaction hash to check for internal transactions, like `0x36dc7f05085d0e4f9c3e4116345a2a487ac8f23f7e71bcc0ae20e27abbfa931d`. Only non-zero value internal transactions are returned.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "blockNumber": "1743059",
        "timeStamp": "1466489498",
        "from": "0x2cac6e4b11d6b58f6d3c1c9d5fe8faa89f60e5a2",
        "to": "0x66a1c3eaf0f1ffc28d209c0763ed0ca614f3b002",
        "value": "7106740000000000",
        "contractAddress": "",
        "input": "",
        "type": "call",
        "gas": "2300",
        "gasUsed": "0",
        "isError": "0",
        "errCode": ""
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).