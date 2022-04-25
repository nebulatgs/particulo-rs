@vertex
fn vs_main(@location(0)  aVertexPosition: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(aVertexPosition, 1.0, 1.0);
}

fn aastep(threshold: f32, value: f32) -> f32 {
    let afwidth = 0.7 * length(vec2<f32>(dpdx(value), dpdy(value)));
    return smoothstep(threshold - afwidth, threshold + afwidth, value);
}

@fragment
fn fs_main(@builtin(position) inp: vec4<f32>) -> @location(0) vec4<f32> {
    let r: f32 = 1. - aastep(100., length(inp - vec4<f32>(100., 100., 0., 0.)));

    return vec4<f32>(r, r, r, 1.);
}