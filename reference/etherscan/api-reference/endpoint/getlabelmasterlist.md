> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Label Master List (V1)

<Note>
  This is a PRO endpoint, available to the [Enterprise tier](https://etherscan.io/apis)
</Note>

### Query Parameters

<ParamField query="apikey" default="YourApiKeyToken" type="string">
  Your Etherscan API key.
</ParamField>

<ParamField query="module" default="nametag" type="string">
  Set to `nametag` for this endpoint.
</ParamField>

<ParamField query="action" default="getlabelmasterlist" type="string">
  Set to `getlabelmasterlist` for this endpoint.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
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