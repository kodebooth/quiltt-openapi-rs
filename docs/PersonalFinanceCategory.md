# PersonalFinanceCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**confidence_level** | Option<**String**> | A description of how confident we are that the provided categories accurately describe the transaction intent.  `VERY_HIGH`: We are more than 98% confident that this category reflects the intent of the transaction. `HIGH`: We are more than 90% confident that this category reflects the intent of the transaction. `MEDIUM`: We are moderately confident that this category reflects the intent of the transaction. `LOW`: This category may reflect the intent, but there may be other categories that are more accurate. `UNKNOWN`: We don’t know the confidence level for this category. | [optional]
**detailed** | **String** | A granular category conveying the transaction's intent. This field can also be used as a unique identifier for the category. | 
**primary** | **String** | A high level category that communicates the broad category of the transaction. | 
**version** | Option<**Version**> | Indicates which version of the personal finance category taxonomy is being used. [View PFCv2 and PFCv1 taxonomies](https://plaid.com/documents/pfc-taxonomy-all.csv).  If you enabled Transactions or Enrich before December 2025 you will receive the `v1` taxonomy by default and may request `v2` by explicitly setting this field to `v2` in the request.  If you enabled Transactions or Enrich on or after December 2025, you may only receive the `v2` taxonomy.  (enum: v1, v2) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


