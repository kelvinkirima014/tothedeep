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
		panic!("Oh noes");
	}
}