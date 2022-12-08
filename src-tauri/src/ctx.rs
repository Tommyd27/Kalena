use crate::prelude::*;
use crate::Store;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};

pub struct Ctx {
	store: Arc<Store>,
	app_handle: AppHandle<Wry>,
}

impl Ctx {
	pub fn from_app(app: AppHandle<Wry>) -> Result<Arc<Ctx>> {
		Ok(Arc::new(Ctx::new(app)))
	}
}

impl Ctx {
	pub fn new(app_handle: AppHandle<Wry>) -> Self {
		Ctx {
			store: (*app_handle.state::<Arc<Store>>()).clone(),
			app_handle,
		}
	}

	pub fn get_store(&self) -> Arc<Store> {
		self.store.clone()
	}
}