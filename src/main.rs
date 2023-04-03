use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// The Player object
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(move_player)
        .add_system(display_events.in_base_set(CoreSet::PostUpdate))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 30.0, 100.0)
            .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // Lighting
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.01;

    commands.spawn((
        // TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(ground_size).into()),
            material: materials.add(Color::hex("#DDDDDD").unwrap().into()),
            transform: Transform::from_xyz(0.0, -ground_height, 0.0),
            ..default()
        },
    ));

    // Spawn player
    commands.spawn((
        RigidBody::Dynamic,
        Player,
        Collider::ball(2.0),
        ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
        Velocity::default(),
        ActiveEvents::COLLISION_EVENTS,
        ContactForceEventThreshold(30.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 16,
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.0, 0.15, 0.8).into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
    ));

    // Spawn obstacles
    let obstacle_size = 2.0;
    commands.spawn((
        // RigidBody::Fixed,
        Collider::cuboid(obstacle_size, obstacle_size, obstacle_size),
        ColliderDebugColor(Color::hsl(0.0, 1.0, 220.3)),
        // ActiveEvents::COLLISION_EVENTS,
        // ContactForceEventThreshold(30.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                obstacle_size,
                obstacle_size,
                obstacle_size,
            ))),
            material: materials.add(Color::rgb(0.9, 0.15, 0.2).into()),
            transform: Transform::from_xyz(20.0, 1.0, 0.0),
            ..default()
        },
    ));
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.linvel -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.linvel += Vec3::new(1.0, 0.0, 0.0);
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {collision_event:?}");
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, event) => {
                // @TODO: Destroy the non-player entity
                // and trigger "merge" with player with destroyed entity's mesh
            }
            CollisionEvent::Stopped(_, _, _) => todo!(),
        }
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {contact_force_event:?}");
    }
}
