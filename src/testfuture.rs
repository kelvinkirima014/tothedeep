use std::{
	future::Future,
	pin::Pin,
	task::{Context, Poll},
};

use tracing::info;

pub struct TestFuture {

}

impl Future for TestFuture {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
		info!("Hello, from a dumb future world!");
		Poll::Ready(())
	}
}