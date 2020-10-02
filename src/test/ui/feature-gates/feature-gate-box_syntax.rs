// Test that the use of the box syntax is gated by `box_syntax` feature gate.

#[cfg(FALSE)]
fn foo() {
    let x = box 3;
    //~^ ERROR box expression syntax is experimental; you can call `Box::new` instead
}

fn main() {}
