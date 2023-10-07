#[instruction_set(arm::a32)]
fn arm_fn(n: i32) {
    println!("Hello from ARM!, {n}");
}

#[instruction_set(arm::t32)]
fn thumb_fn(n: i32) {
    println!("Hello from Thumb!, {n}");
    arm_fn(n + 1);
}

#[instruction_set(arm::a32)]
fn main() {
    thumb_fn(0);
}
