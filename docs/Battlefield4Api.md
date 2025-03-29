# \Battlefield4Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf4_all_bf4_all_get**](Battlefield4Api.md#bf4_all_bf4_all_get) | **GET** /bf4/all/ | Get all stats for website
[**bf4detailedserver_bf4_detailedserver_get**](Battlefield4Api.md#bf4detailedserver_bf4_detailedserver_get) | **GET** /bf4/detailedserver/ | Get detailed info about 1 server
[**bf4servers_bf4_servers_get**](Battlefield4Api.md#bf4servers_bf4_servers_get) | **GET** /bf4/servers/ | Get a list of servers based on given name
[**bf4stats_bf4_stats_get**](Battlefield4Api.md#bf4stats_bf4_stats_get) | **GET** /bf4/stats/ | Get stats from the given player for bf4
[**bf4vehicles_bf4_vehicles_get**](Battlefield4Api.md#bf4vehicles_bf4_vehicles_get) | **GET** /bf4/vehicles/ | Get weapon stats from the given player for bf4
[**bf4weapons_bf4_weapons_get**](Battlefield4Api.md#bf4weapons_bf4_weapons_get) | **GET** /bf4/weapons/ | Get weapon stats from the given player for bf4
[**logged_server_data_bf4_serverarray_get**](Battlefield4Api.md#logged_server_data_bf4_serverarray_get) | **GET** /bf4/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bf4_statusarray_get**](Battlefield4Api.md#print_logged_data_bf4_statusarray_get) | **GET** /bf4/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**statsarray_bf4_statsarray_get**](Battlefield4Api.md#statsarray_bf4_statsarray_get) | **GET** /bf4/statsarray/ | Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)
[**status_bf4_status_get**](Battlefield4Api.md#status_bf4_status_get) | **GET** /bf4/status/ | Get the player- / serveramount for all regions.



## bf4_all_bf4_all_get

> models::Bf4Combined bf4_all_bf4_all_get(format_values, name, playerid, platform)
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

[**models::Bf4Combined**](Bf4Combined.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf4detailedserver_bf4_detailedserver_get

> models::Bf4DetailedServerInfo bf4detailedserver_bf4_detailedserver_get(name, gameid, platform, lang)
Get detailed info about 1 server

For platform there is pc, xboxone and ps4. players is only for pc (has longer cache, 1 mins)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**gameid** | Option<**String**> | Id of the server you want to search for |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform of the server you want to search for |  |[default to pc]
**lang** | Option<**String**> |  |  |

### Return type

[**models::Bf4DetailedServerInfo**](Bf4DetailedServerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf4servers_bf4_servers_get

> models::Bf4Search bf4servers_bf4_servers_get(name, platform, limit, region, gamemode_filters, player_filters, server_type_filters, is_password_protected, lang)
Get a list of servers based on given name

The regions available are: all (all regions), eu, asia, nam (north america), sam (south america), au (Australia) or oc (Oceana).                     For platform there is pc, xboxone and ps4 (cache of 10 seconds)                     Limit is the max amount of servers it will gather, give it a number between 1 and 100.                     For player filter you can use oneToFive, sixToTen, tenPlus or none, comma seperated list is also a option example: \"&player_filters=onetofive,sixtoten\" to filter on the amount of free slots.                     Same for gamemode filter as with player filter but with the name of the gamemode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the server you want to search for | [required] |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform of the server you want to search for |  |[default to pc]
**limit** | Option<**i32**> | Number of servers to return |  |[default to 10]
**region** | Option<**String**> |  |  |[default to all]
**gamemode_filters** | Option<**String**> |  |  |[default to ]
**player_filters** | Option<**String**> |  |  |[default to ]
**server_type_filters** | Option<**String**> |  |  |[default to ]
**is_password_protected** | Option<**bool**> |  |  |
**lang** | Option<**String**> |  |  |

### Return type

[**models::Bf4Search**](Bf4Search.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf4stats_bf4_stats_get

> models::FrostbiteMainStats bf4stats_bf4_stats_get(format_values, name, playerid, platform)
Get stats from the given player for bf4

Available options for platform: pc, ps3, ps4, xbox360, xboxone.  for nametypes you only have to fill in one of them: name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur&platform=pc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::FrostbiteMainStats**](FrostbiteMainStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf4vehicles_bf4_vehicles_get

> models::BattlelogVehicleStats bf4vehicles_bf4_vehicles_get(name, playerid, platform)
Get weapon stats from the given player for bf4

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


## bf4weapons_bf4_weapons_get

> models::BattlelogWeaponStats bf4weapons_bf4_weapons_get(format_values, name, playerid, platform)
Get weapon stats from the given player for bf4

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


## logged_server_data_bf4_serverarray_get

> serde_json::Value logged_server_data_bf4_serverarray_get(days, gameid, serverid, servername, platform)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**gameid** | Option<**String**> | Id of the server to get historic values for |  |
**serverid** | Option<**String**> | A more permanent id of the server to get historic values for |  |
**servername** | Option<**String**> | Name of the server to get historic values for |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform to get historic values for |  |[default to pc]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bf4_statusarray_get

> models::StatusArray print_logged_data_bf4_statusarray_get(days, region, platform, r#type)
Get the player- / serveramount for all regions from database gathered every hour.

Possible regions are: \"ALL\", \"EU\", \"Asia\", \"NAm\", \"SAm\", \"AU\" and \"OC\".             For platform there is \"pc\", \"xboxone\", \"ps4\" and \"all\"             For type: \"amounts\", \"maps\" (serveramount) and \"modes\" (serveramount)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**region** | Option<[**FrostbiteRegionsWithMultiple**](.md)> | Region to get historic values for |  |[default to all]
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform to get historic values for |  |[default to pc]
**r#type** | Option<[**StatusArrayType**](.md)> | Type of historic data to return |  |[default to amounts]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statsarray_bf4_statsarray_get

> models::StatsArray statsarray_bf4_statsarray_get(days, name, playerid, platform)
Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**platform** | Option<**String**> | Platform the player uses |  |[default to pc]

### Return type

[**models::StatsArray**](StatsArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_bf4_status_get

> models::GameStatus status_bf4_status_get(platform)
Get the player- / serveramount for all regions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> | Platform to get amounts for |  |[default to pc]

### Return type

[**models::GameStatus**](GameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

