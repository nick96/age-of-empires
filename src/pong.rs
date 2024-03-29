use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat, Camera, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
};

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const BALL_VX: f32 = 75.0;
pub const BALL_VY: f32 = 50.0;
pub const BALL_RAIDUS: f32 = 2.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Ball>();

        init_ball(world, sprite_sheet_handle.clone());
        init_paddles(world, sprite_sheet_handle);
        init_camera(world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transf = Transform::default();
    let mut right_transf = Transform::default();
    let y = ARENA_HEIGHT / 2.0;

    left_transf.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transf.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transf)
        .build();
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Right))
        .with(right_transf)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let ldr = world.read_resource::<Loader>();
        let store = world.read_resource::<AssetStorage<Texture>>();
        ldr.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &store,
        )
    };

    let ldr = world.read_resource::<Loader>();
    let store = world.read_resource::<AssetStorage<SpriteSheet>>();
    ldr.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &store,
    )
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn init_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transf = Transform::default();
    local_transf.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RAIDUS,
            velocity: [BALL_VX, BALL_VY],
        })
        .with(local_transf)
        .build();
}
