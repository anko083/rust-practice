fn draw_tree(triangles: usize) {
    let height = 5;  
    let width = height * 2 - 1; 
    
    (1..=triangles).for_each(|i| {
        (0..height).for_each(|j| {
            let stars = 2 * j + 1; 
            let spaces = (width - stars) / 2; 

                    
                    (0..spaces).for_each(|_| print!(" "));
                    (0..stars).for_each(|_| print!("*"));
                    println!(); 
                });
            });
        
           
            (0..2).for_each(|_| {
                (0..(width / 3)).for_each(|_| print!(" "));
                (0..(width / 3)).for_each(|_| print!("*"));
                println!();
            });
        }
        
        fn main() {
            let triangles = 3; 
            draw_tree(triangles);
        }