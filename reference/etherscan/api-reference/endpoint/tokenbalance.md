> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get ERC20-Token Account Balance for TokenContractAddress

> Retrieves the current ERC-20 token balance for a specified address.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="tokenbalance">
  Set to `tokenbalance` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x57d90b64a1a57749b0f932f1a3395792e12e7055">
  Contract address of the ERC-20 token.
</ParamField>

<ParamField query="address" type="string" default="0xe04f27eb70e025b78871a2ad7eabe85e61212761">
  Address to check for token balance.
</ParamField>

<ParamField query="tag" type="string" default="latest">
  Use `latest` for the last block number of the chain.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=account&action=tokenbalance&contractaddress=0x57d90b64a1a57749b0f932f1a3395792e12e7055&address=0xe04f27eb70e025b78871a2ad7eabe85e61212761&tag=latest&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "135499"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).