//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use juniper::parser::{ParseError, ScalarToken, Token};
use juniper::{ParseScalarResult, Value};
use novatel_oem6_api::{Component, ReceiverStatusFlags};

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    /// No mutations have been executed
    None,
    /// No-Op
    Noop,
    /// System power state was changed
    ControlPower,
    /// System configuration was updated
    ConfigureHardware,
    /// A hardware test was performed
    TestHardware,
    /// A raw command was passed through to the system
    IssueRawCommand,
}

/// Input structure for 'configureHardware' mutation
#[derive(GraphQLInputObject)]
pub struct ConfigStruct {
    /// Configuration option to send to the system
    pub option: ConfigOption,
    /// (For "Log*" config options) Indicates whether the logging should persist
    /// through an "UnlogAll" request
    #[graphql(default = "false")]
    pub hold: bool,
    /// (For "Log*" config options) The interval, in seconds, at which log
    /// messages should be generated by the system
    #[graphql(default = "0.0")]
    pub interval: f64,
    /// (For "Log*" config options) The offset, in seconds, at which log
    /// messages should be generated by the system
    #[graphql(default = "0.0")]
    pub offset: f64,
}

/// Input field for 'configureHardware' mutation
///
/// Indicates which configuration operation should be performed
#[derive(GraphQLEnum, Debug)]
pub enum ConfigOption {
    /// Configure system to output error data when errors or events occur
    LogErrorData,
    /// Configure system to output position data at a requested interval
    LogPositionData,
    /// Stop generation of all output data from device
    UnlogAll,
    /// Stop generation of error data from device
    UnlogErrorData,
    /// Stop generation of position data from device
    UnlogPositionData,
}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject, Clone)]
pub struct ConfigureHardwareResponse {
    /// An echo of the configuration options which were requested
    pub config: String,
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Input field for 'testHardware' mutation
///
/// Indicates which test should be run
#[derive(GraphQLEnum)]
pub enum TestType {
    /// Integration (non-invasive) test
    Integration,
    /// Hardware (invasive) test
    Hardware,
}

/// Enum for the 'testHardware' mutation response union
pub enum TestResults {
    /// Integration test results
    Integration(IntegrationTestResults),
    /// Hardware test results
    Hardware(HardwareTestResults),
}

/// Response union for 'testHardware' mutation
graphql_union!(TestResults: () where Scalar = <S> |&self| {
    instance_resolvers: |&_| {
        &IntegrationTestResults => match *self {
            TestResults::Integration(ref i) => Some(i),
            _ => None
        },
        &HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
    }
});

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Nominal telemetry
    pub telemetry_debug: Option<VersionInfo>,
    /// Debug telemetry
    pub telemetry_nominal: TelemetryNominal,
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Test results
    pub data: String,
}

/// Response fields for `lockStatus` query
#[derive(Clone)]
pub struct LockStatus {
    /// Validity of time data
    pub time_status: u8,
    /// Timestamp from last BestXYZ log message received
    pub time: OEMTime,
    /// Validity of position data
    pub position_status: u32,
    /// Position data type
    pub position_type: u32,
    /// Validity of velocity data
    pub velocity_status: u32,
    /// Velocity data type
    pub velocity_type: u32,
}

impl Default for LockStatus {
    fn default() -> LockStatus {
        LockStatus {
            time_status: 20, // Unknown
            time: OEMTime { week: 0, ms: 0 },
            position_status: 1, // Insufficient Observations
            position_type: 0,   // None
            velocity_status: 1, // Insufficient Observations
            velocity_type: 0,   // None
        }
    }
}

/// Time structure for `lockStatus` and `lockInfo` response fields
#[derive(Clone, Default, GraphQLObject)]
pub struct OEMTime {
    /// GPS reference week number
    pub week: i32,
    /// Milliseconds from the beginning of the GPS reference week
    pub ms: i32,
}

/// Enum for the `positionStatus` and `velocityStatus` response fields
/// of the `lockStatus` query
#[derive(GraphQLEnum, Debug)]
pub enum SolutionStatus {
    /// Solution computed
    SolComputed,
    /// Insufficient observations
    InsufficientObservations,
    /// No convergence
    NoConvergence,
    /// Singularity at parameters matrix
    Singularity,
    /// Covariance trace exceeds maximum (trace > 1000 m)
    CovarianceTraceExceeded,
    /// Test distance exceeded (maximum of 3 rejections if distance >10 km)
    TestDistanceExceeded,
    /// Not yet converged from cold start
    ColdStart,
    /// Height or velocity limits exceeded
    HeightVelocityExceeded,
    /// Variance exceeds limits
    VarianceExceeded,
    /// Residuals are too large
    ResidualsTooLarge,
    /// Large residuals make position unreliable
    IntegrityWarning,
    /// Position being computed
    Pending,
    /// Invalid fixed position
    InvalidFix,
    /// Position type is unauthorized
    Unauthorized,
    /// Unknown solution status value encountered
    KubosInvalid,
}

impl From<u32> for SolutionStatus {
    fn from(t: u32) -> SolutionStatus {
        match t {
            0 => SolutionStatus::SolComputed,
            1 => SolutionStatus::InsufficientObservations,
            2 => SolutionStatus::NoConvergence,
            3 => SolutionStatus::Singularity,
            4 => SolutionStatus::CovarianceTraceExceeded,
            5 => SolutionStatus::TestDistanceExceeded,
            6 => SolutionStatus::ColdStart,
            7 => SolutionStatus::HeightVelocityExceeded,
            8 => SolutionStatus::VarianceExceeded,
            9 => SolutionStatus::ResidualsTooLarge,
            13 => SolutionStatus::IntegrityWarning,
            18 => SolutionStatus::Pending,
            19 => SolutionStatus::InvalidFix,
            20 => SolutionStatus::Unauthorized,
            _ => SolutionStatus::KubosInvalid,
        }
    }
}

/// Enum for the `positionType` and `velocityType` response fields
/// of the `lockStatus` query
#[derive(GraphQLEnum, Debug)]
pub enum PosVelType {
    /// No solution
    None,
    /// Fixed position
    FixedPos,
    /// Fixed position
    FixedHeight,
    /// Velocity computed using instantaneous Doppler
    DopplerVelocity,
    /// Single point position
    Single,
    /// Pseudorange differential solution
    PSRDiff,
    /// Solution calculated using corrections from an WAAS
    WAAS,
    /// Propagated by a Kalman filter without new observations
    Propagated,
    /// OmniSTAR VBS position
    Omnistar,
    /// Floating L1 ambiguity solution
    L1Float,
    /// Floating ionospheric-free ambiguity solution
    IonoFreeFloat,
    /// Floating narrow-lane ambiguity solution
    NarrowFloat,
    /// Integer L1 ambiguity solution
    L1Integer,
    /// Integer narrow-lane ambiguity solution
    NarrowInteger,
    /// OmniSTAR HP position
    OmnistarHP,
    /// OmniSTAR XP or G2 position
    OmnistarXP,
    /// Converging TerraStar-C solution
    PPPConverging,
    /// Converged TerraStar-C solution
    PPP,
    /// Solution accuracy is within UAL operational limit
    Operational,
    /// Solution accuracy is outside UAL operational limit but within warning limit
    Warning,
    /// Solution accuracy is outside UAL limits
    OutOfBounds,
    /// Converging TerraStar-L solution
    PPPBasicConverging,
    /// Converged TerraStar-L solution
    PPPBasic,
    /// Unknown type value encountered
    KubosInvalid,
}

impl From<u32> for PosVelType {
    fn from(t: u32) -> PosVelType {
        match t {
            0 => PosVelType::None,
            1 => PosVelType::FixedPos,
            2 => PosVelType::FixedHeight,
            8 => PosVelType::DopplerVelocity,
            16 => PosVelType::Single,
            17 => PosVelType::PSRDiff,
            18 => PosVelType::WAAS,
            19 => PosVelType::Propagated,
            20 => PosVelType::Omnistar,
            32 => PosVelType::L1Float,
            33 => PosVelType::IonoFreeFloat,
            34 => PosVelType::NarrowFloat,
            48 => PosVelType::L1Integer,
            50 => PosVelType::NarrowInteger,
            64 => PosVelType::OmnistarHP,
            65 => PosVelType::OmnistarXP,
            68 => PosVelType::PPPConverging,
            69 => PosVelType::PPP,
            70 => PosVelType::Operational,
            71 => PosVelType::Warning,
            72 => PosVelType::OutOfBounds,
            77 => PosVelType::PPPBasicConverging,
            78 => PosVelType::PPPBasic,
            _ => PosVelType::KubosInvalid,
        }
    }
}

/// Enum for the `TimeStatus` response field of the `lockStatus` query
#[derive(GraphQLEnum, Debug)]
pub enum RefTimeStatus {
    /// Time validity is unknown
    Unknown,
    /// Time is set approximately
    Approximate,
    /// Time is approaching coarse precision
    CoarseAdjusting,
    /// Time is valid to coarse precision
    Coarse,
    /// Time is coarse set and is being steered
    CoarseSteering,
    /// Position is lost and the range bias cannot be calculated
    FreeWheeling,
    /// Time is adjusting to fine precision
    FineAdjusting,
    /// Time has fine precision
    Fine,
    /// Time is fine set and is being steered by the backup system
    FineBackupSteering,
    /// Time is fine set and is being steered
    FineSteering,
    /// Time from satellite. Only used in logs containing satellite data such as ephemeris and almanac
    SatTime,
    /// Unknown status value encountered
    KubosInvalid,
}

impl From<u8> for RefTimeStatus {
    fn from(t: u8) -> RefTimeStatus {
        match t {
            20 => RefTimeStatus::Unknown,
            60 => RefTimeStatus::Approximate,
            80 => RefTimeStatus::CoarseAdjusting,
            100 => RefTimeStatus::Coarse,
            120 => RefTimeStatus::CoarseSteering,
            130 => RefTimeStatus::FreeWheeling,
            140 => RefTimeStatus::FineAdjusting,
            160 => RefTimeStatus::Fine,
            170 => RefTimeStatus::FineBackupSteering,
            180 => RefTimeStatus::FineSteering,
            200 => RefTimeStatus::SatTime,
            _ => RefTimeStatus::KubosInvalid,
        }
    }
}

graphql_object!(LockStatus: () where Scalar = <S> | &self | {

    field time_status() -> RefTimeStatus {
        self.time_status.into()
    }

    field time() -> OEMTime {
        self.time.clone()
    }

    field position_status() -> SolutionStatus {
        self.position_status.into()
    }

    field position_type() -> PosVelType {
        self.position_type.into()
    }

    field velocity_status() -> SolutionStatus {
        self.velocity_status.into()
    }

    field velocity_type() -> PosVelType {
        self.velocity_type.into()
    }
});

/// Current system lock information. Used in the response fields of
/// the `lockInfo` query
#[derive(Clone, Default)]
pub struct LockInfo {
    /// Timestamp when the other fields were last updated
    pub time: OEMTime,
    /// Last known good position
    pub position: [f64; 3],
    /// Last known good velocity
    pub velocity: [f64; 3],
}

graphql_object!(LockInfo: ()  where Scalar = <S> | &self | {
    field time() -> OEMTime {
        self.time.clone()
    }

    field position() -> Vec<f64> {
        self.position.to_vec()
    }

    field velocity() -> Vec<f64> {
        self.velocity.to_vec()
    }
});

/// Response field for 'power' query
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    /// System is on
    On,
    /// System is off or unavailable
    Off,
}

/// Response fields for 'power' query
#[derive(GraphQLObject)]
pub struct GetPowerResponse {
    /// Current power state of the system
    pub state: PowerState,
    /// A value of 1 confirms that the system is on.
    /// A value of 0 confirms that the system is off or unavailable
    ///
    /// Note: This field is named "uptime" to help maintain parity
    /// with the other services. The OEM6 does not give a
    /// traditional uptime value.
    pub uptime: i32,
}

/// Response fields for `systemStatus` query
#[derive(Clone, GraphQLObject)]
pub struct SystemStatus {
    /// Current receiver status. If all flags are present, then the service was unable to acquire
    /// the current status value.
    pub status: ReceiverStatus,
    /// Error messages received from the system
    pub errors: Vec<String>,
}

/// Receiver status
#[derive(Clone)]
pub struct ReceiverStatus(pub ReceiverStatusFlags);

graphql_scalar!(ReceiverStatus where Scalar = <S> {
   resolve(&self) -> Value {
        Value::list(self.0.to_vec().iter().map(|flag| Value::scalar(flag.to_owned())).collect())
    }

    // The macro requires that we have this function,
    // but it won't ever actually be used
    from_input_value(_v: &InputValue) -> Option<ReceiverStatus> {
        None
    }

    // The macro requires that we have this function,
    // but it won't ever actually be used
    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        if let ScalarToken::String(value) =  value {
            Ok(S::from(value.to_owned()))
        } else {
            Err(ParseError::UnexpectedToken(Token::Scalar(value)))
        }
    }
});

/// Response fields for `telemetry` query
#[derive(GraphQLObject)]
pub struct Telemetry {
    /// Nominal telemetry
    pub nominal: TelemetryNominal,
    /// Debug telemetry
    pub debug: Option<VersionInfo>,
}

/// Response fields for 'telemetry(telem: NOMINAL)' query
#[derive(Clone, GraphQLObject)]
pub struct TelemetryNominal {
    /// System status
    pub system_status: SystemStatus,
    /// Last known lock status
    pub lock_status: Option<LockStatus>,
    /// Last known good lock information
    pub lock_info: Option<LockInfo>,
}

/// Version information about the device, returned as the
/// `telemetryDebug` response field
#[derive(Clone, GraphQLObject)]
pub struct VersionInfo {
    /// Number of system components
    pub num_components: i32,
    /// Data for each system component
    pub components: Vec<VersionComponent>,
}

/// System component data
#[derive(Clone)]
pub struct VersionComponent(pub Component);

graphql_object!(VersionComponent: () where Scalar = <S> | &self | {
    field comp_type() -> i32 {
        self.0.comp_type as i32
    }

    field model() -> String {
        self.0.model.clone()
    }

    field serial_num() -> String {
        self.0.serial_num.clone()
    }

    field hw_version() -> String {
        self.0.hw_version.clone()
    }

    field sw_version() -> String {
        self.0.sw_version.clone()
    }

    field boot_version() -> String {
        self.0.boot_version.clone()
    }

    field compile_date() -> String {
        self.0.compile_date.clone()
    }

    field compile_time() -> String {
        self.0.compile_time.clone()
    }
});
