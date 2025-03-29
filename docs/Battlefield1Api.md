# \Battlefield1Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf1_all_bf1_all_get**](Battlefield1Api.md#bf1_all_bf1_all_get) | **GET** /bf1/all/ | Get all stats for website
[**bf1classes_bf1_classes_get**](Battlefield1Api.md#bf1classes_bf1_classes_get) | **GET** /bf1/classes/ | Get class stats from the given player for bf1
[**bf1detailedservers_bf1_detailedserver_get**](Battlefield1Api.md#bf1detailedservers_bf1_detailedserver_get) | **GET** /bf1/detailedserver/ | Get detailed info about 1 server
[**bf1gamemode_bf1_gamemode_get**](Battlefield1Api.md#bf1gamemode_bf1_gamemode_get) | **GET** /bf1/gamemode/ | Get gamemode stats from the given player for bf1
[**bf1multiple_bf1_multiple_post**](Battlefield1Api.md#bf1multiple_bf1_multiple_post) | **POST** /bf1/multiple/ | Get for multiple players via blaze (max 64 players at a time)
[**bf1player_bf1_player_get**](Battlefield1Api.md#bf1player_bf1_player_get) | **GET** /bf1/player/ | Get id of a player within bf1
[**bf1players_bf1_players_get**](Battlefield1Api.md#bf1players_bf1_players_get) | **GET** /bf1/players/ | get a list of players of a given server, use \",\" between gameids to request multiple (when you request multiple you wont get platoon info of players, max 10 servers per server)
[**bf1progress_bf1_progress_get**](Battlefield1Api.md#bf1progress_bf1_progress_get) | **GET** /bf1/progress/ | Get progress of the medals of the given player for bf1
[**bf1seederplayers_bf1_seederplayers_get**](Battlefield1Api.md#bf1seederplayers_bf1_seederplayers_get) | **GET** /bf1/seederplayers/ | get a list of players of a given server from the \"playerlist sender.exe\" or the seeding systems' automated message system.
[**bf1servers_bf1_servers_get**](Battlefield1Api.md#bf1servers_bf1_servers_get) | **GET** /bf1/servers/ | Get a list of servers based on given name
[**bf1stats_bf1_stats_get**](Battlefield1Api.md#bf1stats_bf1_stats_get) | **GET** /bf1/stats/ | Get stats from the given player for bf1
[**bf1sus_bf1_sus_get**](Battlefield1Api.md#bf1sus_bf1_sus_get) | **GET** /bf1/sus/ | Get sus weapon stats from the given player for bf1
[**bf1vehicles_bf1_vehicles_get**](Battlefield1Api.md#bf1vehicles_bf1_vehicles_get) | **GET** /bf1/vehicles/ | Get vehicle stats from the given player for bf1
[**bf1weapons_bf1_weapons_get**](Battlefield1Api.md#bf1weapons_bf1_weapons_get) | **GET** /bf1/weapons/ | Get weapon stats from the given player for bf1
[**logged_server_data_bf1_serverarray_get**](Battlefield1Api.md#logged_server_data_bf1_serverarray_get) | **GET** /bf1/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bf1_statusarray_get**](Battlefield1Api.md#print_logged_data_bf1_statusarray_get) | **GET** /bf1/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour.
[**statsarray_bf1_statsarray_get**](Battlefield1Api.md#statsarray_bf1_statsarray_get) | **GET** /bf1/statsarray/ | Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)
[**status_bf1_status_get**](Battlefield1Api.md#status_bf1_status_get) | **GET** /bf1/status/ | Get the player- / serveramount for all regions.



## bf1_all_bf1_all_get

> models::Bf1Combined bf1_all_bf1_all_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
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

[**models::Bf1Combined**](Bf1Combined.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1classes_bf1_classes_get

> models::FrostbiteClassStats bf1classes_bf1_classes_get(name, playerid, oid, platform, skip_battlelog, lang)
Get class stats from the given player for bf1

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


## bf1detailedservers_bf1_detailedserver_get

> models::Bf1DetailedServerInfo bf1detailedservers_bf1_detailedserver_get(name, gameid, platform, lang)
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

[**models::Bf1DetailedServerInfo**](Bf1DetailedServerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1gamemode_bf1_gamemode_get

> models::GamemodeStats bf1gamemode_bf1_gamemode_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
Get gamemode stats from the given player for bf1

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

[**models::GamemodeStats**](GamemodeStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1multiple_bf1_multiple_post

> std::collections::HashMap<String, models::Bf1MultiplePlayer> bf1multiple_bf1_multiple_post(request_body, raw, format_values)
Get for multiple players via blaze (max 64 players at a time)

Send a list of player id's in a array where you want stats from (Only pc is supported)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<i32>**](i32.md) |  | [required] |
**raw** | Option<**bool**> | If it needs to return the raw stats |  |[default to false]
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]

### Return type

[**std::collections::HashMap<String, models::Bf1MultiplePlayer>**](Bf1MultiplePlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1player_bf1_player_get

> models::PersonaInfo bf1player_bf1_player_get(name, playerid, oid, platform, skip_battlelog)
Get id of a player within bf1

you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**models::PersonaInfo**](PersonaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1players_bf1_players_get

> models::Bf1ServerPlayers bf1players_bf1_players_get(name, gameid)
get a list of players of a given server, use \",\" between gameids to request multiple (when you request multiple you wont get platoon info of players, max 10 servers per server)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**gameid** | Option<**String**> | Id of the server you want to search for |  |

### Return type

[**models::Bf1ServerPlayers**](Bf1ServerPlayers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1progress_bf1_progress_get

> models::ProgressStats bf1progress_bf1_progress_get(name, playerid, oid, platform, skip_battlelog, lang)
Get progress of the medals of the given player for bf1

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

[**models::ProgressStats**](ProgressStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1seederplayers_bf1_seederplayers_get

> models::Bf1SeederServerPlayers bf1seederplayers_bf1_seederplayers_get(name, id, gameid)
get a list of players of a given server from the \"playerlist sender.exe\" or the seeding systems' automated message system.

This returns some extra info that can only be gathered from a real game session: score, kills and deaths.              Use the one above as backup, as it only returns when someone has a sender running

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the server you want to search for |  |
**id** | Option<**String**> | unique sender id |  |
**gameid** | Option<**i32**> | Id of the server you want to search for |  |

### Return type

[**models::Bf1SeederServerPlayers**](Bf1SeederServerPlayers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1servers_bf1_servers_get

> models::FrostbiteSearch bf1servers_bf1_servers_get(name, platform, limit, region, gamemode_filters, map_filters, player_filters, server_type_filters, is_password_protected, lang)
Get a list of servers based on given name

ownerId == None, can't be gathered for bf1. used in bf5.                     The regions available are: all (all regions), eu, asia, nam (north america), sam (south america), au (Australia) or oc (Oceana).                     For platform there is pc, xboxone and ps4 (cache of 10 seconds)                     For player filter you can use oneToFive, sixToTen, tenPlus or none, comma seperated list is also a option example: \"&player_filters=onetofive,sixtoten\" to filter on the amount of free slots.                     Same for gamemode filter as with player filter but with the name of the gamemode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the server you want to search for | [required] |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform of the server you want to search for |  |[default to pc]
**limit** | Option<**i32**> | Number of servers to return |  |[default to 10]
**region** | Option<**String**> |  |  |[default to all]
**gamemode_filters** | Option<**String**> |  |  |[default to ]
**map_filters** | Option<**String**> |  |  |[default to ]
**player_filters** | Option<**String**> |  |  |[default to ]
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


## bf1stats_bf1_stats_get

> models::FrostbiteMainStats bf1stats_bf1_stats_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
Get stats from the given player for bf1

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

[**models::FrostbiteMainStats**](FrostbiteMainStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf1sus_bf1_sus_get

> models::FrostbiteSusStats bf1sus_bf1_sus_get(name, playerid, oid, platform, skip_battlelog, lang)
Get sus weapon stats from the given player for bf1

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


## bf1vehicles_bf1_vehicles_get

> models::FrostbiteVehicleStats bf1vehicles_bf1_vehicles_get(name, playerid, oid, platform, skip_battlelog, lang)
Get vehicle stats from the given player for bf1

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


## bf1weapons_bf1_weapons_get

> models::FrostbiteWeaponStats bf1weapons_bf1_weapons_get(format_values, name, playerid, oid, platform, skip_battlelog, lang)
Get weapon stats from the given player for bf1

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


## logged_server_data_bf1_serverarray_get

> serde_json::Value logged_server_data_bf1_serverarray_get(days, gameid, serverid, servername, platform)
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


## print_logged_data_bf1_statusarray_get

> models::FrostbiteStatusArray print_logged_data_bf1_statusarray_get(days, region, platform, r#type)
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


## statsarray_bf1_statsarray_get

> models::StatsArray statsarray_bf1_statsarray_get(days, name, playerid, oid, platform, skip_battlelog)
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


## status_bf1_status_get

> models::FrostbiteGameStatus status_bf1_status_get(platform)
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

