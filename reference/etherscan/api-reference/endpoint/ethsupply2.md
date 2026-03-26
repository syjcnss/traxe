> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Total Supply of Ether 2

> Retrieves the current Ether supply, including circulation, ETH2 staking rewards, EIP-1559 burned fees, and total ETH withdrawn from the beacon chain.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="stats">
  Set to `stats` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="ethsupply2">
  Set to `ethsupply2` for this endpoint.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=stats&action=ethsupply2&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "EthSupply": "122373866217800000000000000",
      "Eth2Staking": "2940327167090033000000000",
      "BurntFees": "4610942856337430575173814",
      "WithdrawnTotal": "7618584348954597000000000"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).