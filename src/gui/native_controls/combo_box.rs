use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of ComboBox
	base: BaseNativeControl,
	events: ComboBoxEvents,
	_pin: PhantomPinned,
}

/// Native
/// [combo box](https://learn.microsoft.com/en-us/windows/win32/controls/about-combo-boxes)
/// control.
#[derive(Clone)]
pub struct ComboBox(Pin<Arc<Obj>>);

unsafe impl Send for ComboBox {}

impl AsRef<BaseNativeControl> for ComboBox {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl GuiWindow for ComboBox {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for ComboBox {}

impl GuiChild for ComboBox {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiChildFocus for ComboBox {}

impl GuiNativeControl for ComboBox {}

impl GuiNativeControlEvents<ComboBoxEvents> for ComboBox {
	fn on(&self) -> &ComboBoxEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl ComboBox {
	/// Instantiates a new `ComboBox` object, to be created on the parent window
	/// with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ComboBox` in an event closure.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let cmb = gui::ComboBox::new(
	///     &wnd,
	///     gui::ComboBoxOpts {
	///         position: (10, 10),
	///         width: 140,
	///         items: vec![
	///             "Avocado".to_owned(),
	///             "Banana".to_owned(),
	///             "Grape".to_owned(),
	///             "Orange".to_owned(),
	///         ],
	///         selected_item: Some(0),
	///         ..Default::default()
	///     },
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: ComboBoxOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ComboBoxEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_create_or_initdialog(move |_, _| {
			self2.create(OptsResz::Wnd(&opts))?;
			Ok(WmRet::NotHandled)
		});

		new_self
	}

	/// Instantiates a new `ComboBox` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ComboBox` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: ComboBoxEvents::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_init_dialog(move |_| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(false) // return value is discarded
		});

		new_self
	}

	fn create(&self, opts_resz: OptsResz<&ComboBoxOpts>) -> SysResult<()> {
		if opts_resz.resize_behavior().1 == Vert::Resize {
			panic!("ComboBox cannot be resized with Vert::Resize.");
		}

		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.width as _, 0);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"COMBOBOX", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.combo_box_style.into(),
				)?;

				unsafe {
					self.hwnd().SendMessage(wm::SetFont {
						hfont: ui_font(),
						redraw: true,
					});
				}
				self.items().add(&opts.items);
				self.items().select(opts.selected_item);
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent()
			.add_to_layout_arranger(self.hwnd(), opts_resz.resize_behavior())
	}

	/// Item methods.
	#[must_use]
	pub const fn items(&self) -> ComboBoxItems<'_> {
		ComboBoxItems::new(self)
	}
}

/// Options to create a [`ComboBox`](crate::gui::ComboBox) programmatically with
/// [`ComboBox::new`](crate::gui::ComboBox::new).
pub struct ComboBoxOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the value is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// Defaults to `120`.
	pub width: u32,
	/// Combo box styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `CBS::DROPDOWNLIST`.
	///
	/// Suggestions:
	/// * replace with `CBS::DROPDOWN` to allow the user to type a text;
	/// * add `CBS::SORT` to automatically sort the items.
	pub combo_box_style: co::CBS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// **Note:** A `ComboBox` cannot be resized vertically, so it will panic if
	/// you use `Vert::Resize`.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),

	/// Items to be added right away to the control.
	///
	/// Defaults to none.
	pub items: Vec<String>,
	/// Index of the item initially selected. The item must exist.
	///
	/// Defaults to `None`.
	pub selected_item: Option<u32>,
}

impl Default for ComboBoxOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			width: 120,
			combo_box_style: co::CBS::DROPDOWNLIST,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
			items: Vec::<String>::new(),
			selected_item: None,
		}
	}
}

impl ResizeBehavior for &ComboBoxOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for ComboBoxOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
