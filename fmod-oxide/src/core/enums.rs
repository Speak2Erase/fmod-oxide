// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum SpeakerMode {
    Default = FMOD_SPEAKERMODE_DEFAULT,
    Raw = FMOD_SPEAKERMODE_RAW,
    Mono = FMOD_SPEAKERMODE_MONO,
    Stereo = FMOD_SPEAKERMODE_STEREO,
    Quad = FMOD_SPEAKERMODE_QUAD,
    Surround = FMOD_SPEAKERMODE_SURROUND,
    FivePointOne = FMOD_SPEAKERMODE_5POINT1,
    SevenPointOne = FMOD_SPEAKERMODE_7POINT1,
    SevenPointOneFour = FMOD_SPEAKERMODE_7POINT1POINT4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum OutputType {
    AutoDetect = FMOD_OUTPUTTYPE_AUTODETECT,
    Unknown = FMOD_OUTPUTTYPE_UNKNOWN,
    NoSound = FMOD_OUTPUTTYPE_NOSOUND,
    WavWriter = FMOD_OUTPUTTYPE_WAVWRITER,
    NoSoundNRT = FMOD_OUTPUTTYPE_NOSOUND_NRT,
    WavWriterNRT = FMOD_OUTPUTTYPE_WAVWRITER_NRT,
    WASAPI = FMOD_OUTPUTTYPE_WASAPI,
    ASIO = FMOD_OUTPUTTYPE_ASIO,
    PulseAudio = FMOD_OUTPUTTYPE_PULSEAUDIO,
    Alsa = FMOD_OUTPUTTYPE_ALSA,
    CoreAudio = FMOD_OUTPUTTYPE_COREAUDIO,
    AudioTrack = FMOD_OUTPUTTYPE_AUDIOTRACK,
    OpenSL = FMOD_OUTPUTTYPE_OPENSL,
    AudioOut = FMOD_OUTPUTTYPE_AUDIOOUT,
    WebAudio = FMOD_OUTPUTTYPE_WEBAUDIO,
    NNAudio = FMOD_OUTPUTTYPE_NNAUDIO,
    WinSonic = FMOD_OUTPUTTYPE_WINSONIC,
    AAudio = FMOD_OUTPUTTYPE_AAUDIO,
    AudioWorklet = FMOD_OUTPUTTYPE_AUDIOWORKLET,
    Phase = FMOD_OUTPUTTYPE_PHASE,
    OHAudio = FMOD_OUTPUTTYPE_OHAUDIO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum ThreadType {
    Mixer = FMOD_THREAD_TYPE_MIXER,
    Feeder = FMOD_THREAD_TYPE_FEEDER,
    Stream = FMOD_THREAD_TYPE_STREAM,
    File = FMOD_THREAD_TYPE_FILE,
    NonBlocking = FMOD_THREAD_TYPE_NONBLOCKING,
    Record = FMOD_THREAD_TYPE_RECORD,
    Geometry = FMOD_THREAD_TYPE_GEOMETRY,
    Profiler = FMOD_THREAD_TYPE_PROFILER,
    StudioUpdate = FMOD_THREAD_TYPE_STUDIO_UPDATE,
    StudioLoadBank = FMOD_THREAD_TYPE_STUDIO_LOAD_BANK,
    StudioLoadSample = FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE,
    Convolution1 = FMOD_THREAD_TYPE_CONVOLUTION1,
    Convolution2 = FMOD_THREAD_TYPE_CONVOLUTION2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum TimeUnit {
    MS = FMOD_TIMEUNIT_MS,
    PCM = FMOD_TIMEUNIT_PCM,
    PCMBytes = FMOD_TIMEUNIT_PCMBYTES,
    RawBytes = FMOD_TIMEUNIT_RAWBYTES,
    PCMFraction = FMOD_TIMEUNIT_PCMFRACTION,
    ModOrder = FMOD_TIMEUNIT_MODORDER,
    ModRow = FMOD_TIMEUNIT_MODROW,
    ModPattern = FMOD_TIMEUNIT_MODPATTERN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(i32)]
pub enum Speaker {
    None = FMOD_SPEAKER_NONE,
    FrontLeft = FMOD_SPEAKER_FRONT_LEFT,
    FrontRight = FMOD_SPEAKER_FRONT_RIGHT,
    FrontCenter = FMOD_SPEAKER_FRONT_CENTER,
    LowFrequency = FMOD_SPEAKER_LOW_FREQUENCY,
    SurroundLeft = FMOD_SPEAKER_SURROUND_LEFT,
    SurroundRight = FMOD_SPEAKER_SURROUND_RIGHT,
    BackLeft = FMOD_SPEAKER_BACK_LEFT,
    BackRight = FMOD_SPEAKER_BACK_RIGHT,
    TopFrontLeft = FMOD_SPEAKER_TOP_FRONT_LEFT,
    TopFrontRight = FMOD_SPEAKER_TOP_FRONT_RIGHT,
    TopBackLeft = FMOD_SPEAKER_TOP_BACK_LEFT,
    TopBackRight = FMOD_SPEAKER_TOP_BACK_RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum PluginType {
    Output = FMOD_PLUGINTYPE_OUTPUT,
    Codec = FMOD_PLUGINTYPE_CODEC,
    DSP = FMOD_PLUGINTYPE_DSP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum DspType {
    Unknown = FMOD_DSP_TYPE_UNKNOWN,
    Mixer = FMOD_DSP_TYPE_MIXER,
    Oscillator = FMOD_DSP_TYPE_OSCILLATOR,
    Lowpass = FMOD_DSP_TYPE_LOWPASS,
    ItLowpass = FMOD_DSP_TYPE_ITLOWPASS,
    Highpass = FMOD_DSP_TYPE_HIGHPASS,
    Echo = FMOD_DSP_TYPE_ECHO,
    Fader = FMOD_DSP_TYPE_FADER,
    Flange = FMOD_DSP_TYPE_FLANGE,
    Distortion = FMOD_DSP_TYPE_DISTORTION,
    Normalize = FMOD_DSP_TYPE_NORMALIZE,
    Limiter = FMOD_DSP_TYPE_LIMITER,
    ParamEq = FMOD_DSP_TYPE_PARAMEQ,
    PitchShift = FMOD_DSP_TYPE_PITCHSHIFT,
    Chorus = FMOD_DSP_TYPE_CHORUS,
    VstPlugin = FMOD_DSP_TYPE_VSTPLUGIN,
    WinampPlugin = FMOD_DSP_TYPE_WINAMPPLUGIN,
    ItEcho = FMOD_DSP_TYPE_ITECHO,
    Compressor = FMOD_DSP_TYPE_COMPRESSOR,
    SfxReverb = FMOD_DSP_TYPE_SFXREVERB,
    LowpassSimple = FMOD_DSP_TYPE_LOWPASS_SIMPLE,
    Delay = FMOD_DSP_TYPE_DELAY,
    Tremolo = FMOD_DSP_TYPE_TREMOLO,
    LadspaPlugin = FMOD_DSP_TYPE_LADSPAPLUGIN,
    Send = FMOD_DSP_TYPE_SEND,
    Return = FMOD_DSP_TYPE_RETURN,
    HighpassSimple = FMOD_DSP_TYPE_HIGHPASS_SIMPLE,
    Pan = FMOD_DSP_TYPE_PAN,
    ThreeEq = FMOD_DSP_TYPE_THREE_EQ,
    Fft = FMOD_DSP_TYPE_FFT,
    LoudnessMeter = FMOD_DSP_TYPE_LOUDNESS_METER,
    EnvelopeFollower = FMOD_DSP_TYPE_ENVELOPEFOLLOWER,
    ConvolutionReverb = FMOD_DSP_TYPE_CONVOLUTIONREVERB,
    ChannelMix = FMOD_DSP_TYPE_CHANNELMIX,
    Transceiver = FMOD_DSP_TYPE_TRANSCEIVER,
    ObjectPan = FMOD_DSP_TYPE_OBJECTPAN,
    MultibandEq = FMOD_DSP_TYPE_MULTIBAND_EQ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum PortType {
    Music = FMOD_PORT_TYPE_MUSIC,
    CopyrightMusic = FMOD_PORT_TYPE_COPYRIGHT_MUSIC,
    Voice = FMOD_PORT_TYPE_VOICE,
    Controller = FMOD_PORT_TYPE_CONTROLLER,
    Personal = FMOD_PORT_TYPE_PERSONAL,
    Vibration = FMOD_PORT_TYPE_VIBRATION,
    AUX = FMOD_PORT_TYPE_AUX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum SoundGroupBehavior {
    Fail = FMOD_SOUNDGROUP_BEHAVIOR_FAIL,
    Mute = FMOD_SOUNDGROUP_BEHAVIOR_MUTE,
    StealLowest = FMOD_SOUNDGROUP_BEHAVIOR_STEALLOWEST,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum DspConnectionType {
    Standard = FMOD_DSPCONNECTION_TYPE_STANDARD,
    Sidechain = FMOD_DSPCONNECTION_TYPE_SIDECHAIN,
    Send = FMOD_DSPCONNECTION_TYPE_SEND,
    SendSidechain = FMOD_DSPCONNECTION_TYPE_SEND_SIDECHAIN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[repr(i32)]
pub enum DspParameterDataType {
    User = FMOD_DSP_PARAMETER_DATA_TYPE_USER,
    OverAlign = FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN,
    Attributes3D = FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    Sidechain = FMOD_DSP_PARAMETER_DATA_TYPE_SIDECHAIN,
    FFT = FMOD_DSP_PARAMETER_DATA_TYPE_FFT,
    Attributes3DMulti = FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES_MULTI,
    AttenuationRange = FMOD_DSP_PARAMETER_DATA_TYPE_ATTENUATION_RANGE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum SoundType {
    Unknown = FMOD_SOUND_TYPE_UNKNOWN,
    AIFF = FMOD_SOUND_TYPE_AIFF,
    ASF = FMOD_SOUND_TYPE_ASF,
    DLS = FMOD_SOUND_TYPE_DLS,
    FLAC = FMOD_SOUND_TYPE_FLAC,
    FSB = FMOD_SOUND_TYPE_FSB,
    IT = FMOD_SOUND_TYPE_IT,
    MIDI = FMOD_SOUND_TYPE_MIDI,
    MOD = FMOD_SOUND_TYPE_MOD,
    MPEG = FMOD_SOUND_TYPE_MPEG,
    OGGVORBIS = FMOD_SOUND_TYPE_OGGVORBIS,
    Playlist = FMOD_SOUND_TYPE_PLAYLIST,
    RAW = FMOD_SOUND_TYPE_RAW,
    S3M = FMOD_SOUND_TYPE_S3M,
    User = FMOD_SOUND_TYPE_USER,
    WAV = FMOD_SOUND_TYPE_WAV,
    XM = FMOD_SOUND_TYPE_XM,
    XMA = FMOD_SOUND_TYPE_XMA,
    AudioQueue = FMOD_SOUND_TYPE_AUDIOQUEUE,
    AT9 = FMOD_SOUND_TYPE_AT9,
    Vorbis = FMOD_SOUND_TYPE_VORBIS,
    MediaFoundation = FMOD_SOUND_TYPE_MEDIA_FOUNDATION,
    MediaCodec = FMOD_SOUND_TYPE_MEDIACODEC,
    FADPCM = FMOD_SOUND_TYPE_FADPCM,
    OPUS = FMOD_SOUND_TYPE_OPUS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum SoundFormat {
    None = FMOD_SOUND_FORMAT_NONE,
    PCM8 = FMOD_SOUND_FORMAT_PCM8,
    PCM16 = FMOD_SOUND_FORMAT_PCM16,
    PCM24 = FMOD_SOUND_FORMAT_PCM24,
    PCM32 = FMOD_SOUND_FORMAT_PCM32,
    PCMFloat = FMOD_SOUND_FORMAT_PCMFLOAT,
    BitStream = FMOD_SOUND_FORMAT_BITSTREAM,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum TagType {
    Unknown = FMOD_TAGTYPE_UNKNOWN,
    ID3V1 = FMOD_TAGTYPE_ID3V1,
    ID3V2 = FMOD_TAGTYPE_ID3V2,
    VorbisComment = FMOD_TAGTYPE_VORBISCOMMENT,
    ShoutCast = FMOD_TAGTYPE_SHOUTCAST,
    IceCast = FMOD_TAGTYPE_ICECAST,
    ASF = FMOD_TAGTYPE_ASF,
    MIDI = FMOD_TAGTYPE_MIDI,
    Playlist = FMOD_TAGTYPE_PLAYLIST,
    Fmod = FMOD_TAGTYPE_FMOD,
    User = FMOD_TAGTYPE_USER,
}
