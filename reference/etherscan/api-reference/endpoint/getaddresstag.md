> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Metadata for an Address

> Get nametags and metadata for the specified address.

<Note>This is a PRO endpoint and is available exclusively to the [Pro Plus tier](https://etherscan.io/apis).</Note>
<Warning>This endpoint is throttled to **2 calls/second** regardless of API Pro tier.</Warning>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="nametag">
  Set to `nametag` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getaddresstag">
  Set to `getaddresstag` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x0000db5C8B030AE20308AC975898E09741E70000">
  The address to query, like `0x0000db5C8B030AE20308AC975898E09741E70000`.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
     "status":"1",
     "message":"OK",
     "result":[
        {
           "address":"0xa9d1e08c7793af67e9d92fe308d5697fb81d3e43",
           "nametag":"Coinbase 10",
           "internal_nametag":"",
           "url":"https://coinbase.com",
           "shortdescription":"",
           "notes_1":"",
           "notes_2":"",
           "labels":[
              "Coinbase",
              "Exchange"
           ],
           "labels_slug":[
              "coinbase",
              "exchange"
           ],
           "reputation":0,
           "other_attributes":[
              
           ],
           "lastupdatedtimestamp":1721899658
        }
     ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).