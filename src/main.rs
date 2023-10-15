use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::common_interfaces::std_msgs,pr_info,
};

fn main() -> Result<(), DynError>{
    let ctx = Context::new()?;
    let node = ctx.create_node("support_wheel_drive", None, Default::default())?;
    let subscriber = node.create_subscriber::<std_msgs::msg::Int32>("support_drive_topic", None)?;
    let publisher = node.create_publisher::<drobo_interfaces::msg::MdLibMsg>("md_driver_topic", None)?;
    let mut pub_msg = drobo_interfaces::msg::MdLibMsg::new().unwrap();
    pub_msg.address = 0x05;
    pub_msg.mode = 2;

    let logger = Logger::new("support_wheel_drive");

    let mut selelctor = ctx.create_selector()?;

    selelctor.add_subscriber(
        subscriber, 
        Box::new(move |msg| {
            let phase = msg.data >= 0;
            let power = msg.data.abs() as u16;
            pub_msg.phase = phase;
            pub_msg.power = power;
            pr_info!(logger, "中央輪, phase: {}, power: {}", phase, power);
            publisher.send(&pub_msg).unwrap();
        }),
    );

    loop {
        selelctor.wait()?;
    }
}
