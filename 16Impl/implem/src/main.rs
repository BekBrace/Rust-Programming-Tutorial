// IMPL or Implementation: Way to add methods to a struct to make it more useful.

struct House {
    width : i32,
    height : i32,
}

impl House {
    fn prnt_dim_house(&self) {
        println!("House Dimensions: {} X {} ", self.width, self.height);
    }

    fn calc_dim_house(&self) {
        let calculation: i32 = self.width * self.height;
        println!("House Dimension Calc: {} X {} = {} ", self.width, self.height, calculation );
    }
}
fn main() {

    let my_house = House{width: 500, height: 250};
    my_house.prnt_dim_house();
    my_house.calc_dim_house();

}
