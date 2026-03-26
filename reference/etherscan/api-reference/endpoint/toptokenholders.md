> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Top Token Holders

> Returns the list of top holders for a specified ERC-20 token.

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>
<Warning>This endpoint is throttled to **2 calls/second** regardless of API Pro tier.</Warning>
<Callout icon="globe" iconType="regular">HyperEVM (999) is not supported yet</Callout>

<img src="https://mintcdn.com/etherscan/_gIsWVKPgcVhiZQW/images/features/holders.png?fit=max&auto=format&n=_gIsWVKPgcVhiZQW&q=85&s=9e6457139c74e1173d79e860ae4c9b08" alt="Token holders" width="1440" height="550" data-path="images/features/holders.png" />

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

<ParamField query="action" type="string" default="topholders">
  Set to `topholders` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9">
  Contract address of the ERC-20 token.
</ParamField>

<ParamField query="offset" type="integer" default="100">
  Number of top holders to return, up to 1000.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=token&action=topholders&contractaddress=0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9&offset=100&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "Ok",
    "result": [
      {
        "TokenHolderAddress": "0x4da27a545c0c5b758a6ba100e3a049001de870f5",
        "TokenHolderQuantity": "2696124.3026660371030000",
        "TokenHolderAddressType": "C"
      },
      {
        "TokenHolderAddress": "0xa700b4eb416be35b2911fd5dee80678ff64ff6c9",
        "TokenHolderQuantity": "1650828.8050095955930000",
        "TokenHolderAddressType": "C"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).