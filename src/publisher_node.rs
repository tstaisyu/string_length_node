// src/publisher_node.rs
use std::sync::{Arc, Mutex};
use rclrs::{Publisher, Node, Context, RclrsError};
use std_msgs::msg::String as StringMsg;

pub struct PublisherNode {
    pub node: Arc<Node>,
    pub publisher: Arc<Publisher<StringMsg>>,
}

impl PublisherNode {
    pub fn new(context: &Context) -> Result<Self, RclrsError> {
        let node = Node::new(context, "example_publisher")?;
        let publisher = node.create_publisher("topic", rclrs::QOS_PROFILE_DEFAULT)?;
        Ok(Self {
            node: Arc::new(node),
            publisher: Arc::new(publisher),
        })
    }

    pub fn publish(&self) -> Result<(), RclrsError> {
        let msg = StringMsg { data: "Hello, world!".to_string() };
        self.publisher.publish(msg)?;
        Ok(())
    }
}

// Provide a public function to run the publishing logic
pub fn run_publisher() -> Result<(), RclrsError> {
    let context = Context::new(std::env::args())?;
    let publisher_node = PublisherNode::new(&context)?;
    loop {
        publisher_node.publish()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
