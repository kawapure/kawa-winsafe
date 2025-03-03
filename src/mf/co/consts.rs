#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::*;

const_ordinary! { ME: u32;
	/// [`IMFMediaEvent::GetType`](crate::prelude::mf_IMFMediaEvent::GetType)
	/// return value (`u32`).
	=>
	Unknown 0
	Error 1
	ExtendedType 2
	NonFatalError 3
	GenericV1Anchor Self::NonFatalError.0
	SessionUnknown 100
	SessionTopologySet 101
	SessionTopologiesCleared 102
	SessionStarted 103
	SessionPaused 104
	SessionStopped 105
	SessionClosed 106
	SessionEnded 107
	SessionRateChanged 108
	SessionScrubSampleComplete 109
	SessionCapabilitiesChanged 110
	SessionTopologyStatus 111
	SessionNotifyPresentationTime 112
	NewPresentation 113
	LicenseAcquisitionStart 114
	LicenseAcquisitionCompleted 115
	IndividualizationStart 116
	IndividualizationCompleted 117
	EnablerProgress 118
	EnablerCompleted 119
	PolicyError 120
	PolicyReport 121
	BufferingStarted 122
	BufferingStopped 123
	ConnectStart 124
	ConnectEnd 125
	ReconnectStart 126
	ReconnectEnd 127
	RendererEvent 128
	SessionStreamSinkFormatChanged 129
	SessionV1Anchor Self::SessionStreamSinkFormatChanged.0
	SourceUnknown 200
	SourceStarted 201
	StreamStarted 202
	SourceSeeked 203
	StreamSeeked 204
	NewStream 205
	UpdatedStream 206
	SourceStopped 207
	StreamStopped 208
	SourcePaused 209
	StreamPaused 210
	EndOfPresentation 211
	EndOfStream 212
	MediaSample 213
	StreamTick 214
	StreamThinMode 215
	StreamFormatChanged 216
	SourceRateChanged 217
	EndOfPresentationSegment 218
	SourceCharacteristicsChanged 219
	SourceRateChangeRequested 220
	SourceMetadataChanged 221
	SequencerSourceTopologyUpdated 222
	SourceV1Anchor Self::SequencerSourceTopologyUpdated.0
	SinkUnknown 300
	StreamSinkStarted 301
	StreamSinkStopped 302
	StreamSinkPaused 303
	StreamSinkRateChanged 304
	StreamSinkRequestSample 305
	StreamSinkMarker 306
	StreamSinkPrerolled 307
	StreamSinkScrubSampleComplete 308
	StreamSinkFormatChanged 309
	StreamSinkDeviceChanged 310
	QualityNotify 311
	SinkInvalidated 312
	AudioSessionNameChanged 313
	AudioSessionVolumeChanged 314
	AudioSessionDeviceRemoved 315
	AudioSessionServerShutdown 316
	AudioSessionGroupingParamChanged 317
	AudioSessionIconChanged 318
	AudioSessionFormatChanged 319
	AudioSessionDisconnected 320
	AudioSessionExclusiveModeOverride 321
	SinkV1Anchor Self::AudioSessionExclusiveModeOverride.0
	CaptureAudioSessionVolumeChanged 322
	CaptureAudioSessionDeviceRemoved 323
	CaptureAudioSessionFormatChanged 324
	CaptureAudioSessionDisconnected 325
	CaptureAudioSessionExclusiveModeOverride 326
	CaptureAudioSessionServerShutdown 327
	SinkV2Anchor Self::CaptureAudioSessionServerShutdown.0
	TrustUnknown 400
	PolicyChanged 401
	ContentProtectionMessage 402
	PolicySet 403
	TrustV1Anchor Self::PolicySet.0
	WMDRMLicenseBackupCompleted 500
	WMDRMLicenseBackupProgress 501
	WMDRMLicenseRestoreCompleted 502
	WMDRMLicenseRestoreProgress 503
	WMDRMLicenseAcquisitionCompleted 506
	WMDRMIndividualizationCompleted 508
	WMDRMIndividualizationProgress 513
	WMDRMProximityCompleted 514
	WMDRMLicenseStoreCleaned 515
	WMDRMRevocationDownloadCompleted 516
	WMDRMV1Anchor Self::WMDRMRevocationDownloadCompleted.0
	TransformUnknown 600
	TransformNeedInput Self::TransformUnknown.0 + 1
	TransformHaveOutput Self::TransformNeedInput.0 + 1
	TransformDrainComplete Self::TransformHaveOutput.0 + 1
	TransformMarker Self::TransformDrainComplete.0 + 1
	TransformInputStreamStateChanged Self::TransformMarker.0 + 1
	ByteStreamCharacteristicsChanged 700
	VideoCaptureDeviceRemoved 800
	VideoCaptureDevicePreempted 801
	StreamSinkFormatInvalidated 802
	EncodingParameters 803
	ContentProtectionMetadata 900
	DeviceThermalStateChanged 950
}

const_ordinary! { MF_ATTRIBUTE: u32;
	/// [`MF_ATTRIBUTE_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/ne-mfobjects-mf_attribute_type)
	/// enumeration (`u32`).
	=>
	UINT32 VT::UI4.raw() as _
	UINT64 VT::UI8.raw() as _
	DOUBLE VT::R8.raw() as _
	GUID VT::CLSID.raw() as _
	STRING VT::LPWSTR.raw() as _
	BLOB (VT::VECTOR.raw() | VT::UI1.raw()) as _
	IUNKNOWN VT::UNKNOWN.raw() as _
}

const_ordinary! { MF_ATTRIBUTES_MATCH: u32;
	/// [`MF_ATTRIBUTES_MATCH_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/ne-mfobjects-mf_attributes_match_type)
	/// enumeration (`u32`).
	=>
	OUR_ITEMS 0
	THEIR_ITEMS 1
	ALL_ITEMS 2
	INTERSECTION 3
	SMALLER 4
}

const_ordinary! { MF_EVENT_FLAG: u32;
	/// [`IMFMediaEvent::GetType`](crate::prelude::mf_IMFMediaEvent::GetType)
	/// return type (`u32`).
	=>
	NO_WAIT 0x0000_0001
}

const_ordinary! { MF_OBJECT: u32;
	/// [`MF_OBJECT_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mf_object_type)
	/// enumeration (`u32`).
	=>
	MEDIASOURCE 0
	BYTESTREAM 1
	INVALID 2
}

const_bitflag! { MF_RESOLUTION: u32;
	/// Source resolver [flags](https://learn.microsoft.com/en-us/windows/win32/medfound/source-resolver-flags)
	/// (`u32`).
	=>
	MEDIASOURCE 0x1
	BYTESTREAM 0x2
	CONTENT_DOES_NOT_HAVE_TO_MATCH_EXTENSION_OR_MIME_TYPE 0x10
	KEEP_BYTE_STREAM_ALIVE_ON_FAIL 0x20
	DISABLE_LOCAL_PLUGINS 0x40
	PLUGIN_CONTROL_POLICY_APPROVED_ONLY 0x80
	PLUGIN_CONTROL_POLICY_WEB_ONLY 0x100
	PLUGIN_CONTROL_POLICY_WEB_ONLY_EDGEMODE 0x200
	ENABLE_STORE_PLUGINS 0x400
	REA 0x1_0000
	WRITE 0x2_0000
}

const_ordinary! { MF_TOPOLOGY: u32;
	/// [`MF_TOPOLOGY_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mf_topology_type)
	/// enumeration (`u32`).
	=>
	OUTPUT_NODE 0
	SOURCESTREAM_NODE 1
	TRANSFORM_NODE 2
	TEE_NODE 3
}

const_bitflag! { MFASYNC: u32;
	/// [`IMFAsyncCallback::GetParameters`](crate::IMFAsyncCallback::GetParameters)
	/// `flags` (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue 0
	FAST_IO_PROCESSING_CALLBACK 0x0000_0001
	SIGNAL_CALLBACK 0x0000_0002
	BLOCKING_CALLBACK 0x0000_0004
	REPLY_CALLBACK 0x0000_0008
}

const_bitflag! { MFBYTESTREAM: u32;
	/// [`IMFByteStream::GetCapabilities`](crate::prelude::mf_IMFByteStream::GetCapabilities)
	/// flags (`u32`).
	=>
	IS_READABLE 0x0000_0001
	IS_WRITABLE 0x0000_0002
	IS_SEEKABLE 0x0000_0004
	IS_REMOTE 0x0000_0008
	IS_DIRECTORY 0x0000_0080
	HAS_SLOW_SEEK 0x0000_0100
	IS_PARTIALLY_DOWNLOADED 0x0000_0200
	SHARE_WRITE 0x0000_0400
	DOES_NOT_USE_NETWORK 0x0000_0800
}

const_bitflag! { MFCLOCK_CHARACTERISTICS_FLAG: u32;
	/// [`MFCLOCK_CHARACTERISTICS_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfclock_characteristics_flags)
	/// enumeration (`u32`).
	=>
	FREQUENCY_10MHZ 0x2
	ALWAYS_RUNNING 0x4
	IS_SYSTEM_CLOCK 0x8
}

const_bitflag! { MFCLOCK_RELATIONAL_FLAG: u32;
	/// [`MFCLOCK_RELATIONAL_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfclock_relational_flags)
	/// enumeration (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue 0
	JITTER_NEVER_AHEAD 0x1
}

const_ordinary! { MFCLOCK_STATE: u32;
	/// [`MFCLOCK_STATE`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfclock_state)
	/// enumeration (`u32`).
	=>
	INVALID 0
	RUNNING 1
	STOPPED 2
	PAUSED 3
}

const_bitflag! { MFMEDIASOURCE: u32;
	/// [`MFMEDIASOURCE_CHARACTERISTICS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfmediasource_characteristics)
	/// enumeration (`u32`).
	=>
	IS_LIVE 0x1
	CAN_SEEK 0x2
	CAN_PAUSE 0x4
	HAS_SLOW_SEEK 0x8
	HAS_MULTIPLE_PRESENTATIONS 0x10
	CAN_SKIPFORWARD 0x20
	CAN_SKIPBACKWARD 0x40
	DOES_NOT_USE_NETWORK 0x80
}

const_bitflag! { MFBYTESTREAM_SEEK_FLAG: u32;
	/// [`IMFByteStream::Seek`](crate::prelude::mf_IMFByteStream::Seek) flags
	/// (`u32`).
	=>
	CANCEL_PENDING_IO 1
}

const_ordinary! { MFBYTESTREAM_SEEK_ORIGIN: u32;
	/// [`MFBYTESTREAM_SEEK_ORIGIN`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/ne-mfobjects-mfbytestream_seek_origin)
	/// enumeration (`u32`).
	=>
	Begin 0
	Current 1
}

const_bitflag! { MFSESSION_GETFULLTOPOLOGY: u32;
	/// [`MFSESSION_GETFULLTOPOLOGY_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfsession_getfulltopology_flags)
	/// enumeration (`u32`).
	=>
	CURRENT 0x1
}

const_bitflag! { MFSESSION_SETTOPOLOGY: u32;
	/// [`MFSESSION_SETTOPOLOGY_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfsession_settopology_flags)
	/// enumeration (`u32`).
	=>
	IMMEDIATE 0x1
	NORESOLUTION 0x2
	CLEAR_CURRENT 0x4
}

const_bitflag! { MFSESSIONCAP: u32;
	/// [`IMFMediaSession::GetSessionCapabilities`](crate::prelude::mf_IMFMediaSession::GetSessionCapabilities)
	/// `caps` (`u32`).
	=>
	START 0x0000_0001
	SEEK 0x0000_0002
	PAUSE 0x0000_0004
	RATE_FORWARD 0x0000_0010
	RATE_REVERSE 0x0000_0020
	DOES_NOT_USE_NETWORK 0x0000_0040
}

const_ordinary! { MFSTARTUP: u32;
	/// [`MFStartup`](crate::MFStartup) `flags` (`u32`).
	=>
	NOSOCKET 0x1
	LITE Self::NOSOCKET.0
	FULL 0
}

const_ordinary! { MFVideoARMode: u32;
	/// [`MFVideoAspectRatioMode`](https://learn.microsoft.com/en-us/windows/win32/api/evr/ne-evr-mfvideoaspectratiomode)
	/// enumeration (`u32`).
	=>
	None 0
	PreservePicture 0x1
	PreservePixel 0x2
	NonLinearStretch 0x4
}

const_bitflag! { MFVideoRenderPrefs: u32;
	/// [`MFVideoRenderPrefs`](https://learn.microsoft.com/en-us/windows/win32/api/evr/ne-evr-mfvideorenderprefs)
	/// enumeration (`u32`).
	=>
	DoNotRenderBorder 0x1
	DoNotClipToDevice 0x2
	AllowOutputThrottling 0x4
	ForceOutputThrottling 0x8
	ForceBatching 0x10
	AllowBatching 0x20
	ForceScaling 0x40
	AllowScaling 0x80
	DoNotRepaintOnStop 0x100
}
