#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_favicon_url = "https://www.fmod.com/assets/fmod-logo.svg")]
#![doc(html_logo_url = "https://www.fmod.com/assets/fmod-logo.svg")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    Fmod(FMOD_RESULT), // FIXME make FMOD_RESULT be a NonZero
    NulError(std::ffi::NulError),
    EnumFromPrivitive { name: &'static str, primitive: u32 },
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Error");
        match self {
            Self::Fmod(code) => debug_struct
                .field("code", code)
                .field("message", &error_code_to_str(*code))
                .finish(),
            Self::NulError(e) => debug_struct.field("nul error", e).finish(),
            Self::EnumFromPrivitive { name, primitive } => debug_struct
                .field("name", name)
                .field("primitive", primitive)
                .finish(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fmod(code) => f.write_str(error_code_to_str(*code)),
            Self::NulError(error) => error.fmt(f),
            Self::EnumFromPrivitive { name, primitive } => f.write_fmt(format_args!(
                "No discriminant in enum `{name}` matches the value `{primitive:?}"
            )),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<FMOD_RESULT> for Error {
    fn from(value: FMOD_RESULT) -> Self {
        Self::Fmod(value)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(value: std::ffi::NulError) -> Self {
        Self::NulError(value)
    }
}

impl<T> From<num_enum::TryFromPrimitiveError<T>> for Error
where
    T: num_enum::TryFromPrimitive,
    T::Primitive: Into<u32>,
{
    fn from(value: num_enum::TryFromPrimitiveError<T>) -> Self {
        Self::EnumFromPrivitive {
            name: T::NAME,
            primitive: value.number.into(),
        }
    }
}

impl From<FMOD_RESULT> for Result<()> {
    fn from(value: FMOD_RESULT) -> Self {
        if matches!(value, FMOD_RESULT::FMOD_OK) {
            Ok(())
        } else {
            Err(Error::Fmod(value))
        }
    }
}

impl<T> From<Result<T>> for FMOD_RESULT {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(_) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.into(),
        }
    }
}

impl PartialEq<FMOD_RESULT> for Error {
    fn eq(&self, other: &FMOD_RESULT) -> bool {
        match self {
            Self::Fmod(code) => code == other,
            _ => false,
        }
    }
}

impl From<Error> for FMOD_RESULT {
    fn from(val: Error) -> Self {
        match val {
            Error::Fmod(code) => code,
            Error::NulError(_) => FMOD_RESULT::FMOD_ERR_INVALID_PARAM,
            Error::EnumFromPrivitive { .. } => FMOD_RESULT::FMOD_ERR_INVALID_PARAM,
        }
    }
}

impl FMOD_RESULT {
    pub fn to_result(self) -> Result<()> {
        self.into()
    }

    pub fn to_error(self) -> Option<Error> {
        self.to_result().err()
    }
}

impl From<FMOD_BOOL> for bool {
    fn from(val: FMOD_BOOL) -> Self {
        val.0 > 0
    }
}

impl From<bool> for FMOD_BOOL {
    fn from(value: bool) -> Self {
        Self(value as _)
    }
}

impl FMOD_BOOL {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}

#[allow(non_snake_case)]
pub const fn error_code_to_str(result: FMOD_RESULT) -> &'static str {
    match result
    {
        FMOD_RESULT::FMOD_OK=>                             "No errors.",
        FMOD_RESULT::FMOD_ERR_BADCOMMAND=>                 "Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound=>=>lock on a streaming sound).",
        FMOD_RESULT::FMOD_ERR_CHANNEL_ALLOC=>              "Error trying to allocate a channel.",
        FMOD_RESULT::FMOD_ERR_CHANNEL_STOLEN=>             "The specified channel has been reused to play another sound.",
        FMOD_RESULT::FMOD_ERR_DMA=>                        "DMA Failure.  See debug output for more information.",
        FMOD_RESULT::FMOD_ERR_DSP_CONNECTION=>             "DSP connection error.  Connection possibly caused a cyclic dependency or connected dsps with incompatible buffer counts.",
        FMOD_RESULT::FMOD_ERR_DSP_DONTPROCESS=>            "DSP  code from a DSP process query callback.  Tells mixer not to call the process callback and therefore not consume CPU.  Use this to optimize the DSP graph.",
        FMOD_RESULT::FMOD_ERR_DSP_FORMAT=>                 "DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format, or a matrix may have been set with the wrong size if the target unit has a specified channel map.",
        FMOD_RESULT::FMOD_ERR_DSP_INUSE=>                  "DSP is already in the mixer's DSP network. It must be removed before being reinserted or released.",
        FMOD_RESULT::FMOD_ERR_DSP_NOTFOUND=>               "DSP connection error.  Couldn't find the DSP unit specified.",
        FMOD_RESULT::FMOD_ERR_DSP_RESERVED=>               "DSP operation error.  Cannot perform operation on this DSP as it is reserved by the system.",
        FMOD_RESULT::FMOD_ERR_DSP_SILENCE=>                "DSP  code from a DSP process query callback.  Tells mixer silence would be produced from read, so go idle and not consume CPU.  Use this to optimize the DSP graph.",
        FMOD_RESULT::FMOD_ERR_DSP_TYPE=>                   "DSP operation cannot be performed on a DSP of this type.",
        FMOD_RESULT::FMOD_ERR_FILE_BAD=>                   "Error loading file.",
        FMOD_RESULT::FMOD_ERR_FILE_COULDNOTSEEK=>          "Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format.",
        FMOD_RESULT::FMOD_ERR_FILE_DISKEJECTED=>           "Media was ejected while reading.",
        FMOD_RESULT::FMOD_ERR_FILE_EOF=>                   "End of file unexpectedly reached while trying to read essential data (truncated?).",
        FMOD_RESULT::FMOD_ERR_FILE_ENDOFDATA=>             "End of current chunk reached while trying to read data.",
        FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND=>              "File not found.",
        FMOD_RESULT::FMOD_ERR_FORMAT=>                     "Unsupported file or audio format.",
        FMOD_RESULT::FMOD_ERR_HEADER_MISMATCH=>            "There is a version mismatch between the FMOD header and either the FMOD Studio library or the FMOD Low Level library.",
        FMOD_RESULT::FMOD_ERR_HTTP=>                       "A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.",
        FMOD_RESULT::FMOD_ERR_HTTP_ACCESS=>                "The specified resource requires authentication or is forbidden.",
        FMOD_RESULT::FMOD_ERR_HTTP_PROXY_AUTH=>            "Proxy authentication is required to access the specified resource.",
        FMOD_RESULT::FMOD_ERR_HTTP_SERVER_ERROR=>          "A HTTP server error occurred.",
        FMOD_RESULT::FMOD_ERR_HTTP_TIMEOUT=>               "The HTTP request timed out.",
        FMOD_RESULT::FMOD_ERR_INITIALIZATION=>             "FMOD was not initialized correctly to support this function.",
        FMOD_RESULT::FMOD_ERR_INITIALIZED=>                "Cannot call this command after System=>=>init.",
        FMOD_RESULT::FMOD_ERR_INTERNAL=>                   "An error occured in the FMOD system. Use the logging version of FMOD for more information.",
        FMOD_RESULT::FMOD_ERR_INVALID_FLOAT=>              "Value passed in was a NaN, Inf or denormalized float.",
        FMOD_RESULT::FMOD_ERR_INVALID_HANDLE=>             "An invalid object handle was used.",
        FMOD_RESULT::FMOD_ERR_INVALID_PARAM=>              "An invalid parameter was passed to this function.",
        FMOD_RESULT::FMOD_ERR_INVALID_POSITION=>           "An invalid seek position was passed to this function.",
        FMOD_RESULT::FMOD_ERR_INVALID_SPEAKER=>            "An invalid speaker was passed to this function based on the current speaker mode.",
        FMOD_RESULT::FMOD_ERR_INVALID_SYNCPOINT=>          "The syncpoint did not come from this sound handle.",
        FMOD_RESULT::FMOD_ERR_INVALID_THREAD=>             "Tried to call a function on a thread that is not supported.",
        FMOD_RESULT::FMOD_ERR_INVALID_VECTOR=>             "The vectors passed in are not unit length, or perpendicular.",
        FMOD_RESULT::FMOD_ERR_MAXAUDIBLE=>                 "Reached maximum audible playback count for this sound's soundgroup.",
        FMOD_RESULT::FMOD_ERR_MEMORY=>                     "Not enough memory or resources.",
        FMOD_RESULT::FMOD_ERR_MEMORY_CANTPOINT=>           "Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used.",
        FMOD_RESULT::FMOD_ERR_NEEDS3D=>                    "Tried to call a command on a 2d sound when the command was meant for 3d sound.",
        FMOD_RESULT::FMOD_ERR_NEEDSHARDWARE=>              "Tried to use a feature that requires hardware support.",
        FMOD_RESULT::FMOD_ERR_NET_CONNECT=>                "Couldn't connect to the specified host.",
        FMOD_RESULT::FMOD_ERR_NET_SOCKET_ERROR=>           "A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere.",
        FMOD_RESULT::FMOD_ERR_NET_URL=>                    "The specified URL couldn't be resolved.",
        FMOD_RESULT::FMOD_ERR_NET_WOULD_BLOCK=>            "Operation on a non-blocking socket could not complete immediately.",
        FMOD_RESULT::FMOD_ERR_NOTREADY=>                   "Operation could not be performed because specified sound/DSP connection is not ready.",
        FMOD_RESULT::FMOD_ERR_OUTPUT_ALLOCATED=>           "Error initializing output device, but more specifically, the output device is already in use and cannot be reused.",
        FMOD_RESULT::FMOD_ERR_OUTPUT_CREATEBUFFER=>        "Error creating hardware sound buffer.",
        FMOD_RESULT::FMOD_ERR_OUTPUT_DRIVERCALL=>          "A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted.",
        FMOD_RESULT::FMOD_ERR_OUTPUT_FORMAT=>              "Soundcard does not support the specified format.",
        FMOD_RESULT::FMOD_ERR_OUTPUT_INIT=>                "Error initializing output device.",
        FMOD_RESULT::FMOD_ERR_OUTPUT_NODRIVERS=>           "The output device has no drivers installed.  If pre-init, FMOD_OUTPUT_NOSOUND is selected as the output mode.  If post-init, the function just fails.",
        FMOD_RESULT::FMOD_ERR_PLUGIN=>                     "An unspecified error has been ed from a plugin.",
        FMOD_RESULT::FMOD_ERR_PLUGIN_MISSING=>             "A requested output, dsp unit type or codec was not available.",
        FMOD_RESULT::FMOD_ERR_PLUGIN_RESOURCE=>            "A resource that the plugin requires cannot be allocated or found. (ie the DLS file for MIDI playback)",
        FMOD_RESULT::FMOD_ERR_PLUGIN_VERSION=>             "A plugin was built with an unsupported SDK version.",
        FMOD_RESULT::FMOD_ERR_RECORD=>                     "An error occurred trying to initialize the recording device.",
        FMOD_RESULT::FMOD_ERR_REVERB_CHANNELGROUP=>        "Reverb properties cannot be set on this channel because a parent channelgroup owns the reverb connection.",
        FMOD_RESULT::FMOD_ERR_REVERB_INSTANCE=>            "Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesn't exist.",
        FMOD_RESULT::FMOD_ERR_SUBSOUNDS=>                  "The error occurred because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound.",
        FMOD_RESULT::FMOD_ERR_SUBSOUND_ALLOCATED=>         "This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first.",
        FMOD_RESULT::FMOD_ERR_SUBSOUND_CANTMOVE=>          "Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file.",
        FMOD_RESULT::FMOD_ERR_TAGNOTFOUND=>                "The specified tag could not be found or there are no tags.",
        FMOD_RESULT::FMOD_ERR_TOOMANYCHANNELS=>            "The sound created exceeds the allowable input channel count.  This can be increased using the 'maxinputchannels' parameter in System=>=>setSoftwareFormat.",
        FMOD_RESULT::FMOD_ERR_TRUNCATED=>                  "The retrieved string is too long to fit in the supplied buffer and has been truncated.",
        FMOD_RESULT::FMOD_ERR_UNIMPLEMENTED=>              "Something in FMOD hasn't been implemented when it should be. Contact support.",
        FMOD_RESULT::FMOD_ERR_UNINITIALIZED=>              "This command failed because System=>=>init or System=>=>setDriver was not called.",
        FMOD_RESULT::FMOD_ERR_UNSUPPORTED=>                "A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified.",
        FMOD_RESULT::FMOD_ERR_VERSION=>                    "The version number of this file format is not supported.",
        FMOD_RESULT::FMOD_ERR_EVENT_ALREADY_LOADED=>       "The specified bank has already been loaded.",
        FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_BUSY=>      "The live update connection failed due to the game already being connected.",
        FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH=>  "The live update connection failed due to the game data being out of sync with the tool.",
        FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT=>   "The live update connection timed out.",
        FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND=>             "The requested event, parameter, bus or vca could not be found.",
        FMOD_RESULT::FMOD_ERR_STUDIO_UNINITIALIZED=>       "The Studio=>=>System object is not yet initialized.",
        FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED=>          "The specified resource is not loaded, so it can't be unloaded.",
        FMOD_RESULT::FMOD_ERR_INVALID_STRING=>             "An invalid string was passed to this function.",
        FMOD_RESULT::FMOD_ERR_ALREADY_LOCKED=>             "The specified resource is already locked.",
        FMOD_RESULT::FMOD_ERR_NOT_LOCKED=>                 "The specified resource is not locked, so it can't be unlocked.",
        FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED=>        "The specified recording driver has been disconnected.",
        FMOD_RESULT::FMOD_ERR_TOOMANYSAMPLES=>             "The length provided exceeds the allowable limit.",
        _ =>                                               "Unknown error.",
    }
}
