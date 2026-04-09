let SystemStatus: i32 = 1;
let GlobalTorque: f32 = 0.0;

fn main() {
    if (SystemStatus > 0) {
        GlobalTorque = 100.0;
    } else {
        GlobalTorque = 0.0;
    }
}
