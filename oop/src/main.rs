use oop::AveragedCollection;
fn main() {
    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    println!("average = {}", collection.average());
}
