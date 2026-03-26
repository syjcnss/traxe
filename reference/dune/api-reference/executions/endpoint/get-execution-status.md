> ## Documentation Index
> Fetch the complete documentation index at: https://docs.dune.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Execution Status

> Check the status of an execution request

You must pass the `execution_id` obtained from making an [execute query](/api-reference/executions/endpoint/execute-query) or [execute SQL](/api-reference/executions/endpoint/execute-sql) POST request.

Returns the status of a query execution along with relevant metadata of the results if the execution is completed. **This endpoint now includes detailed error information when a query fails**, eliminating the need to call the results endpoint just to get error details.

## Pricing

This endpoint has **no charge** and does not consume credits.

## Execution States

Once an execution is triggered, it can be in one of the following states:

* `QUERY_STATE_PENDING`: The query execution is waiting for an execution slot.
* `QUERY_STATE_EXECUTING`: The query is currently executing.
* `QUERY_STATE_FAILED`: The query execution failed. This is a terminal state. **The response will include detailed error information.**
* `QUERY_STATE_COMPLETED`: The query execution completed successfully. You can retrieve the query result for this execution\_id.
* `QUERY_STATE_CANCELED`: The query execution was canceled by the user.
* `QUERY_STATE_EXPIRED`: The query execution expired, and the result is no longer available.
* `QUERY_STATE_COMPLETED_PARTIAL`: The query execution was successful, but the result was truncated because it was too large. To receive the truncated result, set the `allow_partial_results` flag to `true` in the API request to fetch the result.

## Error Response Fields

When a query execution fails (`QUERY_STATE_FAILED`), the response includes an `error` object with the following fields:

* **type** (string): Error type classification (e.g., `FAILED_TYPE_EXECUTION_FAILED`)
* **message** (string): Detailed error message explaining what went wrong, including line and column information
* **metadata** (object): Additional error context
  * **line** (integer): Line number where the error occurred
  * **column** (integer): Column position where the error occurred

<RequestExample>
  ```bash cURL theme={null}

  curl -X GET "https://api.dune.com/api/v1/execution/{{execution_id}}/status" -H x-dune-api-key:{{api_key}}

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

  result = dune.get_execution_status(job_id = '01HM79YYNXADPK66YWZK5X0NXB') # pass in the execution_id

  ```

  ```python Python theme={null}
  import requests

  url = "https://api.dune.com/api/v1/execution/{execution_id}/status"

  headers = {"X-DUNE-API-KEY": "<x-dune-api-key>"}

  response = requests.request("GET", url, headers=headers)

  print(response.text)

  ```

  ```javascript Javascript theme={null}
  const options = {method: 'GET', headers: {'X-DUNE-API-KEY': '<x-dune-api-key>'}};

  fetch('https://api.dune.com/api/v1/execution/{execution_id}/status', options)
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

  	url := "https://api.dune.com/api/v1/execution/{execution_id}/status"

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
    CURLOPT_URL => "https://api.dune.com/api/v1/execution/{execution_id}/status",
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
  HttpResponse<String> response = Unirest.get("https://api.dune.com/api/v1/execution/{execution_id}/status")
    .header("X-DUNE-API-KEY", "<x-dune-api-key>")
    .asString();

  ```
</RequestExample>

<ResponseExample>
  ```json Success Response theme={null}
  {
    "execution_id": "01HKZJ2683PHF9Q9PHHQ8FW4Q1",
    "query_id": 3493826,
    "is_execution_finished": true,
    "state": "QUERY_STATE_COMPLETED",
    "submitted_at": "2025-10-22T10:31:04.222464Z",
    "expires_at": "2026-01-20T10:31:04.36241Z",
    "execution_started_at": "2025-10-22T10:31:05.123456Z",
    "execution_ended_at": "2025-10-22T10:31:15.789012Z",
    "execution_cost_credits": 10
  }
  ```

  ```json Error Response theme={null}
  {
    "execution_id": "01K85QHSS0RBGW4BHDM0RFCG9C",
    "query_id": 6012998,
    "is_execution_finished": true,
    "state": "QUERY_STATE_FAILED",
    "submitted_at": "2025-10-22T10:31:04.222464Z",
    "expires_at": "2026-01-20T10:31:04.36241Z",
    "execution_cost_credits": 0,
    "error": {
      "type": "FAILED_TYPE_EXECUTION_FAILED",
      "message": "line 1:8: Column 'x' cannot be resolved at line 1, position 8. Please verify that this column exists using the data explorer. Learn how to use the data explorer here: https://dune.com/docs/app/query-editor/data-explorer/ [Execution ID: 01K85QHSS0RBGW4BHDM0RFCG9C]",
      "metadata": {
        "line": 1,
        "column": 8
      }
    }
  }
  ```
</ResponseExample>


## OpenAPI

````yaml GET /v1/execution/{execution_id}/status
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
  /v1/execution/{execution_id}/status:
    get:
      summary: Check the status of an execution request
      description: Check the status of an execution request
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
      responses:
        '200':
          description: OK
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.GetExecutionStatusResponse'
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
    models.GetExecutionStatusResponse:
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
        execution_cost_credits:
          type: number
          description: Cost of the execution
        execution_ended_at:
          type: string
          description: Timestamp of when the query execution ended.
          example: '2024-12-20T11:04:18.724658237Z'
        execution_id:
          type: string
          description: >-
            Unique identifier for the execution of the query and corresponding
            result.
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
        max_inflight_interactive_executions:
          type: integer
          description: >-
            Number of interactive executions this customer can have running in
            parallel
          example: 3
        max_inflight_interactive_reached:
          type: integer
          description: >-
            Total number of interactive executions this user has submitted which
            are still in progress

            only set to > 0 if the user has reached the limit of concurrent
            interactive executions
          example: 5
        query_id:
          type: integer
          description: Unique identifier of the query.
          example: 1234
        queue_position:
          type: integer
          example: 1
        result_metadata:
          type: object
          description: >-
            Metadata about the execution of the query, including details like
            column names, row counts, and execution times.
          allOf:
            - $ref: '#/components/schemas/models.ExecutionResultMetadata'
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

````

Built with [Mintlify](https://mintlify.com).