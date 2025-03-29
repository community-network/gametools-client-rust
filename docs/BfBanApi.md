# \BfBanApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf_bans_check_bfban_checkban_post**](BfBanApi.md#bf_bans_check_bfban_checkban_post) | **POST** /bfban/checkban/ | check if list of player is banned in bfban
[**get_all_banned_players_bfban_banned_players_get**](BfBanApi.md#get_all_banned_players_bfban_banned_players_get) | **GET** /bfban/banned_players/ | Get all banned players ids from bfban (used internally, cached for 1 hour)
[**get_bf_bans_check_bfban_checkban_get**](BfBanApi.md#get_bf_bans_check_bfban_checkban_get) | **GET** /bfban/checkban/ | check if list of player is banned in bfban



## bf_bans_check_bfban_checkban_post

> serde_json::Value bf_bans_check_bfban_checkban_post(request_body)
check if list of player is banned in bfban

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<std::collections::HashMap<String, String>>**](std::collections::HashMap.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_banned_players_bfban_banned_players_get

> serde_json::Value get_all_banned_players_bfban_banned_players_get()
Get all banned players ids from bfban (used internally, cached for 1 hour)

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bf_bans_check_bfban_checkban_get

> serde_json::Value get_bf_bans_check_bfban_checkban_get(names, userids, personaids)
check if list of player is banned in bfban

Give a list of names, userids and/or personaids seperated with \",\"  this is a GET request because POST isn't allowed when allow_origins=\"*\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**names** | Option<**String**> |  |  |[default to ]
**userids** | Option<**String**> |  |  |[default to ]
**personaids** | Option<**String**> |  |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

