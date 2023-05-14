use std::time::Instant;

use lib_raytracing::{
    camera::Camera,
    hittable::HittableList,
    material::Material,
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    objects::sphere::Sphere,
    render_scene,
    scenes::{
        loader::Loader,
        objloader::{self, OBJLoader},
    },
    util::{self, clamp},
    vec3::Vec3,
};
use log::info;
use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
fn random_scene<'a>() -> HittableList {
    let mut world: HittableList = Default::default();

    let ground_material = Lambertian {
        color: Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    };
    world.add(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        material: Box::new(ground_material),
        radius: 1000.0,
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random();
            let center = Vec3::new(
                (a as f64) + 0.9 * util::random(),
                0.2,
                (b as f64) + 0.9 * util::random(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    Box::new(Lambertian {
                        color: Vec3::random() * Vec3::random(),
                    })
                } else if choose_mat < 0.95 {
                    Box::new(Metal {
                        color: Vec3::random_range(0.5, 1.0),
                        fuzz: util::random_range(0.0, 0.5),
                    })
                } else {
                    Box::new(Dielectric {
                        index_of_refraction: 1.5,
                    })
                };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    world.add(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        material: Box::new(Dielectric {
            index_of_refraction: 1.5,
        }),
        radius: 1.0,
    });
    world.add(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        material: Box::new(Lambertian {
            color: Vec3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        }),
        radius: 1.0,
    });
    world.add(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        material: Box::new(Metal {
            color: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        }),
        radius: 1.0,
    });
    world
}

fn main() {
    // init logging
    dotenvy::dotenv().expect("could not load .env file");
    env_logger::init();

    // constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1080;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE: u32 = 50;
    const FIELD_OF_VIEW: f64 = 20.0;
    const GAMMA: f64 = 2.0;

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0); // position of the camera
    let lookat = Vec3::new(0.0, 0.0, 0.0); // the position the camera looks at
    let view_up = Vec3::new(0.0, 1.0, 0.0); // tilt of the camera
                                            // dist_to_focus & aperture are used for defocus blur or depth of field
                                            // the higher the dist_to_focus and the lower the aperture ... the smaller is the sharp area
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        view_up,
        FIELD_OF_VIEW,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // world
    let loader = OBJLoader {};
    let world = loader.load_file("test.obj"); //random_scene();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(IMAGE_WIDTH, IMAGE_HEIGHT);
        WindowBuilder::new()
            .with_title("Rust raytracing")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixel_frame_buffer = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(IMAGE_WIDTH, IMAGE_HEIGHT, surface_texture)
            .expect("Failed to create pixxel buffer")
    };

    let mut pixels = vec![Vec3::new(0.0, 0.0, 0.0); (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    let mut calculated_samples = 0.0;
    let mut start = Instant::now();
    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        if let Event::RedrawRequested(_) = event {
            if let Err(err) = pixel_frame_buffer.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixel_frame_buffer.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            if calculated_samples < SAMPLES_PER_PIXEL as f64 {
                let start_time = Instant::now();
                pixels = render_scene(&world, &camera, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_BOUNCE)
                    .par_iter()
                    .zip(&pixels)
                    .map(|(a, b)| *a + *b)
                    .collect();
                calculated_samples += 1.0;
                frame_copy(
                    pixels
                        .par_iter()
                        .map(|pixel| (*pixel / calculated_samples).pow(1.0 / GAMMA))
                        .collect::<Vec<Vec3>>(),
                    pixel_frame_buffer.frame_mut(),
                );
                info!("Rendering sample took {:?}", start_time.elapsed());

                window.request_redraw();
            } else if calculated_samples == SAMPLES_PER_PIXEL as f64 {
                info!("Rendering scene took {:?}", start.elapsed());
                calculated_samples += 1.0;
            }
        }
    });
}

fn frame_copy(pixels: Vec<Vec3>, frame_mut: &mut [u8]) {
    for (i, pixel) in frame_mut.chunks_exact_mut(4).enumerate() {
        let rgba = [
            (256.0 * clamp(pixels[i].x, 0.0, 0.999)).round() as u8,
            (256.0 * clamp(pixels[i].y, 0.0, 0.999)).round() as u8,
            (256.0 * clamp(pixels[i].z, 0.0, 0.999)).round() as u8,
            0xff,
        ];
        pixel.copy_from_slice(&rgba);
    }
}
