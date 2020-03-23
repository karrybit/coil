use web_sys::{WebGlProgram, WebGlRenderingContext};

pub fn setup(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    attribute: &str,
    offset: f32,
) {
    let position_buffer = context.create_buffer().unwrap();
    // .ok_or("failed to create position buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let x1 = 0.0;
    let x2 = 0.0 + offset;
    let y1 = 0.0;
    let y2 = 0.0 + offset;
    unsafe {
        let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        let vert_array = js_sys::Float32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let position_attribute_location = context.get_attrib_location(program, attribute);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

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
}
