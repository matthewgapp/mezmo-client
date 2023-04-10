# IndexRateAlertConfigResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_lines** | Option<**i32**> | The number of lines required in order to trigger the alert. | [optional]
**max_z_score** | Option<**i32**> | The number of standard deviations above the 30 day average lines in order to trigger the alert. | [optional]
**threshold_alert** | Option<**String**> | Determines if you want alerts to be triggered if both the max lines and standard deviation have been triggered, or if one of the thresholds has been reached. | [optional]
**frequency** | Option<**String**> | Notify recipients once per hour, or once per day, (starting when the threshold is passed the first time) until the index rate drops below the thresholds. When the index rate drops below the thresholds, alerting stops. | [optional]
**channels** | Option<[**crate::models::IndexRateAlertConfigResponseChannels**](indexRateAlertConfigResponse_channels.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


