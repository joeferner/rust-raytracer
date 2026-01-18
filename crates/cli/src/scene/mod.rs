#![allow(clippy::vec_init_then_push)]
pub mod checkered_spheres;
pub mod cornell_box;
pub mod cornell_box_smoke;
pub mod earth;
pub mod final_scene;
pub mod lighted_cone_frustum;
pub mod lighted_sphere;
pub mod perlin_spheres;
pub mod quads;
pub mod random_spheres;
pub mod three_spheres;

use std::{path::Path, sync::Arc};

use ariadne::{Label, Report, ReportKind, Source as AriadneSource};
use caustic_core::{RenderContext, SceneData};
use caustic_openscad::{
    Message, MessageLevel, run_openscad,
    source::{FileSource, Source},
};

use crate::{
    CliError, Result,
    scene::{
        checkered_spheres::create_checkered_spheres_scene, cornell_box::create_cornell_box_scene,
        cornell_box_smoke::create_cornell_box_smoke_scene, earth::create_earth_scene,
        final_scene::create_final_scene, lighted_cone_frustum::create_lighted_cone_frustum_scene,
        lighted_sphere::create_lighted_sphere_scene, perlin_spheres::create_perlin_spheres_scene,
        quads::create_quads_scene, random_spheres::create_random_spheres_scene,
        three_spheres::create_three_spheres_scene,
    },
};

pub enum Scene {
    ThreeSpheres,
    RandomSpheres,
    CheckeredSpheres,
    Earth,
    PerlinSpheres,
    Quads,
    LightedSphere,
    LightedConeFrustum,
    CornellBox,
    CornellBoxSmoke,
    Final,
    OpenScad(String),
}

pub fn get_scene(ctx: &RenderContext, scene: Scene) -> Result<SceneData> {
    match scene {
        Scene::ThreeSpheres => Ok(create_three_spheres_scene(ctx)),
        Scene::RandomSpheres => Ok(create_random_spheres_scene(ctx)),
        Scene::CheckeredSpheres => Ok(create_checkered_spheres_scene(ctx)),
        Scene::Earth => Ok(create_earth_scene(ctx)),
        Scene::PerlinSpheres => Ok(create_perlin_spheres_scene(ctx)),
        Scene::Quads => Ok(create_quads_scene(ctx)),
        Scene::LightedSphere => Ok(create_lighted_sphere_scene(ctx)),
        Scene::LightedConeFrustum => Ok(create_lighted_cone_frustum_scene(ctx)),
        Scene::CornellBox => Ok(create_cornell_box_scene(ctx)),
        Scene::CornellBoxSmoke => Ok(create_cornell_box_smoke_scene(ctx)),
        Scene::Final => Ok(create_final_scene(ctx)),
        Scene::OpenScad(filename) => {
            let source = FileSource::new(Path::new(&filename)).map_err(|err| {
                eprintln!("failed to read \"{filename}\": {err}");
                CliError::OpenscadError
            })?;

            let source: Arc<Box<dyn Source>> = Arc::new(Box::new(source));
            let results = run_openscad(source, ctx.random.clone());
            for message in results.messages {
                print_message(&message);
            }
            match results.scene_data {
                Some(scene_data) => Ok(scene_data),
                None => Err(CliError::OpenscadError),
            }
        }
    }
}

fn print_message(message: &Message) {
    if message.level == MessageLevel::Echo {
        println!("ECHO {}", message.message);
    } else {
        let filename = message.position.source.get_filename();
        let span_start = message.position.start;
        let span_end = message.position.end;
        let code = message.position.source.get_code();
        Report::build(to_kind(&message.level), (filename, span_start..span_end))
            .with_message(message.message.to_owned())
            .with_label(
                Label::new((filename, span_start..span_end))
                    .with_message(message.message.to_owned()),
            )
            .finish()
            .eprint((filename, AriadneSource::from(code)))
            .unwrap();
    }
}

fn to_kind(level: &MessageLevel) -> ariadne::ReportKind<'static> {
    match level {
        MessageLevel::Echo => ReportKind::Advice,
        MessageLevel::Warning => ReportKind::Warning,
        MessageLevel::Error => ReportKind::Error,
    }
}
