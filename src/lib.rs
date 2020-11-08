use image;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

mod attribute;
mod loader;
mod processor;
mod shader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Pager {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::WebGlRenderingContext,
    position_attribute_location: u32,
    tex_coord_attribute_location: u32,
}

#[wasm_bindgen]
impl Pager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Pager, JsValue> {
        let canvas = get_element_by("canvas").dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()?;

        let vert_shader = shader::new_vertex_shader(&context)?;
        let frag_shader = shader::new_fragment_shader(&context)?;
        let program = processor::link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let texture = context.create_texture();
        context.bind_texture(web_sys::WebGlRenderingContext::TEXTURE_2D, texture.as_ref());

        context.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_S,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_WRAP_T,
            web_sys::WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MIN_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );
        context.tex_parameteri(
            web_sys::WebGlRenderingContext::TEXTURE_2D,
            web_sys::WebGlRenderingContext::TEXTURE_MAG_FILTER,
            web_sys::WebGlRenderingContext::NEAREST as i32,
        );

        context.pixel_storei(web_sys::WebGlRenderingContext::UNPACK_ALIGNMENT, 1);

        let position_attribute_location = context.get_attrib_location(&program, "a_position") as u32;
        context.enable_vertex_attrib_array(position_attribute_location);
        let tex_coord_attribute_location = context.get_attrib_location(&program, "a_texCoord") as u32;
        context.enable_vertex_attrib_array(tex_coord_attribute_location);

        let (width, height) = (canvas.width(), canvas.height());
        context.viewport(0, 0, width as i32, height as i32);

        let resolution_location = context.get_uniform_location(&program, "u_resolution");
        context.uniform2f(
            resolution_location.as_ref(),
            width as f32,
            height as f32,
        );

        Ok(Pager{
            context,
            canvas,
            position_attribute_location,
            tex_coord_attribute_location,
        })
    }

    pub fn log(&self) {
        unsafe {
            log(";lasdkfj;lsadfjlasd");
        }
    }
}

fn to_rgba(data: Vec<u8>) -> Vec<u8> {
    let img = image::load_from_memory(&data);
    let img = img.unwrap();
    let img = img.to_rgba();
    let pixels = img.pixels();
    let rgba = pixels
        .map(|ref rgba| rgba.0.iter())
        .flatten()
        .collect::<Vec<&u8>>();
    let rgba = rgba.clone().into_iter().copied().collect::<Vec<u8>>();
    rgba
}

#[wasm_bindgen]
pub async fn transition(before_data: Vec<u8>, after_data: Vec<u8>, rev: bool) -> Result<(), JsValue> {
    let pager = Pager::new()?;

    let before_rgba = to_rgba(before_data);
    let after_rgba = to_rgba(after_data);

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    let diff = pager.canvas.width() / 10;
    let mut i = 0u32;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > pager.canvas.width() {
            let t = get_element_by("t");
            t.set_text_content(Some("All done!"));
            let _ = f.borrow_mut().take();
            return;
        }

        unsafe {
            let vert_array = js_sys::Uint8Array::view(&before_rgba);
            let _ = pager.context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    web_sys::WebGlRenderingContext::TEXTURE_2D,
                    0,
                    web_sys::WebGlRenderingContext::RGBA as i32,
                    pager.canvas.width() as i32,
                    pager.canvas.height() as i32,
                    0,
                    web_sys::WebGlRenderingContext::RGBA,
                    web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                    Some(&vert_array),
                );
        }

        let x = if rev {
            std::cmp::min(i, pager.canvas.width()) as f32
        } else {
            std::cmp::max(-(i as i32), -(pager.canvas.height() as i32)) as f32
        };
        let y = 0.0;
        unsafe {
            log(format!("i:{}, x:{}, y:{}", i, x, y).as_str());
        }
        // attributeの設定
        attribute::setup(
            &pager.context,
            (x, y),
            pager.canvas.width() as f32,
            pager.canvas.height() as f32,
            pager.position_attribute_location,
        );
        attribute::setup(
            &pager.context,
            (0.0, 0.0),
            1.0,
            1.0,
            pager.tex_coord_attribute_location,
        );

        pager.context.draw_arrays(web_sys::WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);

        unsafe {
            let vert_array = js_sys::Uint8Array::view(&after_rgba);
            let _ = pager.context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    web_sys::WebGlRenderingContext::TEXTURE_2D,
                    0,
                    web_sys::WebGlRenderingContext::RGBA as i32,
                    pager.canvas.width() as i32,
                    pager.canvas.height() as i32,
                    0,
                    web_sys::WebGlRenderingContext::RGBA,
                    web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                    Some(&vert_array),
                );
        }
        let x = if rev {
            std::cmp::min(0, -((pager.canvas.width()-i) as i32)) as f32
        } else {
            std::cmp::max(0, pager.canvas.width()-i) as f32
        };
        let y = 0.0;
        unsafe {
            log(format!("i:{}, x:{}, y:{}", i, x, y).as_str());
        }

        // attributeの設定
        attribute::setup(
            &pager.context,
            (x, y),
            pager.canvas.width() as f32,
            pager.canvas.height() as f32,
            pager.position_attribute_location as _,
        );
        attribute::setup(
            &pager.context,
            (0.0, 0.0),
            1.0,
            1.0,
            pager.tex_coord_attribute_location as _
        );

        pager.context.draw_arrays(web_sys::WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);

        i += diff;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no document")
}

fn get_element_by(id: &str) -> web_sys::Element {
    document().get_element_by_id(id).expect(format!("no exist element by id: {}", id).as_str())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
