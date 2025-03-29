# \Battlefield1942Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf1942_servers_bf1942_servers_get**](Battlefield1942Api.md#bf1942_servers_bf1942_servers_get) | **GET** /bf1942/servers/ | Get a list of servers based on given name
[**bf1942_statusarray_bf1942_statusarray_get**](Battlefield1942Api.md#bf1942_statusarray_bf1942_statusarray_get) | **GET** /bf1942/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**bf1942status_bf1942_status_get**](Battlefield1942Api.md#bf1942status_bf1942_status_get) | **GET** /bf1942/status/ | Get the player- / serveramount for all regions.
[**logged_server_data_bf1942_serverarray_get**](Battlefield1942Api.md#logged_server_data_bf1942_serverarray_get) | **GET** /bf1942/serverarray/ | Get the servers playeramount over time



## bf1942_servers_bf1942_servers_get

> models::OlderTitleSearch bf1942_servers_bf1942_servers_get(name, r#type)
Get a list of servers based on given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]

### Return type

[**models::OlderTitleSearch**](OlderTitleSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1942_statusarray_bf1942_statusarray_get

> models::StatusArray bf1942_statusarray_bf1942_statusarray_get(days)
Get the player- / serveramount for all regions from database gathered every hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1942status_bf1942_status_get

> models::OlderGameStatus bf1942status_bf1942_status_get()
Get the player- / serveramount for all regions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OlderGameStatus**](OlderGameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bf1942_serverarray_get

> serde_json::Value logged_server_data_bf1942_serverarray_get(name, r#type, days)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

