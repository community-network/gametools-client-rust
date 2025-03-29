# \BattlefieldVietnamApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bfbc2servers_bfvietnam_servers_get**](BattlefieldVietnamApi.md#bfbc2servers_bfvietnam_servers_get) | **GET** /bfvietnam/servers/ | Get a list of servers based on given name or ip:port
[**bfvietnamstatus_bfvietnam_status_get**](BattlefieldVietnamApi.md#bfvietnamstatus_bfvietnam_status_get) | **GET** /bfvietnam/status/ | Get the player- / serveramount for all regions.
[**logged_server_data_bfvietnam_serverarray_get**](BattlefieldVietnamApi.md#logged_server_data_bfvietnam_serverarray_get) | **GET** /bfvietnam/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bfvietnam_statusarray_get**](BattlefieldVietnamApi.md#print_logged_data_bfvietnam_statusarray_get) | **GET** /bfvietnam/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.



## bfbc2servers_bfvietnam_servers_get

> models::OlderTitleSearch bfbc2servers_bfvietnam_servers_get(name, r#type, service)
Get a list of servers based on given name or ip:port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**service** | Option<[**BfVietnamPlatform**](.md)> | Service/platform the server uses |  |[default to qtracker]

### Return type

[**models::OlderTitleSearch**](OlderTitleSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvietnamstatus_bfvietnam_status_get

> models::OlderGameStatus bfvietnamstatus_bfvietnam_status_get(service)
Get the player- / serveramount for all regions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<[**BfVietnamPlatform**](.md)> | Platform/service to get amounts for |  |[default to openspy]

### Return type

[**models::OlderGameStatus**](OlderGameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bfvietnam_serverarray_get

> serde_json::Value logged_server_data_bfvietnam_serverarray_get(name, r#type, service, days)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**service** | Option<[**BfVietnamPlatform**](.md)> | Platform/service to get amounts for |  |[default to openspy]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bfvietnam_statusarray_get

> models::StatusArray print_logged_data_bfvietnam_statusarray_get(service, days)
Get the player- / serveramount for all regions from database gathered every hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<[**BfVietnamPlatform**](.md)> | Platform/service to get amounts for |  |[default to openspy]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

