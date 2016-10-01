// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
/// all the traits in one file


/// trait for Objects in this world
pub trait StdTrait<T> {

    fn update (&mut self)
    {
        println!("update");
    }

    fn serialize (&self) -> String;

    fn new_from_deserialized (input: &String) -> T;

    // TODO why does this default implementation not work with locations?
    // fn new_from_deserialized (input: &String) -> T
    // {
    //     serde_json::from_str(&input).unwrap()
    // }

}
