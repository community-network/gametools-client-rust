# \BattlefieldHardlineApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bfh_all_bfh_all_get**](BattlefieldHardlineApi.md#bfh_all_bfh_all_get) | **GET** /bfh/all/ | Get all stats for website
[**bfhdetailedservers_bfh_detailedserver_get**](BattlefieldHardlineApi.md#bfhdetailedservers_bfh_detailedserver_get) | **GET** /bfh/detailedserver/ | Get detailed info about 1 server
[**bfhservers_bfh_servers_get**](BattlefieldHardlineApi.md#bfhservers_bfh_servers_get) | **GET** /bfh/servers/ | Get a list of servers based on given name
[**bfhstats_bfh_stats_get**](BattlefieldHardlineApi.md#bfhstats_bfh_stats_get) | **GET** /bfh/stats/ | Get stats from the given player for bfh
[**bfhvehicles_bfh_vehicles_get**](BattlefieldHardlineApi.md#bfhvehicles_bfh_vehicles_get) | **GET** /bfh/vehicles/ | Get weapon stats from the given player for bfh
[**bfhweapons_bfh_weapons_get**](BattlefieldHardlineApi.md#bfhweapons_bfh_weapons_get) | **GET** /bfh/weapons/ | Get weapon stats from the given player for bfh
[**logged_server_data_bfh_serverarray_get**](BattlefieldHardlineApi.md#logged_server_data_bfh_serverarray_get) | **GET** /bfh/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bfh_statusarray_get**](BattlefieldHardlineApi.md#print_logged_data_bfh_statusarray_get) | **GET** /bfh/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**status_bfh_status_get**](BattlefieldHardlineApi.md#status_bfh_status_get) | **GET** /bfh/status/ | Get the player- / serveramount for all regions.



## bfh_all_bfh_all_get

> models::BfhCombined bfh_all_bfh_all_get(format_values, name, playerid, platform)
Get all stats for website

Available options for platform: pc, ps3, ps4, xbox360, xboxone.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::BfhCombined**](BfhCombined.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfhdetailedservers_bfh_detailedserver_get

> models::BfhDetailedServerInfo bfhdetailedservers_bfh_detailedserver_get(name, gameid)
Get detailed info about 1 server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**gameid** | Option<**String**> | Id of the server you want to search for |  |

### Return type

[**models::BfhDetailedServerInfo**](BfhDetailedServerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfhservers_bfh_servers_get

> models::BattlelogSearch bfhservers_bfh_servers_get(name, gamemode_filters, map_filters, player_filters)
Get a list of servers based on given name

(cache of 10 seconds)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the server you want to search for | [required] |
**gamemode_filters** | Option<**String**> |  |  |[default to ]
**map_filters** | Option<**String**> |  |  |[default to ]
**player_filters** | Option<**String**> |  |  |[default to ]

### Return type

[**models::BattlelogSearch**](BattlelogSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfhstats_bfh_stats_get

> models::BfhMainStats bfhstats_bfh_stats_get(format_values, name, playerid, platform)
Get stats from the given player for bfh

Available options for platform: pc, ps3, ps4, xbox360, xboxone.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::BfhMainStats**](BfhMainStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfhvehicles_bfh_vehicles_get

> models::BattlelogVehicleStats bfhvehicles_bfh_vehicles_get(name, playerid, platform)
Get weapon stats from the given player for bfh

Available options for platform: pc, ps3, ps4, xbox360, xboxone.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::BattlelogVehicleStats**](BattlelogVehicleStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfhweapons_bfh_weapons_get

> models::BattlelogWeaponStats bfhweapons_bfh_weapons_get(format_values, name, playerid, platform)
Get weapon stats from the given player for bfh

Available options for platform: pc, ps3, ps4, xbox360, xboxone.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::BattlelogWeaponStats**](BattlelogWeaponStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bfh_serverarray_get

> serde_json::Value logged_server_data_bfh_serverarray_get(days, gameid, serverid, servername, platform)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**gameid** | Option<**String**> |  |  |
**serverid** | Option<**String**> |  |  |
**servername** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |[default to pc]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bfh_statusarray_get

> models::StatusArray print_logged_data_bfh_statusarray_get(days, region)
Get the player- / serveramount for all regions from database gathered every hour.

Possible regions are: ALL, EU, Asia, NAm, SAm, AU, OC

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**region** | Option<[**FrostbiteRegionsWithMultiple**](.md)> | Region to get historic values for |  |[default to all]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_bfh_status_get

> models::GameStatus status_bfh_status_get()
Get the player- / serveramount for all regions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GameStatus**](GameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

