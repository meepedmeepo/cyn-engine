
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;
@group(2) @binding(0)
var<uniform> spritesheet: vec2<u32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct SpriteClip {
    row: u32,
    col: u32,
    clip_width: u32,
    clip_height: u32,
}

//todo: change sprite_clip so it is calculated in vertex shader?
struct InstanceInput {
    @location(4) sprite_clip: vec4<f32>,
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );

    let sprite_clip = SpriteClip(
        instance.sprite_clip.r,
        instance.sprite_clip.g,
        instance.sprite_clip.b,
        instance.sprite_clip.a,
    );

    //todo: add a uniform for SpriteSheet size and add it to the correct bindgroup and render pipeline layout
    let dx = sprite_clip.clip_width / spritesheet.x;
    let dy = sprite_clip.clip_height / spritesheet.y;

    let uv_x = dx * model.tex_coords.x + sprite_clip.row;
    let ux_y = dy * model.tex_coords.y + sprite_clip.col

    var out: VertexOutput;
    //change tex cords to be affected by sprite_clip
    out.tex_coords = vec2<f32>(uv_x, uv_y);
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);

    return out;
}


@fragment

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
