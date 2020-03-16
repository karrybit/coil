use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // vertex shaderの初期化
    // vertex shader用のプログラムをコンパイルする
    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec2 position;
        uniform vec2 resolution;
        void main() {
            vec2 zeroToOne = position / resolution;
            vec2 zeroToTwo = zeroToOne * 2.0;
            vec2 clipSpace = zeroToTwo - 1.0;
            gl_Position = vec4(clipSpace, 0, 1);
        }
    "#,
    )?;
    // fragment shaderの初期化
    // fragment shader用のプログラムをコンパイルする
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        void main() {
            gl_FragColor = vec4(1, 0, 0.5, 1);
        }
    "#,
    )?;
    // コンパイルしたshdaerプログラムをリンクする
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    // context.use_program(Some(&program));
    let position_attribute_location = context.get_attrib_location(&program, "position");
    let resolution_uniform_location = context.get_uniform_location(&program, "resolution");

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let vertices: [f32; 12] = [
        10.0, 20.0, 80.0, 20.0, 10.0, 30.0, 10.0, 30.0, 80.0, 20.0, 80.0, 30.0,
    ];
    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        // vertexの配列
        let vert_array = js_sys::Float32Array::view(&vertices);

        // GPUにあるbufferにpositionをアップロードする
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.use_program(Some(&program));

    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let size = 2; // 2 components per iteration
    let type_ = WebGlRenderingContext::FLOAT; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0; // start at the beginning of the buffer
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        size,
        type_,
        normalize,
        stride,
        offset,
    );

    context.uniform2f(
        resolution_uniform_location.as_ref(),
        canvas.width() as f32,
        canvas.height() as f32,
    );

    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    Ok(())
}

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
