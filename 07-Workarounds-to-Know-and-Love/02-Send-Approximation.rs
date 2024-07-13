use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar() {}

// Works, but not in variable
async fn foo() {
    let x = NotSend::default();
    bar().await;
}

/* Not compiles
async fn foo() {
    let x = NotSend::default();
    bar().await;
}
 */
// Works! Value dropped
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}


fn require_send(_: impl Send) {}

fn main() {
    require_send(foo());
}