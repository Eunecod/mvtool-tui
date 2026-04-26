// source/widgets/helper/toast.rs

use notify_rust::Notification;

pub struct ToastPayload {
    pub title: String,
    pub message: String,
	pub duration: f32,
}

pub struct ToastWidget;

impl ToastWidget {
	pub fn show(payload: ToastPayload) {
		let mut notification = Notification::new();
		#[cfg(windows)]
		{
			notification.app_id("com.mvtool.desktop");
		}

		let _ = notification.appname("mvtool")
			.summary(&payload.title)
			.body(&payload.message).timeout((payload.duration * 1000.0) as i32)
			.auto_icon()
		.show();
	}
}