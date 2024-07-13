fn main() {
    let fut = async {
        foo().await?;
        bar().await?;
        Ok(())
    };

    /*
    error[E0282]: type annotations needed
 --> src/main.rs:5:9
  |
4 |     let fut = async {
  |         --- consider giving `fut` a type
5 |         foo().await?;
  |         ^^^^^^^^^^^^ cannot infer type
     */

    // Nice!
    let fut = async {
        foo().await?;
        bar().await?;
        Ok::<(), MyError>(()) // <- note the explicit type annotation here
    };
}