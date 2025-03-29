# \Battlefield2042Api

All URIs are relative to *https://api.gametools.network*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bf2042_available_tags_bf2042_availabletags_get**](Battlefield2042Api.md#bf2042_available_tags_bf2042_availabletags_get) | **GET** /bf2042/availabletags/ | get list of available server/experience tags
[**bf2042_collections_bf2042_scheduledcollections_get**](Battlefield2042Api.md#bf2042_collections_bf2042_scheduledcollections_get) | **GET** /bf2042/scheduledcollections/ | All current menuitems of the game
[**bf2042_get_feslid_bf2042_feslid_get**](Battlefield2042Api.md#bf2042_get_feslid_bf2042_feslid_get) | **GET** /bf2042/feslid/ | Ask for the info about the username/avatar seperately (get request for CORS)
[**bf2042_get_progression_types_bf2042_progressiontypes_get**](Battlefield2042Api.md#bf2042_get_progression_types_bf2042_progressiontypes_get) | **GET** /bf2042/progressiontypes/ | Get the available progression types for Portal
[**bf2042_get_store_catalog_bf2042_storecatalog_get**](Battlefield2042Api.md#bf2042_get_store_catalog_bf2042_storecatalog_get) | **GET** /bf2042/storecatalog/ | Get list of dlc's that can be bought
[**bf2042_mix_info_bf2042_mixinfo_get**](Battlefield2042Api.md#bf2042_mix_info_bf2042_mixinfo_get) | **GET** /bf2042/mixinfo/ | Get info about a submenu entry (via mixId)
[**bf2042_offers_bf2042_offers_get**](Battlefield2042Api.md#bf2042_offers_bf2042_offers_get) | **GET** /bf2042/offers/ | Get the offers available in the store  (warning: a lot of data)
[**bf2042_playground_bf2042_playground_get**](Battlefield2042Api.md#bf2042_playground_bf2042_playground_get) | **GET** /bf2042/playground/ | Gets some basic info about a experience made in portal
[**bf2042_post_feslid_bf2042_feslid_post**](Battlefield2042Api.md#bf2042_post_feslid_bf2042_feslid_post) | **POST** /bf2042/feslid/ | Ask for the info about the username/avatar seperately
[**bf2042blueprints_bf2042_scheduledblueprint_get**](Battlefield2042Api.md#bf2042blueprints_bf2042_scheduledblueprint_get) | **GET** /bf2042/scheduledblueprint/ | all currently available portal settings (warning: a lot of data)
[**bf2042constraints_bf2042_constraints_get**](Battlefield2042Api.md#bf2042constraints_bf2042_constraints_get) | **GET** /bf2042/constraints/ | get the global limits for portal
[**bf2042detailed_servers_bf2042_detailedserver_get**](Battlefield2042Api.md#bf2042detailed_servers_bf2042_detailedserver_get) | **GET** /bf2042/detailedserver/ | Get a list of servers from portal based on given name
[**bf2042inventory_bf2042_inventory_get**](Battlefield2042Api.md#bf2042inventory_bf2042_inventory_get) | **GET** /bf2042/inventory/ | playercard info, no info added about it yet
[**bf2042multiple_bf2042_multiple_post**](Battlefield2042Api.md#bf2042multiple_bf2042_multiple_post) | **POST** /bf2042/multiple/ | Get for multiple players via grpc (max 128 players at a time)
[**bf2042player_bf2042_player_get**](Battlefield2042Api.md#bf2042player_bf2042_player_get) | **GET** /bf2042/player/ | Get id of a player within bf2042
[**bf2042servers_bf2042_servers_get**](Battlefield2042Api.md#bf2042servers_bf2042_servers_get) | **GET** /bf2042/servers/ | Get a list of servers from portal based on given name
[**bf2042stats_bf2042_stats_get**](Battlefield2042Api.md#bf2042stats_bf2042_stats_get) | **GET** /bf2042/stats/ | Get stats from the given player for bf2042
[**bf2042translations_bf2042_translations_get**](Battlefield2042Api.md#bf2042translations_bf2042_translations_get) | **GET** /bf2042/translations/ | translation files used for portal and serveritems (warning: a lot of data)
[**game_status_bf2042_status_get**](Battlefield2042Api.md#game_status_bf2042_status_get) | **GET** /bf2042/status/ | Get the player- / serveramount for all regions for Battlefield 2042 portal.
[**logged_server_data_bf2042_serverarray_get**](Battlefield2042Api.md#logged_server_data_bf2042_serverarray_get) | **GET** /bf2042/serverarray/ | Get the servers playeramount over time
[**print_logged_data_bf2042_statusarray_get**](Battlefield2042Api.md#print_logged_data_bf2042_statusarray_get) | **GET** /bf2042/statusarray/ | Get the player- / serveramount for all regions from database gathered every hour for Battlefield 2042 portal.
[**statsarray_bf2042_statsarray_get**](Battlefield2042Api.md#statsarray_bf2042_statsarray_get) | **GET** /bf2042/statsarray/ | Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)



## bf2042_available_tags_bf2042_availabletags_get

> models::AvailableTagsResponse bf2042_available_tags_bf2042_availabletags_get(lang)
get list of available server/experience tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |

### Return type

[**models::AvailableTagsResponse**](AvailableTagsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_collections_bf2042_scheduledcollections_get

> models::ScheduledCollectionsResponse bf2042_collections_bf2042_scheduledcollections_get(lang)
All current menuitems of the game

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |

### Return type

[**models::ScheduledCollectionsResponse**](ScheduledCollectionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_get_feslid_bf2042_feslid_get

> models::FeslPlayerInfo bf2042_get_feslid_bf2042_feslid_get(platformid, personaid, nucleusid)
Ask for the info about the username/avatar seperately (get request for CORS)

Returns username and avatar based on the given info (get request for CORS)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platformid** | Option<**i32**> | platform id (1 == pc, 2, ps4 and 3 xboxone) |  |
**personaid** | Option<**i32**> | ID of the playe |  |
**nucleusid** | Option<**i32**> | Platform specific id of the player (oid) |  |

### Return type

[**models::FeslPlayerInfo**](FeslPlayerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_get_progression_types_bf2042_progressiontypes_get

> models::GetProgressionTypesResponse bf2042_get_progression_types_bf2042_progressiontypes_get()
Get the available progression types for Portal

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetProgressionTypesResponse**](GetProgressionTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_get_store_catalog_bf2042_storecatalog_get

> models::GetStoreCatalogResponse bf2042_get_store_catalog_bf2042_storecatalog_get()
Get list of dlc's that can be bought

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetStoreCatalogResponse**](GetStoreCatalogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_mix_info_bf2042_mixinfo_get

> models::GetMixesByIdResponse bf2042_mix_info_bf2042_mixinfo_get(mixid)
Get info about a submenu entry (via mixId)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mixid** | **String** | MixId of a menuitem | [required] |

### Return type

[**models::GetMixesByIdResponse**](GetMixesByIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_offers_bf2042_offers_get

> models::GetOffersResponse bf2042_offers_bf2042_offers_get(lang)
Get the offers available in the store  (warning: a lot of data)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |

### Return type

[**models::GetOffersResponse**](GetOffersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_playground_bf2042_playground_get

> models::PlaygroundInfo bf2042_playground_bf2042_playground_get(playgroundid, experiencecode, blockydata, return_ownername, lang)
Gets some basic info about a experience made in portal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playgroundid** | Option<**String**> | Playground id |  |[default to ]
**experiencecode** | Option<**String**> | Experience code |  |[default to ]
**blockydata** | Option<**bool**> | Wheter to add blockly data. If you use the experiencecode instead of playgroundID, playgroundID, owner and blockydata cant be gathered! |  |[default to true]
**return_ownername** | Option<**bool**> | return_ownername will make it return the info a bit slower but returns the owner's name, use /bf2042/feslid/ if you want it seperate |  |[default to true]
**lang** | Option<**String**> |  |  |

### Return type

[**models::PlaygroundInfo**](PlaygroundInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042_post_feslid_bf2042_feslid_post

> models::FeslPlayerInfo bf2042_post_feslid_bf2042_feslid_post(body)
Ask for the info about the username/avatar seperately

Returns username and avatar based on the given info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::FeslPlayerInfo**](FeslPlayerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042blueprints_bf2042_scheduledblueprint_get

> models::ScheduledBlueprintResponse bf2042blueprints_bf2042_scheduledblueprint_get(lang)
all currently available portal settings (warning: a lot of data)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |

### Return type

[**models::ScheduledBlueprintResponse**](ScheduledBlueprintResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042constraints_bf2042_constraints_get

> models::ConstraintsResponse bf2042constraints_bf2042_constraints_get()
get the global limits for portal

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ConstraintsResponse**](ConstraintsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042detailed_servers_bf2042_detailedserver_get

> models::Bf2042DetailedServerInfo bf2042detailed_servers_bf2042_detailedserver_get(name, experiencename, serverid, return_ownername, lang)
Get a list of servers from portal based on given name

Available langauges for 'lang': ar-sa, de-de, en-us, es-es, es-mx, fr-fr, it-it, ja-jp, ko-kr, pl-pl, ru-ru, zh-cn and zh-tw. It defaults to en-us.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | searching based on servername |  |
**experiencename** | Option<**String**> | the name of the experience within portal.battlefield.com. |  |
**serverid** | Option<**String**> | Using a more permanent id of the server |  |
**return_ownername** | Option<**bool**> | return_ownername will make it return the info a bit slower but returns the owner's name, use /bf2042/feslid/ if you want it seperate |  |[default to true]
**lang** | Option<**String**> |  |  |

### Return type

[**models::Bf2042DetailedServerInfo**](Bf2042DetailedServerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042inventory_bf2042_inventory_get

> serde_json::Value bf2042inventory_bf2042_inventory_get(name, playerid, nucleus_id, platform, skip_battlelog)
playercard info, no info added about it yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**nucleus_id** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**KingstonPlatforms**](.md)> | Platform the player uses (both ps4, ps5 and xboxone and xboxseries are as if they are the same platform for stats) |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042multiple_bf2042_multiple_post

> serde_json::Value bf2042multiple_bf2042_multiple_post(player_info, raw, format_values)
Get for multiple players via grpc (max 128 players at a time)

Send a list of players in a array where you want stats from

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**player_info** | [**Vec<models::PlayerInfo>**](PlayerInfo.md) |  | [required] |
**raw** | Option<**bool**> | If it needs to return the raw stats |  |[default to false]
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042player_bf2042_player_get

> models::Bf2042PlayerSearch bf2042player_bf2042_player_get(name)
Get id of a player within bf2042

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the player to search for | [required] |

### Return type

[**models::Bf2042PlayerSearch**](Bf2042PlayerSearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042servers_bf2042_servers_get

> models::Bf2042Search bf2042servers_bf2042_servers_get(name, experiencename, region, maps, modes, limit, platform, has_password)
Get a list of servers from portal based on given name

To get multiple maps or regions, seperate them in a list with ';'.(cache of 10 seconds)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | searching based on servername |  |
**experiencename** | Option<**String**> | the name of the experience within portal.battlefield.com. |  |
**region** | Option<**String**> | The regions available are: all (all regions), eu, asia, nam (north america), sam (south america), afr (Africa) or oc (Oceana). |  |[default to all]
**maps** | Option<**String**> | The available maps are: 'Arica Harbor', 'Valparaiso', 'Battle of the Bulge', 'El Alamein', 'Caspian Border', 'Noshahr Canals', 'Orbital', 'Hourglass', 'Kaleidoscope', 'Breakaway', 'Discarded', 'Manifest' and 'Renewal' |  |[default to ]
**modes** | Option<**String**> | The available modes are: 'Breakthrough Large', 'Breakthrough', 'Conquest', 'Custom', 'Rush' and 'Conquest large' |  |[default to ]
**limit** | Option<**i32**> | the max amount of servers it will gather, give it a number between 1 and 250. |  |[default to 10]
**platform** | Option<**String**> | For platform there is pc, xboxone and ps4, ps5 and xboxseries |  |
**has_password** | Option<**bool**> | If you want to filter out servers with/without a password |  |

### Return type

[**models::Bf2042Search**](Bf2042Search.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042stats_bf2042_stats_get

> serde_json::Value bf2042stats_bf2042_stats_get(raw, format_values, name, playerid, nucleus_id, platform, skip_battlelog)
Get stats from the given player for bf2042

You can choose to only send the playername and optional platform (defaults to pc)  or send if (playerId) nucleus_id and platform to skip name conversion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**raw** | Option<**bool**> | If it needs to return the raw stats |  |[default to false]
**format_values** | Option<**bool**> | If precentage values have to be returned as string, example: '50%' (enabled by default for backwards compatability) |  |[default to true]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**nucleus_id** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**KingstonPlatforms**](.md)> | Platform the player uses (both ps4, ps5 and xboxone and xboxseries are as if they are the same platform for stats) |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bf2042translations_bf2042_translations_get

> models::TranslationsResponse bf2042translations_bf2042_translations_get(lang)
translation files used for portal and serveritems (warning: a lot of data)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |

### Return type

[**models::TranslationsResponse**](TranslationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## game_status_bf2042_status_get

> models::Bf2042GameStatus game_status_bf2042_status_get()
Get the player- / serveramount for all regions for Battlefield 2042 portal.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Bf2042GameStatus**](Bf2042GameStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_server_data_bf2042_serverarray_get

> serde_json::Value logged_server_data_bf2042_serverarray_get(days, gameid, serverid, servername)
Get the servers playeramount over time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**gameid** | Option<**String**> | Id of the server to get historic values for |  |
**serverid** | Option<**String**> | A more permanent id of the server to get historic values for |  |
**servername** | Option<**String**> | Name of the server to get historic values for |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_logged_data_bf2042_statusarray_get

> models::StatusArray print_logged_data_bf2042_statusarray_get(days, region, r#type)
Get the player- / serveramount for all regions from database gathered every hour for Battlefield 2042 portal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**region** | Option<[**FrostbiteRegionsWithMultiple**](.md)> | Region to get historic values for |  |[default to all]
**r#type** | Option<[**ExpandedStatusArrayType**](.md)> | Type of historic data to return |  |[default to amounts]

### Return type

[**models::StatusArray**](StatusArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statsarray_bf2042_statsarray_get

> models::StatsArray statsarray_bf2042_statsarray_get(days, name, playerid, nucleus_id, platform, skip_battlelog)
Get changes in players' stats over time, this gets saved every time you use the standard stats command in the api or gamestats bot (stats are per day, max 1 month)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Number of days to get historic values for |  |[default to 7]
**name** | Option<**String**> | Name of the player to get stats for |  |
**playerid** | Option<**i32**> | ID of the player to get stats for |  |
**nucleus_id** | Option<**i32**> | Platform specific id of the player (oid) |  |
**platform** | Option<[**KingstonPlatforms**](.md)> | Platform the player uses (both ps4, ps5 and xboxone and xboxseries are as if they are the same platform for stats) |  |[default to pc]
**skip_battlelog** | Option<**bool**> | If it needs to skip the battlelog player search |  |[default to false]

### Return type

[**models::StatsArray**](StatsArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

