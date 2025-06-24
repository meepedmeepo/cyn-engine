use bytemuck::{Pod, Zeroable};
use cgmath::{Point3, Vector3};
use wgpu::util::DeviceExt;

use crate::utils::OPENGL_TO_WGPU_MATRIX;

pub struct Camera {
    eye: cgmath::Point3<f32>,
    focus: cgmath::Point3<f32>,
    up_dir: cgmath::Vector3<f32>,
    aspect: f32,
    fov: f32,
    z_near: f32,
    z_far: f32,
    zoom: f32,
}

impl Camera {
    pub fn new(
        eye: cgmath::Point3<f32>,
        focus: cgmath::Point3<f32>,
        up_dir: cgmath::Vector3<f32>,
        aspect: f32,
        fov: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Camera {
            eye,
            focus,
            up_dir,
            aspect,
            fov,
            z_near,
            z_far,
            zoom: 2.5,
        }
    }

    ///Currently uses perspective but need to change to Orthographic camera.
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.focus, self.up_dir);

        //let projection =
        //    cgmath::perspective(cgmath::Deg(self.fov), self.aspect, self.z_near, self.z_far);

        let width = self.zoom;
        let height = width / self.aspect;

        let left = width * -0.5;
        let right = width * 0.5;
        let bottom = height * -0.5;
        let top = height * 0.5;

        let proj = cgmath::ortho(left, right, bottom, top, self.z_near, self.z_far);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

fn create_camera_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("camera_bindgroup_layout"),
    })
}

fn create_camera_bind_group(
    device: &wgpu::Device,
    camera_bindgroup_layout: &wgpu::BindGroupLayout,
    camera_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("camera_bindgroup"),
        layout: &camera_bindgroup_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
    })
}

fn create_camera_buffer(device: &wgpu::Device, camera_uniform: CameraUniform) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

pub struct CameraResource {
    label: String,
    camera: Camera,
    camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group: wgpu::BindGroup,
}

impl CameraResource {
    pub fn new<S>(device: &wgpu::Device, screen_dimensions: (f32, f32), label: S) -> Self
    where
        S: Into<String>,
    {
        let layout = create_camera_bindgroup_layout(&device);
        let camera = Camera::new(
            Point3::new(0., 1., 2.),
            Point3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Vector3::unit_y(),
            screen_dimensions.0 / screen_dimensions.1,
            45.0,
            0.01,
            5.0,
        );

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = create_camera_buffer(device, camera_uniform);

        let bind_group = create_camera_bind_group(device, &layout, &camera_buffer);

        CameraResource {
            camera,
            camera_bind_group_layout: layout,
            camera_bind_group: bind_group,
            label: label.into(),
            camera_buffer,
            camera_uniform,
        }
    }
}
