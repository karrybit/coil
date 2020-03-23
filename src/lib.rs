use image;
use reqwest;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod processor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

async fn fetch_image(url: &str) -> std::result::Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new().get(url).send().await
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let wasm_image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/180px-WebAssembly_Logo.svg.png";
    let wasm_image = fetch_image(wasm_image_url).await;
    let js_image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png";
    let js_image = fetch_image(js_image_url).await;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // vertex shaderの初期化
    // vertex shader用のプログラムをコンパイルする
    let vert_shader = processor::compile_shader(
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
    let frag_shader = processor::compile_shader(
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
    let program = processor::link_program(&context, &vert_shader, &frag_shader)?;
    let position_attribute_location = context.get_attrib_location(&program, "a_position");
    let tex_coord_location = context.get_attrib_location(&program, "a_texCoord");

    let position_buffer = context
        .create_buffer()
        .ok_or("failed to create position buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let x1 = 0.0;
    let x2 = 0.0 + 180.0;
    let y1 = 0.0;
    let y2 = 0.0 + 180.0;
    unsafe {
        let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        let vert_array = js_sys::Float32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let tex_coord_buffer = context
        .create_buffer()
        .ok_or("failed to create tex coord buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&tex_coord_buffer));

    let x1 = 0.0;
    let x2 = 0.0 + 1.0;
    let y1 = 0.0;
    let y2 = 0.0 + 1.0;
    unsafe {
        let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        let vert_array = js_sys::Float32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // テクスチャの作成とbind
    let texture = context.create_texture();
    // context.active_texture(WebGlRenderingContext::TEXTURE0);
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());

    // textureの設定（どんな画像サイズでも表示できるように）
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

    let bytes = wasm_image.unwrap().bytes().await.unwrap();
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

    // 読み込みビット？バイト？の単位を1にする（デフォルトは4）
    context.pixel_storei(WebGlRenderingContext::UNPACK_ALIGNMENT, 1);

    // 画像データを流しこむ
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

    canvas.set_width(360);
    canvas.set_height(360);

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

    // ARRAY_BUFFERに結合されたバッファーを、現在の頂点バッファーオブジェクトの一般的な頂点属性に結合して、そのレイアウトを指定する
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

    // ARRAY_BUFFERに結合されたバッファーを、現在の頂点バッファーオブジェクトの一般的な頂点属性に結合して、そのレイアウトを指定する
    context.vertex_attrib_pointer_with_f64(
        tex_coord_location as u32,
        size,
        type_,
        normalize,
        stride,
        offset,
    );

    let resolution_location = context.get_uniform_location(&program, "u_resolution");
    context.uniform2f(
        resolution_location.as_ref(),
        canvas.width() as f32,
        canvas.height() as f32,
    );
    context.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);
    Ok(())
}
