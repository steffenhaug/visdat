extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod obj_parser;

use glutin::event::{
    Event,
    WindowEvent,
    DeviceEvent,
    KeyboardInput,
    ElementState::{Pressed, Released},
    VirtualKeyCode::{self, *}
};

use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

// Helper functions to make interacting with OpenGL a little bit 
// prettier. You *WILL* need these! The names should be pretty self 
// explanatory.

// Get # of bytes in an array.
#[inline(always)]
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
#[inline(always)]
fn _size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn _offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}


/* simple "tripod" camera (fps games) */
struct Camera {
    x: f32,
    y: f32,
    z: f32,
    /* euler angles, (0,0) = looking towards z. */
    rho: f32, /* around y */
    th:  f32, /* around z once rho is undone */
}

unsafe fn mkvao(verts: &Vec<f32>, cols: &Vec<f32>, idx: &Vec<u32>) -> u32 {
    /* Stole my own old code here. I like this way of setting it up. */

    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao); /* now it's bound */

    /* all buffer operations here act on our vao */

    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);

    let mut cbo = 0;
    gl::GenBuffers(1, &mut cbo);

    let mut ibo = 0;
    gl::GenBuffers(1, &mut ibo);

    /* bind vertex buffer and push data */
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER, 
                   byte_size_of_array(verts),
                   pointer_to_array(verts),
                   gl::STATIC_DRAW); 
    /* attrib ptrs for verts */
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

    /* now, bind color buffer and push data */
    gl::BindBuffer(gl::ARRAY_BUFFER, cbo);
    gl::BufferData(gl::ARRAY_BUFFER, 
                   byte_size_of_array(cols),
                   pointer_to_array(cols),
                   gl::STATIC_DRAW); 
    /* attrib ptrs for colors */
    // im pretty sure this works so attrib 0 comes from buffer 1 and attrib 1 comes from buffer 2
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

    /* bind index buffer and push indeces */
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   byte_size_of_array(idx),
                   pointer_to_array(idx),
                   gl::STATIC_DRAW);




    vao
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it 
    // to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has 
        // to be done inside of the rendering thread, because an active OpenGL 
        // context cannot safely traverse a thread boundary.

        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO here
        let (_verts, n_tris) = unsafe {
            let vbuf = vec![
                /* square on the ground */
                -5.0, -1.0, -5.0,
                -5.0, -1.0,  5.0,
                 5.0, -1.0, -5.0,
                 5.0, -1.0,  5.0,

                /* triangle 1 */
                -0.6, -0.8, 0.0,
                 0.0,  0.6, 0.0,
                 0.6, -0.6, 0.0,

                /* triangle 2 */
                -0.7, -0.7, 0.5,
                 0.0,  0.7, 0.5,
                 0.7, -0.7, 0.5,
            ];

            let cbuf = vec![
                0.6, 0.6, 0.7, 1.0, /* floor */
                0.7, 0.7, 0.6, 1.0, 
                0.9, 0.6, 0.5, 1.0, 
                0.7, 0.4, 0.6, 1.0, 
                0.2, 0.6, 0.9, 0.2, /* blue triangle */
                0.2, 0.6, 0.9, 0.2,
                0.2, 0.6, 0.9, 0.2,
                1.0, 1.0, 0.0, 0.2, /* yellow triangle */
                1.0, 1.0, 0.0, 0.2,
                1.0, 1.0, 0.0, 0.2,
            ];

            let ibuf = vec![ 
                0, 1, 3,
                0, 3, 2,
                4, 6, 5,
                7, 9, 8,
            ];

            /* drops the buffers, but data should already be in vram i think */
            (mkvao(&vbuf, &cbuf, &ibuf), ibuf.len())
        };

        // == // Testing model loading from obj file. Uncomment to shaddow
        //       variables and render 3D model.
        // let (verts, n_elements) = unsafe {
        //     let (vbuf, ibuf) = obj_parser::ObjBuilder::new()
        //         .load_file("./models/sample.obj")
        //         .generate_simple_buffers();
        //     (mkvao(&vbuf, &ibuf), ibuf.len())
        // };

        // Basic usage of shader helper:
        // The example code below returns a shader object, which contains the field `.program_id`.
        // The snippet is not enough to do the assignment, and will need to be modified (outside of
        // just using the correct path), but it only needs to be called once
        //
        //     shader::ShaderBuilder::new()
        //        .attach_file("./path/to/shader.file")
        //        .link();
        let (u_mvp, _) = unsafe {
            let sh = shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link();

            sh.activate();

            // should be ok to drop shader. code is already uploaded
            // and active.
            let u_mvp = sh.get_uniform_location("u_MVP");

            (u_mvp, sh)
        };

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;


        let mut camera = Camera {
            x:  0.0,
            y:  0.0,
            z: -2.0,
            rho: 0.0,
            th:  0.0,
        };

        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // calculate a matrix to undo the cameras yaw transform
            let rot = glm::rotation(-camera.rho, &glm::vec3(0.0, 1.0, 0.0));

            /* let v,w be basis for the xz-plane where the camera faces in v direction */
            let v = rot * glm::vec4(0.0, 0.0, 1.0, 1.0); /* forward */
            let w = rot * -glm::vec4(1.0, 0.0, 0.0, 1.0); /* right */

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::W => {
                            /* calculate the position delta and adjust position */
                            let dp = delta_time * v;
                            camera.x += dp.x;
                            camera.z += dp.z;
                        },
                        VirtualKeyCode::A => { 
                            let dp = delta_time * -w;
                            camera.x += dp.x;
                            camera.z += dp.z;
                        },
                        VirtualKeyCode::S => { 
                            let dp = delta_time * -v;
                            camera.x += dp.x;
                            camera.z += dp.z;
                        },
                        VirtualKeyCode::D => { 
                            let dp = delta_time * w;
                            camera.x += dp.x;
                            camera.z += dp.z;
                        },
                        /* vim bindings for camera */
                        VirtualKeyCode::H => { camera.rho -= delta_time; },
                        VirtualKeyCode::L => { camera.rho += delta_time; },
                        VirtualKeyCode::K => { camera.th -= delta_time; },
                        VirtualKeyCode::J => { camera.th += delta_time; },
                        /* R and F to fly */
                        VirtualKeyCode::R => { camera.y -= delta_time; },
                        VirtualKeyCode::F => { camera.y += delta_time; },
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                *delta = (0.0, 0.0);
            }

            let aspect = (SCREEN_H as f32) / (SCREEN_W as f32);
            let fov = 1.0; /* around 60 degrees */

            let persp: glm::Mat4 = 
                glm::perspective(aspect, fov, 0.1, 100.0);

            // the view matrix is extremely simple:
            // we just undo the translation and rotation of the camera
            let trans: glm::Mat4 =
                glm::translation(&glm::vec3(camera.x, camera.y, camera.z));

            let yaw: glm::Mat4 =
                glm::rotation(camera.rho, &glm::vec3(0.0, 1.0, 0.0));

            let pitch: glm::Mat4 =
                glm::rotation(camera.th, &glm::vec3(1.0, 0.0, 0.0));

            /* keep in mind the order you need to apply the transformations */
            let mvp = persp * (pitch * yaw * trans);

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                gl::UniformMatrix4fv(u_mvp, 1, 0, mvp.as_ptr());

                // Issue the necessary commands to draw your scene here
                gl::DrawElements(gl::TRIANGLES, n_tris as _, gl::UNSIGNED_INT, std::ptr::null());

            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => { }
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            },
            _ => { }
        }
    });
}
