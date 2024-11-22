# PowerupDimming

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<**String**> | Dimming will set the brightness to the specified value after power up. When setting mode “dimming”, the dimming property must be included. Previous will set brightness to the state it was in before powering off.  | [optional]
**dimming** | Option<**f64**> | Brightness percentage. value cannot be 0, writing 0 changes it to lowest possible brightness | [optional]
**color** | Option<[**models::LightGetAllOfPowerupDimmingColor**](LightGet_allOf_powerup_dimming_color.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


