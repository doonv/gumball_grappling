#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> color: vec4<f32>;
@group(1) @binding(1)
var<uniform> roundness: vec2<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    let temp = abs((uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(2.0, 2.0));
    if(temp.x > 1.0 - roundness.x && temp.y > 1.0 - roundness.y) {
        if(distance(vec2<f32>(1.0 - roundness.x, 1.0 - roundness.y), temp) > max(roundness.x, roundness.y)) {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    }
        
    return color;
}
