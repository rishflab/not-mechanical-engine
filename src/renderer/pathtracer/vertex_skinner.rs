use gfx_hal::{Backend, pso, prelude::*};

use gfx_hal::pso::DescriptorPool;

use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;
use std::path::Path;
use crate::renderer::ENTRY_NAME;
use crate::renderer::core::device::DeviceState;
use crate::renderer::shaders::vertex_skinner_spirv;

pub struct VertexSkinner<B: Backend> {
    pub shader: B::ShaderModule,
    pub set_layout: B::DescriptorSetLayout,
    pub layout: B::PipelineLayout,
    pub pipeline: B::ComputePipeline,
    pub desc_set: B::DescriptorSet,
    pub pool: B::DescriptorPool,
}

impl<B: Backend> VertexSkinner<B> {


    pub unsafe fn write_desc_set(&self, device_state: Rc<RefCell<DeviceState<B>>>,
                                 model_buffer: &B::Buffer, vertex_buffer: &B::Buffer,
                                 triangle_buffer: &B::Buffer, index_buffer: &B::Buffer){

        device_state
            .borrow()
            .device
            .write_descriptor_sets(vec![
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(index_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(vertex_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 2,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(triangle_buffer, None..None)),
                },
                pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 3,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Buffer(model_buffer, None..None)),
                },
            ]);

    }

    pub unsafe fn new(device_state: Rc<RefCell<DeviceState<B>>>) -> Self {

        let device = &device_state
            .borrow()
            .device;

        let shader = device.create_shader_module(&vertex_skinner_spirv()).expect("Could not load shader module");

        let set_layout = device.create_descriptor_set_layout(
            &[
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 2,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 3,
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::COMPUTE,
                    immutable_samplers: false,
                },
            ],
            &[],
        ).expect("Camera ray set layout creation failed");

        let mut pool = device.create_descriptor_pool(
            4,
            &[
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::StorageBuffer,
                    count: 4,
                },
            ],
            pso::DescriptorPoolCreateFlags::empty(),
        ).expect("Camera ray descriptor pool creation failed");


        let desc_set = pool.allocate_set(&set_layout).expect("Vertex skinner set allocation failed");

        let push_constants = vec![(pso::ShaderStageFlags::COMPUTE, 0..8)];

        let layout = device.create_pipeline_layout(Some(&set_layout), push_constants)
            .expect("Camera ray pipeline layout creation failed");

        let pipeline = {
            let shader_entry = pso::EntryPoint {
                entry: ENTRY_NAME,
                module: &shader,
                specialization: pso::Specialization::default(),
            };

            let pipeline_desc = pso::ComputePipelineDesc::new(
                shader_entry,
                &layout,
            );

            device.create_compute_pipeline(&pipeline_desc, None)
                .expect("Could not create vertex skinner pipeline")
        };

        VertexSkinner {
            shader,
            set_layout,
            pool,
            layout,
            desc_set,
            pipeline,
        }
    }

}