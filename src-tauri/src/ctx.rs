use crate::event::HubEvent;
use crate::prelude::*;
use crate::store::Store;
use crate::utils::XInto;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};

pub struct Ctx {
	store: Arc<Store>,
	app_handle: AppHandle<Wry>,
}

impl XInto<Arc<Ctx>> for AppHandle<Wry> {
	fn x_into(self) -> Result<Arc<Ctx>> {
		Ok(Arc::new(Ctx::new(self)))
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

	pub fn emit_hub_event<D: Serialize + Clone>(&self, hub_event: HubEvent<D>) {
		let _ = self.app_handle.emit_all("HubEvent", hub_event);
	}
}
