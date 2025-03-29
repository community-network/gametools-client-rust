# \Battlefield5Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bfv_all_bfv_all_get**](Battlefield5Api.md#bfv_all_bfv_all_get) | **GET** /bfv/all/ | Get all stats for website
[**bfv_constraints_bfv_constraints_get**](Battlefield5Api.md#bfv_constraints_bfv_constraints_get) | **GET** /bfv/constraints/ | get the global limits for bfv servers
[**bfv_detailed_servers_bfv_detailedserver_get**](Battlefield5Api.md#bfv_detailed_servers_bfv_detailedserver_get) | **GET** /bfv/detailedserver/ | Get detailed info about 1 server
[**bfv_playground_bfv_playground_get**](Battlefield5Api.md#bfv_playground_bfv_playground_get) | **GET** /bfv/playground/ | Get info about a experience of a server
[**bfv_playgroundsbyowner_bfv_playgroundsbyowner_get**](Battlefield5Api.md#bfv_playgroundsbyowner_bfv_playgroundsbyowner_get) | **GET** /bfv/playgroundsbyowner/ | Get info about a experience of a servers from a playerid
[**bfv_scheduledblueprint_bfv_scheduledblueprint_get**](Battlefield5Api.md#bfv_scheduledblueprint_bfv_scheduledblueprint_get) | **GET** /bfv/scheduledblueprint/ | all currently available server settings
[**bfvclasses_bfv_classes_get**](Battlefield5Api.md#bfvclasses_bfv_classes_get) | **GET** /bfv/classes/ | Get class stats from the given player for bf5
[**bfvmultiple_bfv_multiple_post**](Battlefield5Api.md#bfvmultiple_bfv_multiple_post) | **POST** /bfv/multiple/ | Get for multiple players via grpc (max 64 players at a time)
[**bfvplayers_bfv_players_get**](Battlefield5Api.md#bfvplayers_bfv_players_get) | **GET** /bfv/players/ | get a list of players of a given server, use \",\" between gameids to request multiple (when you request multiple you wont get platoon info of players, max 10 servers per server)
[**bfvservers_bfv_servers_get**](Battlefield5Api.md#bfvservers_bfv_servers_get) | **GET** /bfv/servers/ | Get a list of servers based on given name
[**bfvstats_bfv_stats_get**](Battlefield5Api.md#bfvstats_bfv_stats_get) | **GET** /bfv/stats/ | Get stats from the given player for bf5
[**bfvsus_bfv_sus_get**](Battlefield5Api.md#bfvsus_bfv_sus_get) | **GET** /bfv/sus/ | Get sus weapon stats from the given player for bf5
[**bfvvehicles_bfv_vehicles_get**](Battlefield5Api.md#bfvvehicles_bfv_vehicles_get) | **GET** /bfv/vehicles/ | Get vehicle stats from the given player for bf5
[**bfvweapons_bfv_weapons_get**](Battlefield5Api.md#bfvweapons_bfv_weapons_get) | **GET** /bfv/weapons/ | Get weapon stats from the given player for bf5
[**logged_server_data_bfv_serverarray_get**](Battlefield5Api.md#logged_server_data_bfv_serverarray_get) | **GET** /bfv/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bfv_statusarray_get**](Battlefield5Api.md#print_logged_data_bfv_statusarray_get) | **GET** /bfv/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**statsarray_bfv_statsarray_get**](Battlefield5Api.md#statsarray_bfv_statsarray_get) | **GET** /bfv/statsarray/ | Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)
[**status_bfv_status_get**](Battlefield5Api.md#status_bfv_status_get) | **GET** /bfv/status/ | Get the player- / serveramount for all regions.



## bfv_all_bfv_all_get

> models::BfvCombined bfv_all_bfv_all_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
Get all stats for website

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]
**lang** | Option<**String**> |  |  |

### Return type

[**models::BfvCombined**](BfvCombined.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfv_constraints_bfv_constraints_get

> serde_json::Value bfv_constraints_bfv_constraints_get()
get the global limits for bfv servers

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


## bfv_detailed_servers_bfv_detailedserver_get

> models::Bf5DetailedServerInfo bfv_detailed_servers_bfv_detailedserver_get(name, gameid, platform, lang)
Get detailed info about 1 server

For platform there is pc, xboxone and ps4 (cache of 10 seconds)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**gameid** | Option<**String**> | Id of the server you want to search for |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform of the server you want to search for |  |[default to pc]
**lang** | Option<**String**> |  |  |

### Return type

[**models::Bf5DetailedServerInfo**](Bf5DetailedServerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfv_playground_bfv_playground_get

> serde_json::Value bfv_playground_bfv_playground_get(playgroundid)
Get info about a experience of a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playgroundid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfv_playgroundsbyowner_bfv_playgroundsbyowner_get

> serde_json::Value bfv_playgroundsbyowner_bfv_playgroundsbyowner_get(player_id)
Get info about a experience of a servers from a playerid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfv_scheduledblueprint_bfv_scheduledblueprint_get

> serde_json::Value bfv_scheduledblueprint_bfv_scheduledblueprint_get()
all currently available server settings

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


## bfvclasses_bfv_classes_get

> models::FrostbiteClassStats bfvclasses_bfv_classes_get(name, playerid, oid, platform, skip_battlelog, lang)
Get class stats from the given player for bf5

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]
**lang** | Option<**String**> |  |  |

### Return type

[**models::FrostbiteClassStats**](FrostbiteClassStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvmultiple_bfv_multiple_post

> std::collections::HashMap<String, models::BfvMultiplePlayer> bfvmultiple_bfv_multiple_post(request_body, raw, format_values)
Get for multiple players via grpc (max 64 players at a time)

Send a list of player id's in a array where you want stats from (Only pc is supported)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<i32>**](i32.md) |  | [required] |
**raw** | Option<**bool**> | If it needs to return the raw stats |  |[default to false]
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]

### Return type

[**std::collections::HashMap<String, models::BfvMultiplePlayer>**](BfvMultiplePlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvplayers_bfv_players_get

> models::BfvServerPlayers bfvplayers_bfv_players_get(name, gameid)
get a list of players of a given server, use \",\" between gameids to request multiple (when you request multiple you wont get platoon info of players, max 10 servers per server)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**gameid** | Option<**String**> | Id of the server you want to search for |  |

### Return type

[**models::BfvServerPlayers**](BfvServerPlayers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvservers_bfv_servers_get

> models::FrostbiteSearch bfvservers_bfv_servers_get(name, platform, limit, region, player_filters, map_filters, server_type_filters, is_password_protected, lang)
Get a list of servers based on given name

ownerId is the id of the player that owns the server                         The regions available are: all (all regions), eu, asia, nam (north america), sam (south america), au (Australia) or oc (Oceana).                         For platform there is pc, xboxone and ps4 (cache of 10 seconds)                         Limit is the max amount of servers it will gather, give it a number between 1 and 200 with 10 as the default.                     For player filter you can use oneToFive, sixToTen, tenPlus or none, comma seperated list is also a option example: \"&player_filters=onetofive,sixtoten\" to filter on the amount of free slots

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the server you want to search for | [required] |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform of the server you want to search for |  |[default to pc]
**limit** | Option<**i32**> | Number of servers to return |  |[default to 10]
**region** | Option<**String**> |  |  |[default to all]
**player_filters** | Option<**String**> |  |  |[default to ]
**map_filters** | Option<**String**> |  |  |[default to ]
**server_type_filters** | Option<**String**> |  |  |[default to ]
**is_password_protected** | Option<**bool**> |  |  |
**lang** | Option<**String**> |  |  |

### Return type

[**models::FrostbiteSearch**](FrostbiteSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvstats_bfv_stats_get

> models::BfvMainStats bfvstats_bfv_stats_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
Get stats from the given player for bf5

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]
**lang** | Option<**String**> |  |  |

### Return type

[**models::BfvMainStats**](BfvMainStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvsus_bfv_sus_get

> models::FrostbiteSusStats bfvsus_bfv_sus_get(name, playerid, oid, platform, skip_battlelog, lang)
Get sus weapon stats from the given player for bf5

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]
**lang** | Option<**String**> |  |  |

### Return type

[**models::FrostbiteSusStats**](FrostbiteSusStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvvehicles_bfv_vehicles_get

> models::FrostbiteVehicleStats bfvvehicles_bfv_vehicles_get(name, playerid, oid, platform, skip_battlelog, lang)
Get vehicle stats from the given player for bf5

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]
**lang** | Option<**String**> |  |  |

### Return type

[**models::FrostbiteVehicleStats**](FrostbiteVehicleStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bfvweapons_bfv_weapons_get

> models::FrostbiteWeaponStats bfvweapons_bfv_weapons_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
Get weapon stats from the given player for bf5

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]
**lang** | Option<**String**> |  |  |

### Return type

[**models::FrostbiteWeaponStats**](FrostbiteWeaponStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bfv_serverarray_get

> serde_json::Value logged_server_data_bfv_serverarray_get(days, gameid, serverid, servername, platform)
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


## print_logged_data_bfv_statusarray_get

> models::FrostbiteStatusArray print_logged_data_bfv_statusarray_get(days, region, platform, r#type)
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

[**models::FrostbiteStatusArray**](FrostbiteStatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statsarray_bfv_statsarray_get

> models::StatsArray statsarray_bfv_statsarray_get(days, name, playerid, oid, platform, skip_battlelog)
Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**models::StatsArray**](StatsArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status_bfv_status_get

> models::FrostbiteGameStatus status_bfv_status_get(platform)
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

