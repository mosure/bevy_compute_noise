use bevy::{prelude::*, ecs::component::Component, reflect::Reflect, render::{render_resource::{BindGroup, BindGroupLayout, ShaderRef}, renderer::RenderDevice}};

pub mod worley_2d;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
pub use worley_2d::Worley2D;

use crate::{image::ComputeNoiseSize, prelude::ComputeNoiseQueue};

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect {
    type Gpu: GpuComputeNoise;
    
    fn gpu_data(&self, size: ComputeNoiseSize) -> Self::Gpu;
    fn shader() -> ShaderRef;
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout;
}
pub trait GpuComputeNoise: Sync + Send + 'static + Default + Clone {
    fn to_bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup;
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct ComputeNoiseComponent<T: ComputeNoise> {
    pub image: Handle<Image>,
    pub noise: T,
}

pub fn update_noise<T: ComputeNoise>(
    mut noise_queue: ResMut<ComputeNoiseQueue<T>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&ComputeNoiseComponent<T>, Changed<ComputeNoiseComponent<T>>>,
) {
    for noise in query.iter() {
        noise_queue.add_image(&mut images, noise.image.clone(), noise.noise.clone());
    }
}