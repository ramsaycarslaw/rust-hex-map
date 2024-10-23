@fragment
fn fs_main(@location(0) in_uv: vec2<f32>) -> @location(0) vec4<f32> {
    let scale: f32 = 5.0; // Adjust the scale of the clouds
    let noiseValue = noise(in_uv * scale);
    let cloud = smoothstep(0.3, 0.5, noiseValue); // Create soft edges for clouds
    
    // Set cloud color
    return vec4<f32>(1.0, 1.0, 1.0, cloud); // White clouds
}

// Simple 2D noise function (pseudo-random)
fn noise(coord: vec2<f32>) -> f32 {
    let x: f32 = fract(coord.x);
    let y: f32 = fract(coord.y);
    
    // Smoothstep to create soft edges
    let fX: f32 = smoothstep(0.0, 1.0, x);
    let fY: f32 = smoothstep(0.0, 1.0, y);
    
    // A pseudo-random value based on coordinate
    return fract(sin(dot(coord, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}


