#[cfg(feature = "glsl")]
pub use spirv_cross::glsl;

#[cfg(feature = "hlsl")]
pub use spirv_cross::hlsl;

#[cfg(feature = "msl")]
pub use spirv_cross::msl;

use derive_more::{Display, Error};
pub use shaderc::ShaderKind;
pub use spirv_cross::spirv;
use spirv_cross::spirv::{Ast, Target};

/** An error wrapper that can be used within this
library to efficiently handle errors from
both [shaderc] and [spirv_cross]. */
#[derive(Debug, Display, Error)]
pub enum Error {
    ShaderC(shaderc::Error),
    SpirVCross(spirv_cross::ErrorCode),
}

pub struct Compiler<'a, T>
where
    T: spirv::Target,
    Ast<T>: spirv::Compile<T> + spirv::Parse<T>,
{
    inner: shaderc::Compiler,
    pub shaderc_options: Option<shaderc::CompileOptions<'a>>,
    pub spirv_options: Option<<Ast<T> as spirv::Compile<T>>::CompilerOptions>,
}

impl<T: Target> Compiler<'_, T>
where
    T: spirv::Target,
    Ast<T>: spirv::Compile<T> + spirv::Parse<T>,
{
    pub fn new() -> Option<Self> {
        Some(Self {
            inner: shaderc::Compiler::new()?,
            shaderc_options: None,
            spirv_options: None,
        })
    }

    pub fn compile(&self, shader: Shader<'_>) -> Result<String, Error> {
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
                self.shaderc_options.as_ref(),
            )
            .map_err(Error::ShaderC)?;

        let module = spirv::Module::from_words(artifact.as_binary());
        let mut ast = Ast::<T>::parse(&module).map_err(Error::SpirVCross)?;
        self.spirv_options
            .as_ref()
            .map(|o| ast.set_compiler_options(o));

        ast.compile().map_err(Error::SpirVCross)
    }
}

/** A simple shader source representation. */
pub struct Shader<'a> {
    pub shader_kind: ShaderKind,
    pub source: &'a str,
}
