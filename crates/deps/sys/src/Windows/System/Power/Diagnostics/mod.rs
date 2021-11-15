#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IBackgroundEnergyDiagnosticsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundEnergyDiagnosticsStatics {}
impl ::core::clone::Clone for IBackgroundEnergyDiagnosticsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IForegroundEnergyDiagnosticsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IForegroundEnergyDiagnosticsStatics {}
impl ::core::clone::Clone for IForegroundEnergyDiagnosticsStatics {
    fn clone(&self) -> Self {
        *self
    }
}