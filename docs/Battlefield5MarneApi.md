# \Battlefield5MarneApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logged_server_data_bfv_marne_serverarray_get**](Battlefield5MarneApi.md#logged_server_data_bfv_marne_serverarray_get) | **GET** /bfv_marne/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bfv_marne_statusarray_get**](Battlefield5MarneApi.md#print_logged_data_bfv_marne_statusarray_get) | **GET** /bfv_marne/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**status_bfv_marne_status_get**](Battlefield5MarneApi.md#status_bfv_marne_status_get) | **GET** /bfv_marne/status/ | Get the player- / serveramount for all regions.



## logged_server_data_bfv_marne_serverarray_get

> serde_json::Value logged_server_data_bfv_marne_serverarray_get(days, serverid, servername)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**serverid** | Option<**String**> | Id of the server to get historic values for |  |
**servername** | Option<**String**> | Name of the server to get historic values for |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bfv_marne_statusarray_get

> models::FrostbiteStatusArray print_logged_data_bfv_marne_statusarray_get(days, region, platform, r#type)
Get the player- / serveramount for all regions from database gathered every hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**region** | Option<[**FrostbiteRegionsWithMultiple**](.md)> | Region to get historic values for |  |[default to all]
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform to get historic values for |  |[default to pc]
**r#type** | Option<[**StatusArrayType**](.md)> | Type of historic data to return |  |[default to amounts]

### Return type

[**models::FrostbiteStatusArray**](FrostbiteStatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_bfv_marne_status_get

> models::FrostbiteGameStatus status_bfv_marne_status_get(platform)
Get the player- / serveramount for all regions.

For platform there is pc, xboxone and ps4

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> | Platform to get amounts for |  |[default to pc]

### Return type

[**models::FrostbiteGameStatus**](FrostbiteGameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

