#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: Self = Self(0i32);
    pub const DownmixTo2Channels: Self = Self(1i32);
    pub const DownmixTo6Channels: Self = Self(2i32);
    pub const DownmixTo8Channels: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioDecoderDegradation {}
impl ::core::clone::Clone for AudioDecoderDegradation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AudioDecoderDegradation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AudioDecoderDegradation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDecoderDegradation {}
impl ::core::fmt::Debug for AudioDecoderDegradation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDecoderDegradation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradation;i4)");
}
impl ::windows::core::DefaultType for AudioDecoderDegradation {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDecoderDegradationReason {}
impl ::core::clone::Clone for AudioDecoderDegradationReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AudioDecoderDegradationReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AudioDecoderDegradationReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDecoderDegradationReason {}
impl ::core::fmt::Debug for AudioDecoderDegradationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradationReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDecoderDegradationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradationReason;i4)");
}
impl ::windows::core::DefaultType for AudioDecoderDegradationReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct AudioStreamDescriptor(::windows::core::IUnknown);
impl AudioStreamDescriptor {
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLeadingEncoderPadding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LeadingEncoderPadding(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrailingEncoderPadding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrailingEncoderPadding(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Copy(&self) -> ::windows::core::Result<AudioStreamDescriptor> {
        let this = &::windows::core::Interface::cast::<IAudioStreamDescriptor3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioStreamDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(encodingproperties: Param0) -> ::windows::core::Result<AudioStreamDescriptor> {
        Self::IAudioStreamDescriptorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioStreamDescriptor>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAudioStreamDescriptorFactory<R, F: FnOnce(&IAudioStreamDescriptorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioStreamDescriptor, IAudioStreamDescriptorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioStreamDescriptor {}
impl ::core::fmt::Debug for AudioStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioStreamDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioStreamDescriptor;{1e3692e4-4027-4847-a70b-df1d9a2a7b04})");
}
unsafe impl ::windows::core::Interface for AudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3692e4_4027_4847_a70b_df1d9a2a7b04);
}
impl ::windows::core::RuntimeName for AudioStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.AudioStreamDescriptor";
}
impl ::core::convert::From<AudioStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: AudioStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: &AudioStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: AudioStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: &AudioStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioStreamDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioStreamDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for &AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioStreamDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioStreamDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor2> for AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor2> for &AudioStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor2> {
        ::core::convert::TryInto::<IMediaStreamDescriptor2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioStreamDescriptor {}
unsafe impl ::core::marker::Sync for AudioStreamDescriptor {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct AudioTrack(::windows::core::IUnknown);
impl AudioTrack {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpenFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioTrack>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Playback'*"]
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows::core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SupportInfo(&self) -> ::windows::core::Result<AudioTrackSupportInfo> {
        let this = &::windows::core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioTrackSupportInfo>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__: MediaTrackKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaTrackKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrack {}
impl ::core::fmt::Debug for AudioTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})");
}
unsafe impl ::windows::core::Interface for AudioTrack {
    type Vtable = IMediaTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
}
impl ::windows::core::RuntimeName for AudioTrack {
    const NAME: &'static str = "Windows.Media.Core.AudioTrack";
}
impl ::core::convert::From<AudioTrack> for ::windows::core::IUnknown {
    fn from(value: AudioTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrack> for ::windows::core::IUnknown {
    fn from(value: &AudioTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioTrack> for ::windows::core::IInspectable {
    fn from(value: AudioTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrack> for ::windows::core::IInspectable {
    fn from(value: &AudioTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioTrack> for IMediaTrack {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioTrack) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioTrack> for IMediaTrack {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioTrack) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaTrack> for AudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaTrack> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaTrack> for &AudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaTrack> {
        ::core::convert::TryInto::<IMediaTrack>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioTrack {}
unsafe impl ::core::marker::Sync for AudioTrack {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct AudioTrackOpenFailedEventArgs(::windows::core::IUnknown);
impl AudioTrackOpenFailedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioTrackOpenFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrackOpenFailedEventArgs {}
impl ::core::fmt::Debug for AudioTrackOpenFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrackOpenFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrackOpenFailedEventArgs;{eeddb9b9-bb7c-4112-bf76-9384676f824b})");
}
unsafe impl ::windows::core::Interface for AudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeddb9b9_bb7c_4112_bf76_9384676f824b);
}
impl ::windows::core::RuntimeName for AudioTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackOpenFailedEventArgs";
}
impl ::core::convert::From<AudioTrackOpenFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackOpenFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioTrackOpenFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackOpenFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AudioTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for AudioTrackOpenFailedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct AudioTrackSupportInfo(::windows::core::IUnknown);
impl AudioTrackSupportInfo {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DecoderStatus(&self) -> ::windows::core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaDecoderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDecoderStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Degradation(&self) -> ::windows::core::Result<AudioDecoderDegradation> {
        let this = self;
        unsafe {
            let mut result__: AudioDecoderDegradation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDecoderDegradation>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DegradationReason(&self) -> ::windows::core::Result<AudioDecoderDegradationReason> {
        let this = self;
        unsafe {
            let mut result__: AudioDecoderDegradationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDecoderDegradationReason>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn MediaSourceStatus(&self) -> ::windows::core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioTrackSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrackSupportInfo {}
impl ::core::fmt::Debug for AudioTrackSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrackSupportInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioTrackSupportInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrackSupportInfo;{178beff7-cc39-44a6-b951-4a5653f073fa})");
}
unsafe impl ::windows::core::Interface for AudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178beff7_cc39_44a6_b951_4a5653f073fa);
}
impl ::windows::core::RuntimeName for AudioTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackSupportInfo";
}
impl ::core::convert::From<AudioTrackSupportInfo> for ::windows::core::IUnknown {
    fn from(value: AudioTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackSupportInfo> for ::windows::core::IUnknown {
    fn from(value: &AudioTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AudioTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioTrackSupportInfo> for ::windows::core::IInspectable {
    fn from(value: AudioTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioTrackSupportInfo> for ::windows::core::IInspectable {
    fn from(value: &AudioTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AudioTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioTrackSupportInfo {}
unsafe impl ::core::marker::Sync for AudioTrackSupportInfo {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct ChapterCue(::windows::core::IUnknown);
impl ChapterCue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChapterCue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ChapterCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChapterCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChapterCue {}
impl ::core::fmt::Debug for ChapterCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChapterCue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChapterCue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.ChapterCue;{72a98001-d38a-4c0a-8fa6-75cddaf4664c})");
}
unsafe impl ::windows::core::Interface for ChapterCue {
    type Vtable = IChapterCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a98001_d38a_4c0a_8fa6_75cddaf4664c);
}
impl ::windows::core::RuntimeName for ChapterCue {
    const NAME: &'static str = "Windows.Media.Core.ChapterCue";
}
impl ::core::convert::From<ChapterCue> for ::windows::core::IUnknown {
    fn from(value: ChapterCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChapterCue> for ::windows::core::IUnknown {
    fn from(value: &ChapterCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChapterCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChapterCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChapterCue> for ::windows::core::IInspectable {
    fn from(value: ChapterCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChapterCue> for ::windows::core::IInspectable {
    fn from(value: &ChapterCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChapterCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChapterCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ChapterCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: ChapterCue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ChapterCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChapterCue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for ChapterCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for &ChapterCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ChapterCue {}
unsafe impl ::core::marker::Sync for ChapterCue {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecCategory {}
impl ::core::clone::Clone for CodecCategory {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CodecCategory {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CodecCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecCategory {}
impl ::core::fmt::Debug for CodecCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CodecCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecCategory;i4)");
}
impl ::windows::core::DefaultType for CodecCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct CodecInfo(::windows::core::IUnknown);
impl CodecInfo {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Kind(&self) -> ::windows::core::Result<CodecKind> {
        let this = self;
        unsafe {
            let mut result__: CodecKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CodecKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Category(&self) -> ::windows::core::Result<CodecCategory> {
        let this = self;
        unsafe {
            let mut result__: CodecCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CodecCategory>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Subtypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsTrusted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecInfo {}
impl ::core::fmt::Debug for CodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CodecInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.CodecInfo;{51e89f85-ea97-499c-86ac-4ce5e73f3a42})");
}
unsafe impl ::windows::core::Interface for CodecInfo {
    type Vtable = ICodecInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e89f85_ea97_499c_86ac_4ce5e73f3a42);
}
impl ::windows::core::RuntimeName for CodecInfo {
    const NAME: &'static str = "Windows.Media.Core.CodecInfo";
}
impl ::core::convert::From<CodecInfo> for ::windows::core::IUnknown {
    fn from(value: CodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecInfo> for ::windows::core::IUnknown {
    fn from(value: &CodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CodecInfo> for ::windows::core::IInspectable {
    fn from(value: CodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecInfo> for ::windows::core::IInspectable {
    fn from(value: &CodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CodecInfo {}
unsafe impl ::core::marker::Sync for CodecInfo {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecKind {}
impl ::core::clone::Clone for CodecKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CodecKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CodecKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecKind {}
impl ::core::fmt::Debug for CodecKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CodecKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecKind;i4)");
}
impl ::windows::core::DefaultType for CodecKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct CodecQuery(::windows::core::IUnknown);
impl CodecQuery {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CodecQuery, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, kind: CodecKind, category: CodecCategory, subtype: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, category, subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>>(result__)
        }
    }
}
impl ::core::clone::Clone for CodecQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CodecQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecQuery {}
impl ::core::fmt::Debug for CodecQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CodecQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.CodecQuery;{222a953a-af61-4e04-808a-a4634e2f3ac4})");
}
unsafe impl ::windows::core::Interface for CodecQuery {
    type Vtable = ICodecQueryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x222a953a_af61_4e04_808a_a4634e2f3ac4);
}
impl ::windows::core::RuntimeName for CodecQuery {
    const NAME: &'static str = "Windows.Media.Core.CodecQuery";
}
impl ::core::convert::From<CodecQuery> for ::windows::core::IUnknown {
    fn from(value: CodecQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecQuery> for ::windows::core::IUnknown {
    fn from(value: &CodecQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CodecQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CodecQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CodecQuery> for ::windows::core::IInspectable {
    fn from(value: CodecQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CodecQuery> for ::windows::core::IInspectable {
    fn from(value: &CodecQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CodecQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CodecQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CodecQuery {}
unsafe impl ::core::marker::Sync for CodecQuery {}
#[doc = "*Required features: 'Media_Core'*"]
pub struct CodecSubtypes {}
impl CodecSubtypes {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDV25() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDV50() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDvc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDvh1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDvhD() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDvsd() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatDvsl() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatH263() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatH264() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatH265() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatH264ES() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatHevc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatHevcES() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatM4S2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMjpg() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMP43() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMP4S() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMP4V() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMpeg2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatVP80() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatVP90() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMpg1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMss1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatMss2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatWmv1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatWmv2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatWmv3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormatWvc1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn VideoFormat420O() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatAac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatAdts() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatAlac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatAmrNB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatAmrWB() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatAmrWP() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatDolbyAC3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatDolbyAC3Spdif() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatDolbyDDPlus() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatDrm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatDts() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatFlac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatFloat() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatMP3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatMPeg() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatMsp1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatOpus() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatPcm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatWmaSpdif() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatWMAudioLossless() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatWMAudioV8() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AudioFormatWMAudioV9() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICodecSubtypesStatics<R, F: FnOnce(&ICodecSubtypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CodecSubtypes, ICodecSubtypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CodecSubtypes {
    const NAME: &'static str = "Windows.Media.Core.CodecSubtypes";
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct DataCue(::windows::core::IUnknown);
impl DataCue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataCue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = &::windows::core::Interface::cast::<IDataCue2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::PropertySet>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DataCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataCue {}
impl ::core::fmt::Debug for DataCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataCue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.DataCue;{7c7f676d-1fbc-4e2d-9a87-ee38bd1dc637})");
}
unsafe impl ::windows::core::Interface for DataCue {
    type Vtable = IDataCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c7f676d_1fbc_4e2d_9a87_ee38bd1dc637);
}
impl ::windows::core::RuntimeName for DataCue {
    const NAME: &'static str = "Windows.Media.Core.DataCue";
}
impl ::core::convert::From<DataCue> for ::windows::core::IUnknown {
    fn from(value: DataCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataCue> for ::windows::core::IUnknown {
    fn from(value: &DataCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DataCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataCue> for ::windows::core::IInspectable {
    fn from(value: DataCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataCue> for ::windows::core::IInspectable {
    fn from(value: &DataCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DataCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DataCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: DataCue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DataCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataCue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for DataCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for &DataCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DataCue {}
unsafe impl ::core::marker::Sync for DataCue {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct FaceDetectedEventArgs(::windows::core::IUnknown);
impl FaceDetectedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ResultFrame(&self) -> ::windows::core::Result<FaceDetectionEffectFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FaceDetectionEffectFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for FaceDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectedEventArgs {}
impl ::core::fmt::Debug for FaceDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FaceDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectedEventArgs;{19918426-c65b-46ba-85f8-13880576c90a})");
}
unsafe impl ::windows::core::Interface for FaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19918426_c65b_46ba_85f8_13880576c90a);
}
impl ::windows::core::RuntimeName for FaceDetectedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectedEventArgs";
}
impl ::core::convert::From<FaceDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FaceDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FaceDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FaceDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FaceDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FaceDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FaceDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FaceDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FaceDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FaceDetectedEventArgs {}
unsafe impl ::core::marker::Sync for FaceDetectedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct FaceDetectionEffect(::windows::core::IUnknown);
impl FaceDetectionEffect {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredDetectionInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredDetectionInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FaceDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFaceDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FaceDetectionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetectionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionEffect {}
impl ::core::fmt::Debug for FaceDetectionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FaceDetectionEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffect;{ae15ebd2-0542-42a9-bc90-f283a29f46c1})");
}
unsafe impl ::windows::core::Interface for FaceDetectionEffect {
    type Vtable = IFaceDetectionEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae15ebd2_0542_42a9_bc90_f283a29f46c1);
}
impl ::windows::core::RuntimeName for FaceDetectionEffect {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffect";
}
impl ::core::convert::From<FaceDetectionEffect> for ::windows::core::IUnknown {
    fn from(value: FaceDetectionEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffect> for ::windows::core::IUnknown {
    fn from(value: &FaceDetectionEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FaceDetectionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FaceDetectionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetectionEffect> for ::windows::core::IInspectable {
    fn from(value: FaceDetectionEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffect> for ::windows::core::IInspectable {
    fn from(value: &FaceDetectionEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FaceDetectionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FaceDetectionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FaceDetectionEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: FaceDetectionEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FaceDetectionEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: &FaceDetectionEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for FaceDetectionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for &FaceDetectionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FaceDetectionEffect {}
unsafe impl ::core::marker::Sync for FaceDetectionEffect {}
#[doc = "*Required features: 'Media_Core', 'Media_Effects'*"]
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
pub struct FaceDetectionEffectDefinition(::windows::core::IUnknown);
#[cfg(feature = "Media_Effects")]
impl FaceDetectionEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FaceDetectionEffectDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetDetectionMode(&self, value: FaceDetectionMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DetectionMode(&self) -> ::windows::core::Result<FaceDetectionMode> {
        let this = &::windows::core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__: FaceDetectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FaceDetectionMode>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetSynchronousDetectionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SynchronousDetectionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Effects'*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections', 'Media_Effects'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::clone::Clone for FaceDetectionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::PartialEq for FaceDetectionEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::Eq for FaceDetectionEffectDefinition {}
#[cfg(feature = "Media_Effects")]
impl ::core::fmt::Debug for FaceDetectionEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows::core::RuntimeType for FaceDetectionEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows::core::Interface for FaceDetectionEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
#[cfg(feature = "Media_Effects")]
impl ::windows::core::RuntimeName for FaceDetectionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<FaceDetectionEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: FaceDetectionEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<&FaceDetectionEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &FaceDetectionEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<FaceDetectionEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: FaceDetectionEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<&FaceDetectionEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &FaceDetectionEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<FaceDetectionEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: FaceDetectionEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&FaceDetectionEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &FaceDetectionEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for &FaceDetectionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IVideoEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for FaceDetectionEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for FaceDetectionEffectDefinition {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct FaceDetectionEffectFrame(::windows::core::IUnknown);
impl FaceDetectionEffectFrame {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections', 'Media_FaceAnalysis'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub fn DetectedFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsDiscontinuous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
impl ::core::clone::Clone for FaceDetectionEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FaceDetectionEffectFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionEffectFrame {}
impl ::core::fmt::Debug for FaceDetectionEffectFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffectFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FaceDetectionEffectFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffectFrame;{8ab08993-5dc8-447b-a247-5270bd802ece})");
}
unsafe impl ::windows::core::Interface for FaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ab08993_5dc8_447b_a247_5270bd802ece);
}
impl ::windows::core::RuntimeName for FaceDetectionEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectFrame";
}
impl ::core::convert::From<FaceDetectionEffectFrame> for ::windows::core::IUnknown {
    fn from(value: FaceDetectionEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffectFrame> for ::windows::core::IUnknown {
    fn from(value: &FaceDetectionEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FaceDetectionEffectFrame> for ::windows::core::IInspectable {
    fn from(value: FaceDetectionEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FaceDetectionEffectFrame> for ::windows::core::IInspectable {
    fn from(value: &FaceDetectionEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FaceDetectionEffectFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FaceDetectionEffectFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FaceDetectionEffectFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FaceDetectionEffectFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FaceDetectionEffectFrame> for super::IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: FaceDetectionEffectFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FaceDetectionEffectFrame> for super::IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &FaceDetectionEffectFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaFrame> for FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaFrame> for &FaceDetectionEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaFrame> {
        ::core::convert::TryInto::<super::IMediaFrame>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FaceDetectionEffectFrame {}
unsafe impl ::core::marker::Sync for FaceDetectionEffectFrame {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
}
impl ::core::marker::Copy for FaceDetectionMode {}
impl ::core::clone::Clone for FaceDetectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FaceDetectionMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FaceDetectionMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionMode {}
impl ::core::fmt::Debug for FaceDetectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FaceDetectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.FaceDetectionMode;i4)");
}
impl ::windows::core::DefaultType for FaceDetectionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct HighDynamicRangeControl(::windows::core::IUnknown);
impl HighDynamicRangeControl {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HighDynamicRangeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HighDynamicRangeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HighDynamicRangeControl {}
impl ::core::fmt::Debug for HighDynamicRangeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HighDynamicRangeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HighDynamicRangeControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.HighDynamicRangeControl;{55f1a7ae-d957-4dc9-9d1c-8553a82a7d99})");
}
unsafe impl ::windows::core::Interface for HighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55f1a7ae_d957_4dc9_9d1c_8553a82a7d99);
}
impl ::windows::core::RuntimeName for HighDynamicRangeControl {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeControl";
}
impl ::core::convert::From<HighDynamicRangeControl> for ::windows::core::IUnknown {
    fn from(value: HighDynamicRangeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeControl> for ::windows::core::IUnknown {
    fn from(value: &HighDynamicRangeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HighDynamicRangeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HighDynamicRangeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HighDynamicRangeControl> for ::windows::core::IInspectable {
    fn from(value: HighDynamicRangeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeControl> for ::windows::core::IInspectable {
    fn from(value: &HighDynamicRangeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HighDynamicRangeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HighDynamicRangeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HighDynamicRangeControl {}
unsafe impl ::core::marker::Sync for HighDynamicRangeControl {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct HighDynamicRangeOutput(::windows::core::IUnknown);
impl HighDynamicRangeOutput {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Certainty(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections', 'Media_Devices_Core'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))]
    pub fn FrameControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>>(result__)
        }
    }
}
impl ::core::clone::Clone for HighDynamicRangeOutput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HighDynamicRangeOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HighDynamicRangeOutput {}
impl ::core::fmt::Debug for HighDynamicRangeOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HighDynamicRangeOutput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HighDynamicRangeOutput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.HighDynamicRangeOutput;{0f57806b-253b-4119-bb40-3a90e51384f7})");
}
unsafe impl ::windows::core::Interface for HighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutputVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f57806b_253b_4119_bb40_3a90e51384f7);
}
impl ::windows::core::RuntimeName for HighDynamicRangeOutput {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeOutput";
}
impl ::core::convert::From<HighDynamicRangeOutput> for ::windows::core::IUnknown {
    fn from(value: HighDynamicRangeOutput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeOutput> for ::windows::core::IUnknown {
    fn from(value: &HighDynamicRangeOutput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HighDynamicRangeOutput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HighDynamicRangeOutput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HighDynamicRangeOutput> for ::windows::core::IInspectable {
    fn from(value: HighDynamicRangeOutput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HighDynamicRangeOutput> for ::windows::core::IInspectable {
    fn from(value: &HighDynamicRangeOutput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HighDynamicRangeOutput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HighDynamicRangeOutput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HighDynamicRangeOutput {}
unsafe impl ::core::marker::Sync for HighDynamicRangeOutput {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3692e4_4027_4847_a70b_df1d9a2a7b04);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptorImpl: IMediaStreamDescriptorImpl {
    fn EncodingProperties();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStreamDescriptor2 {
    type Vtable = IAudioStreamDescriptor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e68f1f6_a448_497b_8840_85082665acf9);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor2Impl: IMediaStreamDescriptorImpl {
    fn SetLeadingEncoderPadding();
    fn LeadingEncoderPadding();
    fn SetTrailingEncoderPadding();
    fn TrailingEncoderPadding();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStreamDescriptor3 {
    type Vtable = IAudioStreamDescriptor3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d220da1_8e83_44ef_8973_2f63e993f36b);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor3Impl {
    fn Copy();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStreamDescriptorFactory {
    type Vtable = IAudioStreamDescriptorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a86ce9e_4cb1_4380_8e0c_83504b7f5bf3);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptorFactoryImpl {
    fn Create();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrack(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioTrack {
    type Vtable = IAudioTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf23b6e77_3ef7_40de_b943_068b1321701d);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackImpl {
    fn OpenFailed();
    fn RemoveOpenFailed();
    fn GetEncodingProperties();
    fn PlaybackItem();
    fn Name();
    fn SupportInfo();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrackOpenFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeddb9b9_bb7c_4112_bf76_9384676f824b);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackOpenFailedEventArgsImpl {
    fn ExtendedError();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackOpenFailedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrackSupportInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178beff7_cc39_44a6_b951_4a5653f073fa);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackSupportInfoImpl {
    fn DecoderStatus();
    fn Degradation();
    fn DegradationReason();
    fn MediaSourceStatus();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackSupportInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradationReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChapterCue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChapterCue {
    type Vtable = IChapterCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a98001_d38a_4c0a_8fa6_75cddaf4664c);
}
#[cfg(feature = "implement_exclusive")]
pub trait IChapterCueImpl: IMediaCueImpl {
    fn SetTitle();
    fn Title();
}
#[repr(C)]
#[doc(hidden)]
pub struct IChapterCueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICodecInfo {
    type Vtable = ICodecInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e89f85_ea97_499c_86ac_4ce5e73f3a42);
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecInfoImpl {
    fn Kind();
    fn Category();
    fn Subtypes();
    fn DisplayName();
    fn IsTrusted();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CodecKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CodecCategory) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICodecQuery {
    type Vtable = ICodecQueryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x222a953a_af61_4e04_808a_a4634e2f3ac4);
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecQueryImpl {
    fn FindAllAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecQueryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: CodecKind, category: CodecCategory, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecSubtypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICodecSubtypesStatics {
    type Vtable = ICodecSubtypesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa66ac4f2_888b_4224_8cf6_2a8d4eb02382);
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecSubtypesStaticsImpl {
    fn VideoFormatDV25();
    fn VideoFormatDV50();
    fn VideoFormatDvc();
    fn VideoFormatDvh1();
    fn VideoFormatDvhD();
    fn VideoFormatDvsd();
    fn VideoFormatDvsl();
    fn VideoFormatH263();
    fn VideoFormatH264();
    fn VideoFormatH265();
    fn VideoFormatH264ES();
    fn VideoFormatHevc();
    fn VideoFormatHevcES();
    fn VideoFormatM4S2();
    fn VideoFormatMjpg();
    fn VideoFormatMP43();
    fn VideoFormatMP4S();
    fn VideoFormatMP4V();
    fn VideoFormatMpeg2();
    fn VideoFormatVP80();
    fn VideoFormatVP90();
    fn VideoFormatMpg1();
    fn VideoFormatMss1();
    fn VideoFormatMss2();
    fn VideoFormatWmv1();
    fn VideoFormatWmv2();
    fn VideoFormatWmv3();
    fn VideoFormatWvc1();
    fn VideoFormat420O();
    fn AudioFormatAac();
    fn AudioFormatAdts();
    fn AudioFormatAlac();
    fn AudioFormatAmrNB();
    fn AudioFormatAmrWB();
    fn AudioFormatAmrWP();
    fn AudioFormatDolbyAC3();
    fn AudioFormatDolbyAC3Spdif();
    fn AudioFormatDolbyDDPlus();
    fn AudioFormatDrm();
    fn AudioFormatDts();
    fn AudioFormatFlac();
    fn AudioFormatFloat();
    fn AudioFormatMP3();
    fn AudioFormatMPeg();
    fn AudioFormatMsp1();
    fn AudioFormatOpus();
    fn AudioFormatPcm();
    fn AudioFormatWmaSpdif();
    fn AudioFormatWMAudioLossless();
    fn AudioFormatWMAudioV8();
    fn AudioFormatWMAudioV9();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecSubtypesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataCue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataCue {
    type Vtable = IDataCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c7f676d_1fbc_4e2d_9a87_ee38bd1dc637);
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataCueImpl: IMediaCueImpl {
    fn SetData();
    fn Data();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataCue2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataCue2 {
    type Vtable = IDataCue2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc561b15_95f2_49e8_96f1_8dd5dac68d93);
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataCue2Impl: IDataCueImpl + IMediaCueImpl {
    fn Properties();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCue2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19918426_c65b_46ba_85f8_13880576c90a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectedEventArgsImpl {
    fn ResultFrame();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceDetectionEffect {
    type Vtable = IFaceDetectionEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae15ebd2_0542_42a9_bc90_f283a29f46c1);
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectionEffectImpl: IMediaExtensionImpl {
    fn SetEnabled();
    fn Enabled();
    fn SetDesiredDetectionInterval();
    fn DesiredDetectionInterval();
    fn FaceDetected();
    fn RemoveFaceDetected();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceDetectionEffectDefinition {
    type Vtable = IFaceDetectionEffectDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43dca081_b848_4f33_b702_1fd2624fb016);
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectDefinitionImpl: IVideoEffectDefinitionImpl {
    fn SetDetectionMode();
    fn DetectionMode();
    fn SetSynchronousDetectionEnabled();
    fn SynchronousDetectionEnabled();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectDefinitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FaceDetectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FaceDetectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffectFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ab08993_5dc8_447b_a247_5270bd802ece);
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectFrameImpl: IClosableImpl + IMediaFrameImpl {
    fn DetectedFaces();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHighDynamicRangeControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55f1a7ae_d957_4dc9_9d1c_8553a82a7d99);
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeControlImpl {
    fn SetEnabled();
    fn Enabled();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHighDynamicRangeControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHighDynamicRangeOutput(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutputVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f57806b_253b_4119_bb40_3a90e51384f7);
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeOutputImpl {
    fn Certainty();
    fn FrameControllers();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHighDynamicRangeOutputVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Devices_Core")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageCue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageCue {
    type Vtable = IImageCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52828282_367b_440b_9116_3c84570dd270);
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageCueImpl: IMediaCueImpl {
    fn Position();
    fn SetPosition();
    fn Extent();
    fn SetExtent();
    fn SetSoftwareBitmap();
    fn SoftwareBitmap();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageCueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25bc45e1_9b08_4c2e_a855_4542f1a75deb);
}
#[cfg(feature = "implement_exclusive")]
pub trait IInitializeMediaStreamSourceRequestedEventArgsImpl {
    fn Source();
    fn RandomAccessStream();
    fn GetDeferral();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeMediaStreamSourceRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLightFusionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLightFusionResult {
    type Vtable = ILowLightFusionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78edbe35_27a0_42e0_9cd3_738d2089de9c);
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLightFusionResultImpl {
    fn Frame();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLightFusionResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLightFusionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLightFusionStatics {
    type Vtable = ILowLightFusionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5305016d_c29e_40e2_87a9_9e1fd2f192f5);
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLightFusionStaticsImpl {
    fn SupportedBitmapPixelFormats();
    fn MaxSupportedFrameCount();
    fn FuseAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLightFusionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBinder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaBinder {
    type Vtable = IMediaBinderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b7e40aa_de07_424f_83f1_f1de46c4fa2e);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBinderImpl {
    fn Binding();
    fn RemoveBinding();
    fn Token();
    fn SetToken();
    fn Source();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBinderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb61cb25a_1b6d_4630_a86d_2f0837f712e5);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgsImpl {
    fn Canceled();
    fn RemoveCanceled();
    fn MediaBinder();
    fn GetDeferral();
    fn SetUri();
    fn SetStream();
    fn SetStreamReference();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaBindingEventArgs2 {
    type Vtable = IMediaBindingEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0464cceb_bb5a_482f_b8ba_f0284c696567);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgs2Impl {
    fn SetAdaptiveMediaSource();
    fn SetStorageFile();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Streaming_Adaptive")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaBindingEventArgs3 {
    type Vtable = IMediaBindingEventArgs3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8eb475e_19be_44fc_a5ed_7aba315037f9);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgs3Impl {
    fn SetDownloadOperation();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_BackgroundTransfer")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))] usize,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct IMediaCue(::windows::core::IUnknown);
impl IMediaCue {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaCue> for ::windows::core::IInspectable {
    fn from(value: IMediaCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaCue> for ::windows::core::IInspectable {
    fn from(value: &IMediaCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IMediaCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaCue> for ::windows::core::IUnknown {
    fn from(value: IMediaCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaCue> for ::windows::core::IUnknown {
    fn from(value: &IMediaCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaCue {}
impl ::core::fmt::Debug for IMediaCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaCue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaCue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c7d15e5d-59dc-431f-a0ee-27744323b36d}");
}
unsafe impl ::windows::core::Interface for IMediaCue {
    type Vtable = IMediaCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7d15e5d_59dc_431f_a0ee_27744323b36d);
}
pub trait IMediaCueImpl {
    fn SetStartTime();
    fn StartTime();
    fn SetDuration();
    fn Duration();
    fn SetId();
    fn Id();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCueEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCueEventArgs {
    type Vtable = IMediaCueEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd12f47f7_5fa4_4e68_9fe5_32160dcee57e);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCueEventArgsImpl {
    fn Cue();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCueEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct IMediaSource(::windows::core::IUnknown);
impl IMediaSource {}
impl ::core::convert::From<IMediaSource> for ::windows::core::IInspectable {
    fn from(value: IMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaSource> for ::windows::core::IInspectable {
    fn from(value: &IMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaSource> for ::windows::core::IUnknown {
    fn from(value: IMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaSource> for ::windows::core::IUnknown {
    fn from(value: &IMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaSource {}
impl ::core::fmt::Debug for IMediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e7bfb599-a09d-4c21-bcdf-20af4f86b3d9}");
}
unsafe impl ::windows::core::Interface for IMediaSource {
    type Vtable = IMediaSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7bfb599_a09d_4c21_bcdf_20af4f86b3d9);
}
pub trait IMediaSourceImpl {}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSource2 {
    type Vtable = IMediaSource2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eb61048_655f_4c37_b813_b4e45dfa0abe);
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource2Impl: IClosableImpl + IMediaPlaybackSourceImpl {
    fn OpenOperationCompleted();
    fn RemoveOpenOperationCompleted();
    fn CustomProperties();
    fn Duration();
    fn IsOpen();
    fn ExternalTimedTextSources();
    fn ExternalTimedMetadataTracks();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSource3 {
    type Vtable = IMediaSource3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb59f0d9b_4b6e_41ed_bbb4_7c7509a994ad);
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource3Impl: IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl {
    fn StateChanged();
    fn RemoveStateChanged();
    fn State();
    fn Reset();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSource4 {
    type Vtable = IMediaSource4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdafad57_8eff_4c63_85a6_84de0ae3e4f2);
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource4Impl: IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl + IMediaSource3Impl {
    fn AdaptiveMediaSource();
    fn MediaStreamSource();
    fn MseStreamSource();
    fn Uri();
    fn OpenAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Streaming_Adaptive")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSource5 {
    type Vtable = IMediaSource5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x331a22ae_ed2e_4a22_94c8_b743a92b3022);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSource5Impl {
    fn DownloadOperation();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_BackgroundTransfer")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61e1ea97_1916_4810_b7f4_b642be829596);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceAppServiceConnectionImpl {
    fn InitializeMediaStreamSourceRequested();
    fn RemoveInitializeMediaStreamSourceRequested();
    fn Start();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAppServiceConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnectionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceAppServiceConnectionFactory {
    type Vtable = IMediaSourceAppServiceConnectionFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65b912eb_80b9_44f9_9c1e_e120f6d92838);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceAppServiceConnectionFactoryImpl {
    fn Create();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAppServiceConnectionFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserviceconnection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceError(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceError {
    type Vtable = IMediaSourceErrorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0a8965_37c5_4e9d_8d21_1cdee90cecc6);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceErrorImpl {
    fn ExtendedError();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceErrorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceOpenOperationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc682ceb_e281_477c_a8e0_1acd654114c8);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceOpenOperationCompletedEventArgsImpl {
    fn Error();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceOpenOperationCompletedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a30af82_9071_4bac_bc39_ca2a93b717a9);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStateChangedEventArgsImpl {
    fn OldState();
    fn NewState();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStateChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceStatics {
    type Vtable = IMediaSourceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf77d6fa4_4652_410e_b1d8_e9a5e245a45c);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStaticsImpl {
    fn CreateFromAdaptiveMediaSource();
    fn CreateFromMediaStreamSource();
    fn CreateFromMseStreamSource();
    fn CreateFromIMediaSource();
    fn CreateFromStorageFile();
    fn CreateFromStream();
    fn CreateFromStreamReference();
    fn CreateFromUri();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Streaming_Adaptive")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceStatics2 {
    type Vtable = IMediaSourceStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeee161a4_7f13_4896_b8cb_df0de5bcb9f1);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics2Impl {
    fn CreateFromMediaBinder();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceStatics3 {
    type Vtable = IMediaSourceStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x453a30d6_2bea_4122_9f73_eace04526e35);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics3Impl {
    fn CreateFromMediaFrameSource();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture_Frames")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framesource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceStatics4 {
    type Vtable = IMediaSourceStatics4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x281b3bfc_e50a_4428_a500_9c4ed918d3f0);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics4Impl {
    fn CreateFromDownloadOperation();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_BackgroundTransfer")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))] usize,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct IMediaStreamDescriptor(::windows::core::IUnknown);
impl IMediaStreamDescriptor {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: IMediaStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: &IMediaStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IMediaStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: IMediaStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: &IMediaStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaStreamDescriptor {}
impl ::core::fmt::Debug for IMediaStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaStreamDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{80f16e6e-92f7-451e-97d2-afd80742da70}");
}
unsafe impl ::windows::core::Interface for IMediaStreamDescriptor {
    type Vtable = IMediaStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80f16e6e_92f7_451e_97d2_afd80742da70);
}
pub trait IMediaStreamDescriptorImpl {
    fn IsSelected();
    fn SetName();
    fn Name();
    fn SetLanguage();
    fn Language();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct IMediaStreamDescriptor2(::windows::core::IUnknown);
impl IMediaStreamDescriptor2 {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaStreamDescriptor2> for ::windows::core::IInspectable {
    fn from(value: IMediaStreamDescriptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor2> for ::windows::core::IInspectable {
    fn from(value: &IMediaStreamDescriptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaStreamDescriptor2> for ::windows::core::IUnknown {
    fn from(value: IMediaStreamDescriptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaStreamDescriptor2> for ::windows::core::IUnknown {
    fn from(value: &IMediaStreamDescriptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IMediaStreamDescriptor2> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: IMediaStreamDescriptor2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMediaStreamDescriptor2> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMediaStreamDescriptor2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for &IMediaStreamDescriptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IMediaStreamDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaStreamDescriptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaStreamDescriptor2 {}
impl ::core::fmt::Debug for IMediaStreamDescriptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaStreamDescriptor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaStreamDescriptor2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5073010f-e8b2-4071-b00b-ebf337a76b58}");
}
unsafe impl ::windows::core::Interface for IMediaStreamDescriptor2 {
    type Vtable = IMediaStreamDescriptor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5073010f_e8b2_4071_b00b_ebf337a76b58);
}
pub trait IMediaStreamDescriptor2Impl: IMediaStreamDescriptorImpl {
    fn SetLabel();
    fn Label();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamDescriptor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSample(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSample {
    type Vtable = IMediaStreamSampleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8db627_4b80_4361_9837_6cb7481ad9d6);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleImpl {
    fn Processed();
    fn RemoveProcessed();
    fn Buffer();
    fn Timestamp();
    fn ExtendedProperties();
    fn Protection();
    fn SetDecodeTimestamp();
    fn DecodeTimestamp();
    fn SetDuration();
    fn Duration();
    fn SetKeyFrame();
    fn KeyFrame();
    fn SetDiscontinuous();
    fn Discontinuous();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSample2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSample2 {
    type Vtable = IMediaStreamSample2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45078691_fce8_4746_a1c8_10c25d3d7cd3);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSample2Impl {
    fn Direct3D11Surface();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSample2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleProtectionProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4eb88292_ecdf_493e_841d_dd4add7caca2);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleProtectionPropertiesImpl {
    fn SetKeyIdentifier();
    fn GetKeyIdentifier();
    fn SetInitializationVector();
    fn GetInitializationVector();
    fn SetSubSampleMapping();
    fn GetSubSampleMapping();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleProtectionPropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSampleStatics {
    type Vtable = IMediaStreamSampleStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfdf218f_a6cf_4579_be41_73dd941ad972);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleStaticsImpl {
    fn CreateFromBuffer();
    fn CreateFromStreamAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, count: u32, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSampleStatics2 {
    type Vtable = IMediaStreamSampleStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9efe9521_6d46_494c_a2f8_d662922e2dd7);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleStatics2Impl {
    fn CreateFromDirect3D11Surface();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSource {
    type Vtable = IMediaStreamSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3712d543_45eb_4138_aa62_c01e26f3843f);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceImpl: IMediaSourceImpl {
    fn Closed();
    fn RemoveClosed();
    fn Starting();
    fn RemoveStarting();
    fn Paused();
    fn RemovePaused();
    fn SampleRequested();
    fn RemoveSampleRequested();
    fn SwitchStreamsRequested();
    fn RemoveSwitchStreamsRequested();
    fn NotifyError();
    fn AddStreamDescriptor();
    fn SetMediaProtectionManager();
    fn MediaProtectionManager();
    fn SetDuration();
    fn Duration();
    fn SetCanSeek();
    fn CanSeek();
    fn SetBufferTime();
    fn BufferTime();
    fn SetBufferedRange();
    fn MusicProperties();
    fn VideoProperties();
    fn SetThumbnail();
    fn Thumbnail();
    fn AddProtectionKey();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Protection")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))] usize,
    #[cfg(feature = "Media_Protection")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamdescriptor: ::windows::core::RawPtr, keyIdentifier_array_size: u32, keyidentifier: *const u8, licenseData_array_size: u32, licensedata: *const u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSource2 {
    type Vtable = IMediaStreamSource2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec55d0ad_2e6a_4f74_adbb_b562d1533849);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource2Impl: IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SampleRendered();
    fn RemoveSampleRendered();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSource3 {
    type Vtable = IMediaStreamSource3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2a2746_3ddd_4ddf_a121_94045ecf9440);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource3Impl: IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetMaxSupportedPlaybackRate();
    fn MaxSupportedPlaybackRate();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSource4 {
    type Vtable = IMediaStreamSource4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d0cfcab_830d_417c_a3a9_2454fd6415c7);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource4Impl: IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetIsLive();
    fn IsLive();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd8c7eb2_4816_4e24_88f0_491ef7386406);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedEventArgsImpl {
    fn Request();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceClosedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceClosedRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x907c00e9_18a3_4951_887a_2c1eebd5c69e);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedRequestImpl {
    fn Reason();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceClosedRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaStreamSourceClosedReason) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceFactory {
    type Vtable = IMediaStreamSourceFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef77e0d9_d158_4b7a_863f_203342fbfd41);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceFactoryImpl {
    fn CreateFromDescriptor();
    fn CreateFromDescriptors();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, descriptor2: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRenderedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d697b05_d4f2_4c7a_9dfe_8d6cd0b3ee84);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRenderedEventArgsImpl {
    fn SampleLag();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRenderedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4db341a9_3501_4d9b_83f9_8f235c822532);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestImpl {
    fn StreamDescriptor();
    fn GetDeferral();
    fn SetSample();
    fn Sample();
    fn ReportSampleProgress();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progress: u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7895cc02_f982_43c8_9d16_c62d999319be);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestDeferralImpl {
    fn Complete();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10f9bb9e_71c5_492f_847f_0da1f35e81f8);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestedEventArgsImpl {
    fn Request();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf41468f2_c274_4940_a5bb_28a572452fa7);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingEventArgsImpl {
    fn Request();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9093e4_35c4_4b1b_a791_0d99db56dd1d);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestImpl {
    fn StartPosition();
    fn GetDeferral();
    fn SetActualStartPosition();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f1356a5_6340_4dc4_9910_068ed9f598f8);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestDeferralImpl {
    fn Complete();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingRequestDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41b8808e_38a9_4ec3_9ba0_b69b85501e90);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestImpl {
    fn OldStreamDescriptor();
    fn NewStreamDescriptor();
    fn GetDeferral();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee3d835_a505_4f9a_b943_2b8cb1b4bbd9);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestDeferralImpl {
    fn Complete();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42202b72_6ea1_4677_981e_350a0da412aa);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl {
    fn Request();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct IMediaTrack(::windows::core::IUnknown);
impl IMediaTrack {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__: MediaTrackKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaTrackKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaTrack> for ::windows::core::IInspectable {
    fn from(value: IMediaTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaTrack> for ::windows::core::IInspectable {
    fn from(value: &IMediaTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IMediaTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaTrack> for ::windows::core::IUnknown {
    fn from(value: IMediaTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaTrack> for ::windows::core::IUnknown {
    fn from(value: &IMediaTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMediaTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaTrack {}
impl ::core::fmt::Debug for IMediaTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{03e1fafc-c931-491a-b46b-c10ee8c256b7}");
}
unsafe impl ::windows::core::Interface for IMediaTrack {
    type Vtable = IMediaTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
}
pub trait IMediaTrackImpl {
    fn Id();
    fn Language();
    fn TrackKind();
    fn SetLabel();
    fn Label();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTrackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseSourceBuffer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMseSourceBuffer {
    type Vtable = IMseSourceBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c1aa3e3_df8d_4079_a3fe_6849184b4e2f);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseSourceBufferImpl {
    fn UpdateStarting();
    fn RemoveUpdateStarting();
    fn Updated();
    fn RemoveUpdated();
    fn UpdateEnded();
    fn RemoveUpdateEnded();
    fn ErrorOccurred();
    fn RemoveErrorOccurred();
    fn Aborted();
    fn RemoveAborted();
    fn Mode();
    fn SetMode();
    fn IsUpdating();
    fn Buffered();
    fn TimestampOffset();
    fn SetTimestampOffset();
    fn AppendWindowStart();
    fn SetAppendWindowStart();
    fn AppendWindowEnd();
    fn SetAppendWindowEnd();
    fn AppendBuffer();
    fn AppendStream();
    fn AppendStreamMaxSize();
    fn Abort();
    fn Remove();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseSourceBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MseAppendMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MseAppendMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, maxsize: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: super::super::Foundation::TimeSpan, end: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseSourceBufferList(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMseSourceBufferList {
    type Vtable = IMseSourceBufferListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95fae8e7_a8e7_4ebf_8927_145e940ba511);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseSourceBufferListImpl {
    fn SourceBufferAdded();
    fn RemoveSourceBufferAdded();
    fn SourceBufferRemoved();
    fn RemoveSourceBufferRemoved();
    fn Buffers();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseSourceBufferListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMseStreamSource {
    type Vtable = IMseStreamSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0b4198d_02f4_4923_88dd_81bc3f360ffa);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceImpl: IMediaSourceImpl {
    fn Opened();
    fn RemoveOpened();
    fn Ended();
    fn RemoveEnded();
    fn Closed();
    fn RemoveClosed();
    fn SourceBuffers();
    fn ActiveSourceBuffers();
    fn ReadyState();
    fn Duration();
    fn SetDuration();
    fn AddSourceBuffer();
    fn RemoveSourceBuffer();
    fn EndOfStream();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MseReadyState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: MseEndOfStreamStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMseStreamSource2 {
    type Vtable = IMseStreamSource2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f57d37_f9e7_418a_9cde_a020e956552b);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSource2Impl {
    fn LiveSeekableRange();
    fn SetLiveSeekableRange();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSource2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMseStreamSourceStatics {
    type Vtable = IMseStreamSourceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465c679d_d570_43ce_ba21_0bff5f3fbd0a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceStaticsImpl {
    fn IsContentTypeSupported();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSourceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc04ba319_ca41_4813_bffd_7b08b0ed2557);
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalysisEffectImpl: IMediaExtensionImpl {
    fn HighDynamicRangeAnalyzer();
    fn SetDesiredAnalysisInterval();
    fn DesiredAnalysisInterval();
    fn SceneAnalyzed();
    fn RemoveSceneAnalyzed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8b10e4c_7fd9_42e1_85eb_6572c297c987);
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrameImpl: IClosableImpl + IMediaFrameImpl {
    fn FrameControlValues();
    fn HighDynamicRange();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISceneAnalysisEffectFrame2 {
    type Vtable = ISceneAnalysisEffectFrame2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4e29be_061f_47ae_9915_02524b5f9a5f);
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrame2Impl: IClosableImpl + IMediaFrameImpl {
    fn AnalysisRecommendation();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectFrame2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneAnalysisRecommendation) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalyzedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146b9588_2851_45e4_ad55_44cf8df8db4d);
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalyzedEventArgsImpl {
    fn ResultFrame();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalyzedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct ISingleSelectMediaTrackList(::windows::core::IUnknown);
impl ISingleSelectMediaTrackList {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectedIndexChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::convert::From<ISingleSelectMediaTrackList> for ::windows::core::IInspectable {
    fn from(value: ISingleSelectMediaTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISingleSelectMediaTrackList> for ::windows::core::IInspectable {
    fn from(value: &ISingleSelectMediaTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISingleSelectMediaTrackList> for ::windows::core::IUnknown {
    fn from(value: ISingleSelectMediaTrackList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISingleSelectMediaTrackList> for ::windows::core::IUnknown {
    fn from(value: &ISingleSelectMediaTrackList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISingleSelectMediaTrackList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISingleSelectMediaTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISingleSelectMediaTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISingleSelectMediaTrackList {}
impl ::core::fmt::Debug for ISingleSelectMediaTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISingleSelectMediaTrackList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISingleSelectMediaTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{77206f1f-c34f-494f-8077-2bad9ff4ecf1}");
}
unsafe impl ::windows::core::Interface for ISingleSelectMediaTrackList {
    type Vtable = ISingleSelectMediaTrackListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77206f1f_c34f_494f_8077_2bad9ff4ecf1);
}
pub trait ISingleSelectMediaTrackListImpl {
    fn SelectedIndexChanged();
    fn RemoveSelectedIndexChanged();
    fn SetSelectedIndex();
    fn SelectedIndex();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleSelectMediaTrackListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechCue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechCue {
    type Vtable = ISpeechCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaee254dc_1725_4bad_8043_a98499b017a2);
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechCueImpl: IMediaCueImpl {
    fn Text();
    fn SetText();
    fn StartPositionInInput();
    fn SetStartPositionInInput();
    fn EndPositionInInput();
    fn SetEndPositionInInput();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataStreamDescriptor {
    type Vtable = ITimedMetadataStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x133336bf_296a_463e_9ff9_01cd25691408);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataStreamDescriptorImpl {
    fn EncodingProperties();
    fn Copy();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataStreamDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataStreamDescriptorFactory {
    type Vtable = ITimedMetadataStreamDescriptorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc027de30_7362_4ff9_98b1_2dfd0b8d1cae);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataStreamDescriptorFactoryImpl {
    fn Create();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataStreamDescriptorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrack(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataTrack {
    type Vtable = ITimedMetadataTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6aed9e_f67a_49a9_b330_cf03b0e9cf07);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackImpl: IMediaTrackImpl {
    fn CueEntered();
    fn RemoveCueEntered();
    fn CueExited();
    fn RemoveCueExited();
    fn TrackFailed();
    fn RemoveTrackFailed();
    fn Cues();
    fn ActiveCues();
    fn TimedMetadataKind();
    fn DispatchType();
    fn AddCue();
    fn RemoveCue();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrack2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataTrack2 {
    type Vtable = ITimedMetadataTrack2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21b4b648_9f9d_40ba_a8f3_1a92753aef0b);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrack2Impl: IMediaTrackImpl + ITimedMetadataTrackImpl {
    fn PlaybackItem();
    fn Name();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackError(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackErrorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3767915_4114_4819_b9d9_dd76089e72f8);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackErrorImpl {
    fn ErrorCode();
    fn ExtendedError();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackErrorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackErrorCode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataTrackFactory {
    type Vtable = ITimedMetadataTrackFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dd57611_97b3_4e1f_852c_0f482c81ad26);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFactoryImpl {
    fn Create();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, kind: TimedMetadataKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa57fc9d1_6789_4d4d_b07f_84b4f31acb70);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFailedEventArgsImpl {
    fn Error();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFailedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct ITimedMetadataTrackProvider(::windows::core::IUnknown);
impl ITimedMetadataTrackProvider {
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>(result__)
        }
    }
}
impl ::core::convert::From<ITimedMetadataTrackProvider> for ::windows::core::IInspectable {
    fn from(value: ITimedMetadataTrackProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimedMetadataTrackProvider> for ::windows::core::IInspectable {
    fn from(value: &ITimedMetadataTrackProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITimedMetadataTrackProvider> for ::windows::core::IUnknown {
    fn from(value: ITimedMetadataTrackProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimedMetadataTrackProvider> for ::windows::core::IUnknown {
    fn from(value: &ITimedMetadataTrackProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITimedMetadataTrackProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITimedMetadataTrackProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITimedMetadataTrackProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimedMetadataTrackProvider {}
impl ::core::fmt::Debug for ITimedMetadataTrackProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimedMetadataTrackProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITimedMetadataTrackProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3b7f2024-f74e-4ade-93c5-219da05b6856}");
}
unsafe impl ::windows::core::Interface for ITimedMetadataTrackProvider {
    type Vtable = ITimedMetadataTrackProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b7f2024_f74e_4ade_93c5_219da05b6856);
}
pub trait ITimedMetadataTrackProviderImpl {
    fn TimedMetadataTracks();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextBouten(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextBouten {
    type Vtable = ITimedTextBoutenVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9062783_5597_5092_820c_8f738e0f774a);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextBoutenImpl {
    fn Type();
    fn SetType();
    fn Color();
    fn SetColor();
    fn Position();
    fn SetPosition();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextBoutenVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextBoutenType) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextBoutenPosition) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextCue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextCue {
    type Vtable = ITimedTextCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51c79e51_3b86_494d_b359_bb2ea7aca9a9);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextCueImpl: IMediaCueImpl {
    fn CueRegion();
    fn SetCueRegion();
    fn CueStyle();
    fn SetCueStyle();
    fn Lines();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextCueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextLine(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextLine {
    type Vtable = ITimedTextLineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x978d7ce2_7308_4c66_be50_65777289f5df);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextLineImpl {
    fn Text();
    fn SetText();
    fn Subformats();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextLineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextRegion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextRegion {
    type Vtable = ITimedTextRegionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ed0881f_8a06_4222_9f59_b21bf40124b4);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRegionImpl {
    fn Name();
    fn SetName();
    fn Position();
    fn SetPosition();
    fn Extent();
    fn SetExtent();
    fn Background();
    fn SetBackground();
    fn WritingMode();
    fn SetWritingMode();
    fn DisplayAlignment();
    fn SetDisplayAlignment();
    fn LineHeight();
    fn SetLineHeight();
    fn IsOverflowClipped();
    fn SetIsOverflowClipped();
    fn Padding();
    fn SetPadding();
    fn TextWrapping();
    fn SetTextWrapping();
    fn ZIndex();
    fn SetZIndex();
    fn ScrollMode();
    fn SetScrollMode();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextRegionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWritingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWritingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDisplayAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDisplayAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPadding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPadding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWrapping) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWrapping) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextScrollMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextScrollMode) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextRuby(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextRuby {
    type Vtable = ITimedTextRubyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10335c29_5b3c_5693_9959_d05a0bd24628);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRubyImpl {
    fn Text();
    fn SetText();
    fn Position();
    fn SetPosition();
    fn Align();
    fn SetAlign();
    fn Reserve();
    fn SetReserve();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextRubyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyPosition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyAlign) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyAlign) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyReserve) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyReserve) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextSource {
    type Vtable = ITimedTextSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4ed9ba6_101f_404d_a949_82f33fcd93b7);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceImpl {
    fn Resolved();
    fn RemoveResolved();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceResolveResultEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48907c9c_dcd8_4c33_9ad3_6cdce7b1c566);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceResolveResultEventArgsImpl {
    fn Error();
    fn Tracks();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceResolveResultEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextSourceStatics {
    type Vtable = ITimedTextSourceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e311853_9aba_4ac4_bb98_2fb176c3bfdd);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceStaticsImpl {
    fn CreateFromStream();
    fn CreateFromUri();
    fn CreateFromStreamWithLanguage();
    fn CreateFromUriWithLanguage();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextSourceStatics2 {
    type Vtable = ITimedTextSourceStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb66b7602_923e_43fa_9633_587075812db5);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceStatics2Impl {
    fn CreateFromStreamWithIndex();
    fn CreateFromUriWithIndex();
    fn CreateFromStreamWithIndexAndLanguage();
    fn CreateFromUriWithIndexAndLanguage();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, indexstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, indexuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, indexstream: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, indexuri: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextStyle {
    type Vtable = ITimedTextStyleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb2384d_a825_40c2_a7f5_281eaedf3b55);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyleImpl {
    fn Name();
    fn SetName();
    fn FontFamily();
    fn SetFontFamily();
    fn FontSize();
    fn SetFontSize();
    fn FontWeight();
    fn SetFontWeight();
    fn Foreground();
    fn SetForeground();
    fn Background();
    fn SetBackground();
    fn IsBackgroundAlwaysShown();
    fn SetIsBackgroundAlwaysShown();
    fn FlowDirection();
    fn SetFlowDirection();
    fn LineAlignment();
    fn SetLineAlignment();
    fn OutlineColor();
    fn SetOutlineColor();
    fn OutlineThickness();
    fn SetOutlineThickness();
    fn OutlineRadius();
    fn SetOutlineRadius();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWeight) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWeight) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFlowDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextFlowDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextLineAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextLineAlignment) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextStyle2 {
    type Vtable = ITimedTextStyle2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x655f492d_6111_4787_89cc_686fece57e14);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle2Impl {
    fn FontStyle();
    fn SetFontStyle();
    fn IsUnderlineEnabled();
    fn SetIsUnderlineEnabled();
    fn IsLineThroughEnabled();
    fn SetIsLineThroughEnabled();
    fn IsOverlineEnabled();
    fn SetIsOverlineEnabled();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFontStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextFontStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextStyle3 {
    type Vtable = ITimedTextStyle3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf803f93b_3e99_595e_bbb7_78a2fa13c270);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle3Impl {
    fn Ruby();
    fn Bouten();
    fn IsTextCombined();
    fn SetIsTextCombined();
    fn FontAngleInDegrees();
    fn SetFontAngleInDegrees();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSubformat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimedTextSubformat {
    type Vtable = ITimedTextSubformatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd713502f_3261_4722_a0c2_b937b2390f14);
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSubformatImpl {
    fn StartIndex();
    fn SetStartIndex();
    fn Length();
    fn SetLength();
    fn SubformatStyle();
    fn SetSubformatStyle();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSubformatVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStabilizationEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0808a650_9698_4e57_877b_bd7cb2ee0f8a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectImpl: IMediaExtensionImpl {
    fn SetEnabled();
    fn Enabled();
    fn EnabledChanged();
    fn RemoveEnabledChanged();
    fn GetRecommendedStreamConfiguration();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStabilizationEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controller: ::windows::core::RawPtr, desiredproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187eff28_67bb_4713_b900_4168da164529);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectEnabledChangedEventArgsImpl {
    fn Reason();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12ee0d55_9c2b_4440_8057_2c7a90f0cbec);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptorImpl: IMediaStreamDescriptorImpl {
    fn EncodingProperties();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoStreamDescriptor2 {
    type Vtable = IVideoStreamDescriptor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b306e10_453e_4088_832d_c36fa4f94af3);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptor2Impl {
    fn Copy();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoStreamDescriptorFactory {
    type Vtable = IVideoStreamDescriptorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x494ef6d1_bb75_43d2_9e5e_7b79a3afced4);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptorFactoryImpl {
    fn Create();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrack(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoTrack {
    type Vtable = IVideoTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99f3b7f3_e298_4396_bb6a_a51be6a2a20a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackImpl {
    fn OpenFailed();
    fn RemoveOpenFailed();
    fn GetEncodingProperties();
    fn PlaybackItem();
    fn Name();
    fn SupportInfo();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrackOpenFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7679e231_04f9_4c82_a4ee_8602c8bb4754);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackOpenFailedEventArgsImpl {
    fn ExtendedError();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackOpenFailedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrackSupportInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bb534a0_fc5f_450d_8ff0_778d590486de);
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackSupportInfoImpl {
    fn DecoderStatus();
    fn MediaSourceStatus();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackSupportInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct ImageCue(::windows::core::IUnknown);
impl ImageCue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageCue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Position(&self) -> ::windows::core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__: TimedTextPoint = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextPoint>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, TimedTextPoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Extent(&self) -> ::windows::core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__: TimedTextSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextSize>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetExtent<'a, Param0: ::windows::core::IntoParam<'a, TimedTextSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Graphics_Imaging'*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetSoftwareBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Graphics_Imaging'*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageCue {}
impl ::core::fmt::Debug for ImageCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageCue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ImageCue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.ImageCue;{52828282-367b-440b-9116-3c84570dd270})");
}
unsafe impl ::windows::core::Interface for ImageCue {
    type Vtable = IImageCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52828282_367b_440b_9116_3c84570dd270);
}
impl ::windows::core::RuntimeName for ImageCue {
    const NAME: &'static str = "Windows.Media.Core.ImageCue";
}
impl ::core::convert::From<ImageCue> for ::windows::core::IUnknown {
    fn from(value: ImageCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageCue> for ::windows::core::IUnknown {
    fn from(value: &ImageCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ImageCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageCue> for ::windows::core::IInspectable {
    fn from(value: ImageCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageCue> for ::windows::core::IInspectable {
    fn from(value: &ImageCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ImageCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageCue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageCue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for ImageCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for &ImageCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageCue {}
unsafe impl ::core::marker::Sync for ImageCue {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct InitializeMediaStreamSourceRequestedEventArgs(::windows::core::IUnknown);
impl InitializeMediaStreamSourceRequestedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Source(&self) -> ::windows::core::Result<MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSource>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RandomAccessStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for InitializeMediaStreamSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InitializeMediaStreamSourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InitializeMediaStreamSourceRequestedEventArgs {}
impl ::core::fmt::Debug for InitializeMediaStreamSourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InitializeMediaStreamSourceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InitializeMediaStreamSourceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs;{25bc45e1-9b08-4c2e-a855-4542f1a75deb})");
}
unsafe impl ::windows::core::Interface for InitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25bc45e1_9b08_4c2e_a855_4542f1a75deb);
}
impl ::windows::core::RuntimeName for InitializeMediaStreamSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs";
}
impl ::core::convert::From<InitializeMediaStreamSourceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InitializeMediaStreamSourceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InitializeMediaStreamSourceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InitializeMediaStreamSourceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InitializeMediaStreamSourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InitializeMediaStreamSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InitializeMediaStreamSourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for InitializeMediaStreamSourceRequestedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
pub struct LowLightFusion {}
impl LowLightFusion {
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections', 'Graphics_Imaging'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn SupportedBitmapPixelFormats() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn MaxSupportedFrameCount() -> ::windows::core::Result<i32> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation', 'Foundation_Collections', 'Graphics_Imaging'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn FuseAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap>>>(frameset: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), frameset.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILowLightFusionStatics<R, F: FnOnce(&ILowLightFusionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LowLightFusion, ILowLightFusionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LowLightFusion {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusion";
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct LowLightFusionResult(::windows::core::IUnknown);
impl LowLightFusionResult {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Graphics_Imaging'*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Frame(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLightFusionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LowLightFusionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLightFusionResult {}
impl ::core::fmt::Debug for LowLightFusionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLightFusionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LowLightFusionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.LowLightFusionResult;{78edbe35-27a0-42e0-9cd3-738d2089de9c})");
}
unsafe impl ::windows::core::Interface for LowLightFusionResult {
    type Vtable = ILowLightFusionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78edbe35_27a0_42e0_9cd3_738d2089de9c);
}
impl ::windows::core::RuntimeName for LowLightFusionResult {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusionResult";
}
impl ::core::convert::From<LowLightFusionResult> for ::windows::core::IUnknown {
    fn from(value: LowLightFusionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLightFusionResult> for ::windows::core::IUnknown {
    fn from(value: &LowLightFusionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLightFusionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LowLightFusionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLightFusionResult> for ::windows::core::IInspectable {
    fn from(value: LowLightFusionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLightFusionResult> for ::windows::core::IInspectable {
    fn from(value: &LowLightFusionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLightFusionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LowLightFusionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LowLightFusionResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LowLightFusionResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LowLightFusionResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LowLightFusionResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for LowLightFusionResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &LowLightFusionResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LowLightFusionResult {}
unsafe impl ::core::marker::Sync for LowLightFusionResult {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaBinder(::windows::core::IUnknown);
impl MediaBinder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaBinder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Binding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBinding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Source(&self) -> ::windows::core::Result<MediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSource>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBinder {}
impl ::core::fmt::Debug for MediaBinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBinder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBinder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaBinder;{2b7e40aa-de07-424f-83f1-f1de46c4fa2e})");
}
unsafe impl ::windows::core::Interface for MediaBinder {
    type Vtable = IMediaBinderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b7e40aa_de07_424f_83f1_f1de46c4fa2e);
}
impl ::windows::core::RuntimeName for MediaBinder {
    const NAME: &'static str = "Windows.Media.Core.MediaBinder";
}
impl ::core::convert::From<MediaBinder> for ::windows::core::IUnknown {
    fn from(value: MediaBinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBinder> for ::windows::core::IUnknown {
    fn from(value: &MediaBinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBinder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaBinder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBinder> for ::windows::core::IInspectable {
    fn from(value: MediaBinder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBinder> for ::windows::core::IInspectable {
    fn from(value: &MediaBinder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBinder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaBinder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBinder {}
unsafe impl ::core::marker::Sync for MediaBinder {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaBindingEventArgs(::windows::core::IUnknown);
impl MediaBindingEventArgs {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn MediaBinder(&self) -> ::windows::core::Result<MediaBinder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaBinder>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, stream: Param0, contenttype: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), stream.into_param().abi(), contenttype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStreamReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, stream: Param0, contenttype: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), stream.into_param().abi(), contenttype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Streaming_Adaptive'*"]
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn SetAdaptiveMediaSource<'a, Param0: ::windows::core::IntoParam<'a, super::Streaming::Adaptive::AdaptiveMediaSource>>(&self, mediasource: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediasource.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn SetStorageFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Networking_BackgroundTransfer'*"]
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn SetDownloadOperation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::BackgroundTransfer::DownloadOperation>>(&self, downloadoperation: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaBindingEventArgs3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), downloadoperation.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaBindingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBindingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBindingEventArgs {}
impl ::core::fmt::Debug for MediaBindingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBindingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBindingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaBindingEventArgs;{b61cb25a-1b6d-4630-a86d-2f0837f712e5})");
}
unsafe impl ::windows::core::Interface for MediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb61cb25a_1b6d_4630_a86d_2f0837f712e5);
}
impl ::windows::core::RuntimeName for MediaBindingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaBindingEventArgs";
}
impl ::core::convert::From<MediaBindingEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaBindingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBindingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaBindingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaBindingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaBindingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaBindingEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaBindingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaBindingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaBindingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaBindingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaBindingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaBindingEventArgs {}
unsafe impl ::core::marker::Sync for MediaBindingEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaCueEventArgs(::windows::core::IUnknown);
impl MediaCueEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Cue(&self) -> ::windows::core::Result<IMediaCue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMediaCue>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCueEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaCueEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCueEventArgs {}
impl ::core::fmt::Debug for MediaCueEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCueEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCueEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaCueEventArgs;{d12f47f7-5fa4-4e68-9fe5-32160dcee57e})");
}
unsafe impl ::windows::core::Interface for MediaCueEventArgs {
    type Vtable = IMediaCueEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd12f47f7_5fa4_4e68_9fe5_32160dcee57e);
}
impl ::windows::core::RuntimeName for MediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaCueEventArgs";
}
impl ::core::convert::From<MediaCueEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCueEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCueEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCueEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCueEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaCueEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCueEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCueEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCueEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCueEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCueEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaCueEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCueEventArgs {}
unsafe impl ::core::marker::Sync for MediaCueEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaDecoderStatus {}
impl ::core::clone::Clone for MediaDecoderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaDecoderStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaDecoderStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaDecoderStatus {}
impl ::core::fmt::Debug for MediaDecoderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDecoderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaDecoderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaDecoderStatus;i4)");
}
impl ::windows::core::DefaultType for MediaDecoderStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSource(::windows::core::IUnknown);
impl MediaSource {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenOperationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpenOperationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExternalTimedTextSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedTextSource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IObservableVector<TimedTextSource>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExternalTimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn State(&self) -> ::windows::core::Result<MediaSourceState> {
        let this = &::windows::core::Interface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__: MediaSourceState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceState>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Streaming_Adaptive'*"]
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn AdaptiveMediaSource(&self) -> ::windows::core::Result<super::Streaming::Adaptive::AdaptiveMediaSource> {
        let this = &::windows::core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streaming::Adaptive::AdaptiveMediaSource>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn MediaStreamSource(&self) -> ::windows::core::Result<MediaStreamSource> {
        let this = &::windows::core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSource>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn MseStreamSource(&self) -> ::windows::core::Result<MseStreamSource> {
        let this = &::windows::core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MseStreamSource>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Networking_BackgroundTransfer'*"]
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn DownloadOperation(&self) -> ::windows::core::Result<super::super::Networking::BackgroundTransfer::DownloadOperation> {
        let this = &::windows::core::Interface::cast::<IMediaSource5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::BackgroundTransfer::DownloadOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Streaming_Adaptive'*"]
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn CreateFromAdaptiveMediaSource<'a, Param0: ::windows::core::IntoParam<'a, super::Streaming::Adaptive::AdaptiveMediaSource>>(mediasource: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CreateFromMediaStreamSource<'a, Param0: ::windows::core::IntoParam<'a, MediaStreamSource>>(mediasource: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CreateFromMseStreamSource<'a, Param0: ::windows::core::IntoParam<'a, MseStreamSource>>(mediasource: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CreateFromIMediaSource<'a, Param0: ::windows::core::IntoParam<'a, IMediaSource>>(mediasource: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn CreateFromStorageFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(stream: Param0, contenttype: Param1) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), stream.into_param().abi(), contenttype.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(stream: Param0, contenttype: Param1) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), stream.into_param().abi(), contenttype.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CreateFromMediaBinder<'a, Param0: ::windows::core::IntoParam<'a, MediaBinder>>(binder: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), binder.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Capture_Frames'*"]
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFromMediaFrameSource<'a, Param0: ::windows::core::IntoParam<'a, super::Capture::Frames::MediaFrameSource>>(framesource: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), framesource.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Networking_BackgroundTransfer'*"]
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn CreateFromDownloadOperation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::BackgroundTransfer::DownloadOperation>>(downloadoperation: Param0) -> ::windows::core::Result<MediaSource> {
        Self::IMediaSourceStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), downloadoperation.into_param().abi(), &mut result__).from_abi::<MediaSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics<R, F: FnOnce(&IMediaSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaSource, IMediaSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics2<R, F: FnOnce(&IMediaSourceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaSource, IMediaSourceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics3<R, F: FnOnce(&IMediaSourceStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaSource, IMediaSourceStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics4<R, F: FnOnce(&IMediaSourceStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaSource, IMediaSourceStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSource {}
impl ::core::fmt::Debug for MediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSource;{2eb61048-655f-4c37-b813-b4e45dfa0abe})");
}
unsafe impl ::windows::core::Interface for MediaSource {
    type Vtable = IMediaSource2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eb61048_655f_4c37_b813_b4e45dfa0abe);
}
impl ::windows::core::RuntimeName for MediaSource {
    const NAME: &'static str = "Windows.Media.Core.MediaSource";
}
impl ::core::convert::From<MediaSource> for ::windows::core::IUnknown {
    fn from(value: MediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSource> for ::windows::core::IUnknown {
    fn from(value: &MediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSource> for ::windows::core::IInspectable {
    fn from(value: MediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSource> for ::windows::core::IInspectable {
    fn from(value: &MediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaSource> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaSource> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Media_Playback")]
impl ::core::convert::TryFrom<MediaSource> for super::Playback::IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Playback")]
impl ::core::convert::TryFrom<&MediaSource> for super::Playback::IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Playback")]
impl<'a> ::windows::core::IntoParam<'a, super::Playback::IMediaPlaybackSource> for MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::Playback::IMediaPlaybackSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Playback")]
impl<'a> ::windows::core::IntoParam<'a, super::Playback::IMediaPlaybackSource> for &MediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::Playback::IMediaPlaybackSource> {
        ::core::convert::TryInto::<super::Playback::IMediaPlaybackSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaSource {}
unsafe impl ::core::marker::Sync for MediaSource {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSourceAppServiceConnection(::windows::core::IUnknown);
impl MediaSourceAppServiceConnection {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn InitializeMediaStreamSourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInitializeMediaStreamSourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'ApplicationModel_AppService'*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::AppService::AppServiceConnection>>(appserviceconnection: Param0) -> ::windows::core::Result<MediaSourceAppServiceConnection> {
        Self::IMediaSourceAppServiceConnectionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appserviceconnection.into_param().abi(), &mut result__).from_abi::<MediaSourceAppServiceConnection>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaSourceAppServiceConnectionFactory<R, F: FnOnce(&IMediaSourceAppServiceConnectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaSourceAppServiceConnection, IMediaSourceAppServiceConnectionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaSourceAppServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceAppServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceAppServiceConnection {}
impl ::core::fmt::Debug for MediaSourceAppServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAppServiceConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceAppServiceConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceAppServiceConnection;{61e1ea97-1916-4810-b7f4-b642be829596})");
}
unsafe impl ::windows::core::Interface for MediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61e1ea97_1916_4810_b7f4_b642be829596);
}
impl ::windows::core::RuntimeName for MediaSourceAppServiceConnection {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceAppServiceConnection";
}
impl ::core::convert::From<MediaSourceAppServiceConnection> for ::windows::core::IUnknown {
    fn from(value: MediaSourceAppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAppServiceConnection> for ::windows::core::IUnknown {
    fn from(value: &MediaSourceAppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceAppServiceConnection> for ::windows::core::IInspectable {
    fn from(value: MediaSourceAppServiceConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAppServiceConnection> for ::windows::core::IInspectable {
    fn from(value: &MediaSourceAppServiceConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaSourceAppServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSourceError(::windows::core::IUnknown);
impl MediaSourceError {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaSourceError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceError {}
impl ::core::fmt::Debug for MediaSourceError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceError;{5c0a8965-37c5-4e9d-8d21-1cdee90cecc6})");
}
unsafe impl ::windows::core::Interface for MediaSourceError {
    type Vtable = IMediaSourceErrorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0a8965_37c5_4e9d_8d21_1cdee90cecc6);
}
impl ::windows::core::RuntimeName for MediaSourceError {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceError";
}
impl ::core::convert::From<MediaSourceError> for ::windows::core::IUnknown {
    fn from(value: MediaSourceError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceError> for ::windows::core::IUnknown {
    fn from(value: &MediaSourceError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSourceError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaSourceError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceError> for ::windows::core::IInspectable {
    fn from(value: MediaSourceError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceError> for ::windows::core::IInspectable {
    fn from(value: &MediaSourceError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSourceError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaSourceError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaSourceError {}
unsafe impl ::core::marker::Sync for MediaSourceError {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSourceOpenOperationCompletedEventArgs(::windows::core::IUnknown);
impl MediaSourceOpenOperationCompletedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Error(&self) -> ::windows::core::Result<MediaSourceError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceError>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaSourceOpenOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceOpenOperationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceOpenOperationCompletedEventArgs {}
impl ::core::fmt::Debug for MediaSourceOpenOperationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceOpenOperationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceOpenOperationCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs;{fc682ceb-e281-477c-a8e0-1acd654114c8})");
}
unsafe impl ::windows::core::Interface for MediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc682ceb_e281_477c_a8e0_1acd654114c8);
}
impl ::windows::core::RuntimeName for MediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs";
}
impl ::core::convert::From<MediaSourceOpenOperationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaSourceOpenOperationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceOpenOperationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaSourceOpenOperationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceOpenOperationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaSourceOpenOperationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceOpenOperationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaSourceOpenOperationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaSourceOpenOperationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaSourceOpenOperationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceOpenOperationCompletedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaSourceState {}
impl ::core::clone::Clone for MediaSourceState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaSourceState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaSourceState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceState {}
impl ::core::fmt::Debug for MediaSourceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceState;i4)");
}
impl ::windows::core::DefaultType for MediaSourceState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(::windows::core::IUnknown);
impl MediaSourceStateChangedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn OldState(&self) -> ::windows::core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceState>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn NewState(&self) -> ::windows::core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceState>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaSourceStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceStateChangedEventArgs {}
impl ::core::fmt::Debug for MediaSourceStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceStateChangedEventArgs;{0a30af82-9071-4bac-bc39-ca2a93b717a9})");
}
unsafe impl ::windows::core::Interface for MediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a30af82_9071_4bac_bc39_ca2a93b717a9);
}
impl ::windows::core::RuntimeName for MediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceStateChangedEventArgs";
}
impl ::core::convert::From<MediaSourceStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaSourceStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaSourceStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaSourceStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaSourceStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaSourceStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaSourceStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceStateChangedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaSourceStatus {}
impl ::core::clone::Clone for MediaSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaSourceStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaSourceStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceStatus {}
impl ::core::fmt::Debug for MediaSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceStatus;i4)");
}
impl ::windows::core::DefaultType for MediaSourceStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSample(::windows::core::IUnknown);
impl MediaStreamSample {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Processed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProcessed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::Buffer>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<MediaStreamSamplePropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSamplePropertySet>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Protection(&self) -> ::windows::core::Result<MediaStreamSampleProtectionProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSampleProtectionProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDecodeTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DecodeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetKeyFrame(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn KeyFrame(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetDiscontinuous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Discontinuous(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Surface(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSample2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(buffer: Param0, timestamp: Param1) -> ::windows::core::Result<MediaStreamSample> {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), timestamp.into_param().abi(), &mut result__).from_abi::<MediaStreamSample>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(stream: Param0, count: u32, timestamp: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaStreamSample>> {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stream.into_param().abi(), count, timestamp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaStreamSample>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateFromDirect3D11Surface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(surface: Param0, timestamp: Param1) -> ::windows::core::Result<MediaStreamSample> {
        Self::IMediaStreamSampleStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), surface.into_param().abi(), timestamp.into_param().abi(), &mut result__).from_abi::<MediaStreamSample>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaStreamSampleStatics<R, F: FnOnce(&IMediaStreamSampleStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMediaStreamSampleStatics2<R, F: FnOnce(&IMediaStreamSampleStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaStreamSample {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSample {}
impl ::core::fmt::Debug for MediaStreamSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSample").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSample {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSample;{5c8db627-4b80-4361-9837-6cb7481ad9d6})");
}
unsafe impl ::windows::core::Interface for MediaStreamSample {
    type Vtable = IMediaStreamSampleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8db627_4b80_4361_9837_6cb7481ad9d6);
}
impl ::windows::core::RuntimeName for MediaStreamSample {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSample";
}
impl ::core::convert::From<MediaStreamSample> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSample) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSample> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSample) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSample> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSample) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSample> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSample) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSample {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSample {}
unsafe impl ::core::marker::Sync for MediaStreamSample {}
#[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct MediaStreamSamplePropertySet(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MediaStreamSamplePropertySet {
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for MediaStreamSamplePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for MediaStreamSamplePropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for MediaStreamSamplePropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSamplePropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for MediaStreamSamplePropertySet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSamplePropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};g16;cinterface(IInspectable)))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for MediaStreamSamplePropertySet {
    type Vtable = super::super::Foundation::Collections::IMapVtbl<::windows::core::GUID, ::windows::core::IInspectable>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for MediaStreamSamplePropertySet {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSamplePropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for MediaStreamSamplePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &MediaStreamSamplePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaStreamSamplePropertySet> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSamplePropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaStreamSamplePropertySet> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSamplePropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<MediaStreamSamplePropertySet> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSamplePropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&MediaStreamSamplePropertySet> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSamplePropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaStreamSamplePropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaStreamSamplePropertySet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaStreamSamplePropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaStreamSamplePropertySet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> for &MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<MediaStreamSamplePropertySet> for super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaStreamSamplePropertySet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&MediaStreamSamplePropertySet> for super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaStreamSamplePropertySet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> for MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> for &MediaStreamSamplePropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaStreamSamplePropertySet {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSampleProtectionProperties(::windows::core::IUnknown);
impl MediaStreamSampleProtectionProperties {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetKeyIdentifier(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn GetKeyIdentifier(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetInitializationVector(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn GetInitializationVector(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetSubSampleMapping(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn GetSubSampleMapping(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSampleProtectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSampleProtectionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSampleProtectionProperties {}
impl ::core::fmt::Debug for MediaStreamSampleProtectionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSampleProtectionProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSampleProtectionProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSampleProtectionProperties;{4eb88292-ecdf-493e-841d-dd4add7caca2})");
}
unsafe impl ::windows::core::Interface for MediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4eb88292_ecdf_493e_841d_dd4add7caca2);
}
impl ::windows::core::RuntimeName for MediaStreamSampleProtectionProperties {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSampleProtectionProperties";
}
impl ::core::convert::From<MediaStreamSampleProtectionProperties> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSampleProtectionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSampleProtectionProperties> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSampleProtectionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSampleProtectionProperties> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSampleProtectionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSampleProtectionProperties> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSampleProtectionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSampleProtectionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSampleProtectionProperties {}
unsafe impl ::core::marker::Sync for MediaStreamSampleProtectionProperties {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSource(::windows::core::IUnknown);
impl MediaStreamSource {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Starting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Paused<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePaused<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SampleRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSampleRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SwitchStreamsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSwitchStreamsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn NotifyError(&self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), errorstatus).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AddStreamDescriptor<'a, Param0: ::windows::core::IntoParam<'a, IMediaStreamDescriptor>>(&self, descriptor: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), descriptor.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Protection'*"]
    #[cfg(feature = "Media_Protection")]
    pub fn SetMediaProtectionManager<'a, Param0: ::windows::core::IntoParam<'a, super::Protection::MediaProtectionManager>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Protection'*"]
    #[cfg(feature = "Media_Protection")]
    pub fn MediaProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Protection::MediaProtectionManager>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetCanSeek(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBufferTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBufferedRange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, startoffset: Param0, endoffset: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), startoffset.into_param().abi(), endoffset.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_FileProperties'*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::super::Storage::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_FileProperties'*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::super::Storage::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AddProtectionKey<'a, Param0: ::windows::core::IntoParam<'a, IMediaStreamDescriptor>>(&self, streamdescriptor: Param0, keyidentifier: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensedata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), streamdescriptor.into_param().abi(), keyidentifier.len() as u32, ::core::mem::transmute(keyidentifier.as_ptr()), licensedata.len() as u32, ::core::mem::transmute(licensedata.as_ptr())).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SampleRendered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSource2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSampleRendered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSupportedPlaybackRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSource3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSupportedPlaybackRate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsLive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSource4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsLive(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaStreamSource4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CreateFromDescriptor<'a, Param0: ::windows::core::IntoParam<'a, IMediaStreamDescriptor>>(descriptor: Param0) -> ::windows::core::Result<MediaStreamSource> {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), &mut result__).from_abi::<MediaStreamSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CreateFromDescriptors<'a, Param0: ::windows::core::IntoParam<'a, IMediaStreamDescriptor>, Param1: ::windows::core::IntoParam<'a, IMediaStreamDescriptor>>(descriptor: Param0, descriptor2: Param1) -> ::windows::core::Result<MediaStreamSource> {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), descriptor2.into_param().abi(), &mut result__).from_abi::<MediaStreamSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaStreamSourceFactory<R, F: FnOnce(&IMediaStreamSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaStreamSource, IMediaStreamSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSource {}
impl ::core::fmt::Debug for MediaStreamSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSource;{3712d543-45eb-4138-aa62-c01e26f3843f})");
}
unsafe impl ::windows::core::Interface for MediaStreamSource {
    type Vtable = IMediaStreamSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3712d543_45eb_4138_aa62_c01e26f3843f);
}
impl ::windows::core::RuntimeName for MediaStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSource";
}
impl ::core::convert::From<MediaStreamSource> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSource> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSource> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSource> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaStreamSource> for IMediaSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaStreamSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaStreamSource> for IMediaSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaStreamSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaSource> for MediaStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaSource> for &MediaStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaSource> {
        ::core::convert::TryInto::<IMediaSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaStreamSource {}
unsafe impl ::core::marker::Sync for MediaStreamSource {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceClosedEventArgs(::windows::core::IUnknown);
impl MediaStreamSourceClosedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Request(&self) -> ::windows::core::Result<MediaStreamSourceClosedRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceClosedRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceClosedEventArgs;{cd8c7eb2-4816-4e24-88f0-491ef7386406})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd8c7eb2_4816_4e24_88f0_491ef7386406);
}
impl ::windows::core::RuntimeName for MediaStreamSourceClosedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceClosedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceClosedReason(pub i32);
impl MediaStreamSourceClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const AppReportedError: Self = Self(2i32);
    pub const UnsupportedProtectionSystem: Self = Self(3i32);
    pub const ProtectionSystemFailure: Self = Self(4i32);
    pub const UnsupportedEncodingFormat: Self = Self(5i32);
    pub const MissingSampleRequestedEventHandler: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaStreamSourceClosedReason {}
impl ::core::clone::Clone for MediaStreamSourceClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaStreamSourceClosedReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedReason {}
impl ::core::fmt::Debug for MediaStreamSourceClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceClosedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceClosedReason;i4)");
}
impl ::windows::core::DefaultType for MediaStreamSourceClosedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(::windows::core::IUnknown);
impl MediaStreamSourceClosedRequest {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Reason(&self) -> ::windows::core::Result<MediaStreamSourceClosedReason> {
        let this = self;
        unsafe {
            let mut result__: MediaStreamSourceClosedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceClosedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceClosedRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedRequest {}
impl ::core::fmt::Debug for MediaStreamSourceClosedRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceClosedRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceClosedRequest;{907c00e9-18a3-4951-887a-2c1eebd5c69e})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x907c00e9_18a3_4951_887a_2c1eebd5c69e);
}
impl ::windows::core::RuntimeName for MediaStreamSourceClosedRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedRequest";
}
impl ::core::convert::From<MediaStreamSourceClosedRequest> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceClosedRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedRequest> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceClosedRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceClosedRequest> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceClosedRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceClosedRequest> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceClosedRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceClosedRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceClosedRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedRequest {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: Self = Self(0i32);
    pub const OutOfMemory: Self = Self(1i32);
    pub const FailedToOpenFile: Self = Self(2i32);
    pub const FailedToConnectToServer: Self = Self(3i32);
    pub const ConnectionToServerLost: Self = Self(4i32);
    pub const UnspecifiedNetworkError: Self = Self(5i32);
    pub const DecodeError: Self = Self(6i32);
    pub const UnsupportedMediaFormat: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaStreamSourceErrorStatus {}
impl ::core::clone::Clone for MediaStreamSourceErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaStreamSourceErrorStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaStreamSourceErrorStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceErrorStatus {}
impl ::core::fmt::Debug for MediaStreamSourceErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceErrorStatus;i4)");
}
impl ::windows::core::DefaultType for MediaStreamSourceErrorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRenderedEventArgs(::windows::core::IUnknown);
impl MediaStreamSourceSampleRenderedEventArgs {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SampleLag(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRenderedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRenderedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRenderedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRenderedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSampleRenderedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs;{9d697b05-d4f2-4c7a-9dfe-8d6cd0b3ee84})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d697b05_d4f2_4c7a_9dfe_8d6cd0b3ee84);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSampleRenderedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceSampleRenderedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSampleRenderedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRenderedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRenderedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRenderedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSampleRenderedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRenderedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRenderedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSampleRenderedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRenderedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRenderedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequest(::windows::core::IUnknown);
impl MediaStreamSourceSampleRequest {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn StreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMediaStreamDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceSampleRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceSampleRequestDeferral>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetSample<'a, Param0: ::windows::core::IntoParam<'a, MediaStreamSample>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Sample(&self) -> ::windows::core::Result<MediaStreamSample> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSample>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ReportSampleProgress(&self, progress: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), progress).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequest {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSampleRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequest;{4db341a9-3501-4d9b-83f9-8f235c822532})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4db341a9_3501_4d9b_83f9_8f235c822532);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSampleRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequest";
}
impl ::core::convert::From<MediaStreamSourceSampleRequest> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSampleRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequest> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRequest> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSampleRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequest> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSampleRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequest {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestDeferral(::windows::core::IUnknown);
impl MediaStreamSourceSampleRequestDeferral {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSampleRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequestDeferral;{7895cc02-f982-43c8-9d16-c62d999319be})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7895cc02_f982_43c8_9d16_c62d999319be);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSampleRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestDeferral";
}
impl ::core::convert::From<MediaStreamSourceSampleRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSampleRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSampleRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSampleRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestDeferral {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestedEventArgs(::windows::core::IUnknown);
impl MediaStreamSourceSampleRequestedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Request(&self) -> ::windows::core::Result<MediaStreamSourceSampleRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceSampleRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequestedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSampleRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs;{10f9bb9e-71c5-492f-847f-0da1f35e81f8})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10f9bb9e_71c5_492f_847f_0da1f35e81f8);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSampleRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceSampleRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSampleRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSampleRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSampleRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSampleRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSampleRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSampleRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSampleRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceStartingEventArgs(::windows::core::IUnknown);
impl MediaStreamSourceStartingEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Request(&self) -> ::windows::core::Result<MediaStreamSourceStartingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceStartingRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingEventArgs;{f41468f2-c274-4940-a5bb-28a572452fa7})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf41468f2_c274_4940_a5bb_28a572452fa7);
}
impl ::windows::core::RuntimeName for MediaStreamSourceStartingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingEventArgs";
}
impl ::core::convert::From<MediaStreamSourceStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequest(::windows::core::IUnknown);
impl MediaStreamSourceStartingRequest {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceStartingRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceStartingRequestDeferral>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActualStartPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, position: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceStartingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingRequest {}
impl ::core::fmt::Debug for MediaStreamSourceStartingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceStartingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingRequest;{2a9093e4-35c4-4b1b-a791-0d99db56dd1d})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9093e4_35c4_4b1b_a791_0d99db56dd1d);
}
impl ::windows::core::RuntimeName for MediaStreamSourceStartingRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequest";
}
impl ::core::convert::From<MediaStreamSourceStartingRequest> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceStartingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequest> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceStartingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceStartingRequest> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceStartingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequest> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceStartingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceStartingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequest {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequestDeferral(::windows::core::IUnknown);
impl MediaStreamSourceStartingRequestDeferral {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceStartingRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceStartingRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceStartingRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingRequestDeferral;{3f1356a5-6340-4dc4-9910-068ed9f598f8})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f1356a5_6340_4dc4_9910_068ed9f598f8);
}
impl ::windows::core::RuntimeName for MediaStreamSourceStartingRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequestDeferral";
}
impl ::core::convert::From<MediaStreamSourceStartingRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceStartingRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceStartingRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceStartingRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceStartingRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceStartingRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceStartingRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceStartingRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequestDeferral {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequest(::windows::core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequest {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn OldStreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMediaStreamDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn NewStreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMediaStreamDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceSwitchStreamsRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequest {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSwitchStreamsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest;{41b8808e-38a9-4ec3-9ba0-b69b85501e90})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41b8808e_38a9_4ec3_9ba0_b69b85501e90);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSwitchStreamsRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest";
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequest> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSwitchStreamsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequest> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSwitchStreamsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequest> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSwitchStreamsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequest> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSwitchStreamsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSwitchStreamsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequest {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestDeferral(::windows::core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequestDeferral {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSwitchStreamsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral;{bee3d835-a505-4f9a-b943-2b8cb1b4bbd9})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee3d835_a505_4f9a_b943_2b8cb1b4bbd9);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSwitchStreamsRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral";
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSwitchStreamsRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestDeferral {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestedEventArgs(::windows::core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequestedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Request(&self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaStreamSourceSwitchStreamsRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs;{42202b72-6ea1-4677-981e-350a0da412aa})");
}
unsafe impl ::windows::core::Interface for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42202b72_6ea1_4677_981e_350a0da412aa);
}
impl ::windows::core::RuntimeName for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs";
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaStreamSourceSwitchStreamsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaStreamSourceSwitchStreamsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaTrackKind {}
impl ::core::clone::Clone for MediaTrackKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MediaTrackKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MediaTrackKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTrackKind {}
impl ::core::fmt::Debug for MediaTrackKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTrackKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaTrackKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaTrackKind;i4)");
}
impl ::windows::core::DefaultType for MediaTrackKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
impl ::core::marker::Copy for MseAppendMode {}
impl ::core::clone::Clone for MseAppendMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MseAppendMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MseAppendMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseAppendMode {}
impl ::core::fmt::Debug for MseAppendMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseAppendMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MseAppendMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseAppendMode;i4)");
}
impl ::windows::core::DefaultType for MseAppendMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MseEndOfStreamStatus {}
impl ::core::clone::Clone for MseEndOfStreamStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MseEndOfStreamStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MseEndOfStreamStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseEndOfStreamStatus {}
impl ::core::fmt::Debug for MseEndOfStreamStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseEndOfStreamStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MseEndOfStreamStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseEndOfStreamStatus;i4)");
}
impl ::windows::core::DefaultType for MseEndOfStreamStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
impl ::core::marker::Copy for MseReadyState {}
impl ::core::clone::Clone for MseReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MseReadyState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MseReadyState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseReadyState {}
impl ::core::fmt::Debug for MseReadyState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseReadyState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MseReadyState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseReadyState;i4)");
}
impl ::windows::core::DefaultType for MseReadyState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MseSourceBuffer(::windows::core::IUnknown);
impl MseSourceBuffer {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Aborted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAborted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Mode(&self) -> ::windows::core::Result<MseAppendMode> {
        let this = self;
        unsafe {
            let mut result__: MseAppendMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MseAppendMode>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetMode(&self, value: MseAppendMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsUpdating(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn Buffered(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseTimeRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MseTimeRange>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TimestampOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTimestampOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AppendWindowStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAppendWindowStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AppendWindowEnd(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAppendWindowEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), stream.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendStreamMaxSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, stream: Param0, maxsize: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), stream.into_param().abi(), maxsize).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Abort(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, start: Param0, end: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), start.into_param().abi(), end.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MseSourceBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MseSourceBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseSourceBuffer {}
impl ::core::fmt::Debug for MseSourceBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseSourceBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MseSourceBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseSourceBuffer;{0c1aa3e3-df8d-4079-a3fe-6849184b4e2f})");
}
unsafe impl ::windows::core::Interface for MseSourceBuffer {
    type Vtable = IMseSourceBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c1aa3e3_df8d_4079_a3fe_6849184b4e2f);
}
impl ::windows::core::RuntimeName for MseSourceBuffer {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBuffer";
}
impl ::core::convert::From<MseSourceBuffer> for ::windows::core::IUnknown {
    fn from(value: MseSourceBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBuffer> for ::windows::core::IUnknown {
    fn from(value: &MseSourceBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MseSourceBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MseSourceBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MseSourceBuffer> for ::windows::core::IInspectable {
    fn from(value: MseSourceBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBuffer> for ::windows::core::IInspectable {
    fn from(value: &MseSourceBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MseSourceBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MseSourceBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MseSourceBuffer {}
unsafe impl ::core::marker::Sync for MseSourceBuffer {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MseSourceBufferList(::windows::core::IUnknown);
impl MseSourceBufferList {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceBufferAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceBufferAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceBufferRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceBufferRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Buffers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>>(result__)
        }
    }
}
impl ::core::clone::Clone for MseSourceBufferList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MseSourceBufferList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseSourceBufferList {}
impl ::core::fmt::Debug for MseSourceBufferList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseSourceBufferList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MseSourceBufferList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseSourceBufferList;{95fae8e7-a8e7-4ebf-8927-145e940ba511})");
}
unsafe impl ::windows::core::Interface for MseSourceBufferList {
    type Vtable = IMseSourceBufferListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95fae8e7_a8e7_4ebf_8927_145e940ba511);
}
impl ::windows::core::RuntimeName for MseSourceBufferList {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBufferList";
}
impl ::core::convert::From<MseSourceBufferList> for ::windows::core::IUnknown {
    fn from(value: MseSourceBufferList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBufferList> for ::windows::core::IUnknown {
    fn from(value: &MseSourceBufferList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MseSourceBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MseSourceBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MseSourceBufferList> for ::windows::core::IInspectable {
    fn from(value: MseSourceBufferList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseSourceBufferList> for ::windows::core::IInspectable {
    fn from(value: &MseSourceBufferList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MseSourceBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MseSourceBufferList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MseSourceBufferList {}
unsafe impl ::core::marker::Sync for MseSourceBufferList {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct MseStreamSource(::windows::core::IUnknown);
impl MseStreamSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MseStreamSource, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Opened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Ended<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SourceBuffers(&self) -> ::windows::core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MseSourceBufferList>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ActiveSourceBuffers(&self) -> ::windows::core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MseSourceBufferList>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ReadyState(&self) -> ::windows::core::Result<MseReadyState> {
        let this = self;
        unsafe {
            let mut result__: MseReadyState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MseReadyState>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AddSourceBuffer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, mimetype: Param0) -> ::windows::core::Result<MseSourceBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), mimetype.into_param().abi(), &mut result__).from_abi::<MseSourceBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn RemoveSourceBuffer<'a, Param0: ::windows::core::IntoParam<'a, MseSourceBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn EndOfStream(&self, status: MseEndOfStreamStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), status).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LiveSeekableRange(&self) -> ::windows::core::Result<super::super::Foundation::IReference<MseTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMseStreamSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<MseTimeRange>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLiveSeekableRange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<MseTimeRange>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMseStreamSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsContentTypeSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contenttype: Param0) -> ::windows::core::Result<bool> {
        Self::IMseStreamSourceStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contenttype.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMseStreamSourceStatics<R, F: FnOnce(&IMseStreamSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MseStreamSource, IMseStreamSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MseStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MseStreamSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseStreamSource {}
impl ::core::fmt::Debug for MseStreamSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseStreamSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MseStreamSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseStreamSource;{b0b4198d-02f4-4923-88dd-81bc3f360ffa})");
}
unsafe impl ::windows::core::Interface for MseStreamSource {
    type Vtable = IMseStreamSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0b4198d_02f4_4923_88dd_81bc3f360ffa);
}
impl ::windows::core::RuntimeName for MseStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MseStreamSource";
}
impl ::core::convert::From<MseStreamSource> for ::windows::core::IUnknown {
    fn from(value: MseStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseStreamSource> for ::windows::core::IUnknown {
    fn from(value: &MseStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MseStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MseStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MseStreamSource> for ::windows::core::IInspectable {
    fn from(value: MseStreamSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MseStreamSource> for ::windows::core::IInspectable {
    fn from(value: &MseStreamSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MseStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MseStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MseStreamSource> for IMediaSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MseStreamSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MseStreamSource> for IMediaSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MseStreamSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaSource> for MseStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaSource> for &MseStreamSource {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaSource> {
        ::core::convert::TryInto::<IMediaSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MseStreamSource {}
unsafe impl ::core::marker::Sync for MseStreamSource {}
#[repr(C)]
#[doc = "*Required features: 'Media_Core', 'Foundation'*"]
#[cfg(feature = "Foundation")]
pub struct MseTimeRange {
    pub Start: super::super::Foundation::TimeSpan,
    pub End: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MseTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MseTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for MseTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MseTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for MseTimeRange {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for MseTimeRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.MseTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for MseTimeRange {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for MseTimeRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MseTimeRange>()) == 0 }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for MseTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for MseTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct SceneAnalysisEffect(::windows::core::IUnknown);
impl SceneAnalysisEffect {
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn HighDynamicRangeAnalyzer(&self) -> ::windows::core::Result<HighDynamicRangeControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HighDynamicRangeControl>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredAnalysisInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredAnalysisInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SceneAnalyzed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSceneAnalyzed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SceneAnalysisEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneAnalysisEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisEffect {}
impl ::core::fmt::Debug for SceneAnalysisEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneAnalysisEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffect;{c04ba319-ca41-4813-bffd-7b08b0ed2557})");
}
unsafe impl ::windows::core::Interface for SceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc04ba319_ca41_4813_bffd_7b08b0ed2557);
}
impl ::windows::core::RuntimeName for SceneAnalysisEffect {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffect";
}
impl ::core::convert::From<SceneAnalysisEffect> for ::windows::core::IUnknown {
    fn from(value: SceneAnalysisEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffect> for ::windows::core::IUnknown {
    fn from(value: &SceneAnalysisEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneAnalysisEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SceneAnalysisEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneAnalysisEffect> for ::windows::core::IInspectable {
    fn from(value: SceneAnalysisEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffect> for ::windows::core::IInspectable {
    fn from(value: &SceneAnalysisEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneAnalysisEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SceneAnalysisEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneAnalysisEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneAnalysisEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneAnalysisEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneAnalysisEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for SceneAnalysisEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for &SceneAnalysisEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneAnalysisEffect {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffect {}
#[doc = "*Required features: 'Media_Core', 'Media_Effects'*"]
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(::windows::core::IUnknown);
#[cfg(feature = "Media_Effects")]
impl SceneAnalysisEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SceneAnalysisEffectDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Effects'*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections', 'Media_Effects'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::clone::Clone for SceneAnalysisEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::PartialEq for SceneAnalysisEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::Eq for SceneAnalysisEffectDefinition {}
#[cfg(feature = "Media_Effects")]
impl ::core::fmt::Debug for SceneAnalysisEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows::core::RuntimeType for SceneAnalysisEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows::core::Interface for SceneAnalysisEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
#[cfg(feature = "Media_Effects")]
impl ::windows::core::RuntimeName for SceneAnalysisEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<SceneAnalysisEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: SceneAnalysisEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<&SceneAnalysisEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &SceneAnalysisEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<SceneAnalysisEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: SceneAnalysisEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<&SceneAnalysisEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &SceneAnalysisEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<SceneAnalysisEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneAnalysisEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&SceneAnalysisEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneAnalysisEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for &SceneAnalysisEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IVideoEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for SceneAnalysisEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for SceneAnalysisEffectDefinition {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(::windows::core::IUnknown);
impl SceneAnalysisEffectFrame {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsDiscontinuous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Capture'*"]
    #[cfg(feature = "Media_Capture")]
    pub fn FrameControlValues(&self) -> ::windows::core::Result<super::Capture::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Capture::CapturedFrameControlValues>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn HighDynamicRange(&self) -> ::windows::core::Result<HighDynamicRangeOutput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HighDynamicRangeOutput>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AnalysisRecommendation(&self) -> ::windows::core::Result<SceneAnalysisRecommendation> {
        let this = &::windows::core::Interface::cast::<ISceneAnalysisEffectFrame2>(self)?;
        unsafe {
            let mut result__: SceneAnalysisRecommendation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneAnalysisRecommendation>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneAnalysisEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneAnalysisEffectFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisEffectFrame {}
impl ::core::fmt::Debug for SceneAnalysisEffectFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffectFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneAnalysisEffectFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffectFrame;{d8b10e4c-7fd9-42e1-85eb-6572c297c987})");
}
unsafe impl ::windows::core::Interface for SceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8b10e4c_7fd9_42e1_85eb_6572c297c987);
}
impl ::windows::core::RuntimeName for SceneAnalysisEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectFrame";
}
impl ::core::convert::From<SceneAnalysisEffectFrame> for ::windows::core::IUnknown {
    fn from(value: SceneAnalysisEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffectFrame> for ::windows::core::IUnknown {
    fn from(value: &SceneAnalysisEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneAnalysisEffectFrame> for ::windows::core::IInspectable {
    fn from(value: SceneAnalysisEffectFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalysisEffectFrame> for ::windows::core::IInspectable {
    fn from(value: &SceneAnalysisEffectFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneAnalysisEffectFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneAnalysisEffectFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneAnalysisEffectFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneAnalysisEffectFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneAnalysisEffectFrame> for super::IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneAnalysisEffectFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneAnalysisEffectFrame> for super::IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneAnalysisEffectFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaFrame> for SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaFrame> for &SceneAnalysisEffectFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaFrame> {
        ::core::convert::TryInto::<super::IMediaFrame>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneAnalysisEffectFrame {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffectFrame {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAnalysisRecommendation {}
impl ::core::clone::Clone for SceneAnalysisRecommendation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SceneAnalysisRecommendation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SceneAnalysisRecommendation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisRecommendation {}
impl ::core::fmt::Debug for SceneAnalysisRecommendation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisRecommendation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneAnalysisRecommendation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.SceneAnalysisRecommendation;i4)");
}
impl ::windows::core::DefaultType for SceneAnalysisRecommendation {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(::windows::core::IUnknown);
impl SceneAnalyzedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ResultFrame(&self) -> ::windows::core::Result<SceneAnalysisEffectFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneAnalysisEffectFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneAnalyzedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneAnalyzedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalyzedEventArgs {}
impl ::core::fmt::Debug for SceneAnalyzedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalyzedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneAnalyzedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalyzedEventArgs;{146b9588-2851-45e4-ad55-44cf8df8db4d})");
}
unsafe impl ::windows::core::Interface for SceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146b9588_2851_45e4_ad55_44cf8df8db4d);
}
impl ::windows::core::RuntimeName for SceneAnalyzedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalyzedEventArgs";
}
impl ::core::convert::From<SceneAnalyzedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SceneAnalyzedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalyzedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SceneAnalyzedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneAnalyzedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SceneAnalyzedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneAnalyzedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SceneAnalyzedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SceneAnalyzedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SceneAnalyzedEventArgs {}
unsafe impl ::core::marker::Sync for SceneAnalyzedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct SpeechCue(::windows::core::IUnknown);
impl SpeechCue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechCue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPositionInInput(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartPositionInInput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EndPositionInInput(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndPositionInInput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SpeechCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechCue {}
impl ::core::fmt::Debug for SpeechCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechCue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechCue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SpeechCue;{aee254dc-1725-4bad-8043-a98499b017a2})");
}
unsafe impl ::windows::core::Interface for SpeechCue {
    type Vtable = ISpeechCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaee254dc_1725_4bad_8043_a98499b017a2);
}
impl ::windows::core::RuntimeName for SpeechCue {
    const NAME: &'static str = "Windows.Media.Core.SpeechCue";
}
impl ::core::convert::From<SpeechCue> for ::windows::core::IUnknown {
    fn from(value: SpeechCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechCue> for ::windows::core::IUnknown {
    fn from(value: &SpeechCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechCue> for ::windows::core::IInspectable {
    fn from(value: SpeechCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechCue> for ::windows::core::IInspectable {
    fn from(value: &SpeechCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechCue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechCue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for SpeechCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for &SpeechCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechCue {}
unsafe impl ::core::marker::Sync for SpeechCue {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: Self = Self(0i32);
    pub const Chapter: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Data: Self = Self(3i32);
    pub const Description: Self = Self(4i32);
    pub const Subtitle: Self = Self(5i32);
    pub const ImageSubtitle: Self = Self(6i32);
    pub const Speech: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedMetadataKind {}
impl ::core::clone::Clone for TimedMetadataKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedMetadataKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedMetadataKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataKind {}
impl ::core::fmt::Debug for TimedMetadataKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataKind;i4)");
}
impl ::windows::core::DefaultType for TimedMetadataKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedMetadataStreamDescriptor(::windows::core::IUnknown);
impl TimedMetadataStreamDescriptor {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::TimedMetadataEncodingProperties> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::TimedMetadataEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Copy(&self) -> ::windows::core::Result<TimedMetadataStreamDescriptor> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataStreamDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::TimedMetadataEncodingProperties>>(encodingproperties: Param0) -> ::windows::core::Result<TimedMetadataStreamDescriptor> {
        Self::ITimedMetadataStreamDescriptorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<TimedMetadataStreamDescriptor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataStreamDescriptorFactory<R, F: FnOnce(&ITimedMetadataStreamDescriptorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedMetadataStreamDescriptor, ITimedMetadataStreamDescriptorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedMetadataStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataStreamDescriptor {}
impl ::core::fmt::Debug for TimedMetadataStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataStreamDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataStreamDescriptor;{80f16e6e-92f7-451e-97d2-afd80742da70})");
}
unsafe impl ::windows::core::Interface for TimedMetadataStreamDescriptor {
    type Vtable = IMediaStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80f16e6e_92f7_451e_97d2_afd80742da70);
}
impl ::windows::core::RuntimeName for TimedMetadataStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataStreamDescriptor";
}
impl ::core::convert::From<TimedMetadataStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: TimedMetadataStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: &TimedMetadataStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: TimedMetadataStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: &TimedMetadataStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedMetadataStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: TimedMetadataStreamDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimedMetadataStreamDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for &TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TimedMetadataStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows::core::Error;
    fn try_from(value: TimedMetadataStreamDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimedMetadataStreamDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor2> for TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor2> for &TimedMetadataStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor2> {
        ::core::convert::TryInto::<IMediaStreamDescriptor2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedMetadataStreamDescriptor {}
unsafe impl ::core::marker::Sync for TimedMetadataStreamDescriptor {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedMetadataTrack(::windows::core::IUnknown);
impl TimedMetadataTrack {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind> {
        let this = &::windows::core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__: MediaTrackKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaTrackKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTrack>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CueEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCueEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CueExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCueExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TrackFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTrackFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Cues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IMediaCue>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActiveCues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<IMediaCue>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn TimedMetadataKind(&self) -> ::windows::core::Result<TimedMetadataKind> {
        let this = self;
        unsafe {
            let mut result__: TimedMetadataKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DispatchType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn AddCue<'a, Param0: ::windows::core::IntoParam<'a, IMediaCue>>(&self, cue: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), cue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn RemoveCue<'a, Param0: ::windows::core::IntoParam<'a, IMediaCue>>(&self, cue: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Playback'*"]
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0, language: Param1, kind: TimedMetadataKind) -> ::windows::core::Result<TimedMetadataTrack> {
        Self::ITimedMetadataTrackFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), language.into_param().abi(), kind, &mut result__).from_abi::<TimedMetadataTrack>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataTrackFactory<R, F: FnOnce(&ITimedMetadataTrackFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedMetadataTrack, ITimedMetadataTrackFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedMetadataTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrack {}
impl ::core::fmt::Debug for TimedMetadataTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrack;{9e6aed9e-f67a-49a9-b330-cf03b0e9cf07})");
}
unsafe impl ::windows::core::Interface for TimedMetadataTrack {
    type Vtable = ITimedMetadataTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6aed9e_f67a_49a9_b330_cf03b0e9cf07);
}
impl ::windows::core::RuntimeName for TimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrack";
}
impl ::core::convert::From<TimedMetadataTrack> for ::windows::core::IUnknown {
    fn from(value: TimedMetadataTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrack> for ::windows::core::IUnknown {
    fn from(value: &TimedMetadataTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedMetadataTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedMetadataTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataTrack> for ::windows::core::IInspectable {
    fn from(value: TimedMetadataTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrack> for ::windows::core::IInspectable {
    fn from(value: &TimedMetadataTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedMetadataTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedMetadataTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedMetadataTrack> for IMediaTrack {
    type Error = ::windows::core::Error;
    fn try_from(value: TimedMetadataTrack) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedMetadataTrack> for IMediaTrack {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimedMetadataTrack) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaTrack> for TimedMetadataTrack {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaTrack> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaTrack> for &TimedMetadataTrack {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaTrack> {
        ::core::convert::TryInto::<IMediaTrack>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedMetadataTrack {}
unsafe impl ::core::marker::Sync for TimedMetadataTrack {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedMetadataTrackError(::windows::core::IUnknown);
impl TimedMetadataTrackError {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<TimedMetadataTrackErrorCode> {
        let this = self;
        unsafe {
            let mut result__: TimedMetadataTrackErrorCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataTrackErrorCode>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedMetadataTrackError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrackError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackError {}
impl ::core::fmt::Debug for TimedMetadataTrackError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataTrackError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrackError;{b3767915-4114-4819-b9d9-dd76089e72f8})");
}
unsafe impl ::windows::core::Interface for TimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackErrorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3767915_4114_4819_b9d9_dd76089e72f8);
}
impl ::windows::core::RuntimeName for TimedMetadataTrackError {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackError";
}
impl ::core::convert::From<TimedMetadataTrackError> for ::windows::core::IUnknown {
    fn from(value: TimedMetadataTrackError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackError> for ::windows::core::IUnknown {
    fn from(value: &TimedMetadataTrackError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedMetadataTrackError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedMetadataTrackError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataTrackError> for ::windows::core::IInspectable {
    fn from(value: TimedMetadataTrackError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackError> for ::windows::core::IInspectable {
    fn from(value: &TimedMetadataTrackError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedMetadataTrackError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedMetadataTrackError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedMetadataTrackError {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackError {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackErrorCode {}
impl ::core::clone::Clone for TimedMetadataTrackErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedMetadataTrackErrorCode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedMetadataTrackErrorCode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackErrorCode {}
impl ::core::fmt::Debug for TimedMetadataTrackErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackErrorCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataTrackErrorCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataTrackErrorCode;i4)");
}
impl ::windows::core::DefaultType for TimedMetadataTrackErrorCode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(::windows::core::IUnknown);
impl TimedMetadataTrackFailedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Error(&self) -> ::windows::core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataTrackError>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedMetadataTrackFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrackFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackFailedEventArgs {}
impl ::core::fmt::Debug for TimedMetadataTrackFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataTrackFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrackFailedEventArgs;{a57fc9d1-6789-4d4d-b07f-84b4f31acb70})");
}
unsafe impl ::windows::core::Interface for TimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa57fc9d1_6789_4d4d_b07f_84b4f31acb70);
}
impl ::windows::core::RuntimeName for TimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackFailedEventArgs";
}
impl ::core::convert::From<TimedMetadataTrackFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TimedMetadataTrackFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TimedMetadataTrackFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedMetadataTrackFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TimedMetadataTrackFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedMetadataTrackFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TimedMetadataTrackFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedMetadataTrackFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedMetadataTrackFailedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackFailedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextBouten(::windows::core::IUnknown);
impl TimedTextBouten {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Type(&self) -> ::windows::core::Result<TimedTextBoutenType> {
        let this = self;
        unsafe {
            let mut result__: TimedTextBoutenType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextBoutenType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetType(&self, value: TimedTextBoutenType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Position(&self) -> ::windows::core::Result<TimedTextBoutenPosition> {
        let this = self;
        unsafe {
            let mut result__: TimedTextBoutenPosition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextBoutenPosition>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetPosition(&self, value: TimedTextBoutenPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextBouten {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextBouten {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextBouten {}
impl ::core::fmt::Debug for TimedTextBouten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBouten").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextBouten {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextBouten;{d9062783-5597-5092-820c-8f738e0f774a})");
}
unsafe impl ::windows::core::Interface for TimedTextBouten {
    type Vtable = ITimedTextBoutenVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9062783_5597_5092_820c_8f738e0f774a);
}
impl ::windows::core::RuntimeName for TimedTextBouten {
    const NAME: &'static str = "Windows.Media.Core.TimedTextBouten";
}
impl ::core::convert::From<TimedTextBouten> for ::windows::core::IUnknown {
    fn from(value: TimedTextBouten) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextBouten> for ::windows::core::IUnknown {
    fn from(value: &TimedTextBouten) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextBouten {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextBouten {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextBouten> for ::windows::core::IInspectable {
    fn from(value: TimedTextBouten) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextBouten> for ::windows::core::IInspectable {
    fn from(value: &TimedTextBouten) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextBouten {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextBouten {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextBouten {}
unsafe impl ::core::marker::Sync for TimedTextBouten {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextBoutenPosition {}
impl ::core::clone::Clone for TimedTextBoutenPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextBoutenPosition {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextBoutenPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextBoutenPosition {}
impl ::core::fmt::Debug for TimedTextBoutenPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextBoutenPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenPosition;i4)");
}
impl ::windows::core::DefaultType for TimedTextBoutenPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const FilledCircle: Self = Self(2i32);
    pub const OpenCircle: Self = Self(3i32);
    pub const FilledDot: Self = Self(4i32);
    pub const OpenDot: Self = Self(5i32);
    pub const FilledSesame: Self = Self(6i32);
    pub const OpenSesame: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedTextBoutenType {}
impl ::core::clone::Clone for TimedTextBoutenType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextBoutenType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextBoutenType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextBoutenType {}
impl ::core::fmt::Debug for TimedTextBoutenType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextBoutenType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenType;i4)");
}
impl ::windows::core::DefaultType for TimedTextBoutenType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextCue(::windows::core::IUnknown);
impl TimedTextCue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextCue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CueRegion(&self) -> ::windows::core::Result<TimedTextRegion> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextRegion>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetCueRegion<'a, Param0: ::windows::core::IntoParam<'a, TimedTextRegion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn CueStyle(&self) -> ::windows::core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextStyle>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetCueStyle<'a, Param0: ::windows::core::IntoParam<'a, TimedTextStyle>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextLine>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<TimedTextLine>>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedTextCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextCue {}
impl ::core::fmt::Debug for TimedTextCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextCue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextCue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextCue;{51c79e51-3b86-494d-b359-bb2ea7aca9a9})");
}
unsafe impl ::windows::core::Interface for TimedTextCue {
    type Vtable = ITimedTextCueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51c79e51_3b86_494d_b359_bb2ea7aca9a9);
}
impl ::windows::core::RuntimeName for TimedTextCue {
    const NAME: &'static str = "Windows.Media.Core.TimedTextCue";
}
impl ::core::convert::From<TimedTextCue> for ::windows::core::IUnknown {
    fn from(value: TimedTextCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextCue> for ::windows::core::IUnknown {
    fn from(value: &TimedTextCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextCue> for ::windows::core::IInspectable {
    fn from(value: TimedTextCue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextCue> for ::windows::core::IInspectable {
    fn from(value: &TimedTextCue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextCue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TimedTextCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: TimedTextCue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TimedTextCue> for IMediaCue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TimedTextCue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for TimedTextCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaCue> for &TimedTextCue {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaCue> {
        ::core::convert::TryInto::<IMediaCue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TimedTextCue {}
unsafe impl ::core::marker::Sync for TimedTextCue {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextDisplayAlignment {}
impl ::core::clone::Clone for TimedTextDisplayAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextDisplayAlignment {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextDisplayAlignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextDisplayAlignment {}
impl ::core::fmt::Debug for TimedTextDisplayAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextDisplayAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextDisplayAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextDisplayAlignment;i4)");
}
impl ::windows::core::DefaultType for TimedTextDisplayAlignment {
    type DefaultType = Self;
}
#[repr(C)]
#[doc = "*Required features: 'Media_Core'*"]
pub struct TimedTextDouble {
    pub Value: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextDouble {}
impl ::core::clone::Clone for TimedTextDouble {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextDouble").field("Value", &self.Value).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows::core::Abi for TimedTextDouble {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimedTextDouble {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextDouble;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::windows::core::DefaultType for TimedTextDouble {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for TimedTextDouble {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextDouble>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextDouble {}
impl ::core::default::Default for TimedTextDouble {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextFlowDirection {}
impl ::core::clone::Clone for TimedTextFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextFlowDirection {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextFlowDirection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextFlowDirection {}
impl ::core::fmt::Debug for TimedTextFlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFlowDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextFlowDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFlowDirection;i4)");
}
impl ::windows::core::DefaultType for TimedTextFlowDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextFontStyle {}
impl ::core::clone::Clone for TimedTextFontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextFontStyle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextFontStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextFontStyle {}
impl ::core::fmt::Debug for TimedTextFontStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFontStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextFontStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFontStyle;i4)");
}
impl ::windows::core::DefaultType for TimedTextFontStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextLine(::windows::core::IUnknown);
impl TimedTextLine {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextLine, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Subformats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextSubformat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<TimedTextSubformat>>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedTextLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextLine {}
impl ::core::fmt::Debug for TimedTextLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextLine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextLine;{978d7ce2-7308-4c66-be50-65777289f5df})");
}
unsafe impl ::windows::core::Interface for TimedTextLine {
    type Vtable = ITimedTextLineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x978d7ce2_7308_4c66_be50_65777289f5df);
}
impl ::windows::core::RuntimeName for TimedTextLine {
    const NAME: &'static str = "Windows.Media.Core.TimedTextLine";
}
impl ::core::convert::From<TimedTextLine> for ::windows::core::IUnknown {
    fn from(value: TimedTextLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextLine> for ::windows::core::IUnknown {
    fn from(value: &TimedTextLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextLine> for ::windows::core::IInspectable {
    fn from(value: TimedTextLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextLine> for ::windows::core::IInspectable {
    fn from(value: &TimedTextLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextLine {}
unsafe impl ::core::marker::Sync for TimedTextLine {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextLineAlignment {}
impl ::core::clone::Clone for TimedTextLineAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextLineAlignment {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextLineAlignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextLineAlignment {}
impl ::core::fmt::Debug for TimedTextLineAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLineAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextLineAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextLineAlignment;i4)");
}
impl ::windows::core::DefaultType for TimedTextLineAlignment {
    type DefaultType = Self;
}
#[repr(C)]
#[doc = "*Required features: 'Media_Core'*"]
pub struct TimedTextPadding {
    pub Before: f64,
    pub After: f64,
    pub Start: f64,
    pub End: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPadding {}
impl ::core::clone::Clone for TimedTextPadding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPadding").field("Before", &self.Before).field("After", &self.After).field("Start", &self.Start).field("End", &self.End).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows::core::Abi for TimedTextPadding {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimedTextPadding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPadding;f8;f8;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::windows::core::DefaultType for TimedTextPadding {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for TimedTextPadding {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextPadding>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextPadding {}
impl ::core::default::Default for TimedTextPadding {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Media_Core'*"]
pub struct TimedTextPoint {
    pub X: f64,
    pub Y: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPoint {}
impl ::core::clone::Clone for TimedTextPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPoint").field("X", &self.X).field("Y", &self.Y).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows::core::Abi for TimedTextPoint {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimedTextPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPoint;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::windows::core::DefaultType for TimedTextPoint {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for TimedTextPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextPoint {}
impl ::core::default::Default for TimedTextPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextRegion(::windows::core::IUnknown);
impl TimedTextRegion {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextRegion, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Position(&self) -> ::windows::core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__: TimedTextPoint = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextPoint>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, TimedTextPoint>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Extent(&self) -> ::windows::core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__: TimedTextSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextSize>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetExtent<'a, Param0: ::windows::core::IntoParam<'a, TimedTextSize>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Background(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn WritingMode(&self) -> ::windows::core::Result<TimedTextWritingMode> {
        let this = self;
        unsafe {
            let mut result__: TimedTextWritingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextWritingMode>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetWritingMode(&self, value: TimedTextWritingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DisplayAlignment(&self) -> ::windows::core::Result<TimedTextDisplayAlignment> {
        let this = self;
        unsafe {
            let mut result__: TimedTextDisplayAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextDisplayAlignment>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetDisplayAlignment(&self, value: TimedTextDisplayAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__: TimedTextDouble = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextDouble>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLineHeight<'a, Param0: ::windows::core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsOverflowClipped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsOverflowClipped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Padding(&self) -> ::windows::core::Result<TimedTextPadding> {
        let this = self;
        unsafe {
            let mut result__: TimedTextPadding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextPadding>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetPadding<'a, Param0: ::windows::core::IntoParam<'a, TimedTextPadding>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn TextWrapping(&self) -> ::windows::core::Result<TimedTextWrapping> {
        let this = self;
        unsafe {
            let mut result__: TimedTextWrapping = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextWrapping>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetTextWrapping(&self, value: TimedTextWrapping) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ZIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ScrollMode(&self) -> ::windows::core::Result<TimedTextScrollMode> {
        let this = self;
        unsafe {
            let mut result__: TimedTextScrollMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextScrollMode>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetScrollMode(&self, value: TimedTextScrollMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRegion {}
impl ::core::fmt::Debug for TimedTextRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRegion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextRegion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextRegion;{1ed0881f-8a06-4222-9f59-b21bf40124b4})");
}
unsafe impl ::windows::core::Interface for TimedTextRegion {
    type Vtable = ITimedTextRegionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ed0881f_8a06_4222_9f59_b21bf40124b4);
}
impl ::windows::core::RuntimeName for TimedTextRegion {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRegion";
}
impl ::core::convert::From<TimedTextRegion> for ::windows::core::IUnknown {
    fn from(value: TimedTextRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRegion> for ::windows::core::IUnknown {
    fn from(value: &TimedTextRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextRegion> for ::windows::core::IInspectable {
    fn from(value: TimedTextRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRegion> for ::windows::core::IInspectable {
    fn from(value: &TimedTextRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextRegion {}
unsafe impl ::core::marker::Sync for TimedTextRegion {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextRuby(::windows::core::IUnknown);
impl TimedTextRuby {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Position(&self) -> ::windows::core::Result<TimedTextRubyPosition> {
        let this = self;
        unsafe {
            let mut result__: TimedTextRubyPosition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextRubyPosition>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetPosition(&self, value: TimedTextRubyPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Align(&self) -> ::windows::core::Result<TimedTextRubyAlign> {
        let this = self;
        unsafe {
            let mut result__: TimedTextRubyAlign = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextRubyAlign>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetAlign(&self, value: TimedTextRubyAlign) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Reserve(&self) -> ::windows::core::Result<TimedTextRubyReserve> {
        let this = self;
        unsafe {
            let mut result__: TimedTextRubyReserve = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextRubyReserve>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetReserve(&self, value: TimedTextRubyReserve) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextRuby {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextRuby {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRuby {}
impl ::core::fmt::Debug for TimedTextRuby {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRuby").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextRuby {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextRuby;{10335c29-5b3c-5693-9959-d05a0bd24628})");
}
unsafe impl ::windows::core::Interface for TimedTextRuby {
    type Vtable = ITimedTextRubyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10335c29_5b3c_5693_9959_d05a0bd24628);
}
impl ::windows::core::RuntimeName for TimedTextRuby {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRuby";
}
impl ::core::convert::From<TimedTextRuby> for ::windows::core::IUnknown {
    fn from(value: TimedTextRuby) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRuby> for ::windows::core::IUnknown {
    fn from(value: &TimedTextRuby) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextRuby {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextRuby {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextRuby> for ::windows::core::IInspectable {
    fn from(value: TimedTextRuby) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextRuby> for ::windows::core::IInspectable {
    fn from(value: &TimedTextRuby) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextRuby {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextRuby {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextRuby {}
unsafe impl ::core::marker::Sync for TimedTextRuby {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
    pub const SpaceAround: Self = Self(3i32);
    pub const SpaceBetween: Self = Self(4i32);
    pub const WithBase: Self = Self(5i32);
}
impl ::core::marker::Copy for TimedTextRubyAlign {}
impl ::core::clone::Clone for TimedTextRubyAlign {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextRubyAlign {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextRubyAlign {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRubyAlign {}
impl ::core::fmt::Debug for TimedTextRubyAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyAlign").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextRubyAlign {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyAlign;i4)");
}
impl ::windows::core::DefaultType for TimedTextRubyAlign {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextRubyPosition {}
impl ::core::clone::Clone for TimedTextRubyPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextRubyPosition {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextRubyPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRubyPosition {}
impl ::core::fmt::Debug for TimedTextRubyPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextRubyPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyPosition;i4)");
}
impl ::windows::core::DefaultType for TimedTextRubyPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: Self = Self(0i32);
    pub const Before: Self = Self(1i32);
    pub const After: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const Outside: Self = Self(4i32);
}
impl ::core::marker::Copy for TimedTextRubyReserve {}
impl ::core::clone::Clone for TimedTextRubyReserve {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextRubyReserve {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextRubyReserve {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRubyReserve {}
impl ::core::fmt::Debug for TimedTextRubyReserve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyReserve").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextRubyReserve {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyReserve;i4)");
}
impl ::windows::core::DefaultType for TimedTextRubyReserve {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextScrollMode {}
impl ::core::clone::Clone for TimedTextScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextScrollMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextScrollMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextScrollMode {}
impl ::core::fmt::Debug for TimedTextScrollMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextScrollMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextScrollMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextScrollMode;i4)");
}
impl ::windows::core::DefaultType for TimedTextScrollMode {
    type DefaultType = Self;
}
#[repr(C)]
#[doc = "*Required features: 'Media_Core'*"]
pub struct TimedTextSize {
    pub Height: f64,
    pub Width: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextSize {}
impl ::core::clone::Clone for TimedTextSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextSize").field("Height", &self.Height).field("Width", &self.Width).field("Unit", &self.Unit).finish()
    }
}
unsafe impl ::windows::core::Abi for TimedTextSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimedTextSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextSize;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::windows::core::DefaultType for TimedTextSize {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for TimedTextSize {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TimedTextSize>()) == 0 }
    }
}
impl ::core::cmp::Eq for TimedTextSize {}
impl ::core::default::Default for TimedTextSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextSource(::windows::core::IUnknown);
impl TimedTextSource {
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Resolved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResolved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(stream: Param0, defaultlanguage: Param1) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stream.into_param().abi(), defaultlanguage.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0, defaultlanguage: Param1) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), defaultlanguage.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndex<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(stream: Param0, indexstream: Param1) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), stream.into_param().abi(), indexstream.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriWithIndex<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0, indexuri: Param1) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), indexuri.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndexAndLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(stream: Param0, indexstream: Param1, defaultlanguage: Param2) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stream.into_param().abi(), indexstream.into_param().abi(), defaultlanguage.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriWithIndexAndLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0, indexuri: Param1, defaultlanguage: Param2) -> ::windows::core::Result<TimedTextSource> {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), indexuri.into_param().abi(), defaultlanguage.into_param().abi(), &mut result__).from_abi::<TimedTextSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedTextSourceStatics<R, F: FnOnce(&ITimedTextSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextSource, ITimedTextSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITimedTextSourceStatics2<R, F: FnOnce(&ITimedTextSourceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextSource, ITimedTextSourceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TimedTextSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSource {}
impl ::core::fmt::Debug for TimedTextSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSource;{c4ed9ba6-101f-404d-a949-82f33fcd93b7})");
}
unsafe impl ::windows::core::Interface for TimedTextSource {
    type Vtable = ITimedTextSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4ed9ba6_101f_404d_a949_82f33fcd93b7);
}
impl ::windows::core::RuntimeName for TimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSource";
}
impl ::core::convert::From<TimedTextSource> for ::windows::core::IUnknown {
    fn from(value: TimedTextSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSource> for ::windows::core::IUnknown {
    fn from(value: &TimedTextSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextSource> for ::windows::core::IInspectable {
    fn from(value: TimedTextSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSource> for ::windows::core::IInspectable {
    fn from(value: &TimedTextSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextSource {}
unsafe impl ::core::marker::Sync for TimedTextSource {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextSourceResolveResultEventArgs(::windows::core::IUnknown);
impl TimedTextSourceResolveResultEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Error(&self) -> ::windows::core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataTrackError>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>(result__)
        }
    }
}
impl ::core::clone::Clone for TimedTextSourceResolveResultEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextSourceResolveResultEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSourceResolveResultEventArgs {}
impl ::core::fmt::Debug for TimedTextSourceResolveResultEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSourceResolveResultEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextSourceResolveResultEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSourceResolveResultEventArgs;{48907c9c-dcd8-4c33-9ad3-6cdce7b1c566})");
}
unsafe impl ::windows::core::Interface for TimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48907c9c_dcd8_4c33_9ad3_6cdce7b1c566);
}
impl ::windows::core::RuntimeName for TimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSourceResolveResultEventArgs";
}
impl ::core::convert::From<TimedTextSourceResolveResultEventArgs> for ::windows::core::IUnknown {
    fn from(value: TimedTextSourceResolveResultEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSourceResolveResultEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TimedTextSourceResolveResultEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextSourceResolveResultEventArgs> for ::windows::core::IInspectable {
    fn from(value: TimedTextSourceResolveResultEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSourceResolveResultEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TimedTextSourceResolveResultEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextSourceResolveResultEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextSourceResolveResultEventArgs {}
unsafe impl ::core::marker::Sync for TimedTextSourceResolveResultEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextStyle(::windows::core::IUnknown);
impl TimedTextStyle {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextStyle, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetFontFamily<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn FontSize(&self) -> ::windows::core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__: TimedTextDouble = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextDouble>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetFontSize<'a, Param0: ::windows::core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn FontWeight(&self) -> ::windows::core::Result<TimedTextWeight> {
        let this = self;
        unsafe {
            let mut result__: TimedTextWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextWeight>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetFontWeight(&self, value: TimedTextWeight) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetForeground<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn Background(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsBackgroundAlwaysShown(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsBackgroundAlwaysShown(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn FlowDirection(&self) -> ::windows::core::Result<TimedTextFlowDirection> {
        let this = self;
        unsafe {
            let mut result__: TimedTextFlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextFlowDirection>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetFlowDirection(&self, value: TimedTextFlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn LineAlignment(&self) -> ::windows::core::Result<TimedTextLineAlignment> {
        let this = self;
        unsafe {
            let mut result__: TimedTextLineAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextLineAlignment>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLineAlignment(&self, value: TimedTextLineAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn OutlineColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn SetOutlineColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn OutlineThickness(&self) -> ::windows::core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__: TimedTextDouble = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextDouble>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetOutlineThickness<'a, Param0: ::windows::core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn OutlineRadius(&self) -> ::windows::core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__: TimedTextDouble = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextDouble>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetOutlineRadius<'a, Param0: ::windows::core::IntoParam<'a, TimedTextDouble>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<TimedTextFontStyle> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__: TimedTextFontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextFontStyle>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetFontStyle(&self, value: TimedTextFontStyle) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsUnderlineEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsUnderlineEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsLineThroughEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsLineThroughEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsOverlineEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsOverlineEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Ruby(&self) -> ::windows::core::Result<TimedTextRuby> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextRuby>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Bouten(&self) -> ::windows::core::Result<TimedTextBouten> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextBouten>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsTextCombined(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetIsTextCombined(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn FontAngleInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetFontAngleInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for TimedTextStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextStyle {}
impl ::core::fmt::Debug for TimedTextStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextStyle;{1bb2384d-a825-40c2-a7f5-281eaedf3b55})");
}
unsafe impl ::windows::core::Interface for TimedTextStyle {
    type Vtable = ITimedTextStyleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb2384d_a825_40c2_a7f5_281eaedf3b55);
}
impl ::windows::core::RuntimeName for TimedTextStyle {
    const NAME: &'static str = "Windows.Media.Core.TimedTextStyle";
}
impl ::core::convert::From<TimedTextStyle> for ::windows::core::IUnknown {
    fn from(value: TimedTextStyle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextStyle> for ::windows::core::IUnknown {
    fn from(value: &TimedTextStyle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextStyle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextStyle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextStyle> for ::windows::core::IInspectable {
    fn from(value: TimedTextStyle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextStyle> for ::windows::core::IInspectable {
    fn from(value: &TimedTextStyle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextStyle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextStyle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextStyle {}
unsafe impl ::core::marker::Sync for TimedTextStyle {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextSubformat(::windows::core::IUnknown);
impl TimedTextSubformat {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TimedTextSubformat, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn StartIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetStartIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLength(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SubformatStyle(&self) -> ::windows::core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimedTextStyle>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetSubformatStyle<'a, Param0: ::windows::core::IntoParam<'a, TimedTextStyle>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for TimedTextSubformat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedTextSubformat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSubformat {}
impl ::core::fmt::Debug for TimedTextSubformat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSubformat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextSubformat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSubformat;{d713502f-3261-4722-a0c2-b937b2390f14})");
}
unsafe impl ::windows::core::Interface for TimedTextSubformat {
    type Vtable = ITimedTextSubformatVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd713502f_3261_4722_a0c2_b937b2390f14);
}
impl ::windows::core::RuntimeName for TimedTextSubformat {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSubformat";
}
impl ::core::convert::From<TimedTextSubformat> for ::windows::core::IUnknown {
    fn from(value: TimedTextSubformat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSubformat> for ::windows::core::IUnknown {
    fn from(value: &TimedTextSubformat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TimedTextSubformat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TimedTextSubformat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TimedTextSubformat> for ::windows::core::IInspectable {
    fn from(value: TimedTextSubformat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TimedTextSubformat> for ::windows::core::IInspectable {
    fn from(value: &TimedTextSubformat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TimedTextSubformat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TimedTextSubformat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TimedTextSubformat {}
unsafe impl ::core::marker::Sync for TimedTextSubformat {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextUnit {}
impl ::core::clone::Clone for TimedTextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextUnit {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextUnit {}
impl ::core::fmt::Debug for TimedTextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextUnit;i4)");
}
impl ::windows::core::DefaultType for TimedTextUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
impl ::core::marker::Copy for TimedTextWeight {}
impl ::core::clone::Clone for TimedTextWeight {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextWeight {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextWeight {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextWeight {}
impl ::core::fmt::Debug for TimedTextWeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWeight").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextWeight {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWeight;i4)");
}
impl ::windows::core::DefaultType for TimedTextWeight {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextWrapping {}
impl ::core::clone::Clone for TimedTextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextWrapping {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextWrapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextWrapping {}
impl ::core::fmt::Debug for TimedTextWrapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWrapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextWrapping {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWrapping;i4)");
}
impl ::windows::core::DefaultType for TimedTextWrapping {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: Self = Self(0i32);
    pub const RightLeftTopBottom: Self = Self(1i32);
    pub const TopBottomRightLeft: Self = Self(2i32);
    pub const TopBottomLeftRight: Self = Self(3i32);
    pub const LeftRight: Self = Self(4i32);
    pub const RightLeft: Self = Self(5i32);
    pub const TopBottom: Self = Self(6i32);
}
impl ::core::marker::Copy for TimedTextWritingMode {}
impl ::core::clone::Clone for TimedTextWritingMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TimedTextWritingMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TimedTextWritingMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextWritingMode {}
impl ::core::fmt::Debug for TimedTextWritingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWritingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedTextWritingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWritingMode;i4)");
}
impl ::windows::core::DefaultType for TimedTextWritingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoStabilizationEffect(::windows::core::IUnknown);
impl VideoStabilizationEffect {
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Capture', 'Media_Devices', 'Media_MediaProperties'*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))]
    pub fn GetRecommendedStreamConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::Devices::VideoDeviceController>, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>>(&self, controller: Param0, desiredproperties: Param1) -> ::windows::core::Result<super::Capture::VideoStreamConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), controller.into_param().abi(), desiredproperties.into_param().abi(), &mut result__).from_abi::<super::Capture::VideoStreamConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoStabilizationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStabilizationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffect {}
impl ::core::fmt::Debug for VideoStabilizationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoStabilizationEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffect;{0808a650-9698-4e57-877b-bd7cb2ee0f8a})");
}
unsafe impl ::windows::core::Interface for VideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0808a650_9698_4e57_877b_bd7cb2ee0f8a);
}
impl ::windows::core::RuntimeName for VideoStabilizationEffect {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffect";
}
impl ::core::convert::From<VideoStabilizationEffect> for ::windows::core::IUnknown {
    fn from(value: VideoStabilizationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffect> for ::windows::core::IUnknown {
    fn from(value: &VideoStabilizationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoStabilizationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoStabilizationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStabilizationEffect> for ::windows::core::IInspectable {
    fn from(value: VideoStabilizationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffect> for ::windows::core::IInspectable {
    fn from(value: &VideoStabilizationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoStabilizationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoStabilizationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoStabilizationEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoStabilizationEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoStabilizationEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoStabilizationEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for VideoStabilizationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for &VideoStabilizationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoStabilizationEffect {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffect {}
#[doc = "*Required features: 'Media_Core', 'Media_Effects'*"]
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
pub struct VideoStabilizationEffectDefinition(::windows::core::IUnknown);
#[cfg(feature = "Media_Effects")]
impl VideoStabilizationEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoStabilizationEffectDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Effects'*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation_Collections', 'Media_Effects'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::clone::Clone for VideoStabilizationEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::PartialEq for VideoStabilizationEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::Eq for VideoStabilizationEffectDefinition {}
#[cfg(feature = "Media_Effects")]
impl ::core::fmt::Debug for VideoStabilizationEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows::core::RuntimeType for VideoStabilizationEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows::core::Interface for VideoStabilizationEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
#[cfg(feature = "Media_Effects")]
impl ::windows::core::RuntimeName for VideoStabilizationEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<VideoStabilizationEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: VideoStabilizationEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<&VideoStabilizationEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &VideoStabilizationEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<VideoStabilizationEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: VideoStabilizationEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::From<&VideoStabilizationEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &VideoStabilizationEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<VideoStabilizationEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoStabilizationEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&VideoStabilizationEffectDefinition> for super::Effects::IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoStabilizationEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition> for &VideoStabilizationEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IVideoEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IVideoEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for VideoStabilizationEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for VideoStabilizationEffectDefinition {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(::windows::core::IUnknown);
impl VideoStabilizationEffectEnabledChangedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Reason(&self) -> ::windows::core::Result<VideoStabilizationEffectEnabledChangedReason> {
        let this = self;
        unsafe {
            let mut result__: VideoStabilizationEffectEnabledChangedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoStabilizationEffectEnabledChangedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStabilizationEffectEnabledChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffectEnabledChangedEventArgs {}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoStabilizationEffectEnabledChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs;{187eff28-67bb-4713-b900-4168da164529})");
}
unsafe impl ::windows::core::Interface for VideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187eff28_67bb_4713_b900_4168da164529);
}
impl ::windows::core::RuntimeName for VideoStabilizationEffectEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs";
}
impl ::core::convert::From<VideoStabilizationEffectEnabledChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffectEnabledChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStabilizationEffectEnabledChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStabilizationEffectEnabledChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VideoStabilizationEffectEnabledChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoStabilizationEffectEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoStabilizationEffectEnabledChangedEventArgs {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffectEnabledChangedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoStabilizationEffectEnabledChangedReason {}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VideoStabilizationEffectEnabledChangedReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VideoStabilizationEffectEnabledChangedReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffectEnabledChangedReason {}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoStabilizationEffectEnabledChangedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Core.VideoStabilizationEffectEnabledChangedReason;i4)");
}
impl ::windows::core::DefaultType for VideoStabilizationEffectEnabledChangedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoStreamDescriptor(::windows::core::IUnknown);
impl VideoStreamDescriptor {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Copy(&self) -> ::windows::core::Result<VideoStreamDescriptor> {
        let this = &::windows::core::Interface::cast::<IVideoStreamDescriptor2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoStreamDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>>(encodingproperties: Param0) -> ::windows::core::Result<VideoStreamDescriptor> {
        Self::IVideoStreamDescriptorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<VideoStreamDescriptor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoStreamDescriptorFactory<R, F: FnOnce(&IVideoStreamDescriptorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoStreamDescriptor, IVideoStreamDescriptorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStreamDescriptor {}
impl ::core::fmt::Debug for VideoStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStreamDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoStreamDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStreamDescriptor;{12ee0d55-9c2b-4440-8057-2c7a90f0cbec})");
}
unsafe impl ::windows::core::Interface for VideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12ee0d55_9c2b_4440_8057_2c7a90f0cbec);
}
impl ::windows::core::RuntimeName for VideoStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.VideoStreamDescriptor";
}
impl ::core::convert::From<VideoStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: VideoStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamDescriptor> for ::windows::core::IUnknown {
    fn from(value: &VideoStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: VideoStreamDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamDescriptor> for ::windows::core::IInspectable {
    fn from(value: &VideoStreamDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoStreamDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoStreamDescriptor> for IMediaStreamDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoStreamDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor> for &VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor> {
        ::core::convert::TryInto::<IMediaStreamDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VideoStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoStreamDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoStreamDescriptor> for IMediaStreamDescriptor2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoStreamDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor2> for VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaStreamDescriptor2> for &VideoStreamDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaStreamDescriptor2> {
        ::core::convert::TryInto::<IMediaStreamDescriptor2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoStreamDescriptor {}
unsafe impl ::core::marker::Sync for VideoStreamDescriptor {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoTrack(::windows::core::IUnknown);
impl VideoTrack {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__: MediaTrackKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaTrackKind>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpenFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTrack>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core', 'Media_Playback'*"]
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows::core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn SupportInfo(&self) -> ::windows::core::Result<VideoTrackSupportInfo> {
        let this = &::windows::core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoTrackSupportInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrack {}
impl ::core::fmt::Debug for VideoTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})");
}
unsafe impl ::windows::core::Interface for VideoTrack {
    type Vtable = IMediaTrackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
}
impl ::windows::core::RuntimeName for VideoTrack {
    const NAME: &'static str = "Windows.Media.Core.VideoTrack";
}
impl ::core::convert::From<VideoTrack> for ::windows::core::IUnknown {
    fn from(value: VideoTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrack> for ::windows::core::IUnknown {
    fn from(value: &VideoTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTrack> for ::windows::core::IInspectable {
    fn from(value: VideoTrack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrack> for ::windows::core::IInspectable {
    fn from(value: &VideoTrack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VideoTrack> for IMediaTrack {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoTrack) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoTrack> for IMediaTrack {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoTrack) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaTrack> for VideoTrack {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaTrack> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaTrack> for &VideoTrack {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaTrack> {
        ::core::convert::TryInto::<IMediaTrack>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoTrack {}
unsafe impl ::core::marker::Sync for VideoTrack {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(::windows::core::IUnknown);
impl VideoTrackOpenFailedEventArgs {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTrackOpenFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrackOpenFailedEventArgs {}
impl ::core::fmt::Debug for VideoTrackOpenFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrackOpenFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrackOpenFailedEventArgs;{7679e231-04f9-4c82-a4ee-8602c8bb4754})");
}
unsafe impl ::windows::core::Interface for VideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7679e231_04f9_4c82_a4ee_8602c8bb4754);
}
impl ::windows::core::RuntimeName for VideoTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackOpenFailedEventArgs";
}
impl ::core::convert::From<VideoTrackOpenFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VideoTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackOpenFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VideoTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTrackOpenFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VideoTrackOpenFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackOpenFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VideoTrackOpenFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoTrackOpenFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for VideoTrackOpenFailedEventArgs {}
#[doc = "*Required features: 'Media_Core'*"]
#[repr(transparent)]
pub struct VideoTrackSupportInfo(::windows::core::IUnknown);
impl VideoTrackSupportInfo {
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn DecoderStatus(&self) -> ::windows::core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaDecoderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaDecoderStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Core'*"]
    pub fn MediaSourceStatus(&self) -> ::windows::core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VideoTrackSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrackSupportInfo {}
impl ::core::fmt::Debug for VideoTrackSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrackSupportInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTrackSupportInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrackSupportInfo;{4bb534a0-fc5f-450d-8ff0-778d590486de})");
}
unsafe impl ::windows::core::Interface for VideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bb534a0_fc5f_450d_8ff0_778d590486de);
}
impl ::windows::core::RuntimeName for VideoTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackSupportInfo";
}
impl ::core::convert::From<VideoTrackSupportInfo> for ::windows::core::IUnknown {
    fn from(value: VideoTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackSupportInfo> for ::windows::core::IUnknown {
    fn from(value: &VideoTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VideoTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoTrackSupportInfo> for ::windows::core::IInspectable {
    fn from(value: VideoTrackSupportInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTrackSupportInfo> for ::windows::core::IInspectable {
    fn from(value: &VideoTrackSupportInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VideoTrackSupportInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoTrackSupportInfo {}
unsafe impl ::core::marker::Sync for VideoTrackSupportInfo {}