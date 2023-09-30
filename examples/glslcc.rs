use glslcc_rs::{glsl, Compiler, Shader, ShaderKind};

fn main() {
    let compiler = Compiler::<glsl::Target>::new().unwrap();
    let shader = compiler
        .compile(Shader {
            shader_kind: ShaderKind::Vertex,
            source: include_str!("main.vert"),
        })
        .unwrap();

    println!("{}", shader);
}
