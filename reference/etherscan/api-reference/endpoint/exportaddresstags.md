> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Export Address Tags (V1)

<Note>This is a PRO endpoint, available to the [Enterprise tier](https://etherscan.io/apis)</Note>

<Warning>This endpoint is throttled to 2 calls/s and 100 calls/day</Warning>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="module" type="string" default="nametag">
  Set to `nametag` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="exportaddresstags">
  Set to `exportaddresstags` for this endpoint.
</ParamField>

<ParamField query="label" type="string" default="ofac-sanctioned">
  Use the [Label Master List](/api-reference/endpoint/getlabelmasterlist) to discover categeories like `phish-hack`, or pass `all` to export every label.
</ParamField>

<ParamField query="format" type="string" default="csv">
  The response format, currently only in `csv`.
</ParamField>

<ResponseExample>
  ```csv Response theme={null}
  "address";"nametag";"internal_nametag";"url";"shortdescription";"notes_1";"notes_2";"labels";"labels_slug";"reputation";"other_attributes";"lastupdatedtimestamp"
  "0x0000000000000000000000000000000000001111";"Fake_Phishing589039";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it. Reported by HashDit.";"";"Phish / Hack";"phish-hack";"2";"";"1728292621"
  "0x00000000001adc2c0b202d0f72ad9d50f0675296";"Fake_Phishing1334182";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it. Reported by BlockSec.";"";"Phish / Hack";"phish-hack";"2";"";"1757057505"
  "0x00000000006a2b6820feb8d5ac00cb6800795d8f";"Fake_Phishing326507";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it.";"";"Phish / Hack";"phish-hack";"2";"";"1711418316"
  "0x0000000000d67e5d5f991a40f04ed40fa3b150df";"Fake_Phishing326508";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it.";"";"Phish / Hack";"phish-hack";"2";"";"1711418362"
  "0x0000000000e7f7e18e6b7890ea1227ff088e6653";"Fake_Phishing686515";"";"";"";"There are reports that this address was used in a Phishing scam. Please exercise caution when interacting with it. Reported by ScamSniffer.";"";"Phish / Hack";"phish-hack";"2";"";"1734599538"
  "0x00000000020e203eb8fc5d0724e7c93e6b002c71";"Fake_Phishing119146";"";"";"";"Warning! This address may be attempting to impersonate a similar looking address. Please proceed with caution.";"";"Phish / Hack";"phish-hack";"2";"";"1738570688"
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).