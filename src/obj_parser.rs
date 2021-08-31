
use std::path::Path;
use std::fs::File;

#[derive(Copy, Clone,Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}
impl std::default::Default for Vertex {
    fn default() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }
}
#[derive(Copy, Clone, Debug)]
struct TexCoord {
    u: f32,
    v: f32,
    w: f32
}
impl std::default::Default for TexCoord {
    fn default() -> Self {
        Self { u: 0.0, v: 0.0, w: 0.0 }
    }
}
#[derive(Copy, Clone, Debug)]
struct Normal {
    x: f32,
    y: f32,
    z: f32
}

type Face = Vec<(u32, u32, u32)>; // indices to vertex, 


#[derive(Clone)]
pub struct Object {
    vertices: Vec::<f32>,
    normals: Vec::<f32>,
    tex_coords: Vec<f32>,
    indices: Vec<u32>
}

#[derive(Default, Debug)]
pub struct ObjBuilder {
    vertices: Vec::<Vertex>,
    tex_coords: Vec::<TexCoord>,
    normals: Vec<Normal>,
    faces: Vec<Face>,
    lines: Vec<u32>,
}

impl ObjBuilder {
    pub fn new() -> Self {
        ObjBuilder { .. Default::default() }
    }
    pub fn load_file<P: AsRef<Path>>(mut self, obj_path: P) -> Self {
        let buf = std::fs::read_to_string(obj_path).unwrap();
        for line in buf.lines() {
            if line.starts_with("#") { continue; }
            let mut line_iter = line.split_whitespace();
            match line_iter.next().unwrap() {
                "v" => { /* Vertex */
                    let v = line_iter.take_while(|&s| !s.starts_with("#")).collect::<Vec<_>>();
                    if v.len() == 3 {
                        self.vertices.push(Vertex { 
                            x: v[0].parse::<_>().unwrap(), y: v[1].parse::<_>().unwrap(),
                            z: v[2].parse::<_>().unwrap(), w: 1.0f32
                        });
                    } else {
                        self.vertices.push(Vertex {
                            x: v[0].parse::<_>().unwrap(), y: v[1].parse::<_>().unwrap(),
                            z: v[2].parse::<_>().unwrap(), w: v[3].parse::<_>().unwrap()
                        });
                    }
                }
                "vt" => { /* Texture coordinate */
                    let v = line_iter.take_while(|&s| !s.starts_with("#")).collect::<Vec<_>>();
                    match v.len() {
                        1 => {
                            self.tex_coords.push(TexCoord { 
                                u: v[0].parse::<_>().unwrap(),
                                .. Default::default() 
                            });
                        }
                        2 => {
                            self.tex_coords.push(TexCoord { 
                                u: v[0].parse::<_>().unwrap(), 
                                v: v[1].parse::<_>().unwrap(), 
                                .. Default::default() 
                            });
                        }
                        3 => {
                            self.tex_coords.push(TexCoord { 
                                u: v[0].parse::<_>().unwrap(), 
                                v: v[1].parse::<_>().unwrap(), 
                                w: v[2].parse::<_>().unwrap() 
                            });
                        }
                        _ => unreachable!()
                    }
                }
                "vn" => { /* Normal */
                    let v = line_iter.take_while(|&s| !s.starts_with("#")).collect::<Vec<_>>();
                    if v.len() != 3 { panic!("Wrong number of normal coordinates. Expected 3, got {}", v.len()) }
                    self.normals.push(Normal {
                        x: v[0].parse::<_>().unwrap(), 
                        y: v[1].parse::<_>().unwrap(), 
                        z: v[2].parse::<_>().unwrap() 
                    });
                }
                "vp" => { /* Parameter face vertices */
                    () // TODO: implement
                }
                "f" => { /* Polygonal face element: v_index/t_index/n_index */
                    let mut f = Face::new();
                    for pt in line_iter.take_while(|&s| !s.starts_with("#")) {
                        /* Index starts at 1, use 0 for 'unused' */
                        let mut pt = pt.split("/").map(|i| i.parse::<u32>().unwrap_or(0));
                        f.push((pt.next().unwrap(),
                                pt.next().unwrap(),
                                pt.next().unwrap()));
                    }
                    self.faces.push(f);
                }
                "l" => { /* Line element */
                    () // TODO: implement
                }
                &_ => ()
            }
        }
        self
    }
    pub fn generate_buffers(self) -> (u32, u32, u32) {
        let (vao, vbo, ibo) = (0, 0, 0);

        (vao, vbo, ibo)
    }
}

#[test]
fn test_object_builder() {
    let sample_file = "./models/sample.obj";
    let obj = ObjBuilder::new()
        .load_file(sample_file);
    eprintln!("{:?}", obj);
    assert_eq!(12, obj.vertices.len()); 
    assert_eq!(19, obj.tex_coords.len());
    assert_eq!(10, obj.normals.len());
    assert_eq!(20, obj.faces.len());

}