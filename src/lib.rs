use std::future::Future;
use std::sync::Arc;

pub async fn run() {
    foo(|| async { create() }).await;
    foo(|| async { create() }).await;
}

async fn foo<F>(create: impl Fn() -> F)
where
    F: Future<Output = Struct>,
{
    call_empty::<()>(create().await, "").await;
}

struct Struct {
    _vec: Vec<u8>,
    _arc: Arc<()>,
}

fn create() -> Struct {
    Struct {
        _vec: Vec::new(),
        _arc: Arc::new(()),
    }
}

async fn call_empty<R>(_: Struct, _: &str) {
    empty()
}

fn empty() {}
