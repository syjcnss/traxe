> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Plasma Deposits by Address

> Retrieves a list of Plasma deposit transactions received by a specified address.

export const chain = '1';

<Callout icon="globe" iconType="regular">This endpoint is only available for Polygon (137), Xdai (100) and BTTC(199)</Callout>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="137">
  Chain ID to query, eg `137` for Polygon from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="txnbridge">
  Set to `txnbridge` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x4880bd4695a8e59dc527d124085749744b6c988e">
  Address to check for Plasma deposits.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="10">
  Number of records per page.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=137&module=account&action=txnbridge&address=0x4880bd4695a8e59dc527d124085749744b6c988e&page=1&offset=10&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "hash": "0xf645deb2b6fbb8b76ccbcf4bde782e28d3520e8a30e9a568b9b8c526e2fd8434",
        "blockNumber": "51844560",
        "timeStamp": "1704181285",
        "from": "0x0000000000000000000000000000000000000000",
        "address": "0x4880bd4695a8e59dc527d124085749744b6c988e",
        "amount": "2341706540000000000",
        "tokenName": "Polygon Token",
        "symbol": "POL",
        "contractAddress": "0x0000000000000000000000000000000000001010",
        "divisor": "18"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).