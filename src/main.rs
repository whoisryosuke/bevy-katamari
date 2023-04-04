use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// The Player object
#[derive(Component)]
struct Player;

// The Floor object. Used to filter some collision events.
#[derive(Component)]
struct Floor;

// Objects that can "attach" to our player's ball
#[derive(Component)]
struct BallObject;

// Camera that follows the player
#[derive(Component)]
struct FollowCamera {
    distance: Vec3,
}
impl Default for FollowCamera {
    fn default() -> Self {
        FollowCamera {
            distance: Vec3::new(0.0, 3.0, 20.0),
        }
    }
}

// Events
// Attach object to player's ball
// @TODO: Maybe wrap Entity in an Option? So we can default to None?
#[derive(Default)]
struct AttachObjectEvent(Option<Entity>);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_event::<AttachObjectEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(camera_follow)
        .add_system(move_player)
        .add_system(display_events.in_base_set(CoreSet::PostUpdate))
        .add_system(attach_event)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-30.0, 30.0, 100.0)
                .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(FollowCamera::default());

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
        Floor,
        Collider::cuboid(ground_size, ground_height, ground_size),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(ground_size).into()),
            material: materials.add(Color::hex("#DDDDDD").unwrap().into()),
            transform: Transform::from_xyz(0.0, -ground_height, 0.0),
            ..default()
        },
    ));

    // Spawn player
    let player_size = 3.0;
    commands.spawn((
        Player,
        // Physics
        // Necessary collider "boxes" around player
        RigidBody::Dynamic,
        Collider::ball(player_size),
        ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
        // Needed to "move" or change speed of object
        Velocity::default(),
        // Needed to detect collision events
        ActiveEvents::COLLISION_EVENTS,
        ContactForceEventThreshold(30.0),
        // Mesh
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: player_size,
                sectors: 16,
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.0, 0.15, 0.8).into()),
            transform: Transform::from_xyz(0.0, player_size * 1.5, 0.0),
            ..default()
        },
    ));

    // Spawn obstacles
    let obstacle_size = 2.0;
    for index in 0..4 {
        let direction = if 0 == index % 2 { 1.0 } else { -1.0 };
        let offset = (index as f32 + 1.0) * direction;
        commands.spawn((
            BallObject,
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
                transform: Transform::from_xyz(10.0 * offset, 1.0, 10.0 * offset),
                ..default()
            },
        ));
    }
}

fn camera_follow(
    mut camera_query: Query<(&FollowCamera, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let (camera_state, mut camera_transform) = camera_query
        .get_single_mut()
        .expect("Follow camera not found.");

    let player_transform = player_query
        .get_single()
        .expect("Player not found for follow camera.");

    camera_transform.translation = player_transform.translation + camera_state.distance;
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.linvel -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.linvel += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.linvel -= Vec3::new(0.0, 0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.linvel += Vec3::new(0.0, 0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::Space) {
        player_velocity.linvel += Vec3::new(0.0, 2.0, 0.0);
    }
}

fn display_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut attach_events: EventWriter<AttachObjectEvent>,
    player_entity: Query<Entity, With<Player>>,
    floor_entity: Query<Entity, With<Floor>>,
) {
    // Get the index of player and floor entities to check for later
    let player = player_entity
        .get_single()
        .expect("Player not found in scene");
    let floor = floor_entity
        .get_single()
        .expect("Player not found in scene");
    let floor_index = floor.index();

    // Check for collisions
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                // If we collided with floor, don't care
                if first_entity.index() == floor_index || second_entity.index() == floor_index {
                    println!("Collided with floor");
                    return;
                }

                // @TODO: Destroy the non-player entity
                // and trigger "merge" with player with destroyed entity's mesh
                println!(
                    "{} collided with {}",
                    first_entity.index(),
                    second_entity.index()
                );

                // Check which object isn't the player
                let mut collider_index = first_entity.index();
                let mut collider_entity = first_entity;
                if collider_index == player.index() {
                    collider_index = second_entity.index();
                    collider_entity = second_entity;
                }

                // Attach object to player
                attach_events.send(AttachObjectEvent(Some(*collider_entity)));

                // Destroy the non-player
                // commands.entity(*collider_entity).despawn();
            }
            CollisionEvent::Stopped(first_entity, second_entity, event) => {}
        }
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {contact_force_event:?}");
    }
}

fn attach_event(
    mut commands: Commands,
    mut attach_events: EventReader<AttachObjectEvent>,
    mut attachable_objects: Query<(Entity, &mut Transform), With<BallObject>>,
    player_entity: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    // Check for events
    if !attach_events.is_empty() {
        // We loop over all events and use the event's collider entity index
        attach_events.iter().for_each(|collider_event| {
            let AttachObjectEvent(collider_entity_result) = collider_event;
            if let Some(mut collider_entity) = collider_entity_result {
                println!("Attaching entity ID {}", collider_entity.index());

                // Get the collided object's transform from query
                // Filters all objects in the scene by the entity passed through the event
                let (_, mut collider_transform) = attachable_objects
                    .get_mut(collider_entity)
                    .expect("Couldn't find collider object to attach. Might have been destroyed.");

                println!("Object position {}", collider_transform.translation);

                // Get player entity from query
                let player_entity = player_entity.get_single().unwrap();

                // Remove the collider from object (you can mutate transform with it gone)
                commands.entity(collider_entity).remove::<Collider>();

                // Check for the "contact point" between player and object
                if let Some(contact_pair) =
                    rapier_context.contact_pair(collider_entity, player_entity)
                {
                    // Get the "contact point" in local space
                    for manifold in contact_pair.manifolds() {
                        // Uses "contact point" local to object (not player)
                        let collision_point = manifold.local_n1();
                        // We pad it a bit by the size of object
                        // @TODO: Grab size of object and use as padding
                        let padding = Vec3::splat(3.0);
                        // Update objects position relative to player (so it "orbits" properly)
                        collider_transform.translation = collision_point * padding;
                    }
                }

                // Attach object to player as child
                commands
                    .entity(player_entity)
                    .push_children(&[collider_entity]);
            }
        });
    }
}
