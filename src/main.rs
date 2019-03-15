#![allow(unused)]

mod bindings;
use bindings::root::xatlas;

use std::ops::Drop;

use crate::bindings::*;

enum IndexFormat {
    Uint16,
    Uint32,
}

impl Default for IndexFormat {
    fn default() -> IndexFormat {
        IndexFormat::Uint16
    }
}

#[repr(C)]
#[derive(Default)]
struct MeshDecl {
    vertex_count: u32,
    vertex_position_data: Vec<f32>,
    vertex_position_stride: u32,

    vertex_normal_data: Vec<u8>,
    vertex_normal_stride: u32,
    vertex_uv_data: Vec<u8>,
    vertex_uv_stride: u32,
    index_count: u32,
    index_data: Vec<u32>, // jb-todo
    index_offset: i32,
    index_format: IndexFormat,
    face_ignore_data: Vec<bool>,
}

struct Xatlas {
    handle: *mut root::xatlas::Atlas,
}

impl Xatlas {
    fn new() -> Xatlas {
        Xatlas {
            handle: unsafe { xatlas::Create() },
        }
    }

    pub fn add_mesh(&self, decl_param: &MeshDecl) {
        let decl = xatlas::MeshDecl {
            vertexCount: decl_param.vertex_count,
            vertexPositionData: decl_param.vertex_position_data.as_ptr() as _,
            vertexPositionStride: decl_param.vertex_position_stride,
            vertexNormalData: if decl_param.vertex_normal_data.is_empty() {
                std::ptr::null()
            } else {
                decl_param.vertex_normal_data.as_ptr() as _
            },
            vertexNormalStride: decl_param.vertex_normal_stride,
            vertexUvData: if decl_param.vertex_uv_data.is_empty() {
                std::ptr::null()
            } else {
                decl_param.vertex_uv_data.as_ptr() as _
            },
            vertexUvStride: decl_param.vertex_uv_stride,
            indexCount: decl_param.index_count,
            indexData: if decl_param.index_data.is_empty() {
                std::ptr::null()
            } else {
                decl_param.index_data.as_ptr() as _
            },
            indexOffset: decl_param.index_offset,
            indexFormat: match decl_param.index_format {
                IndexFormat::Uint16 => xatlas::IndexFormat_Enum_UInt16,
                IndexFormat::Uint32 => xatlas::IndexFormat_Enum_UInt32,
            },
            faceIgnoreData: if decl_param.face_ignore_data.is_empty() {
                std::ptr::null()
            } else {
                decl_param.face_ignore_data.as_ptr() as _
            },
        };

        unsafe {
            xatlas::AddMesh(self.handle, &decl);
        }
    }

    pub fn generate(&self) {
        let chart_ops = xatlas::ChartOptions {
            proxyFitMetricWeight: 2.0,
            roundnessMetricWeight: 0.01,
            straightnessMetricWeight: 6.0,
            normalSeamMetricWeight: 4.0,
            textureSeamMetricWeight: 0.5,
            maxChartArea: 0.0,
            maxBoundaryLength: 0.0,
            maxThreshold: 2.0,
            growFaceCount: 32,
            maxIterations: 1,
        };

        let pack_opts = xatlas::PackOptions {
            attempts: 4096,
            texelsPerUnit: 0.0,
            resolution: 0,
            maxChartSize: 1024,
            blockAlign: false,
            conservative: false,
            padding: 0,
        };

        unsafe {
            xatlas::Generate(
                self.handle,
                chart_ops,
                None,
                pack_opts,
                None,
                std::ptr::null_mut(),
            )
        }
    }

    fn print(&self) {
        unsafe {
            for idx in 0..(*self.handle).meshCount {
                let mesh =  std::ptr::read((*self.handle).meshes.offset(idx as isize));
                println!("{}", idx);
                println!("{:?}", mesh);

                let vertex_array = std::slice::from_raw_parts(
                    mesh.vertexArray,
                    mesh.vertexCount as usize);
                dbg!(vertex_array);
            }
        }
    }
}

impl Drop for Xatlas {
    fn drop(&mut self) {
        unsafe {
            xatlas::Destroy(self.handle);
        }
    }
}

fn main() {
    let atlas = Xatlas::new();

    let mut decl = MeshDecl::default();
    decl.vertex_count = 3;
    decl.vertex_position_data = vec![
        0.0, 0.0, 0.0, //
        0.0, 1.0, 1.0, //
        0.0, 1.0, 0.0, //
    ]; 
    decl.vertex_position_stride = (std::mem::size_of::<f32>() * 3) as u32;
    decl.index_count = 3;
    decl.index_data = vec![0, 1, 2];
    decl.index_format = IndexFormat::Uint32;

    atlas.add_mesh(&decl);
    atlas.generate();
    atlas.print();
}
