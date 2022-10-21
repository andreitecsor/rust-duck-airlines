enum NavigationAids {
    NDB = 2,
    VOR = 4,
    // if we set only VOR to 4, NDB will have the associated value 0 and VORDME 5
    VORDME = 6,
    // FIX { name: String, latitude: f32, longitude: f32 } 
}

fn main() {
    //region ENUMERATION
    println!("================ENUMERATION=================");

    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
    //endregion ENUMERATION
}
