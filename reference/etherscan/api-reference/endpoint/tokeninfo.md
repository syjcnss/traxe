> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Token Info by ContractAddress

> Retrieves project details and social media links for an ERC-20/ERC-721/ERC-1155 token.

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>

<Warning>This endpoint is throttled to **2 calls/second** regardless of API Pro tier.</Warning>

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

<ParamField query="action" type="string" default="tokeninfo">
  Set to `tokeninfo` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x0e3a2a1f2146d86a604adc220b4967a898d7fe07">
  Contract address of the token to retrieve info for.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=token&action=tokeninfo&contractaddress=0x0e3a2a1f2146d86a604adc220b4967a898d7fe07&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "contractAddress": "0x0e3a2a1f2146d86a604adc220b4967a898d7fe07",
        "tokenName": "Gods Unchained Cards",
        "symbol": "CARD",
        "divisor": "0",
        "tokenType": "ERC721",
        "totalSupply": "6972003",
        "blueCheckmark": "true",
        "description": "A TCG on the Ethereum blockchain that uses NFT's to bring real ownership to in-game assets.",
        "website": "https://godsunchained.com/",
        "email": "",
        "blog": "https://medium.com/@fuelgames",
        "reddit": "https://www.reddit.com/r/GodsUnchained/",
        "slack": "",
        "facebook": "https://www.facebook.com/godsunchained/",
        "twitter": "https://twitter.com/godsunchained",
        "bitcointalk": "",
        "github": "",
        "telegram": "",
        "wechat": "",
        "linkedin": "",
        "discord": "https://discordapp.com/invite/DKGr2pW",
        "whitepaper": "",
        "tokenPriceUSD": "0.000000000000000000",
        "image": ""
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).