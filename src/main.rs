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
        .add_system(move_player)
        .add_system(display_events.in_base_set(CoreSet::PostUpdate))
        .add_system(attach_event)
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

                // Filter all objects in the scene by the entity passed through the event
                let (_, mut collider_transform) = attachable_objects
                    .get_mut(collider_entity)
                    .expect("Couldn't find collider object to attach. Might have been destroyed.");

                println!("Object position {}", collider_transform.translation);

                let player_entity = player_entity.get_single().unwrap();

                // Remove the collider from object (you can mutate transform with it gone)
                commands.entity(collider_entity).remove::<Collider>();
                // Change transform from scene space to relative to object
                // collider_transform.translation.x -= collider_transform.translation.x - 2.0;
                // collider_transform.translation.y = 0.0;
                // collider_transform.translation.z -= collider_transform.translation.z - 2.0;

                /* Find the contact pair, if it exists, between two colliders. */
                if let Some(contact_pair) =
                    rapier_context.contact_pair(collider_entity, player_entity)
                {
                    // The contact pair exists meaning that the broad-phase identified a potential contact.
                    if contact_pair.has_any_active_contacts() {
                        // The contact pair has active contacts, meaning that it
                        // contains contacts for which contact forces were computed.
                    }

                    // We may also read the contact manifolds to access the contact geometry.
                    for manifold in contact_pair.manifolds() {
                        println!("Local-space contact normal: {}", manifold.local_n1());
                        println!("Local-space contact normal: {}", manifold.local_n2());
                        println!("World-space contact normal: {}", manifold.normal());

                        let collision_point = manifold.local_n1();
                        let padding = Vec3::splat(3.0);
                        collider_transform.translation = collision_point * padding;

                        // Read the geometric contacts.
                        for contact_point in manifold.points() {
                            // Keep in mind that all the geometric contact data are expressed in the local-space of the colliders.
                            println!(
                                "Found local contact point 1: {:?}",
                                contact_point.local_p1()
                            );
                            println!("Found contact distance: {:?}", contact_point.dist()); // Negative if there is a penetration.
                            println!("Found contact impulse: {}", contact_point.impulse());
                            println!(
                                "Found friction impulse: {:?}",
                                contact_point.tangent_impulse()
                            );
                        }

                        // Read the solver contacts.
                        for solver_contact in manifold.solver_contacts() {
                            // Keep in mind that all the solver contact data are expressed in world-space.
                            println!("Found solver contact point: {:?}", solver_contact.point());
                            println!("Found solver contact distance: {:?}", solver_contact.dist());
                            // Negative if there is a penetration.
                        }
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
