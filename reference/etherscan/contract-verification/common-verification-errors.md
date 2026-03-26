> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Common Verification Errors

### Contract Doesn't Match

<Warning>
  "Compiled contract deployment bytecode does NOT match the transaction deployment bytecode"
</Warning>

The submitted source code does not match the contract code deployed on chain.

Common causes include using a different compiler version or enabling optimisation runs.

For an exact match to be found, both <strong>source code</strong> and <strong>compiler settings</strong> specified have to exactly match the deployment conditions, for the same bytecode to be reproduced.

### Solidity Compilation Error

<Warning>
  "Solidity Compilation Error: Identifier not found or not unique"
</Warning>

A compilation issue occured due to syntax errors in your Solidity source code.

Consider debugging your contract with any compiler such as <a href="https://remix.ethereum.org/"><strong>Remix</strong></a> or <a href="https://hardhat.org/"><strong>Hardhat</strong></a> and reference the error from Solidity's <a href="https://docs.soliditylang.org"><strong>official documentation</strong></a>.

### Contract Not Deployed

<Warning>
  "Unable to locate ContractCode at 0x539a277b12a3f6723f4c1769edb11b0be7c214da"
</Warning>

The contract has not been deployed at the specific address at the specific chain.

Check the contract address you've deployed, if your contract deployment transaction has succeeded or if the [**chainId**](/supported-chains) specified is correct.

We usually pick up deployments within a few blocks from the chaintip, this will be delayed if the chain is having high activity. But no more than a few mins, [**ping us**](/resources/contact-us) if you've verified the contract is deployed with the Etherscan link.

### Missing or Invalid Library Names

<Warning>
  "Library was required but suitable match not found"
</Warning>

A <a href="https://solidity-by-example.org/library/"><strong>library</strong></a> was used in your contract deployment, but was not specified, misspelt or using the wrong library address.

Double check on your library names ( <strong>case sensitive</strong> such as "PRBMath" ) or ensure that a matching library name and library address is provided.

### Missing Contract Name

<Warning>
  "Unable to locate ContractName , did you specify the correct Contract Name ?"
</Warning>

A match was not found with the name of the contract specified when multiple files are provided.

Ensure that you have provided the correct contract name to be matched against, and making sure you submit the <strong>main contract</strong> name not its dependencies.

### Missing/Invalid Constructor Arguments

<Warning>
  "Please check if the correct constructor argument was entered"
</Warning>

if your contract utilized the `constructor` keyword, you should provide it in hex format. Otherwise, leave this field empty as it is.

You may reference your original deployment's constructor arguments or determine it from the <a href="https://info.etherscan.com/contract-verification-constructor-arguments/"><strong>end of your compiled bytecode</strong></a>.

### Mismatched bytecode metadata hash

<Warning>
  "Please check if the correct bytecodehash was specified via standard-json verification."
</Warning>

The <a href="https://docs.soliditylang.org/en/v0.8.17/metadata.html#encoding-of-the-metadata-hash-in-the-bytecode"><strong>metadata hash</strong></a> settings of your submitted source code differs from the settings of your original contract deployment, such as being set to `ipfs` or `none`.

Submit your contract verification using the solc json input format, and <a href="https://github.com/PaulRBerg/hardhat-template/blob/f6406c4e7c9e23d5169b39fb11d528a975b678e6/hardhat.config.ts#L104"><strong>specify the settings</strong></a> accordingly there.

Other submission formats such as single file or multifile <strong>do not support</strong> changing this setting, and will use the compiler defaults.

### Similar Match Found

<Warning>
  "This contract already Similar Matches the deployed ByteCode at 0x4200000000000000000000000000000000000042"
</Warning>

This error indicates that the contract has already been verified via <a href="https://info.etherscan.com/types-of-contract-verification/"><strong>Similar Match</strong></a> to another contract.

Kindly <a href="https://info.etherscan.com/update-on-similar-match-contract-verification/"><strong>reach out</strong></a> to us at this point of time to have this updated to Full Match if required.

### Unsupported Solc Version

<Warning>
  "Invalid or not supported solc version, see [https://etherscan.io/solcversions](https://etherscan.io/solcversions) for list"
</Warning>

This error is thrown when you specify to use an invalid or unsupported version of the Solidity Compiler ie. below `v0.4.11-nightly.2017.3.15+`.

Do [**let us know**](/resources/contact-us) if you need to verify a contract below this supported version such as to prove you deployed the first NFT.

### Source Code Already Verified

<Warning>
  "Source code already verified"
</Warning>

An <a href="https://info.etherscan.com/types-of-contract-verification/"><strong>Exact Match</strong></a> has been obtained, get back to having your <a href="https://media.giphy.com/media/11ISwbgCxEzMyY/giphy.gif"><strong>coffee</strong></a>.

If you think this might be a mistake, do check if you've submitted verification to the right <strong>explorer/chain</strong>, a contract that is verified on Etherscan is <strong>not automatically verified</strong> on other explorers.

### Unsupported File Import Callback

<Warning>
  "Source "@openzeppelin/contracts/ERC20.sol" not found: File import callback not supported"
</Warning>

This error is thrown when contracts reference imports from external sources, such as <a href="https://docs.openzeppelin.com/"><strong>OpenZeppelin</strong></a> libraries or Github links.

Consider <a href="https://hardhat.org/hardhat-runner/docs/advanced/flattening#flattening-your-contracts"><strong>flattening</strong></a> your source code into a single file, or use the Solidity Standard Json Input format that comes with tools such as <a href="https://hardhat.org/hardhat-runner/docs/guides/verifying#verifying-your-contracts"><strong>Hardhat</strong></a> to resolve these external imports.

### Temporary Error

<Warning>
  "This could be a temporary error, please retry or contact us (Error Code 10001/10002/10003)"
</Warning>

Something went wrong on our end, which could include downtime or <a href="https://etherscan.freshstatus.io/"><strong>maintenance</strong></a> windows.

Please retry this in a while or [**ping us**](/resources/contact-us) if this continues to persist.


Built with [Mintlify](https://mintlify.com).