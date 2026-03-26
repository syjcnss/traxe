> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Address ERC20 Token Holding

> Retrieves the current ERC-20 token balance for a specified address.

export const chain = '1';

<Note>
  This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above
</Note>

<Warning>This endpoint is throttled to **2 calls/second** regardless of API Pro tier.</Warning>

### Query Parameters

<ParamField query="apikey" default="YourApiKeyToken" type="string">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" default="1" type="string">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" default="account" type="string">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" default="addresstokenbalance" type="string">
  Set to `addresstokenbalance` for this endpoint.
</ParamField>

<ParamField query="address" default="0x983e3660c0bE01991785F80f266A84B911ab59b0" type="string">
  Address to check for token holdings.
</ParamField>

<ParamField query="page" default="1" type="integer">
  Page number for pagination.
</ParamField>

<ParamField query="offset" default="100" type="integer">
  Number of records per page.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=account&action=addresstokenbalance&address=0x983e3660c0bE01991785F80f266A84B911ab59b0&page=1&offset=100&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "TokenAddress": "0xffffffff2ba8f66d4e51811c5190992176930278",
        "TokenName": "Furucombo",
        "TokenSymbol": "COMBO",
        "TokenQuantity": "1861606940000000000",
        "TokenDivisor": "18",
        "TokenPriceUSD": "0.000891470000000000"
      },
      {
        "TokenAddress": "0x53a1e9912323b8016424d6287286e3b6de263f76",
        "TokenName": "PUTIN Token",
        "TokenSymbol": "PTT",
        "TokenQuantity": "3500000000000000000000",
        "TokenDivisor": "18",
        "TokenPriceUSD": "0.000000000000000000"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).