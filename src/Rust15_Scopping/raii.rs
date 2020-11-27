// resource acquisition is initialization

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self){
        println!("ToDrop is being dropped!");
    }
}

fn create_box() {
    let _box1 = Box::new(3i32);
    // destroyed here, memory freed
}

fn main() {
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0_u32..=1_000 {
        create_box();
    }

    let x = ToDrop;
    // _box2 destroyed here
    // x destroyed and called drop
}