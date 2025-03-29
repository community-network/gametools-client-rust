# \Battlefield2142Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf2142servers_bf2142_servers_get**](Battlefield2142Api.md#bf2142servers_bf2142_servers_get) | **GET** /bf2142/servers/ | Get a list of servers based on given name or ip:port
[**bfbc2servers_bf2142_status_get**](Battlefield2142Api.md#bfbc2servers_bf2142_status_get) | **GET** /bf2142/status/ | Get the player- / serveramount for all regions.
[**logged_server_data_bf2142_serverarray_get**](Battlefield2142Api.md#logged_server_data_bf2142_serverarray_get) | **GET** /bf2142/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bf2142_statusarray_get**](Battlefield2142Api.md#print_logged_data_bf2142_statusarray_get) | **GET** /bf2142/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.



## bf2142servers_bf2142_servers_get

> models::OlderTitleSearch bf2142servers_bf2142_servers_get(name, r#type, service)
Get a list of servers based on given name or ip:port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**service** | Option<[**Bf2142Platform**](.md)> | Service/platform the server uses |  |[default to openspy]

### Return type

[**models::OlderTitleSearch**](OlderTitleSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfbc2servers_bf2142_status_get

> models::OlderGameStatus bfbc2servers_bf2142_status_get(service)
Get the player- / serveramount for all regions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<[**Bf2142Platform**](.md)> | Platform/service to get amounts for |  |[default to openspy]

### Return type

[**models::OlderGameStatus**](OlderGameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bf2142_serverarray_get

> serde_json::Value logged_server_data_bf2142_serverarray_get(name, r#type, service, days)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**service** | Option<[**Bf2142Platform**](.md)> | Platform/service to get amounts for |  |[default to openspy]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bf2142_statusarray_get

> models::StatusArray print_logged_data_bf2142_statusarray_get(service, days)
Get the player- / serveramount for all regions from database gathered every hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<[**Bf2142Platform**](.md)> | Platform/service to get amounts for |  |[default to openspy]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

