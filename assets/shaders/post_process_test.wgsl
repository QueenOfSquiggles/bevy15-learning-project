#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec3<f32>
#endif
}

@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let offset = settings.intensity;
    return vec4<f32> (
        textureSample(screen_texture, texture_sampler, in.uv).r, // + vec2<f32>(offset, -offset)   ).r,
        textureSample(screen_texture, texture_sampler, in.uv).g, // + vec2<f32>(-offset, 0.0)      ).g,
        textureSample(screen_texture, texture_sampler, in.uv).b, // + vec2<f32>(0.0, offset)       ).b,
        1.0
    );
}