use std::{error::Error, fs::File, path::Path};

#[derive(Debug)]
struct DataStruct {
    x: f64,
    y: f64,
    class: i32
}

fn read_data(input: &Path) -> Result<Vec<DataStruct>, Box<dyn Error>> {
    let mut file = File::open(input)?;

    let mut reader = csv::Reader::from_reader(file);

    let mut temp: Vec<DataStruct> = Vec::<DataStruct>::new();
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
        
        let mut fields = record.iter();
        let x = fields.next().ok_or("Missing x")?.trim().parse()?;
        let y = fields.next().ok_or("Missing y")?.trim().parse()?;
        let class = fields.next().ok_or("Missing class")?.trim().parse()?;

        let data_record = DataStruct {x, y, class};
        temp.push(data_record);
    }
    Ok(temp)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start");
    
    let file_path = Path::new("data/iris.csv");
    let iris_data = read_data(file_path)?;

    for data in iris_data.iter() {
        println!("DataStruct: {}, {}, {}", data.x, data.y, data.class);
    }

    println!("Stop");

    Ok(())
}