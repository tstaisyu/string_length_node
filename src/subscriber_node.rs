// src/subscriber_node.rs
use rclrs::{Subscription, Node, Context, RclrsError};
use std_msgs::msg::String as StringMsg;

pub struct SubscriberNode {
    pub node: Node,
    pub subscription: Subscription<StringMsg>,
}

impl SubscriberNode {
    pub fn new(context: &Context) -> Result<Self, RclrsError> {
        let node = Node::new(context, "example_subscriber")?;
        let subscription = node.create_subscription(
            "topic",
            rclrs::QOS_PROFILE_DEFAULT,
            |msg: StringMsg| {
                println!("Received: {}", msg.data);
            },
        )?;
        Ok(Self { node, subscription })
    }
}

// Provide a public function to run the subscription logic
pub fn run_subscriber() -> Result<(), RclrsError> {
    let context = Context::new(std::env::args())?;
    let subscriber_node = SubscriberNode::new(&context)?;
    rclrs::spin(subscriber_node.node)
}
