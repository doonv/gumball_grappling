struct ToonShaderOutlineMaterial {
    color: vec4<f32>,
    sun_dir: vec3<f32>,
    sun_color: vec4<f32>,
    camera_pos: vec3<f32>,
    ambient_color: vec4<f32>,
    outline_color: vec4<f32>
};

@group(1) @binding(0)
var<uniform> material: ToonShaderOutlineMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment (in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = material.color * textureSample(base_color_texture, base_color_sampler, in.uv);
    let normal = normalize(in.world_normal);
    let n_dot_l = dot(material.sun_dir, normal);
    var light_intensity = 0.0;

    if n_dot_l > 0.0 {
        let bands = 3.0;
        var x = n_dot_l * bands;

        x = round(x);

        light_intensity = x / bands;
    } else {
        light_intensity = 0.0;
    }

    let light = light_intensity * material.sun_color.rgb;

    let view_dir: vec3<f32> = normalize(material.camera_pos - in.world_position.xyz);

    let half_vector = normalize(material.sun_dir + view_dir);
    let n_dot_h = dot(normal, half_vector);
    let glossiness = 32.0;
    let specular_intensity = pow(n_dot_h, glossiness * glossiness);

    let specular_intensity_smooth = smoothstep(0.005, 0.01, specular_intensity);
    let specular = specular_intensity_smooth * vec4<f32>(0.9, 0.9 ,0.9 , 0.0);

    let rim_dot = 1.0 - dot(view_dir, normal);

    if rim_dot > 0.5 && material.outline_color.a > 0.0 {
        return material.outline_color * base_color.a;
    } else {
        return base_color * vec4<f32>(light.rgb + material.ambient_color.rgb + specular.rgb, 1.0);
    }

}
