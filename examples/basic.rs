use glslcc_rs::{glsl, Compiler, Shader, shaderc::ShaderKind};

fn main() {
    let compiler = Compiler::<glsl::Target>::new().unwrap();
    let shader = compiler
        .compile(Shader {
            shader_kind: ShaderKind::Vertex,
            source: include_str!("main.vert"), // include_str! can be easily replaced by the File API
        })
        .unwrap();

    println!("{}", shader);
}
