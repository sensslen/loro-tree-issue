use loro::{LoroMap, LoroMovableList, LoroList, LoroText, LoroTree, LoroValue, LoroCounter};

fn main() {
    // Create the main map and movable list
    let map = LoroMap::new();
    let list = LoroMovableList::new();

    // Insert a map at index 0
    let first = LoroMap::new();
    first.insert("optional_string__2", LoroValue::String("test".into())).unwrap();
    list.insert_container(0, first).unwrap();

    // Insert various container types at subsequent indices
    list.insert_container(1, LoroCounter::new()).unwrap();
    list.insert_container(2, LoroList::new()).unwrap();
    list.insert_container(3, LoroMovableList::new()).unwrap();
    list.insert_container(4, LoroText::new()).unwrap();
    list.insert_container(5, LoroTree::new()).unwrap();
    list.insert_container(6, LoroMap::new()).unwrap();

    // Insert another map at index 7
    let last = LoroMap::new();
    last.insert("int64__13", LoroValue::I64(i64::MAX)).unwrap();
    list.insert_container(7, last).unwrap();

    // Insert the list as a movable list container into the map
    let connected_list = map.insert_container("complex_array__1", list).unwrap();

    // Get the deep value and print it
    let deep_value = connected_list.get_deep_value();
    println!("{:?}", deep_value);
}
