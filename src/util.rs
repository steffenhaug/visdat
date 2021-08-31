use std::ffi::CString;

pub unsafe fn get_gl_string(name: gl::types::GLenum) -> String {
    std::ffi::CStr::from_ptr(gl::GetString(name) as *mut i8).to_string_lossy().to_string()
}

// Debug callback to panic upon enountering any OpenGL error
pub extern "system" fn debug_callback(
    source: u32, e_type: u32, id: u32,
    severity: u32, _length: i32,
    msg: *const i8, _data: *mut std::ffi::c_void
) {
    if e_type != gl::DEBUG_TYPE_ERROR { return }
    if severity == gl::DEBUG_SEVERITY_HIGH ||
       severity == gl::DEBUG_SEVERITY_MEDIUM ||
       severity == gl::DEBUG_SEVERITY_LOW
    {
        let severity_string = match severity {
            gl::DEBUG_SEVERITY_HIGH => "high",
            gl::DEBUG_SEVERITY_MEDIUM => "medium",
            gl::DEBUG_SEVERITY_LOW => "low",
            _ => "unknown",
        };
        unsafe {
            let string = CString::from_raw(msg as *mut i8);
            let error_message = String::from_utf8_lossy(string.as_bytes()).to_string();
            panic!("{}: Error of severity {} raised from {}: {}\n",
                id, severity_string, source, error_message);
        }
    }
}

pub mod matrix {
    /// Create view matrix for camera
    /// 
    /// # Arguments
    /// * `position` - Position of camera in the scene
    /// * `direction` - The direction the camera is facing in scene coordinates
    /// * `up` - Vector representing the direction in scene coordinates (Rotation of camera, [0,1,0] means leveled)
    pub fn view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = (f[0]*f[0] + f[1]*f[1] + f[2]*f[2]).sqrt(); /* ||f|| */
            [ f[0] / len, f[1] / len, f[2] / len]
        };
        
        let s = [ /* up x f */
            up[1]*f[2] - up[2]*f[1],
            up[2]*f[0] - up[0]*f[2],
            up[0]*f[1] - up[1]*f[0],
        ];/// Create view matrix for camera
        /// 
        /// # Arguments
        /// * `position` - Position of camera in the scene
        /// * `direction` - The direction the camera is facing in scene coordinates
        /// * `up` - Vector representing the direction in scene coordinates (Rotation of camera, [0,1,0] means leveled)
        pub fn view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
            let f = {
                let f = direction;
                let len = (f[0]*f[0] + f[1]*f[1] + f[2]*f[2]).sqrt(); /* ||f|| */
                [ f[0] / len, f[1] / len, f[2] / len]
            };
            
            let s = [ /* up x f */
                up[1]*f[2] - up[2]*f[1],
                up[2]*f[0] - up[0]*f[2],
                up[0]*f[1] - up[1]*f[0],
            ];
            
            let s_norm = {
                let len = (s[0]*s[0] + s[1]*s[1] + s[2]*s[2]).sqrt(); /* ||f|| */
                [ s[0] / len, s[1] / len, s[2] / len]
            };
        
            let u = [ /* f x s_norm */
                f[1]*s_norm[2] - f[2]*s_norm[1],
                f[2]*s_norm[0] - f[0]*s_norm[2],
                f[0]*s_norm[1] - f[1]*s_norm[0],
            ];
        
            let p = [
                -position[0]*s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                -position[0]*u[0] - position[1] * u[1] - position[2] * u[2],
                -position[0]*f[0] - position[1] * f[1] - position[2] * f[2],
            ];
        
            [
                [ s_norm[0], u[0], f[0], 0.0 ],
                [ s_norm[1], u[1], f[1], 0.0 ],
                [ s_norm[2], u[2], f[2], 0.0 ],
                [   p[0]   , p[1], p[2], 1.0 ]
            ]
        }
        
        /// [Axis] enum
        pub enum Axis {X, Y, Z}
        /// Create a rotation matrix for the given axis
        /// * `angle` - Angle in radians
        /// * `axis` - [`Axis`](enum@Axis) enum
        pub fn rotate(angle: f32, axis: self::Axis) -> [[f32; 4]; 4] {
            use self::Axis::*;
            match axis {
                X => [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, angle.cos(), angle.sin(), 0.0],
                    [0.0, -angle.sin(), angle.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ],
                Y => [
                    [ angle.cos(), 0.0, -angle.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [angle.sin(), 0.0, angle.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ],
                Z => [
                    [ angle.cos(), angle.sin(), 0.0, 0.0],
                    [-angle.sin(), angle.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ]
            }
        }
        
        pub fn perspective(height: f32, width: f32) -> [[f32; 4]; 4] {
            let aspect_ratio = height as f32 / width as f32;
        
            let fov = std::f32::consts::PI / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;
        
            let f = 1.0 / (fov / 2.0).tan(); /* 1 / tan(fov/2) */
        
            // left-handed
            [
                [ f * aspect_ratio,  0.0,                   0.0                 , 0.0 ],
                [       0.0       ,   f ,                   0.0                 , 0.0 ],
                [       0.0       ,  0.0,     (zfar + znear) / (zfar - znear)   , 1.0 ],
                [       0.0       ,  0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0 ],
            ]
        }
        
        let s_norm = {
            let len = (s[0]*s[0] + s[1]*s[1] + s[2]*s[2]).sqrt(); /* ||f|| */
            [ s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [ /* f x s_norm */
            f[1]*s_norm[2] - f[2]*s_norm[1],
            f[2]*s_norm[0] - f[0]*s_norm[2],
            f[0]*s_norm[1] - f[1]*s_norm[0],
        ];
    
        let p = [
            -position[0]*s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
            -position[0]*u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0]*f[0] - position[1] * f[1] - position[2] * f[2],
        ];
    
        [
            [ s_norm[0], u[0], f[0], 0.0 ],
            [ s_norm[1], u[1], f[1], 0.0 ],
            [ s_norm[2], u[2], f[2], 0.0 ],
            [   p[0]   , p[1], p[2], 1.0 ]
        ]
    }
    
    /// [Axis] enum
    pub enum Axis {X, Y, Z}
    /// Create a rotation matrix for the given axis
    /// * `angle` - Angle in radians
    /// * `axis` - [`Axis`](enum@Axis) enum
    pub fn rotate(angle: f32, axis: Axis) -> [[f32; 4]; 4] {
        use Axis::*;
        match axis {
            X => [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, angle.cos(), angle.sin(), 0.0],
                [0.0, -angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            Y => [
                [ angle.cos(), 0.0, -angle.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [angle.sin(), 0.0, angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            Z => [
                [ angle.cos(), angle.sin(), 0.0, 0.0],
                [-angle.sin(), angle.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        }
    }
    
    pub fn perspective(height: f32, width: f32) -> [[f32; 4]; 4] {
        let aspect_ratio = height as f32 / width as f32;
    
        let fov = std::f32::consts::PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
    
        let f = 1.0 / (fov / 2.0).tan(); /* 1 / tan(fov/2) */
    
        // left-handed
        [
            [ f * aspect_ratio,  0.0,                   0.0                 , 0.0 ],
            [       0.0       ,   f ,                   0.0                 , 0.0 ],
            [       0.0       ,  0.0,     (zfar + znear) / (zfar - znear)   , 1.0 ],
            [       0.0       ,  0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0 ],
        ]
    }
}