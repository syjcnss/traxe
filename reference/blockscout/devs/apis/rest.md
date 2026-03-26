> ## Documentation Index
> Fetch the complete documentation index at: https://docs.blockscout.com/llms.txt
> Use this file to discover all available pages before exploring further.

# REST API Endpoints

<Warning>
  REST API methods are now available via the [multichain PRO API.](/devs/pro-api)
</Warning>

REST API methods are used to render the UI for new versions of Blockscout. These can be accessed per instance and used to get many types of information. Methods parameters and schemas are available at *[https://instance-name/api-docs](https://instance-name/api-docs)* (ie [https://eth.blockscout.com/api-docs](https://eth.blockscout.com/api-docs))

<Info>
  Additional information:

  * [Stats queries](/devs/apis/rest/stats-api): Access pre-calculated statistics for a chain
  * [Interpreter queries](/devs/apis/rest/interpreter-api): Transactions populated with contract names, methods, etc for easy interpretation

  <Frame>
        <img src="https://mintcdn.com/blockscout/BBa8nQTQ6isU0DUJ/images/851df441-image.jpeg?fit=max&auto=format&n=BBa8nQTQ6isU0DUJ&q=85&s=5ef902f8c15b7cec19e0e89dd93feb3e" alt="" width="900" height="84" data-path="images/851df441-image.jpeg" />
  </Frame>
</Info>

<Frame>
    <img src="https://mintcdn.com/blockscout/BBa8nQTQ6isU0DUJ/images/ac3d021a-image.jpeg?fit=max&auto=format&n=BBa8nQTQ6isU0DUJ&q=85&s=2a7fe610755af4f9403997df4284f304" alt="" width="2304" height="1288" data-path="images/ac3d021a-image.jpeg" />
</Frame>

## Pagination

Blockscout uses the keyset pagination method to quickly return results. By default an API response returns the first 50 results. To access additional results (in groups of 50), add the `next_page_params` to your query.

For example, open [https://eth.blockscout.com/api/v2/transactions](https://eth.blockscout.com/api/v2/transactions) and scroll to the bottom of the response.

<Frame caption="Example response from transactions query">
    <img src="https://mintcdn.com/blockscout/kl-dO7vK6d_hNvHA/images/d5bea84e-image.jpeg?fit=max&auto=format&n=kl-dO7vK6d_hNvHA&q=85&s=f1fe7cd290d34ec60ff6c8a3cfdb1c1d" alt="" width="1490" height="1400" data-path="images/d5bea84e-image.jpeg" />
</Frame>

You will see the `next_page_params` object. Add the parameters from this object to your next query to receive the next 50 results.

[https://eth.blockscout.com/api/v2/transactions?block\_number=18678766\&index=119\&items\_count=50](https://eth.blockscout.com/api/v2/transactions?block_number=18678766\&index=119\&items_count=50)

Repeat this process to continue receiving results in groups of 50 (remove params and substitute the new `next_page_params` found in the body of the query).

<Frame>
    <img src="https://mintcdn.com/blockscout/9uuzTGHWzjbW9Lu3/images/520935a2-image.jpeg?fit=max&auto=format&n=9uuzTGHWzjbW9Lu3&q=85&s=9984f6ddfda915edd4698ec9d087a84b" alt="" width="1592" height="1378" data-path="images/520935a2-image.jpeg" />
</Frame>

In this example, the query to receive the next 50 results would be:

[https://eth.blockscout.com/api/v2/transactions?block\_number=18678766\&index=69\&items\_count=100](https://eth.blockscout.com/api/v2/transactions?block_number=18678766\&index=69\&items_count=100)


Built with [Mintlify](https://mintlify.com).