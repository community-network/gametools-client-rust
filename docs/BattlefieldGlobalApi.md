# \BattlefieldGlobalApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bfgames_bfglobal_games_get**](BattlefieldGlobalApi.md#bfgames_bfglobal_games_get) | **GET** /bfglobal/games/ | Get the games a user has
[**platoonstat_bfglobal_detailedplatoon_get**](BattlefieldGlobalApi.md#platoonstat_bfglobal_detailedplatoon_get) | **GET** /bfglobal/detailedplatoon/ | Get info about the given platoon
[**print_logged_data_bfglobal_statusarray_get**](BattlefieldGlobalApi.md#print_logged_data_bfglobal_statusarray_get) | **GET** /bfglobal/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**search_platoon_bfglobal_platoons_get**](BattlefieldGlobalApi.md#search_platoon_bfglobal_platoons_get) | **GET** /bfglobal/platoons/ | Search for a platoon
[**search_player_bfglobal_search_get**](BattlefieldGlobalApi.md#search_player_bfglobal_search_get) | **GET** /bfglobal/search/ | Search for a player
[**total_status_array_bfglobal_totalstatusarray_get**](BattlefieldGlobalApi.md#total_status_array_bfglobal_totalstatusarray_get) | **GET** /bfglobal/totalstatusarray/ | Total server and playeramount for all games combined



## bfgames_bfglobal_games_get

> models::PlayerGames bfgames_bfglobal_games_get(include_emblem, name, playerid, oid, platform, skip_battlelog)
Get the games a user has

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_emblem** | Option<**bool**> | Include the player's emblem in the result |  |[default to false]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**models::PlayerGames**](PlayerGames.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platoonstat_bfglobal_detailedplatoon_get

> models::DetailedPlatoonInfo platoonstat_bfglobal_detailedplatoon_get(id, platform, lang)
Get info about the given platoon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Id of the platoon to search for | [required] |
**platform** | Option<**String**> | Platform to get amounts for |  |[default to pc]
**lang** | Option<**String**> |  |  |

### Return type

[**models::DetailedPlatoonInfo**](DetailedPlatoonInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bfglobal_statusarray_get

> models::GlobalStatusArray print_logged_data_bfglobal_statusarray_get(days, platform)
Get the player- / serveramount for all regions from database gathered every hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**platform** | Option<**String**> | Platform to get amounts for |  |[default to pc]

### Return type

[**models::GlobalStatusArray**](GlobalStatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_platoon_bfglobal_platoons_get

> models::PlatoonStats search_platoon_bfglobal_platoons_get(name, platform, lang)
Search for a platoon

It needs a minimum of 3 characters to start the search                          Available options for platform: pc, ps4, xboxone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the platoon to search for | [required] |
**platform** | Option<**String**> | Platform to get amounts for |  |[default to pc]
**lang** | Option<**String**> |  |  |

### Return type

[**models::PlatoonStats**](PlatoonStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_player_bfglobal_search_get

> Vec<models::Player> search_player_bfglobal_search_get(name)
Search for a player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the player to search for | [required] |

### Return type

[**Vec<models::Player>**](Player.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_status_array_bfglobal_totalstatusarray_get

> models::TotalStatusArray total_status_array_bfglobal_totalstatusarray_get(days)
Total server and playeramount for all games combined

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]

### Return type

[**models::TotalStatusArray**](TotalStatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

