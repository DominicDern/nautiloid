use std::{
    collections::BTreeMap,
    hash::{DefaultHasher, Hash, Hasher},
};

type Id = u32;

// Represents a terrain piece
struct Terrain {
    id: Id,
    name: String,
    number_printed: u16,
}

impl Terrain {
    fn new(name: String, id: Id) -> Self {
        Self {
            id,
            name,
            number_printed: 0,
        }
    }
}

struct Set {
    id: Id,
    name: String,
    // (name of piece, amount of pieces needed in set)
    pieces: Vec<(String, u16)>,
}

impl Set {
    fn new(name: String, id: Id) -> Self {
        Self {
            id,
            name,
            pieces: Vec::new(),
        }
    }
}

struct TerrainDb {
    current_terrain_id: Id,
    current_set_id: Id,
    terrain: BTreeMap<Id, Terrain>,
    sets: BTreeMap<Id, Set>,
}

impl TerrainDb {
    fn add_terrain(&mut self, name: String) {
        self.current_terrain_id += 1;
        let terrain_piece = Terrain::new(name, self.current_terrain_id);
        self.terrain.insert(self.current_terrain_id, terrain_piece);
    }
}

fn main() {
    println!("Hello, world!");
}
