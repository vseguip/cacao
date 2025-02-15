use core_graphics::geometry::CGRect;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

#[cfg(feature = "appkit")]
use crate::appkit::toolbar::ToolbarItem;
#[cfg(feature = "appkit")]
use crate::appkit::window::Window;
#[cfg(feature = "appkit")]
use crate::appkit::App;

use crate::foundation::{id, nil, NSString};
use crate::geometry::{Edge, Rect};
use crate::layout::Layout;
use crate::utils::{os, CGSize, Controller};
use crate::view::{View, ViewController, ViewDelegate};

#[derive(Debug, Eq, PartialEq)]
#[repr(i64)]
pub enum PopoverBehaviour {
    /// Your application assumes responsibility for closing the popover.
    ApplicationDefined = 0,
    /// The system will close the popover when the user interacts with a user interface element outside the popover.
    Transient = 1,
    /// The system will close the popover when the user interacts with user interface elements in the window containing the popover's positioning view.
    Semitransient = 2
}

#[derive(Debug)]
pub struct PopoverConfig {
    pub content_size: CGSize,
    pub animates: bool,
    pub behaviour: PopoverBehaviour
}

impl Default for PopoverConfig {
    fn default() -> Self {
        Self {
            content_size: CGSize {
                width: 320.0,
                height: 320.0
            },
            animates: true,
            behaviour: PopoverBehaviour::Transient
        }
    }
}

#[derive(Debug)]
pub struct Popover<Content> {
    /// A reference to the underlying Objective-C NSPopover
    pub objc: ShareId<Object>,

    /// The wrapped ViewController.
    pub view_controller: ViewController<Content>
}

impl<Content> Popover<Content>
where
    Content: ViewDelegate + 'static
{
    pub fn new(content: Content, config: PopoverConfig) -> Self {
        let view_controller = ViewController::new(content);
        let objc = unsafe {
            let pop: id = msg_send![class!(NSPopover), new];
            let _: () = msg_send![pop, setContentSize: config.content_size];
            let _: () = msg_send![pop, setBehavior: config.behaviour as i64];
            let _: () = msg_send![pop, setAnimates: config.animates];
            let _: () = msg_send![pop, setContentViewController: &*view_controller.objc];

            ShareId::from_ptr(pop)
        };

        Popover { objc, view_controller }
    }
}

impl<Content> Popover<Content> {
    /// Show a popover relative to a view
    pub fn show_popover<V: Layout>(&self, relative_to: Rect, view: &V, edge: Edge) {
        let rect: CGRect = relative_to.into();
        unsafe {
            view.with_backing_obj_mut(|obj| {
                let _: () = msg_send![&*self.objc, showRelativeToRect:rect ofView: &*obj preferredEdge: edge as u32];
            });
        }
    }

    /// Show the popover relative to the content view of the main window
    #[cfg(feature = "appkit")]
    pub fn show_popover_main(&self, rect: Rect, edge: Edge) {
        let window = App::main_window();
        unsafe {
            let content_view = window.content_view();
            let rect: CGRect = rect.into();
            let _: () = msg_send![&*self.objc, showRelativeToRect:rect ofView: content_view preferredEdge: edge as u32];
        }
    }
}
