use ash::{vk};
use std::ffi::{CStr, CString};

pub struct GpuDevice {
    pub physical_device: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
}

pub struct GpuContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub device: ash::Device,
    pub physical_device: vk::PhysicalDevice,
    pub queue: vk::Queue,
    pub queue_family_index: u32,
}

pub fn init_vulkan_and_pick_device() -> GpuDevice {
    let entry = ash::Entry::linked();

    let app_name = CString::new("PulseGPU").unwrap();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk::API_VERSION_1_3);

    let instance_create_info = vk::InstanceCreateInfo::builder().application_info(&app_info);

    let instance = unsafe {
        entry
            .create_instance(&instance_create_info, None)
            .expect("Failed to create Vulkan instance")
    };

    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate Vulkan physical devices")
    };

    if physical_devices.is_empty() {
        panic!("No Vulkan physical devices found");
    }

    let physical_device = physical_devices[0];

    let properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device_name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) };
    println!("Using Vulkan device: {}", device_name.to_str().unwrap());

    for &device in &physical_devices {
        let props = unsafe { instance.get_physical_device_properties(device) };
        let name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        let name_str = name.to_str().unwrap_or_default();
        println!("🔍 Found GPU: {}", name_str);

        return GpuDevice {
            physical_device: device,
            properties: props,
        };
    }
    panic!("No valid GPU device found!");
}

pub fn init_vulkan() -> GpuContext {
    let entry = ash::Entry::linked();

    let app_name = CString::new("PulseGPU").unwrap();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk::API_VERSION_1_3);

    let instance_info = vk::InstanceCreateInfo::builder().application_info(&app_info);
    let instance = unsafe {
        entry
            .create_instance(&instance_info, None)
            .expect("Failed to create Vulkan instance")
    };

    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate Vulkan physical devices")
    };

    let physical_device = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate Vulkan physical devices")
    }
        .into_iter()
        .next()
        .expect("No Vulkan physical devices found!");

    for &pd in unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
            .iter()
    } {
        let props = unsafe { instance.get_physical_device_properties(pd) };
        let name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        println!("🔍 Found Vulkan device: {}", name.to_str().unwrap_or_default());
    }

    let properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) };
    println!("Using Vulkan device: {}", name.to_str().unwrap_or_default());

    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let queue_family_index = queue_families
        .iter()
        .enumerate()
        .position(|(i, qf)| qf.queue_flags.contains(vk::QueueFlags::COMPUTE))
        .expect("No Vulkan queue family found") as u32;

    let priorities = [1.0];
    let queue_info = vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&priorities);

    let device_info = vk::DeviceCreateInfo::builder().queue_create_infos(std::slice::from_ref(&queue_info));

    let device = unsafe {
        instance
            .create_device(physical_device, &device_info, None)
            .expect("Failed to create Vulkan device")
    };

    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    GpuContext {
        entry,
        instance,
        device,
        physical_device,
        queue,
        queue_family_index,
    }
}