use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod shader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Pager {
    index: usize,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::WebGlRenderingContext,
    position_attribute_location: u32,
    tex_coord_attribute_location: u32,
}

static mut PAGER: Option<Box<Pager>> = None;
static mut IMAGES: Vec<Vec<u8>> = vec![];

impl Pager {
    fn inner() -> &'static Pager {
        unsafe { PAGER.as_ref().unwrap() }
    }
    fn inner_mut() -> &'static mut Pager {
        unsafe { PAGER.as_mut().unwrap() }
    }
    fn images() -> (&'static [u8], &'static [u8]) {
        unsafe {
            (
                IMAGES.get(Pager::inner().index & 1).unwrap(),
                IMAGES.get((Pager::inner().index + 1) & 1).unwrap(),
            )
        }
    }
}

#[wasm_bindgen]
impl Pager {
    pub fn initialize(before_image: Vec<u8>, after_image: Vec<u8>) -> Result<(), JsValue> {
        unsafe {
            if PAGER.is_some() {
                return Ok(());
            }
        }
        let canvas = get_element_by("canvas").dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()?;

        let vert_shader = shader::new_vertex_shader(&context)?;
        let frag_shader = shader::new_fragment_shader(&context)?;
        let program = shader::link_program(&context, &vert_shader, &frag_shader)?;
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

        let position_attribute_location =
            context.get_attrib_location(&program, "a_position") as u32;
        context.enable_vertex_attrib_array(position_attribute_location);
        let tex_coord_attribute_location =
            context.get_attrib_location(&program, "a_texCoord") as u32;
        context.enable_vertex_attrib_array(tex_coord_attribute_location);

        let (width, height) = (canvas.width(), canvas.height());
        context.viewport(0, 0, width as i32, height as i32);

        let resolution_location = context.get_uniform_location(&program, "u_resolution");
        context.uniform2f(resolution_location.as_ref(), width as f32, height as f32);

        let before_image = to_rgba(before_image);
        let after_image = to_rgba(after_image);

        unsafe {
            IMAGES.push(before_image);
            IMAGES.push(after_image);
        }

        unsafe {
            PAGER = Some(Box::new(Pager {
                index: 0,
                context,
                canvas,
                position_attribute_location,
                tex_coord_attribute_location,
            }));
        }

        Ok(())
    }

    pub async fn up(interval: u32) -> Result<(), JsValue> {
        let inner = Pager::inner();
        let (before_image, after_image) = Pager::images();
        transition(
            interval,
            Direction::Up,
            inner.canvas.width(),
            inner.canvas.height(),
            before_image,
            after_image,
            inner.position_attribute_location,
            inner.tex_coord_attribute_location,
        );
        Ok(())
    }

    pub async fn right(interval: u32) -> Result<(), JsValue> {
        let inner = Pager::inner();
        let (before_image, after_image) = Pager::images();
        transition(
            interval,
            Direction::Right,
            inner.canvas.width(),
            inner.canvas.height(),
            before_image,
            after_image,
            inner.position_attribute_location,
            inner.tex_coord_attribute_location,
        );
        Ok(())
    }

    pub async fn down(interval: u32) -> Result<(), JsValue> {
        let inner = Pager::inner();
        let (before_image, after_image) = Pager::images();
        transition(
            interval,
            Direction::Down,
            inner.canvas.width(),
            inner.canvas.height(),
            before_image,
            after_image,
            inner.position_attribute_location,
            inner.tex_coord_attribute_location,
        );
        Ok(())
    }

    pub async fn left(interval: u32) -> Result<(), JsValue> {
        let inner = Pager::inner();
        let (before_image, after_image) = Pager::images();
        transition(
            interval,
            Direction::Left,
            inner.canvas.width(),
            inner.canvas.height(),
            before_image,
            after_image,
            inner.position_attribute_location,
            inner.tex_coord_attribute_location,
        );
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
    rgba.clone().into_iter().copied().collect::<Vec<u8>>()
}

fn transition(
    interval: u32,
    direction: Direction,
    width: u32,
    height: u32,
    before_image: &'static [u8],
    after_image: &'static [u8],
    position_attribute_location: u32,
    tex_coord_attribute_location: u32,
) {
    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    let speed = width / interval;
    let mut progress = 0u32;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if progress > width {
            progress = width
        }

        let (bx, by) = calc_position(progress, width, height, direction, Position::Before);
        draw(
            &before_image,
            bx,
            by,
            width,
            height,
            position_attribute_location,
            tex_coord_attribute_location,
        );

        let (ax, ay) = calc_position(progress, width, height, direction, Position::After);
        draw(
            &after_image,
            ax,
            ay,
            width,
            height,
            position_attribute_location,
            tex_coord_attribute_location,
        );

        let status = get_element_by("progress");
        let before_image = get_element_by("before_image");
        let after_image = get_element_by("after_image");
        status.set_text_content(Some(format!("progress: {}", progress).as_str()));
        before_image.set_text_content(Some(format!("x: {}, y: {}", bx, by).as_str()));
        after_image.set_text_content(Some(format!("x: {}, y: {}", ax, ay).as_str()));

        if progress == width {
            return;
        }

        progress += speed;

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Pager::inner_mut().index += 1;
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
enum Position {
    Before,
    After,
}

fn calc_position(
    progress: u32,
    width: u32,
    height: u32,
    direction: Direction,
    position: Position,
) -> (f32, f32) {
    use Direction::*;
    use Position::*;

    match (direction, position) {
        (Up, Before) => (
            0f32,
            std::cmp::max(-(progress as i32), -(height as i32)) as f32,
        ),
        (Up, After) => (0f32, std::cmp::max(0, height - progress) as f32),
        (Right, Before) => (std::cmp::min(progress, width) as f32, 0f32),
        (Right, After) => (std::cmp::min(0, -((width - progress) as i32)) as f32, 0f32),
        (Down, Before) => (0f32, std::cmp::min(progress, height) as f32),
        (Down, After) => (0f32, std::cmp::min(0, -((height - progress) as i32)) as f32),
        (Left, Before) => (
            std::cmp::max(-(progress as i32), -(width as i32)) as f32,
            0f32,
        ),
        (Left, After) => (std::cmp::max(0, width - progress) as f32, 0f32),
    }
}

fn draw(
    image: &[u8],
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    position_attribute_location: u32,
    tex_coord_attribute_location: u32,
) {
    unsafe {
        let vert_array = js_sys::Uint8Array::view(&image);
        let _ = Pager::inner()
            .context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                web_sys::WebGlRenderingContext::TEXTURE_2D,
                0,
                web_sys::WebGlRenderingContext::RGBA as i32,
                width as i32,
                height as i32,
                0,
                web_sys::WebGlRenderingContext::RGBA,
                web_sys::WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&vert_array),
            );
    }

    setup_buffer(
        (x, y),
        width as f32,
        height as f32,
        position_attribute_location,
    );
    setup_buffer((0f32, 0f32), 1f32, 1f32, tex_coord_attribute_location);

    Pager::inner()
        .context
        .draw_arrays(web_sys::WebGlRenderingContext::TRIANGLE_STRIP, 0, 6);
}

pub fn setup_buffer(start_point: (f32, f32), width: f32, height: f32, indx: u32) {
    let position_buffer = Pager::inner().context.create_buffer().unwrap();
    Pager::inner().context.bind_buffer(
        web_sys::WebGlRenderingContext::ARRAY_BUFFER,
        Some(&position_buffer),
    );

    let (x1, y1) = start_point;
    let x2 = x1 + width;
    let y2 = y1 + height;
    unsafe {
        let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        let vert_array = js_sys::Float32Array::view(&arr);
        Pager::inner().context.buffer_data_with_array_buffer_view(
            web_sys::WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            web_sys::WebGlRenderingContext::STATIC_DRAW,
        );
    }

    Pager::inner().context.vertex_attrib_pointer_with_f64(
        indx,
        2,
        web_sys::WebGlRenderingContext::FLOAT,
        false,
        0,
        0.0,
    );
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no document")
}

fn get_element_by(id: &str) -> web_sys::Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("no exist element by id: {}", id))
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
