use core::{DungeonGenerator, RoomType, Room, Rectangle};
use rand::{SeedableRng, StdRng, Rng};
use std::collections::VecDeque;
use std::cmp::{min, max};

/// Simple Binary Space Partioning dungeon generation algorithm:
///
///  1. Workspace splitting to random rectangles with the help of binary tree
///  2. In every rectangle generating a random room
///  3. Coridors generating with the help of the binary tree, generated at the first step
pub struct BSPGenerator {
    bs_max_deep: u32,
    deny_split_deep: u32,
    split_chance_dec: u32,
    split_max_coeff: u32,
    split_default_chance: u32,
    min_room_splittable_size: u32,
    coridor_width: u32,
}

impl Default for BSPGenerator {
    fn default() -> BSPGenerator {
        BSPGenerator {
            bs_max_deep: 5,
            deny_split_deep: 3,
            split_chance_dec: 30,
            split_max_coeff: 70,
            split_default_chance: 100,
            min_room_splittable_size: 150,
            coridor_width: 15,
        }
    }
}

impl BSPGenerator {
    /// levels of binary tree
    pub fn bs_max_deep(mut self, value: u32) -> Self {
        self.bs_max_deep = value;
        self
    }

    /// level of tree from which node can deny split
    pub fn deny_split_deep(mut self, value: u32) -> Self {
        self.deny_split_deep = value;
        self
    }

    /// split chance decreasment per tree level starting "deny_split_deep"
    pub fn split_chance_dec(mut self, value: u32) -> Self {
        self.split_chance_dec = value;
        self
    }

    /// max splitting coefficient
    pub fn split_max_coeff(mut self, value: u32) -> Self {
        self.split_max_coeff = value;
        self
    }

    /// default splitting chance
    pub fn split_default_chance(mut self, value: u32) -> Self {
        self.split_default_chance = value;
        self
    }

    /// minimum splittable size of subdungeon
    pub fn min_room_splittable_size(mut self, value: u32) -> Self {
        self.min_room_splittable_size = value;
        self
    }

    /// constant width of corridors
    pub fn coridor_width(mut self, value: u32) -> Self {
        self.coridor_width = value;
        self
    }

    fn generate_coridor_vert(&mut self, bx1: Rectangle, bx2: Rectangle, rng: &mut StdRng, rooms: &mut Vec<Room>) {
        let max_min_x = max(bx1[0], bx2[0]);
        let min_max_x = min(bx1[0]+bx1[2], bx2[0]+bx2[2])-self.coridor_width;
        let mut coridor = [0,0,self.coridor_width,0];
        coridor[1] = bx1[1] + bx1[3];
        coridor[3] = bx2[1] - bx1[1] - bx1[3];
        if max_min_x < min_max_x {
            // I like coridor
            coridor[0] = rng.gen_range(max_min_x, min_max_x);
        }
        else {
            // L like coridor
            coridor[0] = rng.gen_range(bx1[0], bx1[0]+bx1[2]);
            coridor[3] += bx2[3]/2;
            let mut h_coridor = [0,coridor[1]+coridor[3]-self.coridor_width,0,self.coridor_width];
            if max_min_x == bx1[0] {
                h_coridor[0] = min_max_x;
                h_coridor[2] = coridor[0] - h_coridor[0];
            }
            else {
                h_coridor[0] = coridor[0];
                h_coridor[2] = max_min_x - coridor[0];
            }
            rooms.push(Room::new(RoomType::Coridor, h_coridor));
        }
        rooms.push(Room::new(RoomType::Coridor, coridor));
    }

    fn generate_coridor_hor(&mut self, bx1: Rectangle, bx2: Rectangle, rng: &mut StdRng, rooms: &mut Vec<Room>) {
        let max_min_y = max(bx1[1], bx2[1]);
        let min_max_y = min(bx1[1]+bx1[3], bx2[1]+bx2[3])-self.coridor_width;
        let mut coridor = [0,0,0,self.coridor_width];
        coridor[0] = bx1[0] + bx1[2];
        coridor[2] = bx2[0] - bx1[0] - bx1[2];
        if max_min_y < min_max_y {
            // - like coridor
            coridor[1] = rng.gen_range(max_min_y, min_max_y);
        }
        else {
            // L like coridor
            coridor[1] = rng.gen_range(bx1[1], bx1[1]+bx1[3]);
            coridor[2] += bx2[2]/2;
            let mut v_coridor = [coridor[0]+coridor[2]-self.coridor_width,0,self.coridor_width,0];
            if max_min_y == bx1[1] {
                v_coridor[1] = min_max_y;
                v_coridor[3] = coridor[1] - v_coridor[1];
            }
            else {
                v_coridor[1] = coridor[1];
                v_coridor[3] = max_min_y - coridor[1];
            }
            rooms.push(Room::new(RoomType::Coridor, v_coridor));
        }
        rooms.push(Room::new(RoomType::Coridor, coridor));
    }


}

#[derive(Default, Clone, Copy, Debug)]
struct SubDungeon {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub deep: u32,
    pub childs: Option<[usize; 2]>,
    pub parent: Option<usize>,
    pub bounding_rect: Option<Rectangle>,
}

impl SubDungeon {
    pub fn new(x: u32, y: u32, w: u32, h: u32, deep: u32, parent: Option<usize>) -> SubDungeon {
        SubDungeon {
            x: x,
            y: y,
            width: w,
            height: h,
            deep: deep,
            childs: None,
            parent: parent,
            bounding_rect: None,
        }
    }
}

impl DungeonGenerator for BSPGenerator {
    fn generate(&mut self, seed: &[usize], width: u32, height: u32) -> Vec<Room> {
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut dungeon = Vec::<SubDungeon>::new();
        dungeon.push(SubDungeon::new(0, 0, width, height, 0, None));
        let mut queue = VecDeque::<usize>::new();
        queue.push_back(0);
        // generating subdungeons
        while !queue.is_empty() {
            // current index in queue
            let i = queue.pop_front().unwrap();
            // checking now deep we are
            let deep = dungeon[i].deep;
            // counting split chance
            let mut sp = self.split_default_chance;
            if deep >= self.deny_split_deep {
                if deep > self.bs_max_deep {
                    break;
                }
                let deny_split_chance = (deep - self.deny_split_deep + 1)*self.split_chance_dec;
                sp = if deny_split_chance > 100 {
                    0
                }
                else {
                    sp - deny_split_chance
                };
            }
            let w = dungeon[i].width;
            let h = dungeon[i].height;
            // will it be splited, or not?
            let b = ((w > self.min_room_splittable_size) || (h > self.min_room_splittable_size)) && (rng.gen_range(0, 100) < sp);
            if b {
                // How fair will be split
                let coef = rng.gen_range(100 - self.split_max_coeff, self.split_max_coeff);
                let x1 = dungeon[i].x;
                let y1 = dungeon[i].y;
                let mut x2 = x1;
                let mut y2 = y1;
                let mut w1 = w;
                let mut h1 = h;
                // Split vertical or not
                if (h < self.min_room_splittable_size) || ((w > self.min_room_splittable_size) && (rng.gen_range(0, 2) == 1)) {
                    x2 = x1 + w*coef/100;
                    w1 = x2 - x1;
                }
                else {
                    y2 = y1 + h*coef/100;
                    h1 = y2 - y1;
                }
                dungeon.push(SubDungeon::new(x1, y1, w1, h1, deep+1, Some(i)));
                dungeon.push(SubDungeon::new(x2, y2, w-(x2-x1), h-(y2-y1), deep+1, Some(i)));
                dungeon[i].childs = Some([dungeon.len()-2, dungeon.len()-1]);
                queue.push_back(dungeon.len()-2);
                queue.push_back(dungeon.len()-1);
            }
        }
        // generation rooms
        let mut rooms = Vec::<Room>::new();
        queue.clear();
        for i in 0..dungeon.len() {
            //let has_childs = dungeon[i].childs.is_some();
            if dungeon[i].childs.is_none() {
                let parent = {
                    let d = dungeon[i];
                    // generation rooms
                    let mut room = [0,0,0,0];
                    room[2] = rng.gen_range(10*d.width/20, 16*d.width/20);
                    room[3] = rng.gen_range(10*d.height/20, 16*d.height/20);
                    room[0] = rng.gen_range(d.x + d.width/20, d.x + d.width - room[2] - d.width/20);
                    room[1] = rng.gen_range(d.y + d.height/20, d.y + d.height - room[3] - d.height/20);
                    rooms.push(Room::new(RoomType::BasicRoom, room));
                    dungeon[i].bounding_rect = Some(room.clone());
                    dungeon[i].parent.unwrap()
                };
                // push parent to generate corridor
                let childs = dungeon[parent].childs.unwrap();
                if (dungeon[childs[0]].bounding_rect.is_some()) && (dungeon[childs[1]].bounding_rect.is_some()) {
                    queue.push_back(parent);
                }
            }
        }
        // generating coridors
        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            let p = dungeon[i];
            //println!("{:?}", p);
            let childs = p.childs.unwrap();
            let (ch1, ch2) = (dungeon[childs[0]], dungeon[childs[1]]);
            //println!("{:?}", ch1);
            //println!("{:?}", ch2);
            let (bx1, bx2) = (ch1.bounding_rect.unwrap(), ch2.bounding_rect.unwrap());
            if ch1.x == ch2.x {
                self.generate_coridor_vert(bx1, bx2, &mut rng, &mut rooms);
            }
            else {
                self.generate_coridor_hor(bx1, bx2, &mut rng, &mut rooms);
            }
            // generation bounding box for parent subdungeon
            dungeon[i].bounding_rect = {
                if rng.gen_range(0, 2) == 1 {
                    Some(bx1.clone())
                }
                else {
                    Some(bx2.clone())
                }
            };
            // if both childs have bouinding rect => add parent to queue
            if let Some(pp) = dungeon[i].parent {
                let childs = dungeon[pp].childs.unwrap();
                if (dungeon[childs[0]].bounding_rect.is_some()) && (dungeon[childs[1]].bounding_rect.is_some()) {
                    queue.push_back(pp);
                }
            }
        }
        rooms
    }
}
