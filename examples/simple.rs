use xatlas_rs::*;

fn main() {
    let mut atlas = Xatlas::new();

    let vtx_data = &[
        0.0f32, 0.0, 0.0, //
        0.0, 1.0, 1.0, //
        0.0, 1.0, 0.0, //
    ];

    let idx_data = &[0, 1, 2];

    let mut decl = MeshDecl::default();
    decl.vertex_count = 3;
    decl.vertex_position_data = unsafe {
        std::slice::from_raw_parts(
            vtx_data.as_ptr() as _,
            vtx_data.len() * std::mem::size_of::<f32>(),
        )
    };
    decl.vertex_position_stride = (std::mem::size_of::<f32>() * 3) as u32;
    decl.index_count = 3;
    decl.index_data = unsafe {
        std::slice::from_raw_parts(
            idx_data.as_ptr() as _,
            idx_data.len() * std::mem::size_of::<u32>(),
        )
    };
    decl.index_format = IndexFormat::Uint32;

    atlas.add_mesh(&decl);
    atlas.generate_simple(Default::default(), Default::default());

    let meshes = atlas.meshes();

    dbg!(meshes);
}
