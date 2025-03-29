# \Battlefield3Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf3_all_bf3_all_get**](Battlefield3Api.md#bf3_all_bf3_all_get) | **GET** /bf3/all/ | Get all stats for website
[**bf3detailedservers_bf3_detailedserver_get**](Battlefield3Api.md#bf3detailedservers_bf3_detailedserver_get) | **GET** /bf3/detailedserver/ | Get detailed info about 1 server
[**bf3servers_bf3_servers_get**](Battlefield3Api.md#bf3servers_bf3_servers_get) | **GET** /bf3/servers/ | Get a list of servers based on given name
[**bf3stats_bf3_stats_get**](Battlefield3Api.md#bf3stats_bf3_stats_get) | **GET** /bf3/stats/ | Get stats from the given player for bf3
[**bf3vehicles_bf3_vehicles_get**](Battlefield3Api.md#bf3vehicles_bf3_vehicles_get) | **GET** /bf3/vehicles/ | Get weapon stats from the given player for bf3
[**bf3weapons_bf3_weapons_get**](Battlefield3Api.md#bf3weapons_bf3_weapons_get) | **GET** /bf3/weapons/ | Get weapon stats from the given player for bf3
[**logged_server_data_bf3_serverarray_get**](Battlefield3Api.md#logged_server_data_bf3_serverarray_get) | **GET** /bf3/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bf3_statusarray_get**](Battlefield3Api.md#print_logged_data_bf3_statusarray_get) | **GET** /bf3/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**status_bf3_status_get**](Battlefield3Api.md#status_bf3_status_get) | **GET** /bf3/status/ | Get the player- / serveramount for all regions.



## bf3_all_bf3_all_get

> models::Bf3Combined bf3_all_bf3_all_get(format_values, name, playerid, platform)
Get all stats for website

Available options for platform: pc, ps3, xbox360.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::Bf3Combined**](Bf3Combined.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf3detailedservers_bf3_detailedserver_get

> models::Bf3DetailedServerInfo bf3detailedservers_bf3_detailedserver_get(name, gameid)
Get detailed info about 1 server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**gameid** | Option<**String**> | Id of the server you want to search for |  |

### Return type

[**models::Bf3DetailedServerInfo**](Bf3DetailedServerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf3servers_bf3_servers_get

> models::BattlelogSearch bf3servers_bf3_servers_get(name, gamemode_filters, map_filters, player_filters)
Get a list of servers based on given name

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


## bf3stats_bf3_stats_get

> models::Bf3MainStats bf3stats_bf3_stats_get(format_values, name, playerid, platform)
Get stats from the given player for bf3

Available options for platform: pc, ps3, xbox360.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::Bf3MainStats**](Bf3MainStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf3vehicles_bf3_vehicles_get

> models::BattlelogVehicleStats bf3vehicles_bf3_vehicles_get(name, playerid, platform)
Get weapon stats from the given player for bf3

Available options for platform: pc, ps3, xbox360.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

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


## bf3weapons_bf3_weapons_get

> models::BattlelogWeaponInfo bf3weapons_bf3_weapons_get(format_values, name, playerid, platform)
Get weapon stats from the given player for bf3

Available options for platform: pc, ps3, xbox360.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::BattlelogWeaponInfo**](BattlelogWeaponInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bf3_serverarray_get

> serde_json::Value logged_server_data_bf3_serverarray_get(days, gameid, serverid, servername, platform)
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


## print_logged_data_bf3_statusarray_get

> models::StatusArray print_logged_data_bf3_statusarray_get(days, region)
Get the player- / serveramount for all regions from database gathered every hour.

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


## status_bf3_status_get

> models::GameStatus status_bf3_status_get()
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

