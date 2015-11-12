
use std::borrow::Borrow;
use regl::{Context, Program, Shader, ShaderSource, ShaderType};
use ReframeResult;

pub fn create_program_simple(context: &mut Context, vs: &str, fs: &str) -> ReframeResult<Program> {
    let sources = &[ShaderSource(ShaderType::VertexShader, vs),
                    ShaderSource(ShaderType::FragmentShader, fs)];
    create_program(context, sources)
}

pub fn create_program<'a, I, S>(context: &mut Context, sources: I) -> ReframeResult<Program>
    where I: IntoIterator<Item = S>,
          S: Borrow<ShaderSource<'a>>
{
    let sources = sources.into_iter();
    let shaders: Vec<Shader> = try!(sources.map(|s| Shader::new(context, s.borrow())).collect());
    Ok(try!(Program::new(context, &shaders[..])))
}
