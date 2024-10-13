use std::sync::{Arc, Mutex};
// rclrs related imports
use rclrs::{Publisher, RclrsError, Subscription};
use std_msgs::msg::String as StringMsg;
use std_msgs::msg::UInt8 as UInt8Msg;

struct StringLengthNode {
    node: Arc<rclrs::Node>,
    _subscription: Arc<rclrs::Subscription<StringMsg>>,
    data: Arc<Mutex<Option<StringMsg>>>,
    publisher: Arc<rclrs::Publisher<UInt8Msg>>,
    counter: Arc<Mutex<u8>>,
}

impl StringLengthNode {
// This function is called when creating the node.
    fn new(context: &rclrs::Context)-> Result<Self, rclrs::RclrsError>{
        let node = rclrs::Node::new(context, "string_length_node")?;
        let data = Arc::new(Mutex::new(None));
        let data_cb = Arc::clone(&data);
        let _subscription = node.create_subscription(
            "string_topic", rclrs::QOS_PROFILE_DEFAULT,
            move |msg: StringMsg| {
                *data_cb.lock().unwrap() = Some(msg);
            },
        )?;
        let publisher = node.create_publisher("string_length", rclrs::QOS_PROFILE_DEFAULT)?;
        let counter = Arc::new(Mutex::new(0));
        // Return Ok with the constructed node
        Ok(Self{
            node,
            _subscription,
            publisher,
            data,
            counter,
        })
    }
    fn publish(&self) -> Result<(), rclrs::RclrsError> {
        // Get the latest data from the subscription
        let mut count = self.counter.lock().unwrap();
        let length_msg = UInt8Msg { data: *count };
        self.publisher.publish(length_msg)?;
        *count += 1; // カウンターをインクリメント
        Ok(())
    }
} // impl StringLengthNode

fn main() -> Result<(), rclrs::RclrsError>{
    println!("Hello, world! - String length node.");
    // Create the rclrs context.
    let context = rclrs::Context::new(std::env::args())?;
    // Create a node and a clone. The first one will subscribe and the clone will publish
    let string_length_node = Arc::new(StringLengthNode::new(&context)?);
    let string_length_publish_node = Arc::clone(&string_length_node);
    // Thread for timer to publish
    std::thread::spawn(move || -> Result<(), rclrs::RclrsError> {
        loop {
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(1000));
            string_length_publish_node.publish()?;
        }
    });
    // Spin the subscription node
    rclrs::spin(Arc::clone(&string_length_node.node))
}

