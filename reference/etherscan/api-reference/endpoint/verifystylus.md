> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify Stylus Source Code

> Submit Stylus source code for verification.

export const chain = '1';

<Callout icon="globe" iconType="regular">This endpoint is only available for the Arbitrum stack chains.</Callout>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="42161">
  Chain ID to query, eg `42161` for Arbitrum One from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="contract">
  Set to `contract` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="verifysourcecode">
  Set to `verifysourcecode` for this endpoint.
</ParamField>

<ParamField query="codeformat" type="string" default="stylus">
  Use `stylus` for Stylus projects.
</ParamField>

<ParamField query="sourceCode" type="string" default="https://github.com/OffchainLabs/stylus-hello-world">
  Public Git repository that hosts the Stylus source code.
</ParamField>

<ParamField query="contractaddress" type="string" default="0x915f0B2f34F5B5b84D1F066b398D7F0E3C2F8f83">
  The address where the contract is deployed.
</ParamField>

<ParamField query="contractname" type="string" default="stylus_hello_world">
  The contract name that matches your Stylus deployment.
</ParamField>

<ParamField query="compilerversion" type="string" default="stylus:0.5.3">
  Stylus compiler version used for compilation.
</ParamField>

<ParamField query="licenseType" type="integer" default="3">
  License identifier from the [open source license options](https://arbiscan.io/contract-license-types).
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
      "status": "1",
      "message": "OK",
      "result": "a7lpxkm9kpcpicx7daftmjifrfhiuhf5vqqnawhkfhzfrcpnxj"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).