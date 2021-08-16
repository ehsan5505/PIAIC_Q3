mod async_example;
mod thread_example;

fn main () {
  async_example::main();
  println!("***************************");
  thread_example::main();

}