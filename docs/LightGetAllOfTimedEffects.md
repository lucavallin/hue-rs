# LightGetAllOfTimedEffects

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**effect** | Option<[**models::SupportedTimedEffects**](SupportedTimedEffects.md)> |  | [optional]
**effect_values** | Option<[**Vec<models::SupportedTimedEffects>**](SupportedTimedEffects.md)> | Possible timed effect values you can set in a light | [optional]
**status** | Option<[**models::SupportedTimedEffects**](SupportedTimedEffects.md)> |  | [optional]
**status_values** | Option<[**Vec<models::SupportedTimedEffects>**](SupportedTimedEffects.md)> | Possible status values in which a light could be when playing a timed effect. | [optional]
**duration** | Option<**i32**> | Duration is mandatory when timed effect is set except for no_effect. Resolution decreases for a larger duration. e.g Effects with duration smaller than a minute will be rounded to a resolution of 1s, while effects with duration larger than an hour will be arounded up to a resolution of 300s. Duration has a max of 21600000 ms. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


