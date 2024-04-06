use core::{mem::transmute, ptr::addr_of_mut};

use nes::{
    addr::Addr,
    apu::{self, Sfx},
    capped_vec::CappedVec,
    io, ppu,
    ppu_buffer::{self, BufferTrait},
    sprites::SpriteState,
    utils::Sign,
    vec2::{DPos, Orientation, Pos, Vec2},
};

use crate::{
    constants::{
        AT_SPRITE, DT, GRID_SIZE, HEART_SPRITE, HEIGHT, ORIGIN, PLAYER_SPEED, PLAYER_WIDTH, WIDTH,
    },
    level::{draw_level, get_tile_at, make_level, map_pos_to_tile_index, Tile},
    rng::{get_seeds, Rng},
    utils::u8_to_decimal,
    Buffer,
};

// TODO: store only seeds
const SEEDS: [u16; GRID_SIZE as usize] = get_seeds();
const LEVEL_TILES: [Tile; GRID_SIZE as usize] = make_level(&SEEDS);

// called before enabling nmi
pub unsafe fn init() {
    // palettes
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0E, 0x30, 0x12, 0x26]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 3, &[0x15]);

    draw_level(&LEVEL_TILES);

    // text
    ppu::draw_ascii(ORIGIN + 0x06, "HEART-MAN");
}

pub fn frame(game: &mut Game, apu: &mut apu::APU, sprites: &mut SpriteState) {
    Buffer::clear();
    game.step(apu);
    game.draw(sprites);
}

struct Player {
    pos: Pos,
    dead: bool,
}

struct Meanie {
    pos: Pos,
    vel: DPos,
    orientation: Orientation,
    n_turns: u8,
}

type Collision = Vec2<Option<Sign>>;

pub struct Game {
    rng: Rng,
    player: Player,
    grabbed_coins: CappedVec<u16, 30>,
    grabbed_coin_index: Option<u16>,
    n_coins: u8,
    meanies: CappedVec<Meanie, 6>,
}

fn make_meanie(rng: &mut Rng) -> Meanie {
    rng.cycle();
    Meanie {
        pos: random_pos_on_tile(rng, &LEVEL_TILES, Tile::Nothing, PLAYER_WIDTH),
        vel: random_cardinal_dir(rng),
        orientation: random_orientation(rng),
        n_turns: 0,
    }
}

impl Game {
    pub fn new(some_game: &mut Option<Game>) {
        let mut rng = Rng::new(None);
        *some_game = Some(Self {
            rng: rng.clone(),
            player: Player {
                pos: Pos { x: 16, y: 0 },
                dead: false,
            },
            grabbed_coins: CappedVec::new(),
            grabbed_coin_index: None,
            n_coins: 0,
            // more than 4 causes issues unless optimized for speed
            meanies: (0..4).map(|_| make_meanie(&mut rng)).into_iter().collect(),
        });
        let game = some_game.as_mut().unwrap();
        make_level(&SEEDS);

        game.n_coins = LEVEL_TILES
            .iter()
            .map(|t| match t {
                Tile::Coin => 1,
                _ => 0,
            })
            .sum();
    }

    fn step(&mut self, apu: &mut apu::APU) {
        update_player(&mut self.player, &LEVEL_TILES, apu);

        let player_center = self.player.pos.shifted(&DPos::new(4, 4));
        if let Tile::Coin = get_tile_at(&LEVEL_TILES, &player_center) {
            let index = map_pos_to_tile_index(&player_center);
            if !self.grabbed_coins.iter().any(|i| *i == index) {
                self.grabbed_coins.try_push(index).expect("Pockets full!");
                self.grabbed_coin_index = Some(index);
                self.n_coins -= 1;
                apu.play_sfx(Sfx::LevelUp);
            }
        }

        for meanie in self.meanies.iter_mut() {
            update_meanie(&LEVEL_TILES, meanie, &mut self.rng);
            if !self.player.dead && (self.player.pos.l1_dist(&meanie.pos) < PLAYER_WIDTH) {
                on_player_death(apu);
                self.player.dead = true;
            }
        }
    }

    fn draw(&mut self, sprites: &mut SpriteState) {
        draw_digits(Addr(ORIGIN), self.n_coins);

        if let Some(index) = self.grabbed_coin_index {
            Buffer::tile(Addr(ORIGIN + index), HEART_SPRITE);
            self.grabbed_coin_index = None;
        }

        sprites.add_at_pos(
            &self.player.pos,
            if self.player.dead {
                'x' as u8 - 32
            } else {
                HEART_SPRITE
            },
            0,
        );

        for meanie in &self.meanies {
            sprites.add_at_pos(&meanie.pos, AT_SPRITE, 0)
        }
    }
}

fn random_pos(rng: &mut Rng) -> Pos {
    Pos {
        x: { rng.next() % WIDTH },
        y: { rng.next() % HEIGHT },
    }
}

fn random_sign(rng: &mut Rng) -> Sign {
    match rng.next() % 2 {
        0 => Sign::Minus,
        1 => Sign::Plus,
        _ => unreachable!(),
    }
}

fn random_orientation(rng: &mut Rng) -> Orientation {
    match rng.next() % 2 {
        0 => Orientation::Clockwise,
        1 => Orientation::Widdershins,
        _ => unreachable!(),
    }
}

fn random_dir(rng: &mut Rng) -> DPos {
    DPos {
        x: random_sign(rng).to_i8(),
        y: random_sign(rng).to_i8(),
    }
}
fn random_cardinal_dir(rng: &mut Rng) -> DPos {
    match rng.next() % 4 {
        0 => DPos::x_unit(),
        1 => DPos::y_unit(),
        2 => DPos::x_unit().scaled(-1),
        3 => DPos::y_unit().scaled(-1),
        _ => unreachable!(),
    }
}

fn random_pos_on_tile(rng: &mut Rng, tiles: &[Tile], target_tile: Tile, width: u8) -> Pos {
    let mut pos: Pos;
    loop {
        pos = random_pos(rng);
        if (get_tile_at(tiles, &pos) == target_tile)
            & (get_tile_at(tiles, &pos.shifted(&DPos::new(width as i8, width as i8)))
                == target_tile)
        {
            break;
        }
    }
    pos
}

fn draw_digits(addr: Addr, x: u8) {
    let mut digits = [0; 3];
    for (x, y) in digits.iter_mut().rev().zip(u8_to_decimal(x).into_iter()) {
        *x = y
    }
    Buffer::tiles(addr, digits.map(|d| io::digit_to_ascii(d) - 32).into_iter())
}
fn on_player_death(apu: &mut apu::APU) {
    apu.play_sfx(Sfx::Topout);
    Buffer::draw_text(Addr(ORIGIN + 15), " IS DEAD");
}
fn update_player(player: &mut Player, tiles: &[Tile], apu: &mut apu::APU) {
    if player.dead {
        return;
    }
    let mut player_delta = player_movement_delta(io::controller_buttons(), &player.pos);

    let collision = check_box_collision(
        &tiles,
        Tile::Wall,
        PLAYER_WIDTH as i8,
        &player.pos,
        &player_delta,
    );
    if let Some(_) = collision.x {
        player_delta.x = 0;
        if !apu.is_playing() {
            apu.play_sfx(Sfx::Lock);
        }
    }
    if let Some(_) = collision.y {
        player_delta.y = 0;
        if !apu.is_playing() {
            apu.play_sfx(Sfx::Lock);
        }
    }

    player.pos.inc(&player_delta);
}

fn player_movement_delta(buttons: u8, player_pos: &Pos) -> DPos {
    let mut delta = DPos::zero();

    if buttons & io::LEFT != 0 && player_pos.x > 0 {
        delta.x = -PLAYER_SPEED;
    }
    if buttons & io::RIGHT != 0 && player_pos.x + 8 < WIDTH {
        delta.x = PLAYER_SPEED;
    }
    if buttons & io::UP != 0 && player_pos.y > 0 {
        delta.y = -PLAYER_SPEED;
    }
    if buttons & io::DOWN != 0 && player_pos.y + 8 < WIDTH {
        delta.y = PLAYER_SPEED;
    }

    delta
}

fn i8_to_sign(i: i8) -> Option<Sign> {
    if i > 0 {
        Some(Sign::Plus)
    } else if i < 0 {
        Some(Sign::Minus)
    } else {
        None
    }
}

fn check_box_collision(
    tiles: &[Tile],
    colliding_tile: Tile,
    width: i8,
    pos: &Pos,
    pos_delta: &DPos,
) -> Collision {
    let mut collision = Collision { x: None, y: None };
    for box_delta in [
        DPos::new(0, 0),
        DPos::new(0, width),
        DPos::new(width, 0),
        DPos::new(width, width),
    ] {
        let box_pos = pos.shifted(&box_delta);
        if get_tile_at(tiles, &box_pos.shifted(&pos_delta.x_vec())) == colliding_tile {
            collision.x = i8_to_sign(pos_delta.x);
        }
        if get_tile_at(tiles, &box_pos.shifted(&pos_delta.y_vec())) == colliding_tile {
            collision.y = i8_to_sign(pos_delta.y);
        }
    }
    collision
}

fn update_meanie(tiles: &[Tile], meanie: &mut Meanie, rng: &mut Rng) {
    let mut delta = DPos::zero();
    for _ in 0..3 {
        // stop trying after 3 attempts in case we're stuck
        const SPEED: u8 = 1;
        delta = meanie.vel.scaled((SPEED * DT) as i8);
        // need to check if we're going off the top of the screen
        let going_offscreen = (delta.y < 0) & (meanie.pos.y < SPEED * DT);
        if !going_offscreen {
            let collision =
                check_box_collision(tiles, Tile::Wall, PLAYER_WIDTH as i8, &meanie.pos, &delta);
            if let Vec2 { x: None, y: None } = collision {
                break;
            }
        }

        delta = delta.rotate(meanie.orientation);
        meanie.vel = delta;
        meanie.n_turns += 1;
    }

    meanie.pos.inc(&delta);

    if meanie.n_turns > 5 {
        meanie.orientation = random_orientation(rng);
        meanie.n_turns = 0
    }
}
