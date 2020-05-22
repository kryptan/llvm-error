use std::future::Future;

#[test]
fn test() {
    let _: Box<dyn Future<Output = ()>> = Box::new(llvm_eror::run());
}
