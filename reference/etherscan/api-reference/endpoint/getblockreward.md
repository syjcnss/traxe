> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Block and Uncle Rewards by Block Number

> Retrieves block rewards along with associated Uncle block rewards.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID you query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="block">
  Set to `block` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getblockreward">
  Set to `getblockreward` for this endpoint.
</ParamField>

<ParamField query="blockno" type="integer" default="2165403">
  Block number to check rewards for.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=block&action=getblockreward&blockno=2165403&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "blockNumber": "2165403",
      "timeStamp": "1472533979",
      "blockMiner": "0x13a06d3dfe21e0db5c016c03ea7d2509f7f8d1e3",
      "blockReward": "5314181600000000000",
      "uncles": [
        {
          "miner": "0xbcdfc35b86bedf72f0cda046a3c16829a2ef41d1",
          "unclePosition": "0",
          "blockreward": "3750000000000000000"
        },
        {
          "miner": "0x0d0c9855c722ff0c78f21e43aa275a5b8ea60dce",
          "unclePosition": "1",
          "blockreward": "3750000000000000000"
        }
      ],
      "uncleInclusionReward": "312500000000000000"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).