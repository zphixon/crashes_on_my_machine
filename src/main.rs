
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use vulkano::{
    device::{
        Device,
        DeviceExtensions,
    },
    instance::{
        debug::{
            MessageSeverity,
            MessageType,
            DebugCallback
        },
        Instance,
        InstanceExtensions,
        PhysicalDevice,
    },
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::io::Write;

fn main() {
    print!("Press enter...");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let extensions = InstanceExtensions {
        ext_debug_utils: true,
        ..vulkano_win::required_extensions()
    };
    let instance = Instance::new(None, &extensions, vec![
        "VK_LAYER_LUNARG_standard_validation"
    ]).expect("could not create instance");

    let severity = MessageSeverity {
        error: true,
        warning: true,
        information: true,
        verbose: true,
    };

    let ty = MessageType::all();

    let _debug_callback = DebugCallback::new(&instance, severity, ty, |msg| {
        let severity = if msg.severity.error {
            "error"
        } else if msg.severity.warning {
            "warning"
        } else if msg.severity.information {
            "information"
        } else if msg.severity.verbose {
            "verbose"
        } else {
            panic!("no-impl");
        };

        let ty = if msg.ty.general {
            "general"
        } else if msg.ty.validation {
            "validation"
        } else if msg.ty.performance {
            "performance"
        } else {
            panic!("no-impl");
        };

        println!("{} {} {}: {}", msg.layer_prefix, ty, severity, msg.description);
    }).ok();

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
