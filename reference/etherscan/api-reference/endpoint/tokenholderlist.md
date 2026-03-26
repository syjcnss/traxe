> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Token Holder List by Contract Address

> Retrieves the current list of ERC-20 token holders and their token balances.

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="token">
  Set to `token` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="tokenholderlist">
  Set to `tokenholderlist` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0xaaaebe6fe48e54f431b0c390cfaf0b017d09d42d">
  Contract address of the ERC-20 token.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="10">
  Number of records per page.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=token&action=tokenholderlist&contractaddress=0xaaaebe6fe48e54f431b0c390cfaf0b017d09d42d&page=1&offset=10&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "TokenHolderAddress": "0xa5b7d615c99f011a22f16f5809890ca6900200a3",
        "TokenHolderQuantity": "2"
      },
      {
        "TokenHolderAddress": "0x0412a1d25fbdcabc536603198330021ccb13240b",
        "TokenHolderQuantity": "3385700"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).