# \Battlefield2Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf2_status_array_bf2_statusarray_get**](Battlefield2Api.md#bf2_status_array_bf2_statusarray_get) | **GET** /bf2/statusarray/ | Get historic player-/server amounts (gathered every hour)
[**bf2_status_bf2_status_get**](Battlefield2Api.md#bf2_status_bf2_status_get) | **GET** /bf2/status/ | Get the current player-/server amount
[**bf2classes_bf2_classes_get**](Battlefield2Api.md#bf2classes_bf2_classes_get) | **GET** /bf2/classes/ | Get class/kit stats for the given player
[**bf2leaderboard_bf2_leaderboard_get**](Battlefield2Api.md#bf2leaderboard_bf2_leaderboard_get) | **GET** /bf2/leaderboard/ | Get leaderboard of bf2
[**bf2servers_bf2_servers_get**](Battlefield2Api.md#bf2servers_bf2_servers_get) | **GET** /bf2/servers/ | Get a list of servers based on given name or ip:port
[**bf2stats_bf2_stats_get**](Battlefield2Api.md#bf2stats_bf2_stats_get) | **GET** /bf2/stats/ | Get general stats for the given player
[**bf2vehicles_bf2_vehicles_get**](Battlefield2Api.md#bf2vehicles_bf2_vehicles_get) | **GET** /bf2/vehicles/ | Get vehicle stats for the given player
[**bf2weapons_bf2_weapons_get**](Battlefield2Api.md#bf2weapons_bf2_weapons_get) | **GET** /bf2/weapons/ | Get weapon stats for the given player
[**logged_server_data_bf2_serverarray_get**](Battlefield2Api.md#logged_server_data_bf2_serverarray_get) | **GET** /bf2/serverarray/ | Get the servers playeramount over time



## bf2_status_array_bf2_statusarray_get

> models::StatusArray bf2_status_array_bf2_statusarray_get(service, days)
Get historic player-/server amounts (gathered every hour)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<[**Bf2Platform**](.md)> | Platform/service to get amounts for |  |[default to bf2hub]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2_status_bf2_status_get

> models::OlderGameStatus bf2_status_bf2_status_get(service)
Get the current player-/server amount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | Option<[**Bf2PlatformAll**](.md)> | Platform/service to get amounts from |  |[default to all]

### Return type

[**models::OlderGameStatus**](OlderGameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2classes_bf2_classes_get

> models::Bf2ClassStats bf2classes_bf2_classes_get(name, playerid, platform)
Get class/kit stats for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<[**Bf2Platform**](.md)> | Platform/service the player uses |  |[default to bf2hub]

### Return type

[**models::Bf2ClassStats**](Bf2ClassStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2leaderboard_bf2_leaderboard_get

> models::Bf2Leaderboard bf2leaderboard_bf2_leaderboard_get(platform)
Get leaderboard of bf2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<[**Bf2Platform**](.md)> | Platform/service to use for the leaderboard |  |[default to bf2hub]

### Return type

[**models::Bf2Leaderboard**](Bf2Leaderboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2servers_bf2_servers_get

> models::OlderTitleSearch bf2servers_bf2_servers_get(name, r#type, service)
Get a list of servers based on given name or ip:port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**service** | Option<[**Bf2Platform**](.md)> | Service/platform the server uses |  |[default to bf2hub]

### Return type

[**models::OlderTitleSearch**](OlderTitleSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2stats_bf2_stats_get

> models::Bf2MainStats bf2stats_bf2_stats_get(name, playerid, platform)
Get general stats for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<[**Bf2Platform**](.md)> | Platform/service the player uses |  |[default to bf2hub]

### Return type

[**models::Bf2MainStats**](Bf2MainStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2vehicles_bf2_vehicles_get

> models::Bf2VehicleStats bf2vehicles_bf2_vehicles_get(name, playerid, platform)
Get vehicle stats for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<[**Bf2Platform**](.md)> | Platform/service the player uses |  |[default to bf2hub]

### Return type

[**models::Bf2VehicleStats**](Bf2VehicleStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2weapons_bf2_weapons_get

> models::Bf2WeaponStats bf2weapons_bf2_weapons_get(name, playerid, platform)
Get weapon stats for the given player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<[**Bf2Platform**](.md)> | Platform/service the player uses |  |[default to bf2hub]

### Return type

[**models::Bf2WeaponStats**](Bf2WeaponStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bf2_serverarray_get

> serde_json::Value logged_server_data_bf2_serverarray_get(name, r#type, service, days)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | IP:port or hostname/servername of the server you want to search for | [required] |
**r#type** | Option<[**LegacyBfQueryType**](.md)> | Type of server property you want to search for |  |[default to hostname]
**service** | Option<[**Bf2Platform**](.md)> | Platform/service to get amounts for |  |[default to bf2hub]
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

