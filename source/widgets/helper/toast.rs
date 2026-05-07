// source/widgets/helper/toast.rs

use notify_rust::{ Notification, Timeout };

use crate::service::is_registered_aumid;

pub struct Toast {
    pub title: String,
    pub message: String,
}

pub struct ToastWidget;

impl ToastWidget {
	pub fn show(toast: Toast) {
		let mut notification = Notification::new();
		#[cfg(windows)]
		{
			let aumid = "com.mvtool.desktop";

			if is_registered_aumid(aumid) {
				notification.app_id(aumid);
				notification.appname("mvtool");
			}
		}

		notification.summary(&toast.title)
			.body(&toast.message)
			.timeout(Timeout::Default)
			.auto_icon();

		let _ = notification.show();
	}
}