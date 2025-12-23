//! Generate the Platonic solids as STL files via constraints.

mod platonic_solids;
mod relax_solid;
mod solid;
mod triangulate;
mod view;

use std::fs::File;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use platonic_solids::*;
use relax_solid::*;
use solid::*;
use triangulate::*;

use image::codecs::gif::GifEncoder;
use image::ExtendedColorType;
use rayon::ThreadPoolBuilder;
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

const NUM_THREADS: usize = 10;

fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .compact()
        .init();

    let platonic_solid = PlatonicSolid::Dodecahedron;

    let pool = ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS - 2)
        .build()
        .expect("failed to build thread pool.");

    let gif_file = File::create("out.gif").expect("failed to create file for out.gif");
    let mut gif_encoder = GifEncoder::new_with_speed(gif_file, 10);

    let (locations_tx, locations_rx) = channel::<Locations>();
    let (images_tx, images_rx) = channel::<ndarray::Array2<u8>>();

    let view_params = Arc::new(view::ViewParams {
        camera_center: nalgebra::Point3::new(0.0, 0.0, -10.0),
        camera_normal: nalgebra::Vector3::z_axis(),
        image_width_px: 400,
        image_height_px: 400,
        pixel_size: 0.01,
    });

    let relax_params = RelaxParams {
        spring_constant: 1.0,
        repulsion_constant: 0.1,
        natural_length: 1.0,
        step_size: 1e-4,
        total_movement_thresh: 1e-7,
        locations_tx,
    };

    let neighbors = neighbors_for_solid(&platonic_solid);
    thread::spawn(move || {
        let _s = info_span!("relax").entered();
        relax(&neighbors, relax_params);
    });

    {
        let vp = Arc::clone(&view_params);
        thread::spawn(move || {
            let _s = info_span!("encode_gif").entered();

            let w = vp.image_width_px as u32;
            let h = vp.image_height_px as u32;

            while let Ok(image) = images_rx.recv() {
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

                if let Err(e) = gif_encoder.encode(&rgb, w, h, ExtendedColorType::Rgb8) {
                    eprintln!("gif encode failed: {e}");
                    break;
                }
            }
        });
    }

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

fn save_stl(platonic_solid: &PlatonicSolid, solid: &Solid) {
    let mut path = std::env::current_dir().unwrap();
    path.push(format!("{}.stl", platonic_solid.to_string()));
    to_stl(
        platonic_solid.to_string(),
        &path,
        &solid.triangles,
        &solid.locations,
    )
    .unwrap();
}
