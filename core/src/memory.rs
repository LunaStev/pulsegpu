use ash::vk;
use std::ptr;

pub struct GpuBuffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: vk::DeviceSize,
}

impl GpuBuffer {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        memory_props: vk::MemoryPropertyFlags,
    ) -> Self {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe {
            device
                .create_buffer(&buffer_info, None)
                .expect("Failed to create buffer")
        };

        let mem_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        let memory_type_index = find_memory_type(
            instance,
            physical_device,
            mem_requirements.memory_type_bits,
            memory_props,
        );

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe {
            device
                .allocate_memory(&alloc_info, None)
                .expect("Failed to allocate buffer memory")
        };

        unsafe {
            device
                .bind_buffer_memory(buffer, memory, 0)
                .expect("Failed to bind buffer memory");
        }

        Self {
            buffer,
            memory,
            size,
        }
    }

    pub fn upload<T: Copy>(&self, device: &ash::Device, data: &[T]) {
        let ptr = unsafe {
            device
                .map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty())
                .expect("Failed to map memory") as *mut T
        };

        unsafe {
            ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
            device.unmap_memory(self.memory);
        }
    }

    pub fn download<T: Copy>(&self, device: &ash::Device, output: &mut [T]) {
        let ptr = unsafe {
            device
                .map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty())
                .expect("Failed to map memory") as *const T
        };

        unsafe {
            output.copy_from_slice(std::slice::from_raw_parts(ptr, output.len()));
            device.unmap_memory(self.memory);
        }
    }
}

fn find_memory_type(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    let mem_properties = unsafe {
        instance.get_physical_device_memory_properties(physical_device)
    };

    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && mem_properties.memory_types[i as usize]
            .property_flags
            .contains(properties)
        {
            return i;
        }
    }

    panic!("Failed to find suitable memory type!");
}