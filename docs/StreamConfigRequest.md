# StreamConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**brokers** | **Vec<String>** | List of URLs referencing Kafka broker connections via SASL_SSL | 
**topic** | **String** | Name of the relevant Kafka topic category storing log records | 
**user** | **String** | Username in provided credentials to authorize access to brokers | 
**status** | Option<**String**> | Enumerated (binary) string representing whether or not the stream config is active | [optional][readonly]
**password** | **String** | Password in provided credentials to authorize access to brokers | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


