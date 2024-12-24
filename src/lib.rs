// use avian3d::prelude::*;
use bevy::prelude::*;
use camera::MainCamera;

pub mod assets;
pub mod camera;

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut _asset_server: Res<AssetServer>,
) {
    let plane_cmds = cmd.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2 { x: 15.0, y: 20.0 }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::rgb_u8(34, 234, 23)),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let plane_entity = plane_cmds.id();
    let plane_transform = Transform::from_reflect(&plane_entity);
    if plane_transform.is_some() {
        info!("Plane has transform!");
    }
    cmd.spawn((
        MainCamera,
        Transform::from_xyz(20.0, 10.0, 20.0).looking_at(
            if let Some(trans) = plane_transform {
                trans.translation
            } else {
                Vec3::ZERO
            },
            Vec3::Y,
        ),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.025,
            far: 200.0,
            near: 0.0,
            area: Rect::new(-0.5, -0.5, 0.5, 0.5),
            ..OrthographicProjection::default_3d()
        }),
    ));
    cmd.spawn((PointLight::default(), Transform::from_xyz(4.0, 5.0, 6.0)));
    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Srgba::rgb_u8(50, 100, 230)),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
