> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify Source Code on zkSync

> Submit zkSync source code for verification.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="324">
  Chain ID to query, eg `324` for zkSync Era from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="contract">
  Set to `contract` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="verifysourcecode">
  Set to `verifysourcecode` for this endpoint.
</ParamField>

<ParamField query="codeformat" type="string" default="solidity-standard-json-input">
  Use `solidity-single-file` for a single file or `solidity-standard-json-input` for JSON input.
</ParamField>

<ParamField query="sourceCode" type="string">
  The Solidity source code to verify.
</ParamField>

<ParamField query="constructorArguments" type="string" default="00000000000000000000000074271f2282ed7ee35c166122a60c9830354be42a">
  Optional constructor arguments used in contract deployment.
</ParamField>

<ParamField query="contractaddress" type="string" default="0xf66f984e0b73453193b452f84c8fff0ed19f6d81">
  The address where the contract is deployed.
</ParamField>

<ParamField query="contractname" type="string" default="contracts/Verified.sol:Verified">
  The contract name, including path if applicable.
</ParamField>

<ParamField query="compilerversion" type="string" default="v0.8.24+commit.e11b9ed9">
  Compiler version used for compilation.
</ParamField>

<ParamField query="zksolcVersion" type="string" default="1.3.13">
  zkSync compiler version used for compilation.
</ParamField>

<ParamField query="compilermode" type="string" default="zksync">
  zkSync compiler mode to process the build artifacts.
</ParamField>


Built with [Mintlify](https://mintlify.com).