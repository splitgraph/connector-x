use crate::{
    destinations::arrow2::{
        typesystem::{Arrow2TypeSystem, DateTimeWrapperMicro, NaiveDateTimeWrapperMicro},
        Arrow2Destination, Arrow2DestinationError,
    },
    impl_transport,
    sources::oracle::{OracleSource, OracleSourceError, OracleTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleArrow2TransportError {
    #[error(transparent)]
    Source(#[from] OracleSourceError),

    #[error(transparent)]
    Destination(#[from] Arrow2DestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

pub struct OracleArrow2Transport;

impl_transport!(
    name = OracleArrow2Transport,
    error = OracleArrow2TransportError,
    systems = OracleTypeSystem => Arrow2TypeSystem,
    route = OracleSource => Arrow2Destination,
    mappings = {
        { NumFloat[f64]                 => Float64[f64]                 | conversion auto }
        { Float[f64]                    => Float64[f64]                 | conversion none }
        { BinaryFloat[f64]              => Float64[f64]                 | conversion none }
        { BinaryDouble[f64]             => Float64[f64]                 | conversion none }
        { NumInt[i64]                   => Int64[i64]                   | conversion auto }
        { Blob[Vec<u8>]                 => LargeBinary[Vec<u8>]         | conversion auto }
        { Clob[String]                  => LargeUtf8[String]            | conversion none }
        { VarChar[String]               => LargeUtf8[String]            | conversion auto }
        { Char[String]                  => LargeUtf8[String]            | conversion none }
        { NVarChar[String]              => LargeUtf8[String]            | conversion none }
        { NChar[String]                 => LargeUtf8[String]            | conversion none }
        { Date[NaiveDateTime]           => Date64Micro[NaiveDateTimeWrapperMicro]       | conversion option }
        { Timestamp[NaiveDateTime]      => Date64Micro[NaiveDateTimeWrapperMicro]       | conversion none }
        { TimestampNano[NaiveDateTime]      => Date64[NaiveDateTime]        | conversion auto }
        { TimestampTz[DateTime<Utc>]    => DateTimeTzMicro[DateTimeWrapperMicro]        | conversion option }
        { TimestampTzNano[DateTime<Utc>]    => DateTimeTz[DateTime<Utc>]    | conversion auto }
    }
);

impl TypeConversion<NaiveDateTime, NaiveDateTimeWrapperMicro> for OracleArrow2Transport {
    fn convert(val: NaiveDateTime) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val)
    }
}

impl TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for OracleArrow2Transport {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}
