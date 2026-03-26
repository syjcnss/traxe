> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Estimated Block Countdown by Block Number

> Retrieves the estimated time, in seconds, until a specified block is mined.

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

<ParamField query="action" type="string" default="getblockcountdown">
  Set to `getblockcountdown` for this endpoint.
</ParamField>

<ParamField query="blockno" type="integer" default="16701588">
  Block number to estimate time remaining for.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=block&action=getblockcountdown&blockno=16701588&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "0",
    "message": "NOTOK",
    "result": "Error! Block number already pass"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).