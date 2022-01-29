struct TriangleData {
    base: usize,
    height: usize,
    area: usize
}

impl TriangleData {
    fn get_base(&mut self)
    {
        println!();

        println!("Base: ");
        let mut bse: String = String::new();

        std::io::stdin().read_line(&mut bse).unwrap();

        let bse_int: usize = bse.trim().parse().unwrap();

        self.base = bse_int;
    }

    fn get_height(&mut self) {
        println!();
        println!("Height: ");

        let mut ht: String = String::new();

        std::io::stdin().read_line(&mut ht).unwrap();

        let height: usize = ht.trim().parse().unwrap();

        self.height = height;
    }

    fn comput_area(&mut self)
    {
        self.area = (self.base*self.height)/2
    }

    
}

fn main() {
    let mut current_triangle: TriangleData = TriangleData {
        base: 0,
        height: 0,
        area: 0,
    };

    current_triangle.get_base();

    current_triangle.get_height();

    current_triangle.comput_area();

    println!();

    println!(
        "Area: {}",
        current_triangle.area
    );
}
