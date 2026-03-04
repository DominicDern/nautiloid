use std::collections::BTreeMap;

type TerrainId = u32;
type SetId = u32;

// Represents a terrain piece
struct Terrain {
    id: TerrainId,
    name: String,
    number_printed: usize,
}

impl Terrain {
    fn new(name: String, id: TerrainId) -> Self {
        Self {
            id,
            name,
            number_printed: 0,
        }
    }
}

struct Set {
    id: SetId,
    name: String,
    // (name of piece, amount of pieces needed in set)
    pieces: Vec<(TerrainId, usize)>,
}

impl Set {
    fn new(id: SetId, name: String, pieces: Vec<(TerrainId, usize)>) -> Self {
        Self { id, name, pieces }
    }
}

struct TerrainDb {
    current_terrain_id: TerrainId,
    current_set_id: SetId,
    terrain: BTreeMap<TerrainId, Terrain>,
    sets: BTreeMap<SetId, Set>,
}

impl TerrainDb {
    fn new() -> Self {
        Self {
            current_terrain_id: 0,
            current_set_id: 0,
            terrain: BTreeMap::new(),
            sets: BTreeMap::new(),
        }
    }
}

impl TerrainDb {
    fn add_terrain(&mut self, name: String) {
        self.current_terrain_id += 1;
        let terrain_piece = Terrain::new(name, self.current_terrain_id);
        self.terrain.insert(self.current_terrain_id, terrain_piece);
    }

    fn add_set(&mut self, name: String, pieces: Vec<(TerrainId, usize)>) {
        self.current_set_id += 1;
        let set = Set::new(self.current_set_id, name, pieces);
        self.sets.insert(self.current_set_id, set);
    }

    fn remove_terrain(&mut self, name: &str) {
        let mut found: Option<TerrainId> = None;
        for (id, terrain) in &self.terrain {
            if terrain.name == name {
                found = Some(*id)
            }
        }
        if let Some(id) = found {
            self.terrain.remove(&id);
        }
    }

    fn remove_set(&mut self, name: &str) {
        let mut found: Option<SetId> = None;
        for (id, set) in &self.sets {
            if set.name == name {
                found = Some(*id);
            }
        }
        if let Some(id) = found {
            self.sets.remove(&id);
        }
    }
}

fn main() {}
