#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactDataProviderConnection(::windows::core::IUnknown);
impl ContactDataProviderConnection {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerSearchReadBatchRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerSearchReadBatchRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateOrUpdateContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateOrUpdateContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeleteContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ContactDataProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactDataProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactDataProviderConnection {}
impl ::core::fmt::Debug for ContactDataProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDataProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection;{1a398a52-8c9d-4d6f-a4e0-111e9a125a30})");
}
unsafe impl ::windows::core::Interface for ContactDataProviderConnection {
    type Vtable = IContactDataProviderConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a398a52_8c9d_4d6f_a4e0_111e9a125a30);
}
impl ::windows::core::RuntimeName for ContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
}
impl ::core::convert::From<ContactDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: ContactDataProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &ContactDataProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: ContactDataProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &ContactDataProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactDataProviderConnection {}
unsafe impl ::core::marker::Sync for ContactDataProviderConnection {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactDataProviderTriggerDetails(::windows::core::IUnknown);
impl ContactDataProviderTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Connection(&self) -> ::windows::core::Result<ContactDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactDataProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactDataProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactDataProviderTriggerDetails {}
impl ::core::fmt::Debug for ContactDataProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactDataProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails;{527104be-3c62-43c8-9ae7-db531685cd99})");
}
unsafe impl ::windows::core::Interface for ContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x527104be_3c62_43c8_9ae7_db531685cd99);
}
impl ::windows::core::RuntimeName for ContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
}
impl ::core::convert::From<ContactDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ContactDataProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ContactDataProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ContactDataProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ContactDataProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for ContactDataProviderTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequest(::windows::core::IUnknown);
impl ContactListCreateOrUpdateContactRequest {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contact>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Contact>>(&self, createdorupdatedcontact: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), createdorupdatedcontact.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListCreateOrUpdateContactRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListCreateOrUpdateContactRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListCreateOrUpdateContactRequest {}
impl ::core::fmt::Debug for ContactListCreateOrUpdateContactRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListCreateOrUpdateContactRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListCreateOrUpdateContactRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest;{b4af411f-c849-47d0-b119-91cf605b2f2a})");
}
unsafe impl ::windows::core::Interface for ContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4af411f_c849_47d0_b119_91cf605b2f2a);
}
impl ::windows::core::RuntimeName for ContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListCreateOrUpdateContactRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListCreateOrUpdateContactRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListCreateOrUpdateContactRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListCreateOrUpdateContactRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListCreateOrUpdateContactRequest {}
unsafe impl ::core::marker::Sync for ContactListCreateOrUpdateContactRequest {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(::windows::core::IUnknown);
impl ContactListCreateOrUpdateContactRequestEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Request(&self) -> ::windows::core::Result<ContactListCreateOrUpdateContactRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListCreateOrUpdateContactRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListCreateOrUpdateContactRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListCreateOrUpdateContactRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListCreateOrUpdateContactRequestEventArgs {}
impl ::core::fmt::Debug for ContactListCreateOrUpdateContactRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListCreateOrUpdateContactRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListCreateOrUpdateContactRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs;{851c1690-1a51-4b0c-aeef-1240ac5bed75})");
}
unsafe impl ::windows::core::Interface for ContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851c1690_1a51_4b0c_aeef_1240ac5bed75);
}
impl ::windows::core::RuntimeName for ContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListCreateOrUpdateContactRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListCreateOrUpdateContactRequestEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListDeleteContactRequest(::windows::core::IUnknown);
impl ContactListDeleteContactRequest {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListDeleteContactRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListDeleteContactRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListDeleteContactRequest {}
impl ::core::fmt::Debug for ContactListDeleteContactRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListDeleteContactRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListDeleteContactRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest;{5e114687-ce03-4de5-8557-9ccf552d472a})");
}
unsafe impl ::windows::core::Interface for ContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e114687_ce03_4de5_8557_9ccf552d472a);
}
impl ::windows::core::RuntimeName for ContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
}
impl ::core::convert::From<ContactListDeleteContactRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListDeleteContactRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListDeleteContactRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListDeleteContactRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListDeleteContactRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListDeleteContactRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListDeleteContactRequest {}
unsafe impl ::core::marker::Sync for ContactListDeleteContactRequest {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListDeleteContactRequestEventArgs(::windows::core::IUnknown);
impl ContactListDeleteContactRequestEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Request(&self) -> ::windows::core::Result<ContactListDeleteContactRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListDeleteContactRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListDeleteContactRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListDeleteContactRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListDeleteContactRequestEventArgs {}
impl ::core::fmt::Debug for ContactListDeleteContactRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListDeleteContactRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListDeleteContactRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs;{b22054a1-e8fa-4db5-9389-2d12ee7d15ee})");
}
unsafe impl ::windows::core::Interface for ContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22054a1_e8fa_4db5_9389_2d12ee7d15ee);
}
impl ::windows::core::RuntimeName for ContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
}
impl ::core::convert::From<ContactListDeleteContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListDeleteContactRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListDeleteContactRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListDeleteContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListDeleteContactRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListDeleteContactRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListDeleteContactRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListDeleteContactRequestEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequest(::windows::core::IUnknown);
impl ContactListServerSearchReadBatchRequest {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Options(&self) -> ::windows::core::Result<super::ContactQueryOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ContactQueryOptions>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveContactAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Contact>>(&self, contact: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), batchstatus, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListServerSearchReadBatchRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListServerSearchReadBatchRequest {}
impl ::core::fmt::Debug for ContactListServerSearchReadBatchRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListServerSearchReadBatchRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest;{ba776a97-4030-4925-9fb4-143b295e653b})");
}
unsafe impl ::windows::core::Interface for ContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba776a97_4030_4925_9fb4_143b295e653b);
}
impl ::windows::core::RuntimeName for ContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListServerSearchReadBatchRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListServerSearchReadBatchRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListServerSearchReadBatchRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListServerSearchReadBatchRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListServerSearchReadBatchRequest {}
unsafe impl ::core::marker::Sync for ContactListServerSearchReadBatchRequest {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(::windows::core::IUnknown);
impl ContactListServerSearchReadBatchRequestEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Request(&self) -> ::windows::core::Result<ContactListServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListServerSearchReadBatchRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListServerSearchReadBatchRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListServerSearchReadBatchRequestEventArgs {}
impl ::core::fmt::Debug for ContactListServerSearchReadBatchRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListServerSearchReadBatchRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs;{1a27e87b-69d7-4e4e-8042-861cba61471e})");
}
unsafe impl ::windows::core::Interface for ContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a27e87b_69d7_4e4e_8042_861cba61471e);
}
impl ::windows::core::RuntimeName for ContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListServerSearchReadBatchRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListServerSearchReadBatchRequestEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequest(::windows::core::IUnknown);
impl ContactListSyncManagerSyncRequest {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListSyncManagerSyncRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListSyncManagerSyncRequest {}
impl ::core::fmt::Debug for ContactListSyncManagerSyncRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncManagerSyncRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest;{3c0e57a4-c4e7-4970-9a8f-9a66a2bb6c1a})");
}
unsafe impl ::windows::core::Interface for ContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0e57a4_c4e7_4970_9a8f_9a66a2bb6c1a);
}
impl ::windows::core::RuntimeName for ContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
}
impl ::core::convert::From<ContactListSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListSyncManagerSyncRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListSyncManagerSyncRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListSyncManagerSyncRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListSyncManagerSyncRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for ContactListSyncManagerSyncRequest {}
#[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
impl ContactListSyncManagerSyncRequestEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider'*"]
    pub fn Request(&self) -> ::windows::core::Result<ContactListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListSyncManagerSyncRequest>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Contacts_DataProvider', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactListSyncManagerSyncRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactListSyncManagerSyncRequestEventArgs {}
impl ::core::fmt::Debug for ContactListSyncManagerSyncRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactListSyncManagerSyncRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs;{158e4dac-446d-4f10-afc2-02683ec533a6})");
}
unsafe impl ::windows::core::Interface for ContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x158e4dac_446d_4f10_afc2_02683ec533a6);
}
impl ::windows::core::RuntimeName for ContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
}
impl ::core::convert::From<ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListSyncManagerSyncRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListSyncManagerSyncRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactListSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListSyncManagerSyncRequestEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDataProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactDataProviderConnection {
    type Vtable = IContactDataProviderConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a398a52_8c9d_4d6f_a4e0_111e9a125a30);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderConnectionImpl {
    fn SyncRequested();
    fn RemoveSyncRequested();
    fn ServerSearchReadBatchRequested();
    fn RemoveServerSearchReadBatchRequested();
    fn Start();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDataProviderConnection2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactDataProviderConnection2 {
    type Vtable = IContactDataProviderConnection2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1d327b0_196c_4bfd_8f0f_c68d67f249d3);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderConnection2Impl {
    fn CreateOrUpdateContactRequested();
    fn RemoveCreateOrUpdateContactRequested();
    fn DeleteContactRequested();
    fn RemoveDeleteContactRequested();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactDataProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x527104be_3c62_43c8_9ae7_db531685cd99);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactDataProviderTriggerDetailsImpl {
    fn Connection();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderTriggerDetailsVtbl(
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
pub struct IContactListCreateOrUpdateContactRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4af411f_c849_47d0_b119_91cf605b2f2a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListCreateOrUpdateContactRequestImpl {
    fn ContactListId();
    fn Contact();
    fn ReportCompletedAsync();
    fn ReportFailedAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, createdorupdatedcontact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851c1690_1a51_4b0c_aeef_1240ac5bed75);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListCreateOrUpdateContactRequestEventArgsImpl {
    fn Request();
    fn GetDeferral();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListDeleteContactRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e114687_ce03_4de5_8557_9ccf552d472a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListDeleteContactRequestImpl {
    fn ContactListId();
    fn ContactId();
    fn ReportCompletedAsync();
    fn ReportFailedAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListDeleteContactRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22054a1_e8fa_4db5_9389_2d12ee7d15ee);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListDeleteContactRequestEventArgsImpl {
    fn Request();
    fn GetDeferral();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequestEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba776a97_4030_4925_9fb4_143b295e653b);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListServerSearchReadBatchRequestImpl {
    fn SessionId();
    fn ContactListId();
    fn Options();
    fn SuggestedBatchSize();
    fn SaveContactAsync();
    fn ReportCompletedAsync();
    fn ReportFailedAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, batchstatus: super::ContactBatchStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a27e87b_69d7_4e4e_8042_861cba61471e);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListServerSearchReadBatchRequestEventArgsImpl {
    fn Request();
    fn GetDeferral();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0e57a4_c4e7_4970_9a8f_9a66a2bb6c1a);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerSyncRequestImpl {
    fn ContactListId();
    fn ReportCompletedAsync();
    fn ReportFailedAsync();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x158e4dac_446d_4f10_afc2_02683ec533a6);
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactListSyncManagerSyncRequestEventArgsImpl {
    fn Request();
    fn GetDeferral();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequestEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);