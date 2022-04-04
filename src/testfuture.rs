use std::{
	future::Future,
	pin::Pin,
	task::{Context, Poll},
};

pub struct TestFuture {

}

impl Future for TestFuture {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
		unsafe {
			*(0xF00D as *mut u64) = 0x0;
		}
		unreachable!();
	}
}