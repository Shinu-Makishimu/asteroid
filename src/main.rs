use std::f32::consts::PI;

use bevy::{
    prelude::*, 
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle, transform, input::{keyboard::KeyboardInput, ButtonState}
};

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 720;
const ASTEROID_VELOCITY:f32 = 1.0;
const SHIP_ROTATION_SPEED: f32 = 0.15;
const SHIP_ACCELERATE_SPEED: f32 = 1.0;
const BULLET_VELOCITY:f32 = ASTEROID_VELOCITY * 2.0;
const FIRE_RANGE: f32 = WINDOW_HEIGHT as f32 / 3.0;
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
        .add_system(ship_rotation)
        .add_system(fire_range)
        .add_system(keyboard_events)
        .run();
}

#[derive(Debug)]
enum AsteroidSize{
    Big, Medium, Small
}

#[derive(Component)]
struct Ship{
    rotation: f32,
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
        transform.scale = Vec3::splat(match asteroid.size {
            AsteroidSize::Big => 100.0,
            AsteroidSize::Medium => 50.0,
            AsteroidSize::Small => 25.0,
        })
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
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<(&mut Ship, &Position, &mut Velocity)>,
) {
    for (mut ship, position, mut velocity) in &mut query {
        if keys.pressed(KeyCode::Left) {
            ship.rotation += SHIP_ROTATION_SPEED;
        } else if keys.pressed(KeyCode::Right) {
            ship.rotation -= SHIP_ROTATION_SPEED;
        }

        if keys.pressed(KeyCode::Up) {
            velocity.0 = velocity.0.normalize_or_zero() * (velocity.0.length() + SHIP_ACCELERATE_SPEED);
        }
    

    for ev in key_evr.iter() {
        if let (ButtonState::Pressed, Some(KeyCode::Space)) = (ev.state, ev.key_code) {
            let (y,x) = (ship.rotation + 2.0 * PI/ 4.0).sin_cos();
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
            .insert(Velocity(Vec2::new(x,y).normalize() * BULLET_VELOCITY));
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