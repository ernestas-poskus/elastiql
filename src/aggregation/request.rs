//! Request, response and other types used when [aggregating] documents.
//!
//! [aggregating]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html

pub use super::response::*;
use super::types::*;

use crate::search::query::CompoundQuery;
#[cfg(feature = "graphql")]
use crate::search::query::CompoundQueryInput;

/// An [aggregation] can be seen as a unit-of-work that builds analytic
/// information over a set of documents.
///
/// **Note**: Even though they are all marked optional by GraphQL, you **must**
/// specify **exactly one** aggregate field (apart from the optional
/// `aggregations` field). This is due to a limitation of GraphQL and will be
/// changed once [union input types] are supported.
///
/// [aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html
/// [union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Clone, Debug)]
#[graphql(name = "AggregationInput")]
pub struct RequestInput {
    /// The name for this aggregation.
    ///
    /// **NOTE**: this must be unique otherwise the query will only return
    /// results for the last one with this name.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub name: String,

    /// A `single-value` [*metrics*] aggregation that computes the average of
    /// numeric values that are extracted from the aggregated documents. These
    /// values can be extracted either from specific numeric fields in the
    /// documents, or be generated by a provided script.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub avg: Option<InnerAggregationInput>,

    /// A `single-value` [*metrics*] aggregation that computes the weighted average
    /// of numeric values that are extracted from the aggregated documents. These
    /// values can be extracted either from specific numeric fields in the
    /// documents.
    ///
    /// When calculating a regular average, each datapoint has an equal "weight"...
    /// it contributes equally to the final value. Weighted averages, on the other
    /// hand, weight each datapoint differently. The amount that each datapoint
    /// contributes to the final value is extracted from the document, or provided
    /// by a script.
    ///
    /// As a formula, a weighted average is the `∑(value * weight) / ∑(weight)`
    ///
    /// A regular average can be thought of as a weighted average where every value
    /// has an implicit weight of `1`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub weighted_avg: Option<WeightedAverageAggregationInput>,

    /// A `single-value` [*metrics*] aggregation that calculates an approximate
    /// count of distinct values. Values can be extracted either from specific
    /// fields in the document or generated by a script.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub cardinality: Option<InnerAggregationInput>,

    /// A `single-value` [*metrics*] aggregation that keeps track and returns
    /// the maximum value among the numeric values extracted from the aggregated
    /// documents.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub max: Option<InnerAggregationInput>,

    /// A `single-value` [*metrics*] aggregation that keeps track and returns
    /// the minimum value among numeric values extracted from the aggregated
    /// documents.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub min: Option<InnerAggregationInput>,

    /// This `single-value` [*metrics*] aggregation approximates the median
    /// absolute deviation of its search results.
    ///
    /// Median absolute deviation is a measure of variability. It is a robust
    /// statistic, meaning that it is useful for describing data that may have
    /// outliers, or may not be normally distributed. For such data it can be
    /// more descriptive than standard deviation.
    ///
    /// It is calculated as the median of each data point’s deviation from the
    /// median of the entire sample. That is, for a random variable `X`, the
    /// median absolute deviation is `median(|median(X) - Xi|)`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub median_absolute_deviation: Option<InnerAggregationInput>,

    /// A `multi-value` [*metrics*] aggregation that calculates one or more
    /// percentiles over numeric values extracted from the aggregated documents.
    ///
    /// Percentiles show the point at which a certain percentage of observed
    /// values occur. For example, the 95th percentile is the value which is
    /// greater than 95% of the observed values.
    ///
    /// Percentiles are often used to find outliers. In normal distributions,
    /// the 0.13th and 99.87th percentiles represents three standard deviations
    /// from the mean. Any data which falls outside three standard deviations is
    /// often considered an anomaly.
    ///
    /// When a range of percentiles are retrieved, they can be used to estimate
    /// the data distribution and determine if the data is skewed, bimodal, etc.
    ///
    /// *Note*: [Percentiles are (usually) estimated].
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    /// [Percentiles are (usually) estimated]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-percentile-aggregation.html#search-aggregations-metrics-percentile-aggregation-approximation
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentiles: Option<InnerAggregationInput>,

    /// A `multi-value` [*metrics*] aggregation that calculates one or more
    /// percentile ranks over numeric values extracted from the aggregated
    /// documents.
    ///
    /// *Note*: [Percentiles are (usually) estimated].
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    /// [Percentiles are (usually) estimated]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-percentile-aggregation.html#search-aggregations-metrics-percentile-aggregation-approximation
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentile_ranks: Option<InnerAggregationInput>,

    /// A `multi-value` [*metrics*] aggregation that computes stats over numeric
    /// values extracted from the aggregated documents.
    ///
    /// The stats that are returned consist of: `min`, `max`, `sum`, `count` and
    /// `avg`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub stats: Option<InnerAggregationInput>,

    /// A `multi-value` [*metrics*] aggregation that computes stats over numeric
    /// values extracted from the aggregated documents.
    ///
    /// The `extended_stats` aggregations is an extended version of the `stats`
    /// aggregation, where additional metrics are added such as
    /// `sum_of_squares`, `variance`, `std_deviation` and
    /// `std_deviation_bounds`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub extended_stats: Option<InnerAggregationInput>,

    /// A `single-value` [*metrics*] aggregation that sums up numeric values
    /// that are extracted from the aggregated documents.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub sum: Option<InnerAggregationInput>,

    /// A `single-value` [*metrics*] aggregation  that counts the number of
    /// values that are extracted from the aggregated documents. These values
    /// can be extracted either from specific fields in the documents, or be
    /// generated by a provided script. Typically, this aggregator will be used
    /// in conjunction with other `single-value` aggregations.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_count: Option<InnerAggregationInput>,

    /// A [*bucketing*] aggregation that creates a single *bucket* for all
    /// documents matching the filters specified.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub filters: Option<CompoundQueryInput>,

    /// A [*multi-bucketing*] value source based aggregation where buckets are
    /// dynamically built - one per unique value.
    ///
    /// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub terms: Option<TermsAggregationInput>,

    /// A [`range`] ([*bucketing*]) aggregation.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [`range`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub range: Option<RangeAggregationInput>,

    /// A [`range`] ([*bucketing*]) aggregation that is dedicated for date
    /// values and supports [Date Math] expressions.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [`range`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html
    /// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default))]
    pub date_range: Option<DateRangeAggregationInput>,

    /// A [*multi-bucketing*] aggregation similar to the normal [histogram
    /// aggregation], but can only be used with date or date range values.
    ///
    /// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [histogram aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub date_histogram: Option<DateHistogramAggregationInput>,

    /// A [*multi-bucket*] aggregation similar to the [Date histogram aggregation]
    /// except instead of providing an interval to use as the width of each bucket,
    /// a target number of buckets is provided indicating the number of buckets
    /// needed and the interval of the buckets is automatically chosen to best
    /// achieve that target. The number of buckets returned will always be less than
    /// or equal to this target number.
    ///
    /// [Date histogram aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html
    /// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub auto_date_histogram: Option<AutoDateHistogramAggregationInput>,

    /// A [*multi-bucketing*] values source based aggregation that can be
    /// applied on numeric values or numeric range values extracted from the
    /// documents. It dynamically builds fixed size (a.k.a. interval) buckets
    /// over the values.
    ///
    /// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub histogram: Option<HistogramAggregationInput>,

    /// [Variable width histogram] is a [*multi-bucket*] aggregation similar to
    /// [histogram]. However, the width of each bucket is not specified. Rather, a
    /// target number of buckets is provided and bucket intervals are dynamically
    /// determined based on the document distribution.
    ///
    /// [Variable width histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/latest/search-aggregations-bucket-variablewidthhistogram-aggregation.html
    /// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub variable_width_histogram: Option<VariableWidthHistogramInput>,

    /// A parent [*pipeline aggregation*] which executes a [script] which can
    /// perform per bucket computations on specified metrics in the parent
    /// multi-bucket aggregation. The specified metric must be numeric and the
    /// script must return a numeric value.
    ///
    /// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
    /// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub bucket_script: Option<BucketScriptInput>,

    /// A parent [*pipeline aggregation*] which executes a [script] which
    /// determines whether the current bucket will be retained in the parent
    /// multi-bucket aggregation. The specified metric must be numeric and the
    /// script must return a boolean value.
    ///
    /// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
    /// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub bucket_selector: Option<BucketSelectorInput>,

    /// A parent [*pipeline aggregation*] which sorts the buckets of its parent
    /// `multi-bucket` aggregation. Zero or more sort fields may be specified
    /// together with the corresponding sort order. Each bucket may be sorted based
    /// on its `_key`, `_count` or its sub-aggregations. In addition, parameters
    /// from and size may be set in order to truncate the result buckets.
    ///
    /// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub bucket_sort: Option<BucketSortInput>,

    /// A special single [*bucketing*] aggregation that enables aggregating
    /// [nested] documents.
    ///
    /// **Note**: *must* be used with the `aggregations` field as a sibling.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub nested: Option<NestedAggregationInput>,

    /// A special single [*bucketing*] aggregation that enables aggregating on
    /// parent docs from [nested] documents. Effectively this aggregation can
    /// break out of the nested block structure and link to other nested
    /// structures or the root document, which allows nesting other aggregations
    /// that aren’t part of the nested object in a nested aggregation.
    ///
    /// The [`ReverseNestedAggregation`] aggregation must be defined inside a
    /// [`nested`] aggregation.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`nested`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`ReverseNestedAggregation`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub reverse_nested: Option<ReverseNestedAggregationInput>,

    /// The custom [metadata] to associate with this aggregation that will be
    /// returned alongside the results for this aggregation.
    ///
    /// If the optional special key `_skip` is set to `true`, the results for
    /// this aggregation will be calculated but not returned with the other
    /// results.
    ///
    /// [metadata]: https://www.elastic.co/guide/en/elasticsearch/reference/current/agg-metadata.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub metadata: Option<crate::scalars::Map>,

    /// The sub aggregations, if any.
    #[cfg_attr(feature = "builder", builder(default))]
    pub aggregations: Option<Vec<RequestInput>>,
}

/// An [aggregation] can be seen as a unit-of-work that builds analytic
/// information over a set of documents.
///
/// [aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "Aggregation"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Clone, Debug)]
pub struct Request {
    /// The name for this aggregation.
    ///
    /// **NOTE**: this must be unique otherwise the query will only return
    /// results for the last one with this name.
    #[cfg_attr(feature = "builder", builder(setter(into)))]
    pub name: String,

    /// A `single-value` [*metrics*] aggregation that computes the average of
    /// numeric values that are extracted from the aggregated documents. These
    /// values can be extracted either from specific numeric fields in the
    /// documents, or be generated by a provided script.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub avg: Option<InnerAggregation>,

    /// A `single-value` [*metrics*] aggregation that computes the weighted average
    /// of numeric values that are extracted from the aggregated documents. These
    /// values can be extracted either from specific numeric fields in the
    /// documents.
    ///
    /// When calculating a regular average, each datapoint has an equal "weight"...
    /// it contributes equally to the final value. Weighted averages, on the other
    /// hand, weight each datapoint differently. The amount that each datapoint
    /// contributes to the final value is extracted from the document, or provided
    /// by a script.
    ///
    /// As a formula, a weighted average is the `∑(value * weight) / ∑(weight)`
    ///
    /// A regular average can be thought of as a weighted average where every value
    /// has an implicit weight of `1`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub weighted_avg: Option<WeightedAverageAggregation>,

    /// A `single-value` [*metrics*] aggregation that calculates an approximate
    /// count of distinct values. Values can be extracted either from specific
    /// fields in the document or generated by a script.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub cardinality: Option<InnerAggregation>,

    /// A `single-value` [*metrics*] aggregation that keeps track and returns
    /// the maximum value among the numeric values extracted from the aggregated
    /// documents.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub max: Option<InnerAggregation>,

    /// A `single-value` [*metrics*] aggregation that keeps track and returns
    /// the minimum value among numeric values extracted from the aggregated
    /// documents.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub min: Option<InnerAggregation>,

    /// This `single-value` [*metrics*] aggregation approximates the [median
    /// absolute deviation] of its search results.
    ///
    /// Median absolute deviation is a measure of variability. It is a robust
    /// statistic, meaning that it is useful for describing data that may have
    /// outliers, or may not be normally distributed. For such data it can be
    /// more descriptive than standard deviation.
    ///
    /// It is calculated as the median of each data point’s deviation from the
    /// median of the entire sample. That is, for a random variable `X`, the
    /// [median absolute deviation] is `median(|median(X) - Xi|)`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    /// [median absolute deviation]: https://en.wikipedia.org/wiki/Median_absolute_deviation
    #[cfg_attr(feature = "builder", builder(default))]
    pub median_absolute_deviation: Option<InnerAggregation>,

    /// A `multi-value` [*metrics*] aggregation that calculates one or more
    /// percentiles over numeric values extracted from the aggregated documents.
    ///
    /// Percentiles show the point at which a certain percentage of observed
    /// values occur. For example, the 95th percentile is the value which is
    /// greater than 95% of the observed values.
    ///
    /// Percentiles are often used to find outliers. In normal distributions,
    /// the 0.13th and 99.87th percentiles represents three standard deviations
    /// from the mean. Any data which falls outside three standard deviations is
    /// often considered an anomaly.
    ///
    /// When a range of percentiles are retrieved, they can be used to estimate
    /// the data distribution and determine if the data is skewed, bimodal, etc.
    ///
    /// *Note*: [Percentiles are (usually) estimated].
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    /// [Percentiles are (usually) estimated]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-percentile-aggregation.html#search-aggregations-metrics-percentile-aggregation-approximation
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentiles: Option<InnerAggregation>,

    /// A `multi-value` [*metrics*] aggregation that calculates one or more
    /// percentile ranks over numeric values extracted from the aggregated
    /// documents.
    ///
    /// *Note*: [Percentiles are (usually) estimated].
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    /// [Percentiles are (usually) estimated]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-percentile-aggregation.html#search-aggregations-metrics-percentile-aggregation-approximation
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentile_ranks: Option<InnerAggregation>,

    /// A `multi-value` [*metrics*] aggregation that computes stats over numeric
    /// values extracted from the aggregated documents.
    ///
    /// The stats that are returned consist of: `min`, `max`, `sum`, `count` and
    /// `avg`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub stats: Option<InnerAggregation>,

    /// A `multi-value` [*metrics*] aggregation that computes stats over numeric
    /// values extracted from the aggregated documents.
    ///
    /// The `extended_stats` aggregations is an extended version of the `stats`
    /// aggregation, where additional metrics are added such as
    /// `sum_of_squares`, `variance`, `std_deviation` and
    /// `std_deviation_bounds`.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub extended_stats: Option<InnerAggregation>,

    /// A `single-value` [*metrics*] aggregation that sums up numeric values
    /// that are extracted from the aggregated documents.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub sum: Option<InnerAggregation>,

    /// A `single-value` [*metrics*] aggregation  that counts the number of
    /// values that are extracted from the aggregated documents. These values
    /// can be extracted either from specific fields in the documents, or be
    /// generated by a provided script. Typically, this aggregator will be used
    /// in conjunction with other `single-value` aggregations.
    ///
    /// [*metrics*]:  https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_count: Option<InnerAggregation>,

    /// A [*bucketing*] aggregation that creates a single *bucket* for all
    /// documents matching the filters specified.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub filters: Option<CompoundQuery>,

    /// A [*multi-bucketing*] value source based aggregation where buckets are
    /// dynamically built - one per unique value.
    ///
    /// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub terms: Option<TermsAggregation>,

    /// A [`range`] ([*bucketing*]) aggregation.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [`range`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub range: Option<RangeAggregation>,

    /// A [`range`] ([*bucketing*]) aggregation that is dedicated for date
    /// values and supports [Date Math] expressions.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [`range`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html
    /// [Date Math]: https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub date_range: Option<DateRangeAggregation>,

    /// A [*multi-bucketing*] aggregation similar to the normal [histogram
    /// aggregation], but can only be used with date or date range values.
    ///
    /// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [histogram aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-histogram-aggregation.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub date_histogram: Option<DateHistogramAggregation>,

    /// A [*multi-bucket*] aggregation similar to the [Date histogram aggregation]
    /// except instead of providing an interval to use as the width of each bucket,
    /// a target number of buckets is provided indicating the number of buckets
    /// needed and the interval of the buckets is automatically chosen to best
    /// achieve that target. The number of buckets returned will always be less than
    /// or equal to this target number.
    ///
    /// [Date histogram aggregation]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html
    /// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub auto_date_histogram: Option<AutoDateHistogramAggregation>,

    /// A [*multi-bucketing*] values source based aggregation that can be
    /// applied on numeric values or numeric range values extracted from the
    /// documents. It dynamically builds fixed size (a.k.a. interval) buckets
    /// over the values.
    ///
    /// [*multi-bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub histogram: Option<HistogramAggregation>,

    /// [Variable width histogram] is a [*multi-bucket*] aggregation similar to
    /// [histogram]. However, the width of each bucket is not specified. Rather, a
    /// target number of buckets is provided and bucket intervals are dynamically
    /// determined based on the document distribution.
    ///
    /// [Variable width histogram]: https://www.elastic.co/guide/en/elasticsearch/reference/latest/search-aggregations-bucket-variablewidthhistogram-aggregation.html
    /// [*multi-bucket*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub variable_width_histogram: Option<VariableWidthHistogram>,

    /// A parent [*pipeline aggregation*] which executes a [script] which can
    /// perform per bucket computations on specified metrics in the parent
    /// multi-bucket aggregation. The specified metric must be numeric and the
    /// script must return a numeric value.
    ///
    /// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
    /// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub bucket_script: Option<BucketScript>,

    /// A parent [*pipeline aggregation*] which executes a [script] which
    /// determines whether the current bucket will be retained in the parent
    /// multi-bucket aggregation. The specified metric must be numeric and the
    /// script must return a boolean value.
    ///
    /// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
    /// [script]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub bucket_selector: Option<BucketSelector>,

    /// A parent [*pipeline aggregation*] which sorts the buckets of its parent
    /// `multi-bucket` aggregation. Zero or more sort fields may be specified
    /// together with the corresponding sort order. Each bucket may be sorted based
    /// on its `_key`, `_count` or its sub-aggregations. In addition, parameters
    /// from and size may be set in order to truncate the result buckets.
    ///
    /// [*pipeline aggregation*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub bucket_sort: Option<BucketSort>,

    /// A special single [*bucketing*] aggregation that enables aggregating
    /// [nested] documents.
    ///
    /// **Note**: *must* be used with the `aggregations` field as a sibling.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub nested: Option<NestedAggregation>,

    /// A special single [*bucketing*] aggregation that enables aggregating on
    /// parent docs from [nested] documents. Effectively this aggregation can
    /// break out of the nested block structure and link to other nested
    /// structures or the root document, which allows nesting other aggregations
    /// that aren’t part of the nested object in a nested aggregation.
    ///
    /// The [`ReverseNestedAggregation`] aggregation must be defined inside a
    /// [`nested`] aggregation.
    ///
    /// [*bucketing*]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html
    /// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`nested`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
    /// [`ReverseNestedAggregation`]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-reverse-nested-aggregation.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub reverse_nested: Option<ReverseNestedAggregation>,

    /// The custom [metadata] to associate with this aggregation that will be
    /// returned alongside the results for this aggregation.
    ///
    /// If the optional special key `_skip` is set to `true`, the results for
    /// this aggregation will be calculated but not returned with the other
    /// results.
    ///
    /// [metadata]: https://www.elastic.co/guide/en/elasticsearch/reference/current/agg-metadata.html
    #[cfg_attr(feature = "builder", builder(default))]
    pub metadata: Option<crate::scalars::Map>,

    /// The sub aggregation, if any.
    #[cfg_attr(feature = "builder", builder(default, setter(into)))]
    pub aggregations: Option<Vec<Request>>,
}

// TODO: auto generate this with a proc_macro?
#[cfg(feature = "graphql")]
impl From<RequestInput> for Request {
    #[inline]
    fn from(aggregation: RequestInput) -> Self {
        Self {
            name: aggregation.name,
            avg: aggregation.avg.map(Into::into),
            weighted_avg: aggregation.weighted_avg.map(Into::into),
            cardinality: aggregation.cardinality.map(Into::into),
            max: aggregation.max.map(Into::into),
            min: aggregation.min.map(Into::into),
            median_absolute_deviation: aggregation.median_absolute_deviation.map(Into::into),
            percentiles: aggregation.percentiles.map(Into::into),
            percentile_ranks: aggregation.percentile_ranks.map(Into::into),
            stats: aggregation.stats.map(Into::into),
            extended_stats: aggregation.extended_stats.map(Into::into),
            sum: aggregation.sum.map(Into::into),
            value_count: aggregation.value_count.map(Into::into),
            filters: aggregation.filters.map(Into::into),
            terms: aggregation.terms.map(Into::into),
            range: aggregation.range.map(Into::into),
            date_range: aggregation.date_range.map(Into::into),
            date_histogram: aggregation.date_histogram.map(Into::into),
            auto_date_histogram: aggregation.auto_date_histogram.map(Into::into),
            histogram: aggregation.histogram.map(Into::into),
            variable_width_histogram: aggregation.variable_width_histogram.map(Into::into),
            bucket_script: aggregation.bucket_script.map(Into::into),
            bucket_selector: aggregation.bucket_selector.map(Into::into),
            bucket_sort: aggregation.bucket_sort.map(Into::into),
            nested: aggregation.nested.map(Into::into),
            reverse_nested: aggregation.reverse_nested.map(Into::into),
            metadata: aggregation.metadata,
            aggregations: aggregation
                .aggregations
                .map(|aggs| aggs.into_iter().map(Into::into).collect()),
        }
    }
}
