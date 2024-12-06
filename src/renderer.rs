use anyhow::Context;
use ash::vk;
use winit::raw_window_handle::HasDisplayHandle;
pub struct Renderer {
    display_handle: winit::event_loop::OwnedDisplayHandle,
    entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
}

impl Renderer {
    pub fn new(display_handle: winit::event_loop::OwnedDisplayHandle) -> anyhow::Result<Self> {
        let entry = unsafe { ash::Entry::load() }?;
        let required_extensions =
            ash_window::enumerate_required_extensions(display_handle.display_handle()?.as_raw())?;
        let instance = {
            let app_info = vk::ApplicationInfo::default()
                .application_name(c"Vulkan Obj Renderer")
                .application_version(vk::make_api_version(0, 0, 1, 0))
                .engine_name(c"Vulkan Obj Renderer Engine")
                .engine_version(vk::make_api_version(0, 0, 1, 0))
                .api_version(vk::API_VERSION_1_3);

            let info = vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_extension_names(required_extensions);

            unsafe { entry.create_instance(&info, None) }
        }?;

        let physical_device = select_physical_device(&instance)?;

        let queue_idx = select_queue_family(&instance, physical_device, vk::QueueFlags::GRAPHICS)?;
        let device = {
            let extension_names = [vk::KHR_SWAPCHAIN_NAME.as_ptr()];
            let mut features_12 = vk::PhysicalDeviceVulkan12Features::default()
                .buffer_device_address(true)
                .descriptor_indexing(true);
            let mut features_13 = vk::PhysicalDeviceVulkan13Features::default()
                .dynamic_rendering(true)
                .synchronization2(true);

            let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_idx)
                .queue_priorities(&[1.0])];

            let info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&extension_names)
                .push_next(&mut features_12)
                .push_next(&mut features_13);

            unsafe { instance.create_device(physical_device, &info, None) }
        }?;

        let queue = unsafe { device.get_device_queue(queue_idx, 0) };

        Ok(Self {
            display_handle,
            entry,
            instance,
            physical_device,
            device,
            queue,
        })
    }
}

fn select_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    flags: vk::QueueFlags,
) -> anyhow::Result<u32> {
    Ok(
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) }
            .into_iter()
            .enumerate()
            .find(|(_, properties)| properties.queue_flags.contains(flags))
            .context("Selected GPU cannot do graphics??????")?
            .0 as u32,
    )
}

fn select_physical_device(instance: &ash::Instance) -> anyhow::Result<vk::PhysicalDevice> {
    let physical_devices = unsafe { instance.enumerate_physical_devices() }?;

    physical_devices
        .into_iter()
        .max_by_key(|physical_device| {
            match unsafe {
                instance
                    .get_physical_device_properties(*physical_device)
                    .device_type
            } {
                vk::PhysicalDeviceType::DISCRETE_GPU => 100,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 50,
                _ => 0,
            }
        })
        .context("No GPU???")
}
