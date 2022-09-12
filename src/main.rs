use bevy::{prelude::*, input::keyboard, render::camera::{ScalingMode,  Projection}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mover)
        .run();
}

#[derive(Component)]
pub struct Example;

fn setup(
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>) {
     commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.25, 0.0),
        ..default()
    }).insert(Example);

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

}

fn mover(inp: Res<Input<KeyCode>>, time : Res<Time>, mut query: Query<&mut Transform, With<Example>>, mut proj: Query<&mut Projection>) {
    let mut transf = query.single_mut();
    let mut cam = proj.single_mut();
    if inp.pressed(KeyCode::A) {
        transf.scale = Vec3::new(1.0, 1.0, 1.0) * (2.0 * ( time.seconds_since_startup() * 2.0 ).sin().abs()  as f32);
        println!("{}", 10.0 * ( time.seconds_since_startup() / 100.0 ).sin().abs()  as f32);
    }
    if inp.pressed(KeyCode::S) {
        transf.translate_around(Vec3::new(0.0, 0.0, 1.0), Quat::from_rotation_y(25.0));
    }
    if inp.pressed(KeyCode::D) {
       transf.rotate_local_y(0.1);
    }
    if inp.pressed(KeyCode::F) {
        *cam = Projection::Orthographic( OrthographicProjection {
           scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        });
    } else {
        *cam = Projection::Perspective(PerspectiveProjection {..default()})
    }
}



