use image::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod attribute;
mod loader;
mod processor;
mod shader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let wasm_image = loader::fetch_image(loader::FetchURLType::WASM).await;
    let js_image = loader::fetch_image(loader::FetchURLType::JS).await;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let bytes = js_image.unwrap().bytes().await.unwrap();
    let bytes = bytes.iter().map(|&b| b as u8).collect::<Vec<u8>>();
    let img = image::load_from_memory(&bytes);
    let img = img.unwrap();
    let (width, height) = img.dimensions();

    canvas.set_width(width * 2);
    canvas.set_height(height);

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // vertex shaderの初期化
    // vertex shader用のプログラムをコンパイルする
    let vert_shader = shader::new_vertex_shader(&context)?;
    // fragment shaderの初期化
    // fragment shader用のプログラムをコンパイルする
    let frag_shader = shader::new_fragment_shader(&context)?;
    // コンパイルしたshdaerプログラムをリンクする
    let program = processor::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

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

    let img = img.to_rgba();
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
                width as i32,
                height as i32,
                0,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&vert_array),
            );
    }

    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // attributeの設定
    attribute::setup(
        &context,
        &program,
        "a_position",
        (0.0, 0.0),
        width as f32,
        height as f32,
    );
    attribute::setup(&context, &program, "a_texCoord", (0.0, 0.0), 1.0, 1.0);

    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    let resolution_location = context.get_uniform_location(&program, "u_resolution");
    context.uniform2f(
        resolution_location.as_ref(),
        canvas.width() as f32,
        canvas.height() as f32,
    );
    context.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);

    // attributeの設定
    attribute::setup(
        &context,
        &program,
        "a_position",
        (width as f32, 0.0),
        width as f32,
        height as f32,
    );
    attribute::setup(&context, &program, "a_texCoord", (0.0, 0.0), 1.0, 1.0);

    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    let resolution_location = context.get_uniform_location(&program, "u_resolution");
    context.uniform2f(
        resolution_location.as_ref(),
        canvas.width() as f32,
        canvas.height() as f32,
    );
    context.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);

    Ok(())
}
