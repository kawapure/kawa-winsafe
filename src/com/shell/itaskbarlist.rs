#![allow(non_snake_case)]

macro_rules! pub_struct_ITaskbarList {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::shell::vt::ITaskbarListVT;
		use crate::handles::HWND;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn itaskbarlist_vt(&self) -> &ITaskbarListVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
			/// method.
			pub fn ActivateTab(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().ActivateTab)(self.ppvt, hwnd.ptr),
				)
			}

			/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
			/// method.
			pub fn AddTab(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().AddTab)(self.ppvt, hwnd.ptr),
				)
			}

			/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
			/// method.
			pub fn DeleteTab(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().DeleteTab)(self.ppvt, hwnd.ptr),
				)
			}

			/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
			/// method.
			pub fn HrInit(&self) -> WinResult<()> {
				hr_to_winresult((self.itaskbarlist_vt().HrInit)(self.ppvt))
			}

			/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
			/// method.
			pub fn SetActiveAlt(&self, hwnd: HWND) -> WinResult<()> {
				hr_to_winresult(
					(self.itaskbarlist_vt().SetActiveAlt)(self.ppvt, hwnd.ptr),
				)
			}
		}
	};
}

pub_struct_ITaskbarList! {
	/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
	/// COM interface over [`ITaskbarListVT`](crate::shell::vt::ITaskbarListVT).
	/// Inherits from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, CoCreateInstance, shell};
	///
	/// let obj: shell::ITaskbarList = CoCreateInstance(
	///     &shell::clsid::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// ).unwrap();
	/// ```
	ITaskbarList, crate::com::shell::vt::ITaskbarListVT
}
