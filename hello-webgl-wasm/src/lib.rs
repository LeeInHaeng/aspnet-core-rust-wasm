use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
extern crate js_sys;

pub fn init_webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    // dyn_into : 동적 캐스트를 수행
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        // get_context : https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext
        // webgl : WebGLRenderingContext , webgl2 : WebGL2RenderingContext , webgpu : GPUCanvasContext
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    // viewport : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/viewport
    gl.viewport(
        0,
        0,
        canvas.width().try_into().unwrap(),
        canvas.height().try_into().unwrap()
    );

    // gl 순서 : get_context --> viewport

    Ok(gl)
}

pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        // create_shader : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createShader
        // shader_type = VERTEX_SHADER : 모양을 정의하는 점을 처리 , FRAGMENT_SHADER : 모양 내 각 픽셀의 색상을 결정
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    // shader_source : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/shaderSource
    gl.shader_source(&shader, source);
    // compile_shader : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compileShader
    gl.compile_shader(&shader);

    // shader 순서 : create_shader --> shader_source --> compile_shader

    if gl
        // get_shader_parameter : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderParameter
        // DELETE_STATUS : 셰이더가 삭제 되었는지 , COMPILE_STATUS : 셰이더 컴파일 성공 했는지 , SHADER_TYPE : 셰이더가 정점 셰이더 개체인지 조각 셰이더 개체인지
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
            Ok(shader)
    } else {
        Err(JsValue::from_str(
            &gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into())
        ))
    }
}

pub fn setup_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = "
        attribute vec3 coordinates;
        void main(void) {
            gl_Position = vec4(coordinates, 1.0);
        }
        ";

    let fragment_shader_source = "
        precision mediump float;
        uniform vec4 fragColor;
        void main(void) {
            gl_FragColor = fragColor;
        }
        ";

    let vertex_shader = create_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )
    .unwrap();

    let fragment_shader = create_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )
    .unwrap();

    // create_program , attach_shader  , link_program : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createProgram
    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);

    if gl
        // get_program_parameter : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramParameter
        .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        // use_program : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/useProgram
        gl.use_program(Some(&shader_program));
        Ok(shader_program)
    } else {
        return Err(JsValue::from_str(
            &gl.get_program_info_log(&shader_program)
                .unwrap_or_else(|| "Unknown error linking program".into()),
        ));
    }
}

pub fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {
    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
    // create_buffer : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createBuffer
    let vertex_buffer = gl.create_buffer().unwrap();

    // bind_buffer : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindBuffer
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    // buffer_data_with_array_buffer_view : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    // WebGL 이 사용할 데이터를 저장하기 위한 임시 버퍼 공간 생성
    // 바인딩 : 해당 버퍼로 작업 하겠다고 WebGL 에 알림
    // 데이터를 버퍼에 넣음

    // get_attrib_location : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation
    // 셰이더 프로그램에서 속성의 위치를 찾음
    let coordinates_location = gl.get_attrib_location(&shader_program, "coordinates");

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    // vertex_attrib_pointer_with_i32 : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer
    gl.vertex_attrib_pointer_with_i32(
        coordinates_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    // enable_vertex_attrib_array : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enableVertexAttribArray
    gl.enable_vertex_attrib_array(coordinates_location as u32);
}

#[wasm_bindgen]
pub fn draw_triangle(
    canvas_id: &str,
    selected_color: Option<Vec<f32>>,
) -> Result<WebGlRenderingContext, JsValue> {
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    let shader_program: WebGlProgram = setup_shaders(&gl).unwrap();
    let vertices: [f32; 9] = [
        0.0, 1.0, 0.0, // top
        -1.0, -1.0, 0.0, // bottom left
        1.0, -1.0, 0.0, // bottom right
    ];

    setup_vertices(&gl, &vertices, &shader_program);

    let color = selected_color.unwrap_or(vec![1.0, 0.0, 0.0, 1.0]);
    let color_location = gl
        // get_uniform_location : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation
        .get_uniform_location(&shader_program, "fragColor")
        .unwrap();
    // uniform4fv_with_f32_array : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform
    gl.uniform4fv_with_f32_array(Some(&color_location), &color);

    // draw_arrays : https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays
    gl.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    Ok(gl)
}