
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use vulkano::{
    device::{
        Device,
        DeviceExtensions,
    },
    instance::{
        Instance,
        PhysicalDevice,
    },
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
    let required_extensions = vulkano_win::required_extensions();
    let instance = Instance::new(None, &required_extensions, vec![
        "VK_LAYER_LUNARG_standard_validation"
    ]).expect("could not create instance");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("couldn't find physical device");
    println!("Using physical device ({:?}): {}", physical.ty(), physical.name());

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_title(":(")
        .build_vk_surface(&event_loop, instance.clone())
        .expect("couldn't build window");

    let queue_family = physical.queue_families().find(|&q| {
        q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
    }).expect("couldn't get queue families");

    let device_ext = DeviceExtensions {
        khr_swapchain: true, ..DeviceExtensions::none()
    };
    let (_device, _queues) = Device::new(
        physical,
        physical.supported_features(),
        &device_ext,
        [(queue_family, 0.5)].iter().cloned()
    ).expect("couldn't init logical device");

    unreachable!();
}
