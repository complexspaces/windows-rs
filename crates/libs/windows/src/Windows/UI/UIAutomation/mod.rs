#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[doc = "*Required features: 'UI_UIAutomation'*"]
#[repr(transparent)]
pub struct AutomationConnection(::windows::core::IUnknown);
impl AutomationConnection {
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn IsRemoteSystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationConnection {}
impl ::core::fmt::Debug for AutomationConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnection;{aad262ed-0ef4-5d43-97be-a834e27b65b9})");
}
unsafe impl ::windows::core::Interface for AutomationConnection {
    type Vtable = IAutomationConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaad262ed_0ef4_5d43_97be_a834e27b65b9);
}
impl ::windows::core::RuntimeName for AutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnection";
}
impl ::core::convert::From<AutomationConnection> for ::windows::core::IUnknown {
    fn from(value: AutomationConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnection> for ::windows::core::IUnknown {
    fn from(value: &AutomationConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationConnection> for ::windows::core::IInspectable {
    fn from(value: AutomationConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnection> for ::windows::core::IInspectable {
    fn from(value: &AutomationConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationConnection {}
unsafe impl ::core::marker::Sync for AutomationConnection {}
#[doc = "*Required features: 'UI_UIAutomation'*"]
#[repr(transparent)]
pub struct AutomationConnectionBoundObject(::windows::core::IUnknown);
impl AutomationConnectionBoundObject {
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn Connection(&self) -> ::windows::core::Result<AutomationConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AutomationConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationConnectionBoundObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationConnectionBoundObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationConnectionBoundObject {}
impl ::core::fmt::Debug for AutomationConnectionBoundObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationConnectionBoundObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationConnectionBoundObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnectionBoundObject;{5e8558fb-ca52-5b65-9830-dd2905816093})");
}
unsafe impl ::windows::core::Interface for AutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e8558fb_ca52_5b65_9830_dd2905816093);
}
impl ::windows::core::RuntimeName for AutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnectionBoundObject";
}
impl ::core::convert::From<AutomationConnectionBoundObject> for ::windows::core::IUnknown {
    fn from(value: AutomationConnectionBoundObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnectionBoundObject> for ::windows::core::IUnknown {
    fn from(value: &AutomationConnectionBoundObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationConnectionBoundObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationConnectionBoundObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationConnectionBoundObject> for ::windows::core::IInspectable {
    fn from(value: AutomationConnectionBoundObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnectionBoundObject> for ::windows::core::IInspectable {
    fn from(value: &AutomationConnectionBoundObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationConnectionBoundObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationConnectionBoundObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationConnectionBoundObject {}
unsafe impl ::core::marker::Sync for AutomationConnectionBoundObject {}
#[doc = "*Required features: 'UI_UIAutomation'*"]
#[repr(transparent)]
pub struct AutomationElement(::windows::core::IUnknown);
impl AutomationElement {
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn IsRemoteSystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_UIAutomation'*"]
    pub fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationElement {}
impl ::core::fmt::Debug for AutomationElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationElement;{a1898370-2c07-56fd-993f-61a72a08058c})");
}
unsafe impl ::windows::core::Interface for AutomationElement {
    type Vtable = IAutomationElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1898370_2c07_56fd_993f_61a72a08058c);
}
impl ::windows::core::RuntimeName for AutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationElement";
}
impl ::core::convert::From<AutomationElement> for ::windows::core::IUnknown {
    fn from(value: AutomationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElement> for ::windows::core::IUnknown {
    fn from(value: &AutomationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationElement> for ::windows::core::IInspectable {
    fn from(value: AutomationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElement> for ::windows::core::IInspectable {
    fn from(value: &AutomationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationElement {}
unsafe impl ::core::marker::Sync for AutomationElement {}
#[doc = "*Required features: 'UI_UIAutomation'*"]
#[repr(transparent)]
pub struct AutomationTextRange(::windows::core::IUnknown);
impl AutomationTextRange {}
impl ::core::clone::Clone for AutomationTextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationTextRange {}
impl ::core::fmt::Debug for AutomationTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationTextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationTextRange;{7e101b65-40d3-5994-85a9-0a0cb9a4ec98})");
}
unsafe impl ::windows::core::Interface for AutomationTextRange {
    type Vtable = IAutomationTextRangeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e101b65_40d3_5994_85a9_0a0cb9a4ec98);
}
impl ::windows::core::RuntimeName for AutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationTextRange";
}
impl ::core::convert::From<AutomationTextRange> for ::windows::core::IUnknown {
    fn from(value: AutomationTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationTextRange> for ::windows::core::IUnknown {
    fn from(value: &AutomationTextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AutomationTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationTextRange> for ::windows::core::IInspectable {
    fn from(value: AutomationTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationTextRange> for ::windows::core::IInspectable {
    fn from(value: &AutomationTextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AutomationTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationTextRange {}
unsafe impl ::core::marker::Sync for AutomationTextRange {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationConnection {
    type Vtable = IAutomationConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaad262ed_0ef4_5d43_97be_a834e27b65b9);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionImpl {
    fn IsRemoteSystem();
    fn AppUserModelId();
    fn ExecutableFileName();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationConnectionBoundObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e8558fb_ca52_5b65_9830_dd2905816093);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionBoundObjectImpl {
    fn Connection();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnectionBoundObjectVtbl(
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
pub struct IAutomationElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElement {
    type Vtable = IAutomationElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1898370_2c07_56fd_993f_61a72a08058c);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementImpl {
    fn IsRemoteSystem();
    fn AppUserModelId();
    fn ExecutableFileName();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationTextRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationTextRange {
    type Vtable = IAutomationTextRangeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e101b65_40d3_5994_85a9_0a0cb9a4ec98);
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationTextRangeImpl {}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationTextRangeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);