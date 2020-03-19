use image;
use reqwest;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

async fn fetch_image() -> std::result::Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .get("https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/180px-WebAssembly_Logo.svg.png")
        .send().await
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let res = fetch_image().await;
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
        attribute vec2 a_position;
        attribute vec2 a_texCoord;
        uniform vec2 u_resolution;
        varying vec2 v_texCoord;

        void main() {
            // convert the rectangle from pixels to 0.0 to 1.0
            vec2 zeroToOne = a_position / u_resolution;

            // convert from 0->1 to 0->2
            vec2 zeroToTwo = zeroToOne * 2.0;

            // convert from 0->2 to -1->+1 (clipspace)
            vec2 clipSpace = zeroToTwo - 1.0;

            gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);

            // pass the texCoord to the fragment shader
            // The GPU will interpolate this value between points.
            v_texCoord = a_texCoord;
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

        // our texture
        uniform sampler2D u_image;
        
        // the texCoords passed in from the vertex shader.
        varying vec2 v_texCoord;
        
        void main() {
            gl_FragColor = texture2D(u_image, v_texCoord);
        }
    "#,
    )?;
    // コンパイルしたshdaerプログラムをリンクする
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    let position_attribute_location = context.get_attrib_location(&program, "a_position");
    let tex_coord_location = context.get_attrib_location(&program, "a_texCoord");

    let position_buffer = context
        .create_buffer()
        .ok_or("failed to create position buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    set_rectangle(&context, 0.0, 0.0, 180.0, 180.0);

    let tex_coord_buffer = context
        .create_buffer()
        .ok_or("failed to create tex coord buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&tex_coord_buffer));
    unsafe {
        let arr = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];
        let vert_array = js_sys::Float32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let texture = context.create_texture();
    // context.active_texture(WebGlRenderingContext::TEXTURE0);
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());

    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );

    let bytes = res.unwrap().bytes().await.unwrap();
    log(format!("{}", bytes.len()).as_ref());
    log(format!("{:?}", bytes).as_ref());
    let b = bytes.iter().map(|&b| b as u8).collect::<Vec<u8>>();
    log(format!("{:?}", &b).as_ref());
    let img = image::load_from_memory(&b);
    let img = img.unwrap().to_rgba();
    let pixels = img.pixels();
    let rgba = pixels
        .map(|ref rgba| rgba.0.iter())
        .flatten()
        .collect::<Vec<&u8>>();
    let rgba = rgba.clone().into_iter().copied().collect::<Vec<u8>>();

    context.pixel_storei(WebGlRenderingContext::UNPACK_ALIGNMENT, 1);

    unsafe {
        // let vert_array = js_sys::Uint8Array::view(&b);
        let vert_array = js_sys::Uint8Array::view(&rgba);
        let _ = context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                180,
                180,
                0,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&vert_array),
            );
    }

    let resolution_location = context.get_uniform_location(&program, "u_resolution");

    canvas.set_width(180);
    canvas.set_height(180);

    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.use_program(Some(&program));

    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let size = 2; // 2 components per iteration
    let type_ = WebGlRenderingContext::FLOAT; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0.0; // start at the beginning of the buffer
    context.vertex_attrib_pointer_with_f64(
        position_attribute_location as u32,
        size,
        type_,
        normalize,
        stride,
        offset,
    );

    context.enable_vertex_attrib_array(tex_coord_location as u32);
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&tex_coord_buffer));

    let size = 2; // 2 components per iteration
    let type_ = WebGlRenderingContext::FLOAT; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0.0; // start at the beginning of the buffer
    context.vertex_attrib_pointer_with_f64(
        tex_coord_location as u32,
        size,
        type_,
        normalize,
        stride,
        offset,
    );

    context.uniform2f(
        resolution_location.as_ref(),
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

fn set_rectangle(context: &WebGlRenderingContext, x: f32, y: f32, width: f32, height: f32) {
    let x1 = x;
    let x2 = x + width;
    let y1 = y;
    let y2 = y + height;
    unsafe {
        let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        let vert_array = js_sys::Float32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
}
