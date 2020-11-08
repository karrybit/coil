use web_sys;

pub fn setup(
    context: &web_sys::WebGlRenderingContext,
    start_point: (f32, f32),
    width: f32,
    height: f32,
    indx: u32,
) {
    let position_buffer = context.create_buffer().unwrap();
    // .ok_or("failed to create position buffer")?;
    context.bind_buffer(web_sys::WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let (x1, y1) = start_point;
    let x2 = x1 + width;
    let y2 = y1 + height;
    unsafe {
        let arr = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
        let vert_array = js_sys::Float32Array::view(&arr);
        context.buffer_data_with_array_buffer_view(
            web_sys::WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            web_sys::WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // ARRAY_BUFFERに結合されたバッファーを、現在の頂点バッファーオブジェクトの一般的な頂点属性に結合して、そのレイアウトを指定する
    context.vertex_attrib_pointer_with_f64(
        indx,
        2,
        web_sys::WebGlRenderingContext::FLOAT,
        false,
        0,
        0.0,
    );
}
