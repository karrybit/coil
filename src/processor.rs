use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

// shaderのコンパイル
pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    // shaderを作成
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    // GLSLのコードをGPUにアップロード
    context.shader_source(&shader, source);
    // shaderをコンパイル
    context.compile_shader(&shader);

    // 成功判定
    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        // エラーハンドリング
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

// コンパイルしたshaderプログラムのリンク
pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    // プログラムを作成
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    // プログラムにvertex shaderを付ける
    context.attach_shader(&program, vert_shader);
    // プログラムにfragment shaderを付ける
    context.attach_shader(&program, frag_shader);
    // プログラムをリンクする
    context.link_program(&program);

    // 成功判定
    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        // エラーハンドリング
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
