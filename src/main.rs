use std::f32::consts::PI;
use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle, 
    input::{keyboard::KeyboardInput, ButtonState},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

const WINDOW_WIDTH: usize = 1024; //screen width
const WINDOW_HEIGHT: usize = 720; //screen height
const ASTEROID_VELOCITY:f32 = 1.0; //asteroid speed
const ASTEROID_START_SIZE:f32 = 100.0; //asteroid spawn size
const SHIP_ROTATION_SPEED: f32 = 0.05; //ship rotation speed
const SHIP_ACCELERATE_SPEED: f32 = 0.7; //ship acceleration
const SHIP_MAXIMUM_SPEED: f32 = 7.0; //ship max speed
const SHIP_COLLISION_DEVIDER: f32 = 4.0; //Configure collision radius for ship
const SPACE_RESISTANCE: f32 = 0.05; // getting ship slower
const BULLET_VELOCITY:f32 = SHIP_MAXIMUM_SPEED * 2.0; //shup's bullet speed
const FIRE_RANGE: f32 = WINDOW_HEIGHT as f32 / 2.0; // bullet range
const VIEWPORT_MAX_X: f32 = WINDOW_WIDTH as f32 / 2.0;
const VIEWPORT_MIN_X: f32 = -VIEWPORT_MAX_X;
const VIEWPORT_MAX_Y: f32 = WINDOW_HEIGHT as f32 / 2.0;
const VIEWPORT_MIN_Y: f32 = -VIEWPORT_MAX_Y;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_velocity)
        .add_system(update_position)
        .add_system(update_asteroid)
        .add_system(space_resistance)
        .add_system(ship_rotation)
        .add_system(fire_range)
        .add_system(keyboard_events)
        .add_system(detect_ship_collision)
        .add_system(detect_bullet_collision)
        .run();
}

#[derive(Debug, Clone, Copy)]
enum AsteroidSize{
    Big, Medium, Small
}

impl AsteroidSize {
    fn get_scale(&self) ->f32 {
        match self {
            AsteroidSize::Big => ASTEROID_START_SIZE,
            AsteroidSize::Medium => ASTEROID_START_SIZE * 0.5,
            AsteroidSize::Small => ASTEROID_START_SIZE * 0.25,
        }
    }
}

#[derive(Component)]
struct Ship{
    rotation: f32,
}

impl Ship {
    fn get_direction (&self) -> Vec2 {
        let (y,x) = (self.rotation + 2.0 * PI / 4.0).sin_cos();
        Vec2::new(x,y)
    }
}

#[derive(Component)]
struct Bullet {
    start:Vec2,
}

#[derive(Component)]
struct Asteroid {
    size: AsteroidSize,
}

#[derive(Component)]
struct Velocity (Vec2);


#[derive(Component)]
struct Position (Vec2);


fn create_triangle() -> Mesh {
    //create "ship" just a triangle
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.5, 0.0], [-0.25, -0.5, 0.0], [0.25, -0.5, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0,1,2])));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.5,0.0],[0.0,1.0],[1.0,1.0]]
    );
    mesh
}

fn get_random_point() -> Vec2{
    Vec2::new(
        (rand::random::<f32>()*2.0-1.0)* (WINDOW_WIDTH as f32) / 2.0,
        (rand::random::<f32>()*2.0-1.0)* (WINDOW_HEIGHT as f32) / 2.0,
    )

}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    window.set_resolution(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(create_triangle()).into(),
        transform: Transform::default().with_scale(Vec3::splat(50.)),
        material: materials
                .add(ColorMaterial::from(Color::WHITE)),
        ..default()
    })
    .insert(Ship { rotation: 0.0 })
    .insert(Position(Vec2::new(0.0, 0.0)))
    .insert(Velocity(Vec2::splat(0.0)));

    for _ in 0..4 {
        //this cycle placing asteroid in random place in window
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)),
            material: materials
                    .add(ColorMaterial::from(Color::GRAY)),
            ..default()
        })
        .insert(Asteroid {
            size: AsteroidSize::Big,
        })
        .insert(Position(get_random_point()))
        .insert(Velocity(get_random_point().normalize() * ASTEROID_VELOCITY));

    }
}

fn ship_rotation(mut query: Query<(&Ship, &mut Transform)>) {
    for (ship, mut transform) in &mut query {
        let angle = ship.rotation;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn update_position(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation = Vec3::new(position.0.x, position.0.y,transform.translation.z);
    }
}

fn update_asteroid(mut query: Query<(&Asteroid, &mut Transform)>) {
    //function for resize asteroid
    for (asteroid, mut transform) in &mut query {
        transform.scale = Vec3::splat(asteroid.size.get_scale())
    }
}

fn update_velocity(mut query: Query<(&Velocity, &Transform, &mut Position)>) {
    //function for asteroid movieng
    for (velocity, transform, mut position) in &mut query {
        let mut new_position = position.0 + velocity.0;
        let scale = transform.scale.max_element() / 2.0;

        if new_position.x > VIEWPORT_MAX_X + scale{
            new_position.x = VIEWPORT_MIN_X - scale;
        } else if new_position.x < VIEWPORT_MIN_X - scale {
            new_position.x = VIEWPORT_MAX_X + scale;
        }

        if new_position.y > VIEWPORT_MAX_Y + scale {
            new_position.y = VIEWPORT_MIN_Y - scale;
        } else if new_position.y < VIEWPORT_MIN_Y - scale {
            new_position.y = VIEWPORT_MAX_Y + scale;
        }
        position.0 = new_position;
    }
}

fn keyboard_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<(&mut Ship, &Position, &mut Velocity)>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut ship, position, mut velocity) in &mut query {
        if keys.pressed(KeyCode::Left) {
            ship.rotation += SHIP_ROTATION_SPEED;
        } else if keys.pressed(KeyCode::Right) {
            ship.rotation -= SHIP_ROTATION_SPEED;
        }

        if keys.pressed(KeyCode::Up) {
            velocity.0 += ship.get_direction() * SHIP_ACCELERATE_SPEED;

            if velocity.0.length() > SHIP_MAXIMUM_SPEED {
                velocity.0 = velocity.0.normalize_or_zero() * SHIP_MAXIMUM_SPEED;

            }
        }
    

    for ev in key_evr.iter() {
        if let (ButtonState::Pressed, Some(KeyCode::Space)) = (ev.state, ev.key_code) {
            
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::default().with_scale(Vec3::splat(6.0)),
                material: materials
                        .add(ColorMaterial::from(Color::WHITE)),
                ..default()
            })
            .insert(Bullet {
                start: position.0.clone(),
            })
            .insert(Position(position.0.clone()))
            .insert(Velocity(ship.get_direction().normalize() * BULLET_VELOCITY));
        }
    }
    }
}

fn fire_range(
    mut commands: Commands,
    mut query: Query<(Entity, &Bullet, &Position)>
){
    for (entity, bullet, position) in &mut query{
        if (bullet.start - position.0).length() > FIRE_RANGE {
            commands.entity(entity).despawn();
        }
    }
}

fn space_resistance(
    keys: Res<Input<KeyCode>>, 
    mut query: Query<&mut Velocity, With<Ship>>
) {
    if !keys.pressed(KeyCode::Up) {
        for mut velocity in &mut query {
            velocity.0 *= 1.0 - SPACE_RESISTANCE;
        }
    }
}

fn detect_ship_collision(
    mut commands: Commands,
    ship_query: Query<(Entity, &Transform, &Position), With<Ship>>,
    asteroid_query:Query<(&Transform, &Position), With<Asteroid>>,
){
    //ship collision is just a circle inside the triangle. It's most easyest way
    for (ship_entity, ship_transform, ship_position ) in &ship_query {
        for (asteroid_transform, asteroid_position) in &asteroid_query {
            let ship_size = ship_transform.scale.max_element();
            let asteroid_size = asteroid_transform.scale.max_element();

            let distance = (ship_position.0 - asteroid_position.0).length();
            if distance < ship_size / SHIP_COLLISION_DEVIDER  + asteroid_size / 2.0 {
                commands.entity(ship_entity).despawn();
            }
        }
    }
}

fn detect_bullet_collision(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bullet_query: Query<(Entity, &Transform, &Position), With<Bullet>>,
    asteroid_query:Query<(Entity, &Asteroid, &Transform, &Position), With<Asteroid>>,
){
    for (bullet_entity, bullet_transform, bullet_position ) in &bullet_query {
        for (asteroid_entity, asteroid, asteroid_transform, asteroid_position) in &asteroid_query {
            
            let bullet_size = bullet_transform.scale.max_element();
            let asteroid_size = asteroid_transform.scale.max_element();

            let distance = (bullet_position.0 - asteroid_position.0).length();
            if distance < (bullet_size + asteroid_size) / 2.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(asteroid_entity).despawn();

                let asteroid_new_size = match asteroid.size {
                    AsteroidSize::Big => Some(AsteroidSize::Medium),
                    AsteroidSize::Medium => Some(AsteroidSize::Small),
                    AsteroidSize::Small => None,
                };

                if let Some(asteroid_new_size) = asteroid_new_size {
                    for _ in 0..2 {
                        //this cycle placing asteroid in random place in window
                        commands.spawn(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                            transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)),
                            material: materials
                                    .add(ColorMaterial::from(Color::GRAY)),
                            ..default()
                        })
                        .insert(Asteroid {
                            size: asteroid_new_size,
                        })
                        .insert(Position(asteroid_position.0.clone()))
                        .insert(Velocity(get_random_point().normalize() * ASTEROID_VELOCITY));
                
                    }
                }
            }
        }
    }
}
