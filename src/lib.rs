mod cube;
mod math;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ WebGl2RenderingContext, WebGlProgram, WebGlShader };
use std::{rc::Rc, cell::RefCell};
use cube::{ Cube };
use math::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document
                                             .get_element_by_id("canvas")
                                             .unwrap()
                                             .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;
    
    // Shader 
    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
        layout(location = 0) in vec3 aPosition;
        layout(location = 1) in vec4 aColor;

        uniform mat4 uModelMatrix;
        uniform mat4 uViewMatrix;
        uniform mat4 uProjectionMatrix;

        out vec4 vColor;

        void main() {
            gl_Position = uProjectionMatrix * uViewMatrix * uModelMatrix * vec4(aPosition, 1.0);
            vColor = aColor;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
        precision highp float;
        in vec4 vColor;
        out vec4 outColor;
        void main() {
            outColor = vColor;
        }
        "##,
    )?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let cube = Cube::new();

    let position_attribute_location = gl.get_attrib_location(&program, "aPosition");
    let color_attribute_location = gl.get_attrib_location(&program, "aColor");
    
    let model_matrix = gl.get_uniform_location(&program, "uModelMatrix").unwrap();
    let view_matrix = gl.get_uniform_location(&program, "uViewMatrix").unwrap();
    let porojection_matrix = gl.get_uniform_location(&program, "uProjectionMatrix").unwrap();

    let vao = gl.create_vertex_array().ok_or("Could not create vertex array object")?;
    gl.bind_vertex_array(Some(&vao));

    // Position Buffer.
    let position_buffer = gl.create_buffer().ok_or("Failed to create position buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    unsafe {
        let position_array_buf_view = js_sys::Float32Array::view(&cube.vertex);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &position_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position_attribute_location as u32);
    gl.bind_vertex_array(Some(&vao));

    // Color Buffer.
    let color_buffer = gl.create_buffer().ok_or("Failed to careate color buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));

    unsafe {
        let color_array_buf_view = js_sys::Float32Array::view(&cube.color);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(1, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(color_attribute_location as u32);
    gl.bind_vertex_array(Some(&vao));

    // Index Buffer.
    let index_buffer = gl.create_buffer().ok_or("Failed to careate index buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

    unsafe {
        let index_array_buf_view = js_sys::Uint8Array::view(&cube.index);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let mut angle = 0.0;
    let mut eye: [f32; 3] = [1.0, 1.0, 2.0];
    let target: [f32; 3] = [0.0, 0.0, 0.0];

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        let models = rotate_x(angle);
        let view = look_at(eye, target);
        let perspective = perspective(1.32, 1.77, 0.1, 1000.0);
        
        gl.uniform_matrix4fv_with_f32_array(Some(&model_matrix), false, &models);
        gl.uniform_matrix4fv_with_f32_array(Some(&view_matrix), false, &view);
        gl.uniform_matrix4fv_with_f32_array(Some(&porojection_matrix), false, &perspective);

        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 36, WebGl2RenderingContext::UNSIGNED_BYTE, 0);
        
        angle = angle + 0.02;

        let window = web_sys::window().unwrap();
        window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("failed requesting animation frame");
    }) as Box<dyn FnMut()>));

    let window = web_sys::window().unwrap();
    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("failed requesting animation frame");

    Ok(())
}

pub fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
        
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) {
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program")))
        }
}
