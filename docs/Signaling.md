# Signaling

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signal** | Option<**String**> | - `no_signal`: No signal is active. Write “no_signal” to stop active signal. - `on_off`: Toggles between max brightness and Off in fixed color. - `on_off_color`: Toggles between off and max brightness with color provided. - `alternating`: Alternates between 2 provided colors.  | [optional]
**duration** | Option<**i32**> | Duration has a max of 65534000 ms and a stepsize of 1 second. Values inbetween steps will be rounded. Duration is ignored for `no_signal`.  | [optional]
**color** | Option<[**Vec<models::Color>**](Color.md)> | List of colors to apply to the signal (not supported by all signals) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


