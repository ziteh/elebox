use jammdb::{Data, Error, DB};
mod elebox;

fn main() -> Result<(), Error> {
    elebox::init();

    let t1 = elebox::PartType {
        name: "A".to_string(),
        parent_uuid: "a".to_string(),
    };
    elebox::add_part_type(t1);

    let t1 = elebox::PartType {
        name: "B".to_string(),
        parent_uuid: "b".to_string(),
    };
    elebox::add_part_type(t1);

    let tps = elebox::read_part_types();
    for t in tps{
        print!("{}", t.name);
    }

    Ok(())
}
