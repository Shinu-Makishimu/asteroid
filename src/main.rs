use bevy::{
    prelude::*, 
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle, transform
};

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 720;
const ASTEROID_VELOCITY:f32 = 1.0;
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
        .run();
}

#[derive(Debug)]
enum AsteroidSize{
    Big, Medium, Small
}

#[derive(Component, Debug)]
struct Ship;

#[derive(Component, Debug)]
struct Asteroid {
    size: AsteroidSize,
}

#[derive(Component, Debug)]
struct Velocity (Vec2);


#[derive(Component, Debug)]
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
    .insert(Position(Vec2::new(0.0, 0.0)))
    .insert(Velocity(Vec2::splat(0.0)))
    .insert(Ship);

    for _ in 0..4 {
        //this cycle placing asteroid in random place in window
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)),
            material: materials
                    .add(ColorMaterial::from(Color::GRAY)),
            ..default()
        })
        .insert(Velocity(get_random_point().normalize() * ASTEROID_VELOCITY))
        .insert(Asteroid {
            size: AsteroidSize::Big,
        })
        .insert(Position(get_random_point()));

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
            AsteroidSize::  Medium => 50.0,
            AsteroidSize::Small => 25.0,
        })
    }
}

fn update_velocity(mut query: Query<(&Velocity, &mut Position)>) {
    //function for asteroid movieng
    for (velocity, mut position) in &mut query {
        let mut new_position = position.0 + velocity.0;

        if new_position.x > VIEWPORT_MAX_X {
            new_position.x -= WINDOW_WIDTH as f32;
        } else if new_position.x < VIEWPORT_MIN_X{
            new_position.x += WINDOW_WIDTH as f32
        }

        if new_position.y > VIEWPORT_MAX_Y {
            new_position.y -= WINDOW_HEIGHT as f32;
        } else if new_position.y < VIEWPORT_MIN_Y {
            new_position.y += WINDOW_HEIGHT as f32
        }
        position.0 = new_position;
    }
}