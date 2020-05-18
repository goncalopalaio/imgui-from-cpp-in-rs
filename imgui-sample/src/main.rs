extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
    fn imgui_example_main(number: i32);
}

fn main() {
    println!("Hello, world!");

    unsafe {
        println!("foo {}", multiply(5, 7));

        println!("calling imgui_example_main");

        imgui_example_main(12345);
    }

    println!("Exiting");
}
