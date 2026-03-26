> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Label Master List

<Note>This is a PRO endpoint, available to the [Enterprise tier](https://etherscan.io/apis)</Note>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, refer to mainnet networks on our [supported chains](/supported-chains) page. 3 chains temporarily unavailable: Optimism(10), Arbitrum(42161), MegaETH(4326).
</ParamField>

<ParamField query="module" type="string" default="nametag">
  Set to `nametag` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getlabelmasterlist">
  Set to `getlabelmasterlist` for this endpoint.
</ParamField>

<ResponseExample>
  ```json Response theme={null}

  A link to download the CSV file, expires after 5 minutes.

  {
    "status": "1",
    "message": "OK",
    "result": "https://metadata-export.etherscan.io/1/labelmasterlist_latest.json?X-Amz-Expires=300&response-content-disposition=attachment%3B%20filename%3Dexport-labelmasterlist_latest.json&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=0056c267039aefa0000000005/20260305/us-east-005/s3/aws4_request&X-Amz-Date=20260305T082933Z&X-Amz-SignedHeaders=host&X-Amz-Signature=a795d41a87821a3224b3d9e56819f0fdc5b869c43278c6c96773966798a80f71"
  }

  Returns a list of available labels on the specific chain.

  {
     "status":"1",
     "message":"OK",
     "result":[
        {
           "labelname":"Axelar",
           "labelslug":"axelar",
           "shortdescription":"Axelar delivers secure cross-chain communication for Web3. Its infrastructure enables dApp users to interact with any asset or application, on any chain, with one click. Axelar is an overlay network, delivering Turing-complete message passing via proof-of-stake and permissionless protocols.",
           "notes":"",
           "lastupdatedtimestamp":1712897117
        },
        {
           "labelname":"Binance",
           "labelslug":"binance",
           "shortdescription":"Binance is one of the world’s leading blockchain ecosystems, with a product suite that includes the largest digital asset exchange.",
           "notes":"",
           "lastupdatedtimestamp":1759922816
        }
     ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).