use std::future::Future;

pub fn spawn<O, F>(future: F) -> tokio::task::JoinHandle<O>
where
	F: Future<Output = O> + Send + 'static,
	O: Send + 'static,
{
	crate::RT.get().unwrap().spawn(future)
}

pub fn spawn_local<F: Future<Output = ()> + 'static>(future: F) {
	gtk4::glib::MainContext::default().spawn_local(future);
}
