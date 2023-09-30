use glslcc_rs::{Compiler, Shader, ShaderKind, glsl};

fn main() {
    let compiler = Compiler::new().unwrap();
    let shader = compiler
        .compile::<glsl::Target>(
            &Shader {
                shader_kind: ShaderKind::Vertex,
                source: include_str!("main.vert"),
            },
            None,
        )
        .unwrap();

    println!("{}", shader);
}
