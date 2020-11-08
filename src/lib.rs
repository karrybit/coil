use image;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

mod attribute;
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

static mut PAGER: Option<Box<Pager>> = None;

#[wasm_bindgen]
impl Pager {
    pub fn initialize() -> Result<(), JsValue> {
        unsafe {
            if PAGER.is_some() {
                return Ok(())
            }
        }
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

        unsafe {
            PAGER = Some(Box::new(Pager{
                context,
                canvas,
                position_attribute_location,
                tex_coord_attribute_location,
            }));
        }

        Ok(())
    }

    fn inner() -> &'static Pager {
        unsafe {
            PAGER.as_ref().unwrap()
        }
    }

    #[wasm_bindgen]
    pub async fn up(speed: u32, before_data: Vec<u8>, after_data: Vec<u8>) -> Result<(), JsValue> {
        let before_image = to_rgba(before_data);
        let after_image = to_rgba(after_data);
        transition(speed, Direction::Up, Pager::inner().canvas.width(), Pager::inner().canvas.height(), before_image, after_image);
        Ok(())
    }
    #[wasm_bindgen]
    pub async fn right(speed: u32, before_data: Vec<u8>, after_data: Vec<u8>) -> Result<(), JsValue> {
        let before_image = to_rgba(before_data);
        let after_image = to_rgba(after_data);
        transition(speed, Direction::Right, Pager::inner().canvas.width(), Pager::inner().canvas.height(), before_image, after_image);
        Ok(())
    }
    #[wasm_bindgen]
    pub async fn down(speed: u32, before_data: Vec<u8>, after_data: Vec<u8>) -> Result<(), JsValue> {
        let before_image = to_rgba(before_data);
        let after_image = to_rgba(after_data);
        transition(speed, Direction::Down, Pager::inner().canvas.width(), Pager::inner().canvas.height(), before_image, after_image);
        Ok(())
    }
    #[wasm_bindgen]
    pub async fn left(speed: u32, before_data: Vec<u8>, after_data: Vec<u8>) -> Result<(), JsValue> {
        let before_image = to_rgba(before_data);
        let after_image = to_rgba(after_data);
        transition(speed, Direction::Left, Pager::inner().canvas.width(), Pager::inner().canvas.height(), before_image, after_image);
        Ok(())
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

fn draw(image: &[u8], x: f32, y: f32) {
    unsafe {
        let vert_array = js_sys::Uint8Array::view(&image);
        let _ = Pager::inner().context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                Pager::inner().canvas.width() as i32,
                Pager::inner().canvas.height() as i32,
                0,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&vert_array),
            );
    }

    attribute::setup(
        &Pager::inner().context,
        (x, y),
        Pager::inner().canvas.width() as f32,
        Pager::inner().canvas.height() as f32,
        Pager::inner().position_attribute_location,
    );
    attribute::setup(
        &Pager::inner().context,
        (0f32, 0f32),
        1f32,
        1f32,
        Pager::inner().tex_coord_attribute_location,
    );

    Pager::inner().context.draw_arrays(web_sys::WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn calc_before_position(i: u32, width: u32, height: u32, direction: Direction) -> (f32, f32) {
    match direction {
        Direction::Up => {
            (0f32, std::cmp::max(-(i as i32), -(height as i32)) as f32)
        },
        Direction::Right => {
            (std::cmp::min(i, width) as f32, 0f32)
        },
        Direction::Down => {
            (0f32, std::cmp::min(i, height) as f32)
        },
        Direction::Left => {
            (std::cmp::max(-(i as i32), -(width as i32)) as f32, 0f32)
        }
    }
}

fn calc_after_position(i: u32, width: u32, height: u32, direction: Direction) -> (f32, f32) {
    match direction {
        Direction::Up => {
            (0f32, std::cmp::max(0, height-i) as f32)
        },
        Direction::Right => {
            (std::cmp::min(0, -((width-i) as i32)) as f32, 0f32)
        },
        Direction::Down => {
            (0f32, std::cmp::min(0, -((height-i) as i32)) as f32)
        },
        Direction::Left => {
            (std::cmp::max(0, width-i) as f32, 0f32)
        }
    }
}

fn transition(a: u32, direction: Direction, width: u32, height: u32, before_image: Vec<u8>, after_image: Vec<u8>) {
    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    let speed = width / a;
    let mut progress = 0u32;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if progress > width {
            return;
        }

        let (bx, by) = calc_before_position(progress, width, height, direction);
        draw(&before_image, bx, by);

        let (ax, ay) = calc_after_position(progress, width, height, direction);
        draw(&after_image, ax, ay);

        let status = get_element_by("speed");
        let before_image = get_element_by("before_image");
        let after_image = get_element_by("after_image");
        status.set_text_content(Some(format!("speed: {}, progress: {}", speed, progress).as_str()));
        before_image.set_text_content(Some(format!("x: {}, y: {}", bx, by).as_str()));
        after_image.set_text_content(Some(format!("x: {}, y: {}", ax, ay).as_str()));

        progress += speed;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
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
