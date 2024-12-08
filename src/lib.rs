pub mod template;

// Use this file to add helper functions and additional modules.

pub mod map2d {
    pub fn print_map(
        map_size: (u32, u32),
        empty: char,
        poi: &std::collections::HashMap<(u32, u32), char>,
    ) {
        for y in 0..map_size.1 {
            for x in 0..map_size.0 {
                if let Some(p) = poi.get(&(x, y)) {
                    print!("{}", p);
                } else {
                    print!("{}", empty);
                }
            }
            println!();
        }
    }
}
