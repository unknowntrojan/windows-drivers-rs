use wdk_sys::{macros, NTSTATUS, WDFTIMER, WDF_OBJECT_ATTRIBUTES, WDF_TIMER_CONFIG};

use crate::nt_success;

#[allow(clippy::module_name_repetitions)]
// private module + public re-export avoids the module name repetition: https://github.com/rust-lang/rust-clippy/issues/8524
pub struct Timer {
    wdf_timer: WDFTIMER,
}
impl Timer {
    /// Try to construct a WDF Timer object
    ///
    /// # Errors
    ///
    /// This function will return an error if WDF fails to contruct a timer. The error variant will contain a [`NTSTATUS`] of the failure. Full error documentation is available in the [WDFTimer Documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdftimer/nf-wdftimer-wdftimercreate#return-value)
    pub fn try_new(
        timer_config: &mut WDF_TIMER_CONFIG,
        attributes: &mut WDF_OBJECT_ATTRIBUTES,
    ) -> Result<Self, NTSTATUS> {
        let mut timer = Self {
            wdf_timer: core::ptr::null_mut(),
        };
        let nt_status = unsafe {
            macros::call_unsafe_wdf_function_binding!(
                WdfTimerCreate,
                timer_config,
                attributes,
                &mut timer.wdf_timer,
            )
        };
        nt_success(nt_status).then_some(timer).ok_or(nt_status)
    }

    /// Try to construct a WDF Timer object
    ///
    /// # Errors
    ///
    /// This function will return an error if WDF fails to contruct a timer. The error variant will contain a [`NTSTATUS`] of the failure. Full error documentation is available in the [WDFTimer Documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdftimer/nf-wdftimer-wdftimercreate#return-value)
    pub fn create(
        timer_config: &mut WDF_TIMER_CONFIG,
        attributes: &mut WDF_OBJECT_ATTRIBUTES,
    ) -> Result<Self, NTSTATUS> {
        Self::try_new(timer_config, attributes)
    }

    pub fn start(&self, due_time: i64) -> bool {
        let result = unsafe {
            macros::call_unsafe_wdf_function_binding!(WdfTimerStart, self.wdf_timer, due_time)
        };
        result != 0
    }

    pub fn stop(&self, wait: bool) -> bool {
        let result = unsafe {
            macros::call_unsafe_wdf_function_binding!(WdfTimerStop, self.wdf_timer, u8::from(wait))
        };
        result != 0
    }
}
