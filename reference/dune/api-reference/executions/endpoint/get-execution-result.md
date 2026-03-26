> ## Documentation Index
> Fetch the complete documentation index at: https://docs.dune.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Execution Result

> Given an execution ID, returns result of a an execution request

You must pass the `execution_id` obtained from making an [execute query](/api-reference/executions/endpoint/execute-query) POST request.

Result returns the status, metadata, and query results (in JSON) from a query execution.

<Tip>
  * Read more on [Filtering](../filtering), [Sorting](../sorting), and [Sampling](../sampling) to learn more about flexibly grabbing query results.
  * Read more on [Pagination](../pagination) to get the most out of the API and handle large results.
</Tip>

<Info>
  * Results data from an execution are stored for 90 days. This is visible on the API response on the “expires\_at” field in the execution status and results body.
  * Dune internally has a maximum query result size limit (which currently is 32GB). If your query yields more than 32GB of data, the result will be truncated in storage. In such cases, pulling the result data (using pagination) but without specifying `allow_partial_results` set to true will trigger an error message: "error": "Partial Result, please request with 'allows\_partial\_results=true'". If you wish to retrieve partial results, you can pass the parameter `allow_partial_results=true`. But please make sure you indeed want to fetch the truncated result.
</Info>

<Accordion title="Execute query and get result in one call">
  <Tip> If you are using the [Python SDK](https://github.com/duneanalytics/dune-client/tree/d2195b2a9577e2dcae5d2600cb3eddce20987f38) or community [Typescript SDK](https://github.com/bh2smith/ts-dune-client), you can directly execute and fetch a result in one function call, like below: </Tip>

  <Tabs>
    <Tab title="Python SDK">
      ```python  theme={null}

      import json
      from dune_client.types import QueryParameter
      from dune_client.client import DuneClient
      from dune_client.query import QueryBase

      # setup Dune Python client
      dune = DuneClient()

      ```

      Use `run_query` to get result in JSON,
      `run_query_csv` to get result in CSV format,
      and `run_query_dataframe` to get result in Pandas dataframe

      ```

      result = dune.run_query(
          query=query # pass in query to run 
          , performance = 'large' # optionally define which tier to run the execution on (default is "medium")
          )

      ```
    </Tab>

    <Tab title="Typescript SDK">
      ```typescript  theme={null}
      import { QueryParameter, DuneClient } from "@duneanalytics/client-sdk";
      const { DUNE_API_KEY } = process.env;
      const client = new DuneClient(DUNE_API_KEY ?? "");
      ```

      Use `refresh` to get result in JSON

      ```typescript  theme={null}
      const queryID = 1215383;
      const parameters = [
        QueryParameter.text("TextField", "Plain Text"),
        QueryParameter.number("NumberField", 3.1415926535),
        QueryParameter.date("DateField", "2022-05-04 00:00:00"),
        QueryParameter.enum("ListField", "Option 1"),
      ];

      client
        .refresh(queryID, parameters)
        .then((executionResult) => console.log(executionResult.result?.rows));

      // should look like
      // [
      //   {
      //     date_field: "2022-05-04 00:00:00",
      //     list_field: "Option 1",
      //     number_field: "3.1415926535",
      //     text_field: "Plain Text",
      //   },
      // ];
      ```
    </Tab>
  </Tabs>
</Accordion>

<RequestExample>
  ```bash cURL theme={null}

  curl -X GET "https://api.dune.com/api/v1/execution/{{execution_id}}/results" -H x-dune-api-key:{{api_key}}

  ```

  ```python Python SDK theme={null}

  '''
  Download Dune Python SDK by running `pip install dune-client`
  For more info, visit the GitHub repository: https://github.com/duneanalytics/dune-client/tree/d2195b2a9577e2dcae5d2600cb3eddce20987f38
  '''

  import json
  from dune_client.types import QueryParameter
  from dune_client.client import DuneClient
  from dune_client.query import QueryBase

  # setup Dune Python client
  dune = DuneClient()

  result = dune.get_execution_results(job_id = '01HM79YYNXADPK66YWZK5X0NXB') # pass in the execution_id

  ```

  ```python Python theme={null}
  import requests

  url = "https://api.dune.com/api/v1/execution/{execution_id}/results"

  headers = {"X-DUNE-API-KEY": "<x-dune-api-key>"}

  response = requests.request("GET", url, headers=headers)

  print(response.text)

  ```

  ```javascript Javascript theme={null}
  const options = {method: 'GET', headers: {'X-DUNE-API-KEY': '<x-dune-api-key>'}};

  fetch('https://api.dune.com/api/v1/execution/{execution_id}/results', options)
    .then(response => response.json())
    .then(response => console.log(response))
    .catch(err => console.error(err));

  ```

  ```go Go theme={null}
  package main

  import (
  	"fmt"
  	"net/http"
  	"io/ioutil"
  )

  func main() {

  	url := "https://api.dune.com/api/v1/execution/{execution_id}/results"

  	req, _ := http.NewRequest("GET", url, nil)

  	req.Header.Add("X-DUNE-API-KEY", "<x-dune-api-key>")

  	res, _ := http.DefaultClient.Do(req)

  	defer res.Body.Close()
  	body, _ := ioutil.ReadAll(res.Body)

  	fmt.Println(res)
  	fmt.Println(string(body))

  }

  ```

  ```php PHP theme={null}
  <?php

  $curl = curl_init();

  curl_setopt_array($curl, [
    CURLOPT_URL => "https://api.dune.com/api/v1/execution/{execution_id}/results",
    CURLOPT_RETURNTRANSFER => true,
    CURLOPT_ENCODING => "",
    CURLOPT_MAXREDIRS => 10,
    CURLOPT_TIMEOUT => 30,
    CURLOPT_HTTP_VERSION => CURL_HTTP_VERSION_1_1,
    CURLOPT_CUSTOMREQUEST => "GET",
    CURLOPT_HTTPHEADER => [
      "X-DUNE-API-KEY: <x-dune-api-key>"
    ],
  ]);

  $response = curl_exec($curl);
  $err = curl_error($curl);

  curl_close($curl);

  if ($err) {
    echo "cURL Error #:" . $err;
  } else {
    echo $response;
  }

  ```

  ```java Java theme={null}
  HttpResponse<String> response = Unirest.get("https://api.dune.com/api/v1/execution/{execution_id}/results")
    .header("X-DUNE-API-KEY", "<x-dune-api-key>")
    .asString();

  ```
</RequestExample>


## OpenAPI

````yaml GET /v1/execution/{execution_id}/results
openapi: 3.0.1
info:
  title: DuneAPI
  description: Dune API
  contact: {}
  version: '1.0'
servers:
  - url: https://api.dune.com/api
security: []
paths:
  /v1/execution/{execution_id}/results:
    get:
      summary: Given an execution ID, returns result of a an execution request
      description: Given an execution ID, returns result of a an execution request
      parameters:
        - name: X-Dune-Api-Key
          in: header
          description: API Key for the service
          required: true
          schema:
            type: string
        - name: api_key
          in: query
          description: Alternative to using the X-Dune-Api-Key header
          schema:
            type: string
        - name: execution_id
          in: path
          description: Execution ID
          required: true
          schema:
            type: string
        - name: allow_partial_results
          in: query
          description: >-
            This enables returning a query result that was too large and only a
            partial result is

            available. By default, allow_partial_results is set to false and a
            failed state is returned.
          schema:
            type: boolean
        - name: columns
          in: query
          description: >-
            Specifies a comma-separated list of column names to return. If
            omitted, all columns are included.

            Tip: use this to limit the result to specific columns, reducing
            datapoints cost of the call.
          schema:
            type: string
        - name: filters
          in: query
          description: >-
            Expression to filter out rows from the results to return. This
            expression is similar to

            a SQL WHERE clause. More details about it in the Filtering section
            of the doc.

            This parameter is incompatible with sample_count.
          schema:
            type: string
        - name: ignore_max_credits_per_request
          in: query
          description: >-
            To bypass the default max credits per request limit, set
            ignore_max_credits_per_request=true
          schema:
            type: boolean
        - name: limit
          in: query
          description: >-
            Limit number of rows to return. This together with 'offset' allows
            easy pagination through

            results in an incremental and efficient way. This parameter is
            incompatible

            with sampling (sample_count).
          schema:
            type: integer
        - name: offset
          in: query
          description: >-
            Offset row number to start (inclusive, first row means offset=0)
            returning results

            from. This together with 'limit' allows easy pagination through
            results in an

            incremental and efficient way. This parameter is incompatible with
            sampling (sample_count).
          schema:
            type: integer
        - name: sample_count
          in: query
          description: >-
            Number of rows to return from the result by sampling the data. This
            is useful when you

            want to get a uniform sample instead of the entire result. If the
            result has less

            than the sample count, the entire result is returned. Note that this
            will return a

            randomized sample, so not every call will return the same result.
            This parameter is

            incompatible with `offset`, `limit`, and `filters` parameters.
          schema:
            type: integer
        - name: sort_by
          in: query
          description: >-
            Expression to define the order in which the results should be
            returned. This expression

            is similar to a SQL ORDER BY clause. More details about it in the
            Sorting section of the doc.
          schema:
            type: string
      responses:
        '200':
          description: OK
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.ReadExecutionResultResponse'
        '400':
          description: Bad Request
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error400'
        '401':
          description: Unauthorized
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error401'
        '404':
          description: Not Found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error404'
        '500':
          description: Internal Server Error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error500'
components:
  schemas:
    models.ReadExecutionResultResponse:
      type: object
      properties:
        cancelled_at:
          type: string
          description: Timestamp of when the query execution was cancelled, if applicable.
          example: '2024-12-20T11:04:18.724658237Z'
        error:
          type: object
          description: >-
            In case the execution had an error, this object will contain the
            error details
          allOf:
            - $ref: '#/components/schemas/models.QueryResultError'
        execution_ended_at:
          type: string
          description: Timestamp of when the query execution ended.
          example: '2024-12-20T11:04:18.724658237Z'
        execution_id:
          type: string
          description: Unique identifier for the execution of the query.
          example: 01HKZJ2683PHF9Q9PHHQ8FW4Q1
        execution_started_at:
          type: string
          description: Timestamp of when the query execution started.
          example: '2024-12-20T11:04:18.724658237Z'
        expires_at:
          type: string
          description: Timestamp of when the query result expires.
          example: '2024-12-20T11:04:18.724658237Z'
        is_execution_finished:
          type: boolean
          description: >-
            Whether the state of the query execution is terminal. This can be
            used for polling purposes.
          example: true
        next_offset:
          type: integer
          description: Offset that can be used to retrieve the next page of results.
          example: 100
        next_uri:
          type: string
          description: URI that can be used to fetch the next page of results.
          example: >-
            https://api.dune.com/api/v1/execution/01HKZJ2683PHF9Q9PHHQ8FW4Q1/results?offset=100&limit=100
        query_id:
          type: integer
          description: Unique identifier of the query.
          example: 1234
        result:
          type: object
          description: >-
            The object containing the results and metadata of the query
            execution
          allOf:
            - $ref: '#/components/schemas/models.QueryResultData'
        state:
          type: string
          description: The state of the query execution.
          example: QUERY_STATE_COMPLETED
        submitted_at:
          type: string
          description: Timestamp of when the query was submitted.
          example: '2024-12-20T11:04:18.724658237Z'
    models.Error400:
      type: object
      properties:
        error:
          type: string
          example: Bad Request
    models.Error401:
      type: object
      properties:
        error:
          type: string
          example: Invalid API Key
    models.Error404:
      type: object
      properties:
        error:
          type: string
          example: Object not found
    models.Error500:
      type: object
      properties:
        error:
          type: string
          example: Internal error
    models.QueryResultError:
      type: object
      properties:
        message:
          type: string
          description: A descriptive message about the error.
          example: 'Error: Line 1:1: mismatched input ''selecdt'''
        metadata:
          type: object
          description: Metadata about the syntax error that occurred, if applicable.
          allOf:
            - $ref: '#/components/schemas/models.SyntaxErrorMetadata'
        type:
          type: string
          description: The type of error that occurred.
          example: syntax_error
    models.QueryResultData:
      type: object
      properties:
        metadata:
          type: object
          description: >-
            Metadata about the execution of the query, including details like
            column names,

            row counts, and execution times.
          allOf:
            - $ref: '#/components/schemas/models.ExecutionResultMetadata'
        rows:
          type: array
          description: >-
            A list of rows. A row is dictionary of key-value pairs returned by
            the query,

            each pair corresponding to a column
          items:
            $ref: '#/components/schemas/models.Row'
        update_type:
          type: string
          description: The type of update operation from Trino (e.g., "SET SESSION")
    models.SyntaxErrorMetadata:
      type: object
      properties:
        column:
          type: integer
          description: The column number at which the syntax error occurred.
          example: 73
        line:
          type: integer
          description: The line number at which the syntax error occurred in the query.
          example: 10
    models.ExecutionResultMetadata:
      type: object
      properties:
        column_names:
          type: array
          description: Names of the columns in the result set.
          example:
            - Rank
            - Project
            - Volume
          items:
            type: string
        column_types:
          type: array
          description: Types of the columns in the result set.
          example:
            - double
            - varchar
            - bigint
          items:
            type: string
        datapoint_count:
          type: integer
          description: >-
            Results cell count is used for billing/pricing plans

            here we expose the these values to the user, so that they can track
            their costs
          example: 1000
        execution_time_millis:
          type: integer
          description: Time in milliseconds that the query took to execute.
          example: 1000
        pending_time_millis:
          type: integer
          description: Time in milliseconds that the query was pending before execution.
          example: 1000
        result_set_bytes:
          type: integer
          description: >-
            ResultSetBytes represents the raw data bytes returned by the SQL
            execution engine, it includes:
             + total nr of bytes used on 1 line with all the column names (the header of the result set)
             + total nr of bytes for all the row values (the result set of rows)

            it doesn't include overheads such as the presence of column names
            for every row in the JSON result type.

            it also doesn't include opmitizations such as compression
          example: 1000
        row_count:
          type: integer
          description: Number of rows in the result set for the current page of results.
          example: 10
        total_result_set_bytes:
          type: integer
          description: >-
            Total number of bytes in the result set. This doesn't include the
            json representation overhead.
          example: 10000
        total_row_count:
          type: integer
          description: Number of rows in the result set for the entire result set.
          example: 1000
    models.Row:
      type: object
      additionalProperties:
        type: object

````

Built with [Mintlify](https://mintlify.com).