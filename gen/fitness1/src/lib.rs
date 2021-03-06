// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *fitness* crate version *1.0.14+20200707*, where *20200707* is the exact revision of the *fitness:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.14*.
//! 
//! Everything else about the *fitness* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/fit/rest/v1/get-started).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/fitness1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Fitness.html) ... 
//! 
//! * users
//!  * [*data sources create*](struct.UserDataSourceCreateCall.html), [*data sources data point changes list*](struct.UserDataSourceDataPointChangeListCall.html), [*data sources datasets delete*](struct.UserDataSourceDatasetDeleteCall.html), [*data sources datasets get*](struct.UserDataSourceDatasetGetCall.html), [*data sources datasets patch*](struct.UserDataSourceDatasetPatchCall.html), [*data sources delete*](struct.UserDataSourceDeleteCall.html), [*data sources get*](struct.UserDataSourceGetCall.html), [*data sources list*](struct.UserDataSourceListCall.html), [*data sources update*](struct.UserDataSourceUpdateCall.html), [*dataset aggregate*](struct.UserDatasetAggregateCall.html), [*sessions delete*](struct.UserSessionDeleteCall.html), [*sessions list*](struct.UserSessionListCall.html) and [*sessions update*](struct.UserSessionUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Fitness.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.users().data_sources_delete(...).doit()
//! let r = hub.users().data_sources_update(...).doit()
//! let r = hub.users().data_sources_create(...).doit()
//! let r = hub.users().data_sources_get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-fitness1 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_fitness1 as fitness1;
//! use fitness1::DataSource;
//! use fitness1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use fitness1::Fitness;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = DataSource::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.users().data_sources_update(req, "userId", "dataSourceId")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See info about your body measurements and heart rate in Google Fit
    BodyRead,

    /// See and add info about your reproductive health in Google Fit. I consent to Google sharing my reporductive health information with this app.
    ReproductiveHealthWrite,

    /// See info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.
    BloodPressureRead,

    /// See and add to info about your nutrition in Google Fit
    NutritionWrite,

    /// See and add info about your blood glucose to Google Fit. I consent to Google sharing my blood glucose information with this app.
    BloodGlucoseWrite,

    /// See and add info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.
    OxygenSaturationWrite,

    /// See and add to your Google Fit location data
    LocationWrite,

    /// See info about your reproductive health in Google Fit. I consent to Google sharing my reporductive health information with this app.
    ReproductiveHealthRead,

    /// See and add to info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.
    BodyTemperatureWrite,

    /// See and add info about your body measurements and heart rate to Google Fit
    BodyWrite,

    /// Use Google Fit to see and store your physical activity data
    ActivityRead,

    /// See your Google Fit speed and distance data
    LocationRead,

    /// See and add info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.
    BloodPressureWrite,

    /// See info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.
    BodyTemperatureRead,

    /// See and add to your Google Fit physical activity data
    ActivityWrite,

    /// See info about your nutrition in Google Fit
    NutritionRead,

    /// See info about your blood glucose in Google Fit. I consent to Google sharing my blood glucose information with this app.
    BloodGlucoseRead,

    /// See info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.
    OxygenSaturationRead,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::BodyRead => "https://www.googleapis.com/auth/fitness.body.read",
            Scope::ReproductiveHealthWrite => "https://www.googleapis.com/auth/fitness.reproductive_health.write",
            Scope::BloodPressureRead => "https://www.googleapis.com/auth/fitness.blood_pressure.read",
            Scope::NutritionWrite => "https://www.googleapis.com/auth/fitness.nutrition.write",
            Scope::BloodGlucoseWrite => "https://www.googleapis.com/auth/fitness.blood_glucose.write",
            Scope::OxygenSaturationWrite => "https://www.googleapis.com/auth/fitness.oxygen_saturation.write",
            Scope::LocationWrite => "https://www.googleapis.com/auth/fitness.location.write",
            Scope::ReproductiveHealthRead => "https://www.googleapis.com/auth/fitness.reproductive_health.read",
            Scope::BodyTemperatureWrite => "https://www.googleapis.com/auth/fitness.body_temperature.write",
            Scope::BodyWrite => "https://www.googleapis.com/auth/fitness.body.write",
            Scope::ActivityRead => "https://www.googleapis.com/auth/fitness.activity.read",
            Scope::LocationRead => "https://www.googleapis.com/auth/fitness.location.read",
            Scope::BloodPressureWrite => "https://www.googleapis.com/auth/fitness.blood_pressure.write",
            Scope::BodyTemperatureRead => "https://www.googleapis.com/auth/fitness.body_temperature.read",
            Scope::ActivityWrite => "https://www.googleapis.com/auth/fitness.activity.write",
            Scope::NutritionRead => "https://www.googleapis.com/auth/fitness.nutrition.read",
            Scope::BloodGlucoseRead => "https://www.googleapis.com/auth/fitness.blood_glucose.read",
            Scope::OxygenSaturationRead => "https://www.googleapis.com/auth/fitness.oxygen_saturation.read",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::BodyRead
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Fitness related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_fitness1 as fitness1;
/// use fitness1::DataSource;
/// use fitness1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use fitness1::Fitness;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DataSource::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_update(req, "userId", "dataSourceId")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Fitness<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Fitness<C, A> {}

impl<'a, C, A> Fitness<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Fitness<C, A> {
        Fitness {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.14".to_string(),
            _base_url: "https://www.googleapis.com/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn users(&'a self) -> UserMethods<'a, C, A> {
        UserMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.14`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The specification of which data to aggregate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateBy {
    /// A data source ID to aggregate. Only data from the specified data source ID
    /// will be included in the aggregation. If specified, this data source must
    /// exist; the OAuth scopes in the supplied credentials must grant read access
    /// to this data type. The dataset in the response will have the same data
    /// source ID. Note: Data can be aggregated by either the dataTypeName or the
    /// dataSourceId, not both.
    #[serde(rename="dataSourceId")]
    pub data_source_id: Option<String>,
    /// The data type to aggregate. All data sources providing this data type will
    /// contribute data to the aggregation. The response will contain a single
    /// dataset for this data type name. The dataset will have a data source ID of
    /// derived:<output data type name>:com.google.android.gms:aggregated.
    /// If the user has no data for this data type, an empty data set will be
    /// returned. Note: Data can be aggregated by either the dataTypeName or the
    /// dataSourceId, not both.
    #[serde(rename="dataTypeName")]
    pub data_type_name: Option<String>,
}

impl Part for AggregateBy {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sessions list users](struct.UserSessionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSessionsResponse {
    /// Flag to indicate server has more data to transfer.
    /// DO NOT USE THIS FIELD. It is never populated in responses from the server.
    #[serde(rename="hasMoreData")]
    pub has_more_data: Option<bool>,
    /// The sync token which is used to sync further changes. This will only be
    /// provided if both <var>startTime</var> and <var>endTime</var> are omitted
    /// from the request.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// If <code>includeDeleted</code> is set to true in the request, and
    /// <var>startTime</var> and <var>endTime</var> are omitted, this will include
    /// sessions which were deleted since the last sync.
    #[serde(rename="deletedSession")]
    pub deleted_session: Option<Vec<Session>>,
    /// Sessions with an end time that is between <var>startTime</var> and
    /// <var>endTime</var> of the request.
    pub session: Option<Vec<Session>>,
}

impl ResponseResult for ListSessionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources list users](struct.UserDataSourceListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDataSourcesResponse {
    /// A previously created data source.
    #[serde(rename="dataSource")]
    pub data_source: Option<Vec<DataSource>>,
}

impl ResponseResult for ListDataSourcesResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    /// Package name for this application. This is used as a unique
    /// identifier when created by Android applications, but cannot be specified
    /// by REST clients. REST clients will have their developer project number
    /// reflected into the Data Source data stream IDs, instead of the packageName.
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
    /// Version of the application. You should update this field whenever the
    /// application changes in a way that affects the computation of the data.
    pub version: Option<String>,
    /// The name of this application. This is required for REST clients, but we
    /// do not enforce uniqueness of this name. It is provided as a matter of
    /// convenience for other developers who would like to identify which REST
    /// created an Application or Data Source.
    pub name: Option<String>,
    /// An optional URI that can be used to link back to the application.
    #[serde(rename="detailsUrl")]
    pub details_url: Option<String>,
}

impl Part for Application {}


/// Represents a single data point, generated by a particular data source.  A
/// data point holds a value for each field, an end timestamp and an optional
/// start time.  The exact semantics of each of these attributes are specified in
/// the documentation for the particular data type.
/// 
/// A data point can represent an instantaneous measurement, reading or input
/// observation, as well as averages or aggregates over a time interval.  Check
/// the data type documentation to determine which is the case for a particular
/// data type.
/// 
/// Data points always contain one value for each field of the data type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataPoint {
    /// DO NOT USE THIS FIELD. It is ignored, and not stored.
    #[serde(rename="computationTimeMillis")]
    pub computation_time_millis: Option<String>,
    /// Indicates the last time this data point was modified. Useful only in
    /// contexts where we are listing the data changes, rather than representing
    /// the current state of the data.
    #[serde(rename="modifiedTimeMillis")]
    pub modified_time_millis: Option<String>,
    /// The start time of the interval represented by this data point, in
    /// nanoseconds since epoch.
    #[serde(rename="startTimeNanos")]
    pub start_time_nanos: Option<String>,
    /// The data type defining the format of the values in this data point.
    #[serde(rename="dataTypeName")]
    pub data_type_name: Option<String>,
    /// Values of each data type field for the data point. It is expected that each
    /// value corresponding to a data type field will occur in the same order that
    /// the field is listed with in the data type specified in a data source.
    /// 
    /// Only one of integer and floating point fields will be populated, depending
    /// on the format enum value within data source's type field.
    pub value: Option<Vec<Value>>,
    /// The end time of the interval represented by this data point, in
    /// nanoseconds since epoch.
    #[serde(rename="endTimeNanos")]
    pub end_time_nanos: Option<String>,
    /// If the data point is contained in a dataset for a derived data source,
    /// this field will be populated with the data source stream ID that created
    /// the data point originally.
    /// 
    /// WARNING: do not rely on this field for anything other than debugging. The
    /// value of this field, if it is set at all, is an implementation detail and
    /// is not guaranteed to remain consistent.
    #[serde(rename="originDataSourceId")]
    pub origin_data_source_id: Option<String>,
    /// The raw timestamp from the original SensorEvent.
    #[serde(rename="rawTimestampNanos")]
    pub raw_timestamp_nanos: Option<String>,
}

impl Part for DataPoint {}


/// Holder object for the value of a single field in a data point.
/// 
/// A field value has a particular format and is only ever set to one of an
/// integer or a floating point value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// Map value.  The valid key space and units for the corresponding value
    /// of each entry should be documented as part of the data type definition.
    /// Keys should be kept small whenever possible. Data streams with large keys
    /// and high data frequency may be down sampled.
    #[serde(rename="mapVal")]
    pub map_val: Option<Vec<ValueMapValEntry>>,
    /// Floating point value. When this is set, other values must not be set.
    #[serde(rename="fpVal")]
    pub fp_val: Option<f64>,
    /// Integer value. When this is set, other values must not be set.
    #[serde(rename="intVal")]
    pub int_val: Option<i32>,
    /// String value.  When this is set, other values must not be set.
    /// Strings should be kept small whenever possible.  Data streams with large
    /// string values and high data frequency may be down sampled.
    #[serde(rename="stringVal")]
    pub string_val: Option<String>,
}

impl Part for Value {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateBucket {
    /// Available for Bucket.Type.SESSION
    pub session: Option<Session>,
    /// The end time for the aggregated data, in milliseconds since epoch,
    /// inclusive.
    #[serde(rename="endTimeMillis")]
    pub end_time_millis: Option<String>,
    /// Available for Bucket.Type.ACTIVITY_TYPE, Bucket.Type.ACTIVITY_SEGMENT
    pub activity: Option<i32>,
    /// The start time for the aggregated data, in milliseconds since epoch,
    /// inclusive.
    #[serde(rename="startTimeMillis")]
    pub start_time_millis: Option<String>,
    /// The type of a bucket signifies how the data aggregation is performed in the
    /// bucket.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// There will be one dataset per AggregateBy in the request.
    pub dataset: Option<Vec<Dataset>>,
}

impl Part for AggregateBucket {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketByActivity {
    /// The default activity stream will be used if a specific activityDataSourceId
    /// is not specified.
    #[serde(rename="activityDataSourceId")]
    pub activity_data_source_id: Option<String>,
    /// Specifies that only activity segments of duration longer than
    /// minDurationMillis are considered and used as a container for aggregated
    /// data.
    #[serde(rename="minDurationMillis")]
    pub min_duration_millis: Option<String>,
}

impl Part for BucketByActivity {}


/// Next id: 10
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dataset aggregate users](struct.UserDatasetAggregateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateRequest {
    /// The specification of data to be aggregated. At least one aggregateBy spec
    /// must be provided. All data that is specified will be aggregated using the
    /// same bucketing criteria. There will be one dataset in the response for
    /// every aggregateBy spec.
    #[serde(rename="aggregateBy")]
    pub aggregate_by: Option<Vec<AggregateBy>>,
    /// Specifies that data be aggregated each activity segment recored for a user.
    /// Similar to bucketByActivitySegment, but bucketing is done for each activity
    /// segment rather than all segments of the same type. Mutually exclusive of
    /// other bucketing specifications.
    #[serde(rename="bucketByActivitySegment")]
    pub bucket_by_activity_segment: Option<BucketByActivity>,
    /// The end of a window of time. Data that intersects with this time
    /// window will be aggregated. The time is in milliseconds since epoch,
    /// inclusive.
    #[serde(rename="endTimeMillis")]
    pub end_time_millis: Option<String>,
    /// Specifies that data be aggregated by user sessions. Data that does not fall
    /// within the time range of a session will not be included in the response.
    /// Mutually exclusive of other bucketing specifications.
    #[serde(rename="bucketBySession")]
    pub bucket_by_session: Option<BucketBySession>,
    /// Specifies that data be aggregated by the type of activity being performed
    /// when the data was recorded. All data that was recorded during a certain
    /// activity type (.for the given time range) will be aggregated into the same
    /// bucket. Data that was recorded while the user was not active will not be
    /// included in the response. Mutually exclusive of other bucketing
    /// specifications.
    #[serde(rename="bucketByActivityType")]
    pub bucket_by_activity_type: Option<BucketByActivity>,
    /// The start of a window of time. Data that intersects with this time
    /// window will be aggregated. The time is in milliseconds since epoch,
    /// inclusive.
    #[serde(rename="startTimeMillis")]
    pub start_time_millis: Option<String>,
    /// DO NOT POPULATE THIS FIELD. It is ignored.
    #[serde(rename="filteredDataQualityStandard")]
    pub filtered_data_quality_standard: Option<Vec<String>>,
    /// Specifies that data be aggregated by a single time interval. Mutually
    /// exclusive of other bucketing specifications.
    #[serde(rename="bucketByTime")]
    pub bucket_by_time: Option<BucketByTime>,
}

impl RequestValue for AggregateRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dataset aggregate users](struct.UserDatasetAggregateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateResponse {
    /// A list of buckets containing the aggregated data.
    pub bucket: Option<Vec<AggregateBucket>>,
}

impl ResponseResult for AggregateResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueMapValEntry {
    /// no description provided
    pub key: Option<String>,
    /// no description provided
    pub value: Option<MapValue>,
}

impl Part for ValueMapValEntry {}


/// A dataset represents a projection container for data points. They do not
/// carry any info of their own. Datasets represent a set of data points from a
/// particular data source. A data point can be found in more than one dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources datasets patch users](struct.UserDataSourceDatasetPatchCall.html) (request|response)
/// * [data sources datasets get users](struct.UserDataSourceDatasetGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    /// The smallest start time of all data points in this possibly partial
    /// representation of the dataset. Time is in nanoseconds from epoch. This
    /// should also match the first part of the dataset identifier.
    #[serde(rename="minStartTimeNs")]
    pub min_start_time_ns: Option<String>,
    /// This token will be set when a dataset is received in response to a GET
    /// request and the dataset is too large to be included in a single response.
    /// Provide this value in a subsequent GET request to return the next page of
    /// data points within this dataset.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The largest end time of all data points in this possibly partial
    /// representation of the dataset. Time is in nanoseconds from epoch. This
    /// should also match the second part of the dataset identifier.
    #[serde(rename="maxEndTimeNs")]
    pub max_end_time_ns: Option<String>,
    /// The data stream ID of the data source that created the points in this
    /// dataset.
    #[serde(rename="dataSourceId")]
    pub data_source_id: Option<String>,
    /// A partial list of data points contained in the dataset, ordered by largest
    /// endTimeNanos first. This list is considered complete when retrieving a
    /// small dataset and partial when patching a dataset or retrieving a dataset
    /// that is too large to include in a single response.
    pub point: Option<Vec<DataPoint>>,
}

impl RequestValue for Dataset {}
impl ResponseResult for Dataset {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataType {
    /// A field represents one dimension of a data type.
    pub field: Option<Vec<DataTypeField>>,
    /// Each data type has a unique, namespaced, name. All data types in the
    /// com.google namespace are shared as part of the platform.
    pub name: Option<String>,
}

impl Part for DataType {}


/// Sessions contain metadata, such as a user-friendly name and time interval
/// information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sessions update users](struct.UserSessionUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    /// A timestamp that indicates when the session was last modified.
    #[serde(rename="modifiedTimeMillis")]
    pub modified_time_millis: Option<String>,
    /// An end time, in milliseconds since epoch, inclusive.
    #[serde(rename="endTimeMillis")]
    pub end_time_millis: Option<String>,
    /// A description for this session.
    pub description: Option<String>,
    /// The type of activity this session represents.
    #[serde(rename="activityType")]
    pub activity_type: Option<i32>,
    /// The application that created the session.
    pub application: Option<Application>,
    /// A start time, in milliseconds since epoch, inclusive.
    #[serde(rename="startTimeMillis")]
    pub start_time_millis: Option<String>,
    /// Session active time. While start_time_millis and end_time_millis define
    /// the full session time, the active time can be shorter and specified by
    /// active_time_millis.
    /// If the inactive time during the session is known, it should also be
    /// inserted via a com.google.activity.segment data point with a STILL
    /// activity value
    #[serde(rename="activeTimeMillis")]
    pub active_time_millis: Option<String>,
    /// A client-generated identifier that is unique across all sessions owned by
    /// this particular user.
    pub id: Option<String>,
    /// A human readable name of the session.
    pub name: Option<String>,
}

impl RequestValue for Session {}
impl ResponseResult for Session {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketBySession {
    /// Specifies that only sessions of duration longer than minDurationMillis are
    /// considered and used as a container for aggregated data.
    #[serde(rename="minDurationMillis")]
    pub min_duration_millis: Option<String>,
}

impl Part for BucketBySession {}


/// Holder object for the value of an entry in a map field of a data point.
/// 
/// A map value supports a subset of the formats that the regular Value supports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MapValue {
    /// Floating point value.
    #[serde(rename="fpVal")]
    pub fp_val: Option<f64>,
}

impl Part for MapValue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketByTimePeriod {
    /// org.joda.timezone.DateTimeZone
    #[serde(rename="timeZoneId")]
    pub time_zone_id: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// no description provided
    pub value: Option<i32>,
}

impl Part for BucketByTimePeriod {}


/// Representation of an integrated device (such as a phone or a wearable) that
/// can hold sensors. Each sensor is exposed as a data source.
/// 
/// The main purpose of the device information contained in this class is to
/// identify the hardware of a particular data source.  This can be useful in
/// different ways, including:
/// 
/// <ul>
///   <li>Distinguishing two similar sensors on different devices (the step
///       counter on two nexus 5 phones, for instance)
///   <li>Display the source of data to the user (by using the device make /
///       model)
///   <li>Treat data differently depending on sensor type (accelerometers on a
///       watch may give different patterns than those on a phone)
///   <li>Build different analysis models for each device/version.
/// </ul>
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// End-user visible model name for the device.
    pub model: Option<String>,
    /// Version string for the device hardware/software.
    pub version: Option<String>,
    /// A constant representing the type of the device.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The serial number or other unique ID for the hardware. This field is
    /// obfuscated when read by any REST or Android client that did not create
    /// the data source. Only the data source creator will see the uid field in
    /// clear and normal form.
    /// 
    /// The obfuscation preserves equality; that is, given two IDs, if id1 == id2,
    /// obfuscated(id1) == obfuscated(id2).
    pub uid: Option<String>,
    /// Manufacturer of the product/hardware.
    pub manufacturer: Option<String>,
}

impl Part for Device {}


/// In case of multi-dimensional data (such as an accelerometer with x, y, and z
/// axes) each field represents one dimension. Each data type field has a unique
/// name which identifies it. The field also defines the format of the data (int,
/// float, etc.).
/// 
/// This message is only instantiated in code and not used for wire comms or
/// stored in any way.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataTypeField {
    /// no description provided
    pub optional: Option<bool>,
    /// Defines the name and format of data. Unlike data type names, field names
    /// are not namespaced, and only need to be unique within the data type.
    pub name: Option<String>,
    /// The different supported formats for each field in a data type.
    pub format: Option<String>,
}

impl Part for DataTypeField {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources data point changes list users](struct.UserDataSourceDataPointChangeListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDataPointChangesResponse {
    /// The continuation token, which is used to page through large result sets.
    /// Provide this value in a subsequent request to return the next page of
    /// results.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Deleted data points for the user. Note, for modifications this should be
    /// parsed before handling insertions.
    #[serde(rename="deletedDataPoint")]
    pub deleted_data_point: Option<Vec<DataPoint>>,
    /// The data stream ID of the data source with data point changes.
    #[serde(rename="dataSourceId")]
    pub data_source_id: Option<String>,
    /// Inserted data points for the user.
    #[serde(rename="insertedDataPoint")]
    pub inserted_data_point: Option<Vec<DataPoint>>,
}

impl ResponseResult for ListDataPointChangesResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketByTime {
    /// Specifies that result buckets aggregate data by exactly durationMillis time
    /// frames. Time frames that contain no data will be included in the response
    /// with an empty dataset.
    #[serde(rename="durationMillis")]
    pub duration_millis: Option<String>,
    /// no description provided
    pub period: Option<BucketByTimePeriod>,
}

impl Part for BucketByTime {}


/// Definition of a unique source of sensor data.  Data sources can expose raw
/// data coming from hardware sensors on local or companion devices. They can
/// also expose derived data, created by transforming or merging other data
/// sources. Multiple data sources can exist for the same data type. Every data
/// point inserted into or read from this service has an associated data
/// source.
/// 
/// The data source contains enough information to uniquely identify its data,
/// including the hardware device and the application that collected and/or
/// transformed the data. It also holds useful metadata, such as the hardware and
/// application versions, and the device type.
/// 
/// Each data source produces a unique stream of data, with a unique identifier.
/// Not all changes to data source affect the stream identifier, so that data
/// collected by updated versions of the same application/device can still be
/// considered to belong to the same data stream.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources delete users](struct.UserDataSourceDeleteCall.html) (response)
/// * [data sources update users](struct.UserDataSourceUpdateCall.html) (request|response)
/// * [data sources create users](struct.UserDataSourceCreateCall.html) (request|response)
/// * [data sources get users](struct.UserDataSourceGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// An end-user visible name for this data source.
    pub name: Option<String>,
    /// DO NOT POPULATE THIS FIELD. It is never populated in responses from the
    /// platform, and is ignored in queries. It will be removed in a future version
    /// entirely.
    #[serde(rename="dataQualityStandard")]
    pub data_quality_standard: Option<Vec<String>>,
    /// The data type defines the schema for a stream of data being collected by,
    /// inserted into, or queried from the Fitness API.
    #[serde(rename="dataType")]
    pub data_type: Option<DataType>,
    /// The stream name uniquely identifies this particular data source among
    /// other data sources of the same type from the same underlying producer.
    /// Setting the stream name is optional, but should be done whenever an
    /// application exposes two streams for the same data type, or when a device
    /// has two equivalent sensors.
    #[serde(rename="dataStreamName")]
    pub data_stream_name: Option<String>,
    /// Information about an application which feeds sensor data into the platform.
    pub application: Option<Application>,
    /// Representation of an integrated device (such as a phone or a wearable) that
    /// can hold sensors.
    pub device: Option<Device>,
    /// A unique identifier for the data stream produced by this data source. The
    /// identifier includes:<br/><br/>
    /// <ul>
    /// <li>The physical device's manufacturer, model, and serial number
    /// (UID).</li>
    /// <li>The application's package name or name. Package name is used when the
    /// data source was created by an Android application. The developer project
    /// number is used when the data source was created by a REST client.</li>
    /// <li>The data source's type.</li>
    /// <li>The data source's stream name.</li>
    /// </ul>
    /// Note that not all attributes of the data source are used as part of the
    /// stream identifier. In particular, the version of the hardware/the
    /// application isn't used. This allows us to preserve the same stream through
    /// version updates. This also means that two DataSource objects may represent
    /// the same data stream even if they're not equal.
    /// 
    /// The exact format of the data stream ID created by an Android application
    /// is:
    /// <var>type:dataType.name<wbr/>:application.packageName<wbr/>:device.manufacturer<wbr/>:device.model<wbr/>:device.uid<wbr/>:dataStreamName</var>
    /// 
    /// The exact format of the data stream ID created by a REST client is:
    /// <var>type:dataType.name<wbr/>:developer project
    /// number<wbr/>:device.manufacturer<wbr/>:device.model:device.uid<wbr/>:dataStreamName</var>
    /// 
    /// When any of the optional fields that make up the data stream ID are absent,
    /// they will be omitted from the data stream ID. The minimum viable data
    /// stream ID would be:
    /// type:dataType.name:developer project number
    /// 
    /// Finally, the developer project number and device UID are obfuscated when
    /// read by any REST or Android client that did not create the data source.
    /// Only the data source creator will see the developer project number in clear
    /// and normal form. This means a client will see a different set of
    /// data_stream_ids than another client with different credentials.
    #[serde(rename="dataStreamId")]
    pub data_stream_id: Option<String>,
    /// A constant describing the type of this data source. Indicates whether this
    /// data source produces raw or derived data.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl RequestValue for DataSource {}
impl ResponseResult for DataSource {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the `Fitness` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_fitness1 as fitness1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use fitness1::Fitness;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `data_sources_create(...)`, `data_sources_data_point_changes_list(...)`, `data_sources_datasets_delete(...)`, `data_sources_datasets_get(...)`, `data_sources_datasets_patch(...)`, `data_sources_delete(...)`, `data_sources_get(...)`, `data_sources_list(...)`, `data_sources_update(...)`, `dataset_aggregate(...)`, `sessions_delete(...)`, `sessions_list(...)` and `sessions_update(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserMethods<'a, C, A> {}

impl<'a, C, A> UserMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Aggregates data of a certain type or stream into buckets divided by a given
    /// type of boundary. Multiple data sets of multiple types and from multiple
    /// sources can be aggregated into exactly one bucket type per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Aggregate data for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    pub fn dataset_aggregate(&self, request: AggregateRequest, user_id: &str) -> UserDatasetAggregateCall<'a, C, A> {
        UserDatasetAggregateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified data source. The request will fail if the data
    /// source contains any data points.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Retrieve a data source for the person identified. Use <code>me</code> to
    ///              indicate the authenticated user. Only <code>me</code> is supported at this
    ///              time.
    /// * `dataSourceId` - The data stream ID of the data source to delete.
    pub fn data_sources_delete(&self, user_id: &str, data_source_id: &str) -> UserDataSourceDeleteCall<'a, C, A> {
        UserDataSourceDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a dataset containing all data points whose start and end times
    /// overlap with the specified range of the dataset minimum start time and
    /// maximum end time. Specifically, any data point whose start time is less
    /// than or equal to the dataset end time and whose end time is greater than or
    /// equal to the dataset start time.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Retrieve a dataset for the person identified. Use <code>me</code> to
    ///              indicate the authenticated user. Only <code>me</code> is supported at this
    ///              time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    /// * `datasetId` - Dataset identifier that is a composite of the minimum data point start time
    ///                 and maximum data point end time represented as nanoseconds from the epoch.
    ///                 The ID is formatted like: "<var>startTime</var>-<var>endTime</var>"
    ///                 where <var>startTime</var> and <var>endTime</var> are 64 bit integers.
    pub fn data_sources_datasets_get(&self, user_id: &str, data_source_id: &str, dataset_id: &str) -> UserDataSourceDatasetGetCall<'a, C, A> {
        UserDataSourceDatasetGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _limit: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new data source that is unique across all data sources belonging
    /// to this user.
    /// 
    /// A data source is a unique source of sensor data. Data sources can expose
    /// raw data coming from hardware sensors on local or companion devices. They
    /// can also expose derived data, created by transforming or merging other data
    /// sources. Multiple data sources can exist for the same data type. Every data
    /// point in every dataset inserted into or read from the Fitness API has an
    /// associated data source.
    /// 
    /// Each data source produces a unique stream of dataset updates, with a
    /// unique data source identifier. Not all changes to data source affect the
    /// data stream ID, so that data collected by updated versions of the same
    /// application/device can still be considered to belong to the same data
    /// source.
    /// 
    /// Data sources are identified using a string generated by the server, based
    /// on the contents of the source being created. The <code>dataStreamId</code>
    /// field should not be set when invoking this method. It
    /// will be automatically generated by the server with the correct format. If
    /// a <code>dataStreamId</code> is set, it must match the format that the
    /// server would generate. This format is a combination of some fields from the
    /// data source, and has a specific order. If it doesn't match, the request
    /// will fail with an error.
    /// 
    /// Specifying a DataType which is not a known type (beginning with
    /// "com.google.") will create a DataSource with a <em>custom data type</em>.
    /// Custom data types are only readable by the application that created them.
    /// Custom data types are <strong>deprecated</strong>; use standard data types
    /// instead.
    /// 
    /// In addition to the data source fields included in the data source ID, the
    /// developer project number that is authenticated when creating the data
    /// source is included. This developer project number is obfuscated when read
    /// by any other developer reading public data types.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Create the data source for the person identified. Use <code>me</code> to
    ///              indicate the authenticated user. Only <code>me</code> is supported at this
    ///              time.
    pub fn data_sources_create(&self, request: DataSource, user_id: &str) -> UserDataSourceCreateCall<'a, C, A> {
        UserDataSourceCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs an inclusive delete of all data points whose start and end times
    /// have any overlap with the time range specified by the dataset ID. For most
    /// data types, the entire data point will be deleted. For data types where the
    /// time span represents a consistent value (such as
    /// <code>com.google.activity.segment</code>), and a data point straddles
    /// either end point of the dataset, only the overlapping portion of the data
    /// point will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Delete a dataset for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    /// * `datasetId` - Dataset identifier that is a composite of the minimum data point start time
    ///                 and maximum data point end time represented as nanoseconds from the epoch.
    ///                 The ID is formatted like: "<var>startTime</var>-<var>endTime</var>"
    ///                 where <var>startTime</var> and <var>endTime</var> are 64 bit integers.
    pub fn data_sources_datasets_delete(&self, user_id: &str, data_source_id: &str, dataset_id: &str) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        UserDataSourceDatasetDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _modified_time_millis: Default::default(),
            _current_time_millis: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds data points to a dataset. The dataset need not be previously created.
    /// All points within the given dataset will be returned with subsquent calls
    /// to retrieve this dataset. Data points can belong to more than one dataset.
    /// This method does not use patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Patch a dataset for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    /// * `datasetId` - Dataset identifier that is a composite of the minimum data point start time
    ///                 and maximum data point end time represented as nanoseconds from the epoch.
    ///                 The ID is formatted like: "<var>startTime</var>-<var>endTime</var>"
    ///                 where <var>startTime</var> and <var>endTime</var> are 64 bit integers.
    pub fn data_sources_datasets_patch(&self, request: Dataset, user_id: &str, data_source_id: &str, dataset_id: &str) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        UserDataSourceDatasetPatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _current_time_millis: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a session specified by the given session ID.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Delete a session for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    /// * `sessionId` - The ID of the session to be deleted.
    pub fn sessions_delete(&self, user_id: &str, session_id: &str) -> UserSessionDeleteCall<'a, C, A> {
        UserSessionDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _session_id: session_id.to_string(),
            _current_time_millis: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified data source.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Retrieve a data source for the person identified. Use <code>me</code> to
    ///              indicate the authenticated user. Only <code>me</code> is supported at this
    ///              time.
    /// * `dataSourceId` - The data stream ID of the data source to retrieve.
    pub fn data_sources_get(&self, user_id: &str, data_source_id: &str) -> UserDataSourceGetCall<'a, C, A> {
        UserDataSourceGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified data source. The <code>dataStreamId</code>,
    /// <code>dataType</code>, <code>type</code>, <code>dataStreamName</code>, and
    /// <code>device</code> properties with the exception of <code>version</code>,
    /// cannot be modified.
    /// 
    /// Data sources are identified by their <code>dataStreamId</code>.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Update the data source for the person identified. Use <code>me</code> to
    ///              indicate the authenticated user. Only <code>me</code> is supported at this
    ///              time.
    /// * `dataSourceId` - The data stream ID of the data source to update.
    pub fn data_sources_update(&self, request: DataSource, user_id: &str, data_source_id: &str) -> UserDataSourceUpdateCall<'a, C, A> {
        UserDataSourceUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists sessions previously created.
    /// 
    /// # Arguments
    ///
    /// * `userId` - List sessions for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    pub fn sessions_list(&self, user_id: &str) -> UserSessionListCall<'a, C, A> {
        UserSessionListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _start_time: Default::default(),
            _page_token: Default::default(),
            _include_deleted: Default::default(),
            _end_time: Default::default(),
            _activity_type: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or insert a given session.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - Create sessions for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    /// * `sessionId` - The ID of the session to be created.
    pub fn sessions_update(&self, request: Session, user_id: &str, session_id: &str) -> UserSessionUpdateCall<'a, C, A> {
        UserSessionUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _session_id: session_id.to_string(),
            _current_time_millis: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Queries for user's data point changes for a particular data source.
    /// 
    /// # Arguments
    ///
    /// * `userId` - List data points for the person identified. Use <code>me</code> to indicate
    ///              the authenticated user. Only <code>me</code> is supported at this time.
    /// * `dataSourceId` - The data stream ID of the data source that created the dataset.
    pub fn data_sources_data_point_changes_list(&self, user_id: &str, data_source_id: &str) -> UserDataSourceDataPointChangeListCall<'a, C, A> {
        UserDataSourceDataPointChangeListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_source_id: data_source_id.to_string(),
            _page_token: Default::default(),
            _limit: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all data sources that are visible to the developer, using the OAuth
    /// scopes provided. The list is not exhaustive; the user may have private
    /// data sources that are only visible to other developers, or calls using
    /// other scopes.
    /// 
    /// # Arguments
    ///
    /// * `userId` - List data sources for the person identified. Use <code>me</code> to
    ///              indicate the authenticated user. Only <code>me</code> is supported at this
    ///              time.
    pub fn data_sources_list(&self, user_id: &str) -> UserDataSourceListCall<'a, C, A> {
        UserDataSourceListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _data_type_name: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Aggregates data of a certain type or stream into buckets divided by a given
/// type of boundary. Multiple data sets of multiple types and from multiple
/// sources can be aggregated into exactly one bucket type per request.
///
/// A builder for the *dataset.aggregate* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// use fitness1::AggregateRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AggregateRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().dataset_aggregate(req, "userId")
///              .doit();
/// # }
/// ```
pub struct UserDatasetAggregateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _request: AggregateRequest,
    _user_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDatasetAggregateCall<'a, C, A> {}

impl<'a, C, A> UserDatasetAggregateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AggregateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataset.aggregate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataset:aggregate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityRead.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: AggregateRequest) -> UserDatasetAggregateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Aggregate data for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDatasetAggregateCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDatasetAggregateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDatasetAggregateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityRead`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDatasetAggregateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the specified data source. The request will fail if the data
/// source contains any data points.
///
/// A builder for the *dataSources.delete* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_delete("userId", "dataSourceId")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _data_source_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceDeleteCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DataSource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        for &field in ["alt", "userId", "dataSourceId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Retrieve a data source for the person identified. Use <code>me</code> to
    /// indicate the authenticated user. Only <code>me</code> is supported at this
    /// time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceDeleteCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source to delete.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceDeleteCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a dataset containing all data points whose start and end times
/// overlap with the specified range of the dataset minimum start time and
/// maximum end time. Specifically, any data point whose start time is less
/// than or equal to the dataset end time and whose end time is greater than or
/// equal to the dataset start time.
///
/// A builder for the *dataSources.datasets.get* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_datasets_get("userId", "dataSourceId", "datasetId")
///              .page_token("labore")
///              .limit(-9)
///              .doit();
/// # }
/// ```
pub struct UserDataSourceDatasetGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _data_source_id: String,
    _dataset_id: String,
    _page_token: Option<String>,
    _limit: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceDatasetGetCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceDatasetGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.datasets.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._limit {
            params.push(("limit", value.to_string()));
        }
        for &field in ["alt", "userId", "dataSourceId", "datasetId", "pageToken", "limit"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityRead.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId"), ("{datasetId}", "datasetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["datasetId", "dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Retrieve a dataset for the person identified. Use <code>me</code> to
    /// indicate the authenticated user. Only <code>me</code> is supported at this
    /// time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceDatasetGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source that created the dataset.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceDatasetGetCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// Dataset identifier that is a composite of the minimum data point start time
    /// and maximum data point end time represented as nanoseconds from the epoch.
    /// The ID is formatted like: "<var>startTime</var>-<var>endTime</var>"
    /// where <var>startTime</var> and <var>endTime</var> are 64 bit integers.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> UserDataSourceDatasetGetCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large datasets.
    /// To get the next page of a dataset, set this parameter to the value of
    /// <code>nextPageToken</code> from the previous response. Each subsequent
    /// call will yield a partial dataset with data point end timestamps that are
    /// strictly smaller than those in the previous partial response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserDataSourceDatasetGetCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// If specified, no more than this many data points will be included in the
    /// dataset. If there are more data points in the dataset, nextPageToken
    /// will be set in the dataset response.
    ///
    /// Sets the *limit* query property to the given value.
    pub fn limit(mut self, new_value: i32) -> UserDataSourceDatasetGetCall<'a, C, A> {
        self._limit = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceDatasetGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceDatasetGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityRead`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceDatasetGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new data source that is unique across all data sources belonging
/// to this user.
/// 
/// A data source is a unique source of sensor data. Data sources can expose
/// raw data coming from hardware sensors on local or companion devices. They
/// can also expose derived data, created by transforming or merging other data
/// sources. Multiple data sources can exist for the same data type. Every data
/// point in every dataset inserted into or read from the Fitness API has an
/// associated data source.
/// 
/// Each data source produces a unique stream of dataset updates, with a
/// unique data source identifier. Not all changes to data source affect the
/// data stream ID, so that data collected by updated versions of the same
/// application/device can still be considered to belong to the same data
/// source.
/// 
/// Data sources are identified using a string generated by the server, based
/// on the contents of the source being created. The <code>dataStreamId</code>
/// field should not be set when invoking this method. It
/// will be automatically generated by the server with the correct format. If
/// a <code>dataStreamId</code> is set, it must match the format that the
/// server would generate. This format is a combination of some fields from the
/// data source, and has a specific order. If it doesn't match, the request
/// will fail with an error.
/// 
/// Specifying a DataType which is not a known type (beginning with
/// "com.google.") will create a DataSource with a <em>custom data type</em>.
/// Custom data types are only readable by the application that created them.
/// Custom data types are <strong>deprecated</strong>; use standard data types
/// instead.
/// 
/// In addition to the data source fields included in the data source ID, the
/// developer project number that is authenticated when creating the data
/// source is included. This developer project number is obfuscated when read
/// by any other developer reading public data types.
///
/// A builder for the *dataSources.create* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// use fitness1::DataSource;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DataSource::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_create(req, "userId")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _request: DataSource,
    _user_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceCreateCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DataSource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        for &field in ["alt", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: DataSource) -> UserDataSourceCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Create the data source for the person identified. Use <code>me</code> to
    /// indicate the authenticated user. Only <code>me</code> is supported at this
    /// time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceCreateCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceCreateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Performs an inclusive delete of all data points whose start and end times
/// have any overlap with the time range specified by the dataset ID. For most
/// data types, the entire data point will be deleted. For data types where the
/// time span represents a consistent value (such as
/// <code>com.google.activity.segment</code>), and a data point straddles
/// either end point of the dataset, only the overlapping portion of the data
/// point will be deleted.
///
/// A builder for the *dataSources.datasets.delete* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_datasets_delete("userId", "dataSourceId", "datasetId")
///              .modified_time_millis("aliquyam")
///              .current_time_millis("ea")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceDatasetDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _data_source_id: String,
    _dataset_id: String,
    _modified_time_millis: Option<String>,
    _current_time_millis: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceDatasetDeleteCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceDatasetDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.datasets.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        if let Some(value) = self._modified_time_millis {
            params.push(("modifiedTimeMillis", value.to_string()));
        }
        if let Some(value) = self._current_time_millis {
            params.push(("currentTimeMillis", value.to_string()));
        }
        for &field in ["userId", "dataSourceId", "datasetId", "modifiedTimeMillis", "currentTimeMillis"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId"), ("{datasetId}", "datasetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["datasetId", "dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Delete a dataset for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source that created the dataset.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// Dataset identifier that is a composite of the minimum data point start time
    /// and maximum data point end time represented as nanoseconds from the epoch.
    /// The ID is formatted like: "<var>startTime</var>-<var>endTime</var>"
    /// where <var>startTime</var> and <var>endTime</var> are 64 bit integers.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// When the operation was performed on the client.
    ///
    /// Sets the *modified time millis* query property to the given value.
    pub fn modified_time_millis(mut self, new_value: &str) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        self._modified_time_millis = Some(new_value.to_string());
        self
    }
    /// The client's current time in milliseconds since epoch.
    ///
    /// Sets the *current time millis* query property to the given value.
    pub fn current_time_millis(mut self, new_value: &str) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        self._current_time_millis = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceDatasetDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceDatasetDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceDatasetDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Adds data points to a dataset. The dataset need not be previously created.
/// All points within the given dataset will be returned with subsquent calls
/// to retrieve this dataset. Data points can belong to more than one dataset.
/// This method does not use patch semantics.
///
/// A builder for the *dataSources.datasets.patch* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// use fitness1::Dataset;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_datasets_patch(req, "userId", "dataSourceId", "datasetId")
///              .current_time_millis("et")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceDatasetPatchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _request: Dataset,
    _user_id: String,
    _data_source_id: String,
    _dataset_id: String,
    _current_time_millis: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceDatasetPatchCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceDatasetPatchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.datasets.patch",
                               http_method: hyper::method::Method::Patch });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        params.push(("datasetId", self._dataset_id.to_string()));
        if let Some(value) = self._current_time_millis {
            params.push(("currentTimeMillis", value.to_string()));
        }
        for &field in ["alt", "userId", "dataSourceId", "datasetId", "currentTimeMillis"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}/datasets/{datasetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId"), ("{datasetId}", "datasetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["datasetId", "dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Patch, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Dataset) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Patch a dataset for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source that created the dataset.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// Dataset identifier that is a composite of the minimum data point start time
    /// and maximum data point end time represented as nanoseconds from the epoch.
    /// The ID is formatted like: "<var>startTime</var>-<var>endTime</var>"
    /// where <var>startTime</var> and <var>endTime</var> are 64 bit integers.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// The client's current time in milliseconds since epoch. Note that the
    /// <code>minStartTimeNs</code> and <code>maxEndTimeNs</code> properties in
    /// the request body are in nanoseconds instead of milliseconds.
    ///
    /// Sets the *current time millis* query property to the given value.
    pub fn current_time_millis(mut self, new_value: &str) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        self._current_time_millis = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceDatasetPatchCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceDatasetPatchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceDatasetPatchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a session specified by the given session ID.
///
/// A builder for the *sessions.delete* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().sessions_delete("userId", "sessionId")
///              .current_time_millis("ipsum")
///              .doit();
/// # }
/// ```
pub struct UserSessionDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _session_id: String,
    _current_time_millis: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserSessionDeleteCall<'a, C, A> {}

impl<'a, C, A> UserSessionDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.sessions.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("sessionId", self._session_id.to_string()));
        if let Some(value) = self._current_time_millis {
            params.push(("currentTimeMillis", value.to_string()));
        }
        for &field in ["userId", "sessionId", "currentTimeMillis"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/sessions/{sessionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{sessionId}", "sessionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["sessionId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Delete a session for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserSessionDeleteCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The ID of the session to be deleted.
    ///
    /// Sets the *session id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn session_id(mut self, new_value: &str) -> UserSessionDeleteCall<'a, C, A> {
        self._session_id = new_value.to_string();
        self
    }
    /// The client's current time in milliseconds since epoch.
    ///
    /// Sets the *current time millis* query property to the given value.
    pub fn current_time_millis(mut self, new_value: &str) -> UserSessionDeleteCall<'a, C, A> {
        self._current_time_millis = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserSessionDeleteCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserSessionDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserSessionDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the specified data source.
///
/// A builder for the *dataSources.get* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_get("userId", "dataSourceId")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _data_source_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceGetCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DataSource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        for &field in ["alt", "userId", "dataSourceId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityRead.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Retrieve a data source for the person identified. Use <code>me</code> to
    /// indicate the authenticated user. Only <code>me</code> is supported at this
    /// time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceGetCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source to retrieve.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceGetCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityRead`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the specified data source. The <code>dataStreamId</code>,
/// <code>dataType</code>, <code>type</code>, <code>dataStreamName</code>, and
/// <code>device</code> properties with the exception of <code>version</code>,
/// cannot be modified.
/// 
/// Data sources are identified by their <code>dataStreamId</code>.
///
/// A builder for the *dataSources.update* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// use fitness1::DataSource;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DataSource::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_update(req, "userId", "dataSourceId")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _request: DataSource,
    _user_id: String,
    _data_source_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceUpdateCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DataSource)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        for &field in ["alt", "userId", "dataSourceId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: DataSource) -> UserDataSourceUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Update the data source for the person identified. Use <code>me</code> to
    /// indicate the authenticated user. Only <code>me</code> is supported at this
    /// time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceUpdateCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source to update.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceUpdateCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceUpdateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists sessions previously created.
///
/// A builder for the *sessions.list* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().sessions_list("userId")
///              .start_time("Lorem")
///              .page_token("eos")
///              .include_deleted(false)
///              .end_time("sadipscing")
///              .add_activity_type(-48)
///              .doit();
/// # }
/// ```
pub struct UserSessionListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _start_time: Option<String>,
    _page_token: Option<String>,
    _include_deleted: Option<bool>,
    _end_time: Option<String>,
    _activity_type: Vec<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserSessionListCall<'a, C, A> {}

impl<'a, C, A> UserSessionListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListSessionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.sessions.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        if let Some(value) = self._start_time {
            params.push(("startTime", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._include_deleted {
            params.push(("includeDeleted", value.to_string()));
        }
        if let Some(value) = self._end_time {
            params.push(("endTime", value.to_string()));
        }
        if self._activity_type.len() > 0 {
            for f in self._activity_type.iter() {
                params.push(("activityType", f.to_string()));
            }
        }
        for &field in ["alt", "userId", "startTime", "pageToken", "includeDeleted", "endTime", "activityType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/sessions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityRead.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// List sessions for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserSessionListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// An <a href="https://www.ietf.org/rfc/rfc3339.txt">RFC3339</a> timestamp.
    /// Only sessions ending between the start and end times will be included in
    /// the response. If this time is omitted but <var>endTime</var> is specified,
    /// all sessions from the start of time up to <var>endTime</var> will be
    /// returned.
    ///
    /// Sets the *start time* query property to the given value.
    pub fn start_time(mut self, new_value: &str) -> UserSessionListCall<'a, C, A> {
        self._start_time = Some(new_value.to_string());
        self
    }
    /// The continuation token, which is used for incremental syncing.
    /// To get the next batch of changes, set this parameter to the value of
    /// <code>nextPageToken</code> from the previous response. The page token is
    /// ignored if either start or end time is specified. If none of start time,
    /// end time, and the page token is specified, sessions modified in the last
    /// 30 days are returned.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserSessionListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// If true, and if both <var>startTime</var> and <var>endTime</var> are
    /// omitted, session deletions will be returned.
    ///
    /// Sets the *include deleted* query property to the given value.
    pub fn include_deleted(mut self, new_value: bool) -> UserSessionListCall<'a, C, A> {
        self._include_deleted = Some(new_value);
        self
    }
    /// An <a href="https://www.ietf.org/rfc/rfc3339.txt">RFC3339</a> timestamp.
    /// Only sessions ending between the start and end times will be included in
    /// the response. If this time is omitted but <var>startTime</var> is
    /// specified, all sessions from <var>startTime</var> to the end of time will
    /// be returned.
    ///
    /// Sets the *end time* query property to the given value.
    pub fn end_time(mut self, new_value: &str) -> UserSessionListCall<'a, C, A> {
        self._end_time = Some(new_value.to_string());
        self
    }
    /// If non-empty, only sessions with these activity types should be returned.
    ///
    /// Append the given value to the *activity type* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_activity_type(mut self, new_value: i32) -> UserSessionListCall<'a, C, A> {
        self._activity_type.push(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserSessionListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserSessionListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityRead`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserSessionListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates or insert a given session.
///
/// A builder for the *sessions.update* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// use fitness1::Session;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Session::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().sessions_update(req, "userId", "sessionId")
///              .current_time_millis("amet")
///              .doit();
/// # }
/// ```
pub struct UserSessionUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _request: Session,
    _user_id: String,
    _session_id: String,
    _current_time_millis: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserSessionUpdateCall<'a, C, A> {}

impl<'a, C, A> UserSessionUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Session)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.sessions.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("sessionId", self._session_id.to_string()));
        if let Some(value) = self._current_time_millis {
            params.push(("currentTimeMillis", value.to_string()));
        }
        for &field in ["alt", "userId", "sessionId", "currentTimeMillis"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/sessions/{sessionId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityWrite.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{sessionId}", "sessionId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["sessionId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Session) -> UserSessionUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Create sessions for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserSessionUpdateCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The ID of the session to be created.
    ///
    /// Sets the *session id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn session_id(mut self, new_value: &str) -> UserSessionUpdateCall<'a, C, A> {
        self._session_id = new_value.to_string();
        self
    }
    /// The client's current time in milliseconds since epoch.
    ///
    /// Sets the *current time millis* query property to the given value.
    pub fn current_time_millis(mut self, new_value: &str) -> UserSessionUpdateCall<'a, C, A> {
        self._current_time_millis = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserSessionUpdateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserSessionUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityWrite`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserSessionUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Queries for user's data point changes for a particular data source.
///
/// A builder for the *dataSources.dataPointChanges.list* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_data_point_changes_list("userId", "dataSourceId")
///              .page_token("eirmod")
///              .limit(-33)
///              .doit();
/// # }
/// ```
pub struct UserDataSourceDataPointChangeListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _data_source_id: String,
    _page_token: Option<String>,
    _limit: Option<i32>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceDataPointChangeListCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceDataPointChangeListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListDataPointChangesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.dataPointChanges.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        params.push(("dataSourceId", self._data_source_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._limit {
            params.push(("limit", value.to_string()));
        }
        for &field in ["alt", "userId", "dataSourceId", "pageToken", "limit"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources/{dataSourceId}/dataPointChanges";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityRead.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId"), ("{dataSourceId}", "dataSourceId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["dataSourceId", "userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// List data points for the person identified. Use <code>me</code> to indicate
    /// the authenticated user. Only <code>me</code> is supported at this time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceDataPointChangeListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The data stream ID of the data source that created the dataset.
    ///
    /// Sets the *data source id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn data_source_id(mut self, new_value: &str) -> UserDataSourceDataPointChangeListCall<'a, C, A> {
        self._data_source_id = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets.
    /// To get the next page of results, set this parameter to the value of
    /// <code>nextPageToken</code> from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserDataSourceDataPointChangeListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// If specified, no more than this many data point changes will be included
    /// in the response.
    ///
    /// Sets the *limit* query property to the given value.
    pub fn limit(mut self, new_value: i32) -> UserDataSourceDataPointChangeListCall<'a, C, A> {
        self._limit = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceDataPointChangeListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceDataPointChangeListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityRead`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceDataPointChangeListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists all data sources that are visible to the developer, using the OAuth
/// scopes provided. The list is not exhaustive; the user may have private
/// data sources that are only visible to other developers, or calls using
/// other scopes.
///
/// A builder for the *dataSources.list* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_fitness1 as fitness1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use fitness1::Fitness;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Fitness::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().data_sources_list("userId")
///              .add_data_type_name("aliquyam")
///              .doit();
/// # }
/// ```
pub struct UserDataSourceListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Fitness<C, A>,
    _user_id: String,
    _data_type_name: Vec<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for UserDataSourceListCall<'a, C, A> {}

impl<'a, C, A> UserDataSourceListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListDataSourcesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "fitness.users.dataSources.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("userId", self._user_id.to_string()));
        if self._data_type_name.len() > 0 {
            for f in self._data_type_name.iter() {
                params.push(("dataTypeName", f.to_string()));
            }
        }
        for &field in ["alt", "userId", "dataTypeName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "fitness/v1/users/{userId}/dataSources";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ActivityRead.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["userId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// List data sources for the person identified. Use <code>me</code> to
    /// indicate the authenticated user. Only <code>me</code> is supported at this
    /// time.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDataSourceListCall<'a, C, A> {
        self._user_id = new_value.to_string();
        self
    }
    /// The names of data types to include in the list. If not specified, all
    /// data sources will be returned.
    ///
    /// Append the given value to the *data type name* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_data_type_name(mut self, new_value: &str) -> UserDataSourceListCall<'a, C, A> {
        self._data_type_name.push(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> UserDataSourceListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserDataSourceListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ActivityRead`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> UserDataSourceListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


