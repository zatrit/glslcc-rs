use std::fmt::Display;

pub use shaderc::ShaderKind;

#[cfg(feature = "glsl")]
pub use spirv_cross::glsl;

#[cfg(feature = "hlsl")]
pub use spirv_cross::hlsl;

#[cfg(feature = "msl")]
pub use spirv_cross::msl;

pub use spirv_cross::spirv;
use spirv_cross::spirv::Ast;

#[derive(Debug)]
pub enum Error {
    ShaderC(shaderc::Error),
    SpirVCross(spirv_cross::ErrorCode),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ShaderC(e) => e.fmt(f),
            Error::SpirVCross(e) => e.fmt(f),
        }
    }
}

pub struct Compiler<'a> {
    inner: shaderc::Compiler,
    pub options: Option<shaderc::CompileOptions<'a>>,
}

impl Compiler<'_> {
    pub fn new() -> Option<Self> {
        Some(Self {
            inner: shaderc::Compiler::new()?,
            options: None,
        })
    }

    pub fn compile<T>(&self, shader: &Shader<'_>, options: Option<<Ast<T> as spirv::Compile<T>>::CompilerOptions>) -> Result<String, Error>
    where
        T: spirv::Target,
        Ast<T>: spirv::Compile<T> + spirv::Parse<T>,
    {
        let Shader {
            shader_kind,
            source,
        } = shader;

        let artifact = self
            .inner
            .compile_into_spirv(
                source,
                shader_kind.clone(),
                "shader.glsl",
                "main",
                self.options.as_ref(),
            )
            .map_err(Error::ShaderC)?;

        let module = spirv::Module::from_words(artifact.as_binary());
        let mut ast = Ast::<T>::parse(&module).map_err(Error::SpirVCross)?;
        options.as_ref().map(|o| ast.set_compiler_options(o));

        ast.compile().map_err(Error::SpirVCross)
    }
}

pub struct Shader<'a> {
    pub shader_kind: ShaderKind,
    pub source: &'a str,
}
