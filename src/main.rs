//! Generate the Platonic solids as STL files via constraints.

use std::fs::File;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

mod platonic_solids;
mod relax;
mod solid;
mod triangulate;
mod view;

use platonic_solids::*;
use solid::*;
use triangulate::*;

use image::codecs::gif::GifEncoder;
use image::ExtendedColorType;
use rayon::ThreadPoolBuilder;

const NUM_THREADS: usize = 10;

fn main() {
    let platonic_solid = PlatonicSolid::Dodecahedron;

    let (locations_tx, locations_rx) = channel::<Locations>();
    let (images_tx, images_rx) = channel::<ndarray::Array2<u8>>();

    let view_params = Arc::new(view::ViewParams {
        camera_center: nalgebra::Point3::new(0.0, 0.0, -10.0),
        camera_normal: nalgebra::Vector3::z_axis(),
        image_width_px: 400,
        image_height_px: 400,
        pixel_size: 0.01,
    });

    let relax_params = relax::RelaxParams {
        spring_constant: 1.0,
        repulsion_constant: 0.1,
        natural_length: 1.0,
        step_size: 1e-4,
        total_movement_thresh: 1e-7,
        locations_tx,
    };

    // Thread for evolving the shape.
    let neighbors = neighbors_for_solid(&platonic_solid);
    thread::spawn(move || {
        relax::relax(&neighbors, relax_params);
    });

    // Thread for encoding frames into a gif.
    {
        let gif_file = File::create("out.gif").expect("failed to create file for out.gif");
        let mut gif_encoder = GifEncoder::new_with_speed(gif_file, 10);
        gif_encoder
            .set_repeat(image::codecs::gif::Repeat::Infinite)
            .expect("couldn't set repeat");

        let vp = Arc::clone(&view_params);
        thread::spawn(move || {
            let w = vp.image_width_px as u32;
            let h = vp.image_height_px as u32;

            while let Ok(image) = images_rx.recv() {
                add_frame(image, w, h, &mut gif_encoder);
            }
        });
    }

    // Rendering pool.
    let pool = ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS - 2)
        .build()
        .expect("failed to build thread pool.");

    while let Ok(locations) = locations_rx.recv() {
        let vp = Arc::clone(&view_params);
        let tx = images_tx.clone();

        pool.spawn_fifo(move || {
            let triangles = hull_triangles(&locations);
            let solid = solid::Solid {
                locations,
                triangles,
            };
            let image = view::view(&solid, &vp);
            tx.send(image).unwrap();
        });
    }

    drop(images_tx);
}

fn add_frame<W: std::io::Write>(
    image: ndarray::Array2<u8>,
    w: u32,
    h: u32,
    gif_encoder: &mut GifEncoder<W>,
) {
    let luma: Vec<u8> = match image.as_standard_layout().as_slice() {
        Some(slice) => slice.to_vec(),
        None => image.iter().copied().collect(),
    };

    // Expand L8 -> Rgb8
    let mut rgb = Vec::with_capacity(luma.len() * 3);
    for y in luma {
        rgb.push(y);
        rgb.push(y);
        rgb.push(y);
    }

    gif_encoder
        .encode(&rgb, w, h, ExtendedColorType::Rgb8)
        .expect("failed to encode frame");
}

fn save_stl(platonic_solid: &PlatonicSolid, solid: &Solid) {
    let mut path = std::env::current_dir().unwrap();
    path.push(format!("{}.stl", platonic_solid));
    to_stl(
        platonic_solid.to_string(),
        &path,
        &solid.triangles,
        &solid.locations,
    )
    .unwrap();
}
