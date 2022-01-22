fn main() {
    let name = "kkk";
    println!("Hello {0}!", name);

    // Define a tuple
    let foo: (i64, bool) = (1, true);
    println!("{:#?}", foo);

    //using a struct
    let data: Foo = Foo { bar: true, age: 10 };
    println!("Foo age is {} and bar is {}", data.age, data.bar);

    let kansas: City = new_city(2000, false);
    let city_type = if kansas.is_coastal {
        "coastal"
    } else {
        "non-coastal"
    };
    println!("Kansas is a {} city with {} residents", city_type, kansas.residents);
    println!("Kansas city can be described as : {}",kansas.description);

}

struct Foo {
    bar: bool,
    age: i64,
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City {
            description: format!(
                "a *non-coastal* city of approximately {} residents",
                residents
            ),
            residents,
            is_coastal,
        }
    }
}
