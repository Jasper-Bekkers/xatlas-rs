#![allow(unused)]

mod bindings;
use bindings::root::xatlas;

use std::ops::Drop;

use crate::bindings::*;

// jb-todo: find a way to call ParameterizeFunc wrapped in a closure

#[derive(Debug)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

impl Default for IndexFormat {
    fn default() -> IndexFormat {
        IndexFormat::Uint16
    }
}

// xatlas makes internal copies of all of these so just passing references to slices is more then enough
#[repr(C)]
#[derive(Debug)]
pub enum ProgressCategory {
    ComputeCharts,
    ParameterizeCharts,
    PackCharts,
    BuildOutputMeshes,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct MeshDecl<'a> {
    pub vertex_count: u32,
    pub vertex_position_data: &'a [u8],
    pub vertex_position_stride: u32,
    pub vertex_normal_data: &'a [u8],
    pub vertex_normal_stride: u32,
    pub vertex_uv_data: &'a [u8],
    pub vertex_uv_stride: u32,
    pub index_count: u32,
    pub index_data: &'a [u8],
    pub index_offset: i32,
    pub index_format: IndexFormat,
    pub face_ignore_data: &'a [bool],
}

#[derive(Debug)]
pub struct ChartOptions {
    pub proxy_fit_metric_weight: f32,
    pub roundness_metric_weight: f32,
    pub straightness_metric_weight: f32,
    pub normal_seam_metric_weight: f32,
    pub texture_seam_metric_weight: f32,
    pub max_chart_area: f32,
    pub max_boundary_length: f32,
    pub max_threshold: f32,
    pub grow_face_count: u32,
    pub max_iterations: u32,
}

impl ChartOptions {
    fn convert(&self) -> xatlas::ChartOptions {
        xatlas::ChartOptions {
            proxyFitMetricWeight: self.proxy_fit_metric_weight,
            roundnessMetricWeight: self.roundness_metric_weight,
            straightnessMetricWeight: self.straightness_metric_weight,
            normalSeamMetricWeight: self.normal_seam_metric_weight,
            textureSeamMetricWeight: self.texture_seam_metric_weight,
            maxChartArea: self.max_chart_area,
            maxBoundaryLength: self.max_boundary_length,
            maxThreshold: self.max_threshold,
            growFaceCount: self.grow_face_count,
            maxIterations: self.max_iterations,
        }
    }
}

impl Default for ChartOptions {
    fn default() -> ChartOptions {
        ChartOptions {
            proxy_fit_metric_weight: 2.0,
            roundness_metric_weight: 0.01,
            straightness_metric_weight: 6.0,
            normal_seam_metric_weight: 4.0,
            texture_seam_metric_weight: 0.5,
            max_chart_area: 0.0,
            max_boundary_length: 0.0,
            max_threshold: 2.0,
            grow_face_count: 32,
            max_iterations: 1,
        }
    }
}

#[derive(Debug)]
pub struct PackOptions {
    pub attempts: i32,
    pub texels_per_unit: f32,
    pub resolution: u32,
    pub max_chart_size: u32,
    pub block_align: bool,
    pub conservative: bool,
    pub padding: u32,
}

impl PackOptions {
    fn convert(&self) -> xatlas::PackOptions {
        xatlas::PackOptions {
            attempts: self.attempts,
            texelsPerUnit: self.texels_per_unit,
            resolution: self.resolution,
            maxChartSize: self.max_chart_size,
            blockAlign: self.block_align,
            conservative: self.conservative,
            padding: self.padding,
        }
    }
}

impl Default for PackOptions {
    fn default() -> PackOptions {
        PackOptions {
            attempts: 4096,
            texels_per_unit: 0.0,
            resolution: 0,
            max_chart_size: 1024,
            block_align: false,
            conservative: false,
            padding: 0,
        }
    }
}

#[derive(Debug)]
pub struct Chart<'a> {
    pub atlas_index: u32,
    pub indices: &'a [u32],
}

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub atlas_index: u32,
    pub uv: [f32; 2],
    pub xref: u32,
}

#[derive(Debug)]
pub struct Mesh<'a> {
    pub charts: Vec<Chart<'a>>, // need to translate Chart so it's owned
    pub indices: &'a [u32],
    pub vertices: &'a [Vertex],
}

#[derive(Debug)]
pub struct Xatlas {
    handle: *mut root::xatlas::Atlas,
}

unsafe extern "C" fn progress_cb(
    category: root::xatlas::ProgressCategory_Enum,
    progress: ::std::os::raw::c_int,
    user_data: *mut ::std::os::raw::c_void,
) {
    let cb: *mut &mut FnMut(ProgressCategory, i32) = unsafe { std::mem::transmute(user_data) };

    #[allow(non_snake_case)]
    (*cb)(
        match category {
            ProgressCategory_Enum_ComputeCharts => ProgressCategory::ComputeCharts,
            ProgressCategory_Enum_ParameterizeCharts => ProgressCategory::ParameterizeCharts,
            ProgressCategory_Enum_PackCharts => ProgressCategory::PackCharts,
            ProgressCategory_Enum_BuildOutputMeshes => ProgressCategory::BuildOutputMeshes,
        },
        progress,
    );
}

impl<'a> Xatlas {
    pub fn new() -> Self {
        Self {
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

    pub fn generate_simple(&self, chart_opts: ChartOptions, pack_opts: PackOptions) {
        let chart_opts = chart_opts.convert();
        let pack_opts = pack_opts.convert();

        unsafe {
            xatlas::Generate(
                self.handle,
                chart_opts,
                None,
                pack_opts,
                None,
                std::ptr::null_mut(),
            )
        }
    }

    pub fn generate<F>(&self, chart_opts: ChartOptions, pack_opts: PackOptions, mut progress: F)
    where
        F: FnMut(ProgressCategory, i32),
    {
        let chart_opts = chart_opts.convert();
        let pack_opts = pack_opts.convert();

        let mut cb: &mut FnMut(ProgressCategory, i32) = &mut progress;
        let cb = &mut cb as *mut &mut FnMut(ProgressCategory, i32) as *mut std::ffi::c_void;

        unsafe {
            xatlas::Generate(
                self.handle,
                chart_opts,
                None,
                pack_opts,
                Some(progress_cb),
                cb,
            )
        }
    }

    pub fn meshes(&mut self) -> Vec<Mesh<'a>> {
        // shallow copy most data
        let mut meshes = vec![];

        let original_meshes = unsafe {
            std::slice::from_raw_parts((*self.handle).meshes, (*self.handle).meshCount as usize)
        };

        for original_mesh in original_meshes {
            let mut charts = vec![];
            let original_charts = unsafe {
                std::slice::from_raw_parts(
                    original_mesh.chartArray,
                    original_mesh.chartCount as usize,
                )
            };

            for original_chart in original_charts {
                charts.push(Chart {
                    atlas_index: original_chart.atlasIndex,
                    indices: unsafe {
                        std::slice::from_raw_parts(
                            original_chart.indexArray,
                            original_chart.indexCount as usize,
                        )
                    },
                })
            }

            meshes.push(Mesh {
                indices: unsafe {
                    std::slice::from_raw_parts(
                        original_mesh.indexArray,
                        original_mesh.indexCount as usize,
                    )
                },
                vertices: unsafe {
                    std::slice::from_raw_parts(
                        original_mesh.vertexArray as *mut _ as *mut _,
                        original_mesh.vertexCount as usize,
                    )
                },
                charts,
            });
        }

        meshes
    }
}

impl<'a> Drop for Xatlas {
    fn drop(&mut self) {
        unsafe {
            xatlas::Destroy(self.handle);
        }
    }
}
