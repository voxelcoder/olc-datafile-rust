use olc_datafile_rust::Datafile;

fn main() {
    let mut datafile = Datafile::new(Some(','), Some(" "));

    let some_node = datafile.get("some_node");
    some_node.get("name").set_string("Javid", 0);
    some_node.get("age").set_integer(24, 0);
    some_node.get("height").set_real(1.88, 0);

    let code = some_node.get("code");
    code.set_string("c++", 0);
    code.set_string("vhdl", 1);
    code.set_string("lua", 2);

    let pc = some_node.get("pc");
    pc.get("processor").set_string("intel", 0);
    pc.get("ram").set_integer(32, 0);

    datafile
        .write("./datafile.txt")
        .expect("Failed to write datafile");

    let mut datafile = Datafile::new(Some(','), Some(" "));
    datafile
        .read("./datafile.txt")
        .expect("Failed to read datafile");

    println!("{:?}", datafile.get("some_node"));
}
