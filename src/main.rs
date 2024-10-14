mod publisher_node;
mod subscriber_node;

fn main() -> Result<(), rclrs::RclrsError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "pub" {
        publisher_node::run_publisher()?;
    } else {
        subscriber_node::run_subscriber()?;
    }

    Ok(())
}
