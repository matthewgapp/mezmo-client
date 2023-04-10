# IngestlinesLinesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | Option<**String**> | UNIX timestamp, including milliseconds, when the log entry was recorded. | [optional]
**line** | **String** | Text of the log line. | 
**app** | Option<**String**> | Name of the application that generates the log line. | [optional]
**level** | Option<**String**> | Set a value for the level. For example, sample values for this parameter are INFO, WARNING, ERROR. | [optional]
**meta** | Option<**String**> | This field is reserved for custom information that is associated with a log line. To add metadata to an API call, specify the meta field under the lines object. Metadata can be viewed inside that line's context. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


