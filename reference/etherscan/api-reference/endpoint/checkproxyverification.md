> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Check Proxy Contract Verification Status

> Check the status of a proxy contract verification.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="contract">
  Set to `contract` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="checkproxyverification">
  Set to `checkproxyverification` for this endpoint.
</ParamField>

<ParamField query="guid" type="string" default="gwgrrnfy56zf6vc1fljuejwg6pelnc5yns6fg6y2i6zfpgzquz">
  The GUID received from the proxy verification request.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "0",
    "message": "NOTOK",
    "result": "The proxy contract at 0xcbdcd3815b5f975e1a2c944a9b2cd1c985a1cb7f does not seem to be verified. Please verify and publish the contract source before proceeding with this proxy verification."
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).