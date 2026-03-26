> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Export Address Tags

<Note>
  This is a PRO endpoint, available to the [Enterprise tier](https://etherscan.io/apis)
</Note>

<Warning>
  This endpoint is throttled to 2 calls/s and 100 calls/day
</Warning>

### Query Parameters

<ParamField query="apikey" default="YourApiKeyToken" type="string">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" default="1" type="string">
  Chain ID to query, refer to mainnet networks on our [supported chains](/supported-chains) page. 3 chains temporarily unavailable: Optimism(10), Arbitrum(42161), MegaETH(4326).
</ParamField>

<ParamField query="module" default="nametag" type="string">
  Set to `nametag` for this endpoint.
</ParamField>

<ParamField query="action" default="exportaddresstags" type="string">
  Set to `exportaddresstags` for this endpoint.
</ParamField>

<ParamField query="label" default="ofac-sanctioned" type="string">
  Use the [Label Master List](/api-reference/endpoint/getlabelmasterlist) to discover categeories like `phish-hack`, or pass `all` to export every label.
</ParamField>

<ParamField query="format" default="csv" type="string">
  The response format, either `zip` or `csv`.
</ParamField>

<ResponseExample>
  ```json Response theme={null}

  A link to download the CSV file, expires after 5 minutes.

  {
    "status": "1",
    "message": "OK",
    "result": "https://metadata-export.etherscan.io/1/ofac-sanctioned_latest_nametags.csv?X-Amz-Expires=300&response-content-disposition=attachment%3B%20filename%3Dexport-ofac-sanctioned_latest_nametags.csv&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=0056c267039aefa0000000005/20260305/us-east-005/s3/aws4_request&X-Amz-Date=20260305T082747Z&X-Amz-SignedHeaders=host&X-Amz-Signature=ab0edf183956eb13c3cf58d38d6388e8cf16e2e9ec66f51b8ac00766b01b88ae"
  }

  Returns a CSV file containing the address tags, uses ; as delimiter.

  "address";"nametag";"internal_nametag";"url";"shortdescription";"notes_1";"notes_2";"labels";"labels_slug";"reputation";"other_attributes";"lastupdatedtimestamp"
  "0x0000000000000000000000000000000000001111";"Fake_Phishing589039";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it. Reported by HashDit.";"";"Phish / Hack";"phish-hack";"2";"";"1728292621"
  "0x00000000001adc2c0b202d0f72ad9d50f0675296";"Fake_Phishing1334182";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it. Reported by BlockSec.";"";"Phish / Hack";"phish-hack";"2";"";"1757057505"
  "0x00000000006a2b6820feb8d5ac00cb6800795d8f";"Fake_Phishing326507";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it.";"";"Phish / Hack";"phish-hack";"2";"";"1711418316"
  "0x0000000000d67e5d5f991a40f04ed40fa3b150df";"Fake_Phishing326508";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it.";"";"Phish / Hack";"phish-hack";"2";"";"1711418362"
  "0x0000000000e7f7e18e6b7890ea1227ff088e6653";"Fake_Phishing686515";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it. Reported by ScamSniffer.";"";"Phish / Hack";"phish-hack";"2";"";"1734599538"
  "0x00000000020e203eb8fc5d0724e7c93e6b002c71";"Fake_Phishing119146";"";"";"";"Warning! This addre
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).