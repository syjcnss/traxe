> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Normal Transactions By Address

> Retrieves the transaction history of a specified address, with optional pagination.

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

<ParamField query="action" type="string" default="txlist">
  Set to `txlist` for this endpoint
</ParamField>

<ParamField query="address" type="string" default="0xc5102fE9359FD9a28f877a67E36B0F050d81a3CC">
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
        "blockNumber": "23467053",
        "blockHash": "0xf5646226f819fbdd6b2f1cb99de6e5d17da3d876ec166e69f4e9736c8ebf7ab4",
        "timeStamp": "1759129619",
        "hash": "0xf9db905d77704596d3600816bc70201586cfeec13bcf576320e2f38d6ca851a0",
        "nonce": "8",
        "transactionIndex": "184",
        "from": "0x2449ecef5012f0a0e153b278ef4fcc9625bc4c78",
        "to": "0xc5102fe9359fd9a28f877a67e36b0f050d81a3cc",
        "value": "0",
        "gas": "73271",
        "gasPrice": "238744402",
        "input": "0x5c19a95c0000000000000000000000002449ecef5012f0a0e153b278ef4fcc9625bc4c78",
        "methodId": "0x5c19a95c",
        "functionName": "delegate(address to)",
        "contractAddress": "",
        "cumulativeGasUsed": "22498564",
        "txreceipt_status": "1",
        "gasUsed": "48847",
        "confirmations": "589",
        "isError": "0"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).