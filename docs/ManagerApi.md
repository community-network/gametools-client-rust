# \ManagerApi

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**checkban_manager_checkban_get**](ManagerApi.md#checkban_manager_checkban_get) | **GET** /manager/checkban/ | Check if a player is already banned in a server within the manager if the group has made his bans public
[**checkbans_manager_checkbans_get**](ManagerApi.md#checkbans_manager_checkbans_get) | **GET** /manager/checkbans/ | Check if some playerids are already banned in a server within the manager if the group has made his bans public
[**info_manager_info_get**](ManagerApi.md#info_manager_info_get) | **GET** /manager/info/ | Get usage info about the manager.
[**leaderboard_manager_leaderboard_get**](ManagerApi.md#leaderboard_manager_leaderboard_get) | **GET** /manager/leaderboard/ | Get best players of your server
[**leaderboard_v2_manager_leaderboard_v2_get**](ManagerApi.md#leaderboard_v2_manager_leaderboard_v2_get) | **GET** /manager/leaderboard/v2/ | Get best players of your server
[**live_manager_live_get**](ManagerApi.md#live_manager_live_get) | **GET** /manager/live/ | Get the live stats difference since the player joined a server, stats: none means couldn't find stats
[**players_manager_players_get**](ManagerApi.md#players_manager_players_get) | **GET** /manager/players/ | Get playerlist of your server
[**server_sessions_manager_server_sessions_get**](ManagerApi.md#server_sessions_manager_server_sessions_get) | **GET** /manager/server_sessions/ | Get the finished play sessions from a server of our manager.
[**sessions_manager_sessions_get**](ManagerApi.md#sessions_manager_sessions_get) | **GET** /manager/sessions/ | Get the most recent stats from a player of our manager.



## checkban_manager_checkban_get

> serde_json::Value checkban_manager_checkban_get(name, playerid, oid, platform, skip_battlelog)
Check if a player is already banned in a server within the manager if the group has made his bans public

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checkbans_manager_checkbans_get

> serde_json::Value checkbans_manager_checkbans_get(personaids)
Check if some playerids are already banned in a server within the manager if the group has made his bans public

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**personaids** | Option<**String**> |  |  |[default to ]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info_manager_info_get

> models::ManagerInfo info_manager_info_get()
Get usage info about the manager.

This is only the amount of usage for privacy

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ManagerInfo**](ManagerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboard_manager_leaderboard_get

> models::ManagerServerLeaderBoard leaderboard_manager_leaderboard_get(sort, amount, gameid, serverid)
Get best players of your server

It only works on servers using our manager: manager.gametools.network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<[**ManagerLeaderboardSortTypes**](.md)> | Platform of the server you want to search for |  |[default to score]
**amount** | Option<**i32**> | Number of players to return |  |[default to 10]
**gameid** | Option<**String**> | Id of the server you want to search for |  |
**serverid** | Option<**String**> | Id of the server within the manager: \"https://manager.gametools.network/server/...\" |  |

### Return type

[**models::ManagerServerLeaderBoard**](ManagerServerLeaderBoard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboard_v2_manager_leaderboard_v2_get

> models::ManagerServerLeaderBoardV2 leaderboard_v2_manager_leaderboard_v2_get(serverid, gameid, player_name_filter, amount, sort, days)
Get best players of your server

It only works on servers using our manager: manager.gametools.network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serverid** | Option<**String**> | Id of the server within the manager: \"https://manager.gametools.network/server/...\" |  |
**gameid** | Option<**i32**> | Id of the server you want to search for |  |
**player_name_filter** | Option<**String**> | Part of a playername to filter for |  |[default to ]
**amount** | Option<**i32**> | Number of players to return |  |[default to 10]
**sort** | Option<[**V2ManagerLeaderboardSortTypes**](.md)> | Platform of the server you want to search for |  |[default to score]
**days** | Option<**i32**> | Number of days to make a leaderboard of |  |[default to 7]

### Return type

[**models::ManagerServerLeaderBoardV2**](ManagerServerLeaderBoardV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## live_manager_live_get

> models::ManagerLivePlayer live_manager_live_get(name, playerid, oid, platform, skip_battlelog)
Get the live stats difference since the player joined a server, stats: none means couldn't find stats

It only works on servers using our manager: manager.gametools.network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**models::ManagerLivePlayer**](ManagerLivePlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## players_manager_players_get

> models::ManagerServerPlayers players_manager_players_get(gameid, serverid)
Get playerlist of your server

It only works on servers using our manager: manager.gametools.network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gameid** | Option<**String**> | Id of the server you want to search for |  |
**serverid** | Option<**String**> | Id of the server within the manager: \"https://manager.gametools.network/server/...\" |  |

### Return type

[**models::ManagerServerPlayers**](ManagerServerPlayers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_sessions_manager_server_sessions_get

> models::ManagerPlayerSession server_sessions_manager_server_sessions_get(serverid, start_datetime, end_datetime)
Get the finished play sessions from a server of our manager.

It only tracks the session within this week and only works on servers using our manager: manager.gametools.network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serverid** | Option<**String**> | Id of the server within the manager: \"https://manager.gametools.network/server/...\" |  |
**start_datetime** | Option<**String**> | Start date |  |[default to 2025-03-28T00:00:00Z]
**end_datetime** | Option<**String**> | End date |  |[default to 2025-03-28T13:31:05.856410Z]

### Return type

[**models::ManagerPlayerSession**](ManagerPlayerSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sessions_manager_sessions_get

> models::ManagerPlayerSession sessions_manager_sessions_get(name, playerid, oid, platform, skip_battlelog)
Get the most recent stats from a player of our manager.

It only tracks the session within this week and only works on servers using our manager: manager.gametools.network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**oid** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**FrostbitePlatforms**](.md)> | Platform the player uses |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**models::ManagerPlayerSession**](ManagerPlayerSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

