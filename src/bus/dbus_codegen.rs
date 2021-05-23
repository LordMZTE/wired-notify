// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs
// The top half of this code is hand written.

#![allow(dead_code)]
use dbus;
use dbus::arg;
use dbus::tree;
use crate::bus::dbus::Message;
use std::sync::mpsc::Sender;
use std::collections::HashMap;

// TODO: maybe move this stuff somewhere else.
// This enum covers all hint values defined in the desktop notifications specification:
// (https://developer.gnome.org/notification-spec/#hints)
//
// Possible values:
//  'action-icons' - bool
//  'category' - string,
//  'desktop-entry' - string,
//  'image-data' - DBusImage,
//  'image_data' - DBusImage,
//  'image-path' - string,
//  'image_path' - string,
//  'icon_data' - DBusImage,
//  'resident' - bool,
//  'sound-file' - string,
//  'sound-name' - string,
//  'suppress-sound' bool,
//  'transient' - bool,
//  'x' - i32,
//  'y' - i32,
//  'urgency' - u8
#[derive(Debug)]
pub enum Value {
    String(String),
    I32(i32),
    U8(u8),
    Bool(bool),
    Struct(DBusImage),
    Unsupported,
}

#[derive(Debug)]
pub struct DBusImage {
    pub width: i32,
    pub height: i32,
    pub rowstride: i32,
    pub one_point_two_bit_alpha: bool,
    pub bits_per_sample: i32,
    pub channels: i32,
    pub data: Vec<u8>,
}

impl<'a> arg::Get<'a> for DBusImage {
    fn get(i: &mut arg::Iter) -> Option<Self> {
        let width = i.read::<i32>().ok()?;
        let height = i.read::<i32>().ok()?;
        let rowstride = i.read::<i32>().ok()?;
        let one_point_two_bit_alpha = i.read::<bool>().ok()?;
        let bits_per_sample = i.read::<i32>().ok()?;
        let channels = i.read::<i32>().ok()?;
        let data = i.read::<Vec<u8>>().ok()?;

        Some(
            DBusImage {
                width,
                height,
                rowstride,
                one_point_two_bit_alpha,
                bits_per_sample,
                channels,
                data,
        })
    }
}

impl arg::Arg for Value {
    const ARG_TYPE: arg::ArgType = arg::ArgType::Variant;
    fn signature() -> dbus::Signature<'static> {
        dbus::Signature::from_slice(b"v\0").unwrap()
    }
}

impl<'a> arg::Get<'a> for Value {
    fn get(i: &mut arg::Iter) -> Option<Self> {
        use arg::ArgType;

        let arg_type = i.arg_type();
        if let ArgType::Invalid = arg_type {
            return None;
        }
        let signature = i.signature();

        match arg_type {
            // This seems ugly, but is definitely the image data struct.
            ArgType::Struct if *signature == *"(iiibiiay)" =>
                i.recurse(ArgType::Struct).and_then(|mut iter| iter.get::<DBusImage>()).map(Value::Struct),
            ArgType::Boolean => i.get::<bool>().map(Value::Bool),
            ArgType::Byte => i.get::<u8>().map(Value::U8),
            ArgType::Int32 => i.get::<i32>().map(Value::I32),
            ArgType::String => i.get::<String>().map(Value::String),
            ArgType::Variant => i.recurse(ArgType::Variant).and_then(|mut iter| iter.get()),
            _ => None,
        }
    }
}

// ------------ end non-codegen.

pub trait OrgFreedesktopNotifications {
    fn get_capabilities(&self) -> Result<Vec<String>, tree::MethodErr>;
    fn notify(&self, sender: Sender<Message>, app_name: &str, replaces_id: u32, app_icon: &str, summary: &str, body: &str, actions: Vec<&str>, hints: HashMap<String, Value> /*::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>*/, expire_timeout: i32) -> Result<u32, tree::MethodErr>;
    fn close_notification(&self, sender: Sender<Message>, id: u32) -> Result<(), tree::MethodErr>;
    fn get_server_information(&self) -> Result<(String, String, String, String), tree::MethodErr>;
}

pub fn org_freedesktop_notifications_server<F, T, D>(sender: Sender<Message>, factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Signal: Default,
    T: OrgFreedesktopNotifications,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.Notifications", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let capabilities = d.get_capabilities()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(capabilities);
        Ok(vec!(rm))
    };
    let m = factory.method("GetCapabilities", Default::default(), h);
    let m = m.out_arg(("capabilities", "as"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let sclone = sender.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let app_name: &str = i.read()?;
        let replaces_id: u32 = i.read()?;
        let app_icon: &str = i.read()?;
        let summary: &str = i.read()?;
        let body: &str = i.read()?;
        let actions: Vec<&str> = i.read()?;
        let hints: HashMap<String, Value> = i.read()?;
        //let hints: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>> = i.read()?;
        let expire_timeout: i32 = i.read()?;
        let d = fclone(minfo);
        let id = d.notify(sclone.clone(), app_name, replaces_id, app_icon, summary, body, actions, hints, expire_timeout)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(id);
        Ok(vec!(rm))
    };
    let m = factory.method("Notify", Default::default(), h);
    let m = m.in_arg(("app_name", "s"));
    let m = m.in_arg(("replaces_id", "u"));
    let m = m.in_arg(("app_icon", "s"));
    let m = m.in_arg(("summary", "s"));
    let m = m.in_arg(("body", "s"));
    let m = m.in_arg(("actions", "as"));
    let m = m.in_arg(("hints", "a{sv}"));
    let m = m.in_arg(("expire_timeout", "i"));
    let m = m.out_arg(("id", "u"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let sclone = sender.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let id: u32 = i.read()?;
        let d = fclone(minfo);
        d.close_notification(sclone.clone(), id)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("CloseNotification", Default::default(), h);
    let m = m.in_arg(("id", "u"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let (name, vendor, version, spec_version) = d.get_server_information()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(name);
        let rm = rm.append1(vendor);
        let rm = rm.append1(version);
        let rm = rm.append1(spec_version);
        Ok(vec!(rm))
    };
    let m = factory.method("GetServerInformation", Default::default(), h);
    let m = m.out_arg(("name", "s"));
    let m = m.out_arg(("vendor", "s"));
    let m = m.out_arg(("version", "s"));
    let m = m.out_arg(("spec_version", "s"));
    let i = i.add_m(m);
    let s = factory.signal("NotificationClosed", Default::default());
    let s = s.arg(("id", "u"));
    let s = s.arg(("reason", "u"));
    let i = i.add_s(s);
    let s = factory.signal("ActionInvoked", Default::default());
    let s = s.arg(("id", "u"));
    let s = s.arg(("action_key", "s"));
    let i = i.add_s(s);
    i
}

#[derive(Debug)]
pub struct OrgFreedesktopNotificationsNotificationClosed {
    pub id: u32,
    pub reason: u32,
}

impl arg::AppendAll for OrgFreedesktopNotificationsNotificationClosed {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
        arg::RefArg::append(&self.reason, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNotificationsNotificationClosed {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNotificationsNotificationClosed {
            id: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNotificationsNotificationClosed {
    const NAME: &'static str = "NotificationClosed";
    const INTERFACE: &'static str = "org.freedesktop.Notifications";
}

#[derive(Debug)]
pub struct OrgFreedesktopNotificationsActionInvoked {
    pub id: u32,
    pub action_key: String,
}

impl arg::AppendAll for OrgFreedesktopNotificationsActionInvoked {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
        arg::RefArg::append(&self.action_key, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNotificationsActionInvoked {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNotificationsActionInvoked {
            id: i.read()?,
            action_key: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNotificationsActionInvoked {
    const NAME: &'static str = "ActionInvoked";
    const INTERFACE: &'static str = "org.freedesktop.Notifications";
}
