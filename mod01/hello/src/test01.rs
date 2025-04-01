enum Money{
    yen(u32),
    dollar(f64),
    euro(f64),
}

struct Items{
    name: String,
    price: Money,
    quantity:u32,
}

impl Items{
    fn new(name:&str,price:Money,quantity:u32)->Self{
        Self{
            name:name.to_string(),
            price:price,
            quantity:quantity,
        }
    }
    fn get_price(&self)->f64{
        let price_per_unit = match self.price{
            Money::yen(price) => price as f64,
            Money::dollar(price) => price,
            Money::euro(price) => price,};
        let quantity = self.quantity as f64;
        price_per_unit * quantity
    }
}


pub fn run(){
    println!("hello niikun!");
    let item ="mango";
    let price = 100;
    let quantity = 2;
    let fruit = Items::new(item,Money::yen(price),quantity);
    // println!("item: {}, price: {}, quantity: {}",fruit.name,fruit.price,fruit.quantity);
    println!("total price: {}",fruit.get_price());
}