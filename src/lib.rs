use rand;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    log(format!("canvas: {:#?}", canvas).as_ref());

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    log(format!("context: {:#?}", context).as_ref());

    // vertex shaderの初期化
    // vertex shader用のプログラムをコンパイルする
    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec2 a_position;

        uniform vec2 u_resolution;
        
        void main() {
           // convert the rectangle from pixels to 0.0 to 1.0
           vec2 zeroToOne = a_position / u_resolution;
        
           // convert from 0->1 to 0->2
           vec2 zeroToTwo = zeroToOne * 2.0;
        
           // convert from 0->2 to -1->+1 (clipspace)
           vec2 clipSpace = zeroToTwo - 1.0;
        
           gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        }
    "#,
    )?;
    log(format!("vertex_shader: {:#?}", vert_shader).as_ref());
    // fragment shaderの初期化
    // fragment shader用のプログラムをコンパイルする
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        uniform vec4 u_color;
        
        void main() {
           gl_FragColor = u_color;
        }
    "#,
    )?;
    log(format!("fragment_shader: {:#?}", frag_shader).as_ref());
    // コンパイルしたshdaerプログラムをリンクする
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    log(format!("program: {:#?}", program).as_ref());
    // context.use_program(Some(&program));
    let position_attribute_location = context.get_attrib_location(&program, "a_position");
    let resolution_uniform_location = context.get_uniform_location(&program, "u_resolution");
    let color_uniform_location = context.get_uniform_location(&program, "u_color");

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    log(format!("buffer: {:#?}", buffer).as_ref());
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    log(format!(
        "width: {:#?}, height: {:#?}",
        canvas.width(),
        canvas.height()
    )
    .as_ref());
    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

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

    for _ in 0..50 {
        set_rectangle(
            &context,
            random_int(300),
            random_int(300),
            random_int(300),
            random_int(300),
        );
    }

    context.uniform4f(
        color_uniform_location.as_ref(),
        rand::thread_rng().gen::<f32>(),
        rand::thread_rng().gen::<f32>(),
        rand::thread_rng().gen::<f32>(),
        1.0,
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

fn random_int(i: u32) -> u32 {
    rand::thread_rng().gen_range(0, i)
}

fn set_rectangle(context: &WebGlRenderingContext, x: u32, y: u32, width: u32, height: u32) {
    let x1 = x;
    let x2 = x + width;
    let y1 = y;
    let y2 = y + height;
    let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
    unsafe {
        let vert_array = js_sys::Uint32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
}
