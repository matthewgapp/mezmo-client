# Channel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**integration** | Option<**String**> | Defines the type of notification channel. Valid values are: `email`, `webhook`, `slack`, and `pagerduty` | [optional]
**emails** | Option<**Vec<String>**> |  | [optional]
**url** | Option<**String**> | Webhook URL. | [optional]
**key** | Option<**String**> | PagerDuty key. | [optional]
**triggerlimit** | Option<**String**> | Specify the number of log lines that match the view's filtering and search criteria. When the number of log lines is reached, an alert is triggered. | [optional]
**triggerinterval** | Option<**String**> | Specify how often to trigger an alert. Valid values are: 30 seconds, 1 minute, 5 minutes, 15 minutes, 30 minutes, 1 hour, 6 hours, 12 hours, 24 hours | [optional]
**immediate** | Option<**bool**> | Set to true if you want the trigger condition to be evaluated as soon as the triggerlimit is reached. | [optional]
**terminal** | Option<**bool**> | Set to true if you want the trigger condition to be evaluated after the time that you specify in the triggerinterval field is reached. | [optional]
**operator** | Option<**String**> | Type of alert. Valid values are: `presence`and `absence`. | [optional]
**timezone** | Option<**String**> | Timezone used to report timestamps. This can be any value defined in the tz database https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. | [optional]
**method** | Option<[**crate::models::Method**](method.md)> |  | [optional]
**headers** | Option<[**crate::models::Headers**](headers.md)> |  | [optional]
**body_template** | Option<[**crate::models::BodyTemplate**](bodyTemplate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


