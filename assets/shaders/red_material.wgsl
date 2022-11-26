struct RedMaterial {
	color: vec4<f32>,
};

fn rand2(n: vec2<f32>) -> f32 {
  return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn noise2(n: vec2<f32>) -> f32 {
  let d = vec2<f32>(0., 1.);
  let b = floor(n);
  let f = smoothstep(vec2<f32>(0.), vec2<f32>(1.), fract(n));
  return mix(mix(rand2(b), rand2(b + d.yx), f.x), mix(rand2(b + d.xy), rand2(b + d.yy), f.x), f.y);
}

@group(1) @binding(0)
var<uniform> material: RedMaterial;

@fragment
fn fragment(
	@builtin(position) frag_coord: vec4<f32>,
	#import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
	let n = step(0.8, noise2(frag_coord.xy));
	let f = vec4<f32>(vec3<f32>(n), 1.0);
	return material.color * f;
}
