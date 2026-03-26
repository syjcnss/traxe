> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify Vyper Source Code

> Submit a Vyper contract for verification.

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

<ParamField query="action" type="string" default="verifysourcecode">
  Set to `verifysourcecode` for this endpoint.
</ParamField>

<ParamField query="codeformat" type="string" default="vyper-json">
  Use `vyper-json` for Vyper contracts.
</ParamField>

<ParamField query="sourceCode" type="string">
  The Vyper source code in JSON format.
</ParamField>

<ParamField query="constructorArguments" type="string" default="00000000000000000000000074271f2282ed7ee35c166122a60c9830354be42a">
  Optional constructor arguments used in contract deployment.
</ParamField>

<ParamField query="contractaddress" type="string" default="0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413">
  The address where the contract is deployed.
</ParamField>

<ParamField query="contractname" type="string" default="contracts/Verified.vy:Verified">
  The contract name, including path if applicable.
</ParamField>

<ParamField query="compilerversion" type="string" default="vyper:0.4.0">
  Compiler version used for compilation.
</ParamField>

<ParamField query="optimizationUsed" type="string" default="0">
  Use `1` if compiler optimizations were enabled, otherwise `0`.
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