# \BattlebitApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logged_server_data_battlebit_serverarray_get**](BattlebitApi.md#logged_server_data_battlebit_serverarray_get) | **GET** /battlebit/serverarray/ | Get the servers playeramount over time
[**print_logged_data_battlebit_statusarray_get**](BattlebitApi.md#print_logged_data_battlebit_statusarray_get) | **GET** /battlebit/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**status_battlebit_status_get**](BattlebitApi.md#status_battlebit_status_get) | **GET** /battlebit/status/ | Get the player- / serveramount for all regions.



## logged_server_data_battlebit_serverarray_get

> serde_json::Value logged_server_data_battlebit_serverarray_get(servername, days)
Get the servers playeramount over time

This data is gathered by periodicly getting data from their public api: https://publicapi.battlebit.cloud/Servers/GetServerList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**servername** | **String** | Name of the server you want to search for | [required] |
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_battlebit_statusarray_get

> models::FrostbiteStatusArray print_logged_data_battlebit_statusarray_get(days, region, r#type)
Get the player- / serveramount for all regions from database gathered every hour.

This data is gathered by periodicly getting data from their public api: https://publicapi.battlebit.cloud/Servers/GetServerList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**region** | Option<[**BattlebitRegions**](.md)> | Region to get amounts for |  |[default to all]
**r#type** | Option<[**StatusArrayType**](.md)> | Type of historic data to return |  |[default to amounts]

### Return type

[**models::FrostbiteStatusArray**](FrostbiteStatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_battlebit_status_get

> models::FrostbiteGameStatus status_battlebit_status_get()
Get the player- / serveramount for all regions.

This data is gathered by periodicly getting data from their public api: https://publicapi.battlebit.cloud/Servers/GetServerList

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FrostbiteGameStatus**](FrostbiteGameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

