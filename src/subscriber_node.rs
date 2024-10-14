// subscriber_node.rs
use rclrs;
use std_msgs::msg::UInt8;

pub fn listen() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;
    let node = rclrs::Node::new(context, "uint8_subscriber")?;

    let subscription = node.create_subscription::<UInt8>(
        "string_length",
        rclrs::QOS_PROFILE_DEFAULT,
        |msg: UInt8| {
            println!("Received: {}", msg.data);
        },
    )?;

    println!("Rust subscriber node started, listening to 'string_length'");
    rclrs::spin(node)
}
