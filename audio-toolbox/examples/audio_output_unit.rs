extern crate audio_toolbox;

use audio_toolbox::AudioOutputUnit;

fn format_ostype(fcc: audio_toolbox::OSType) -> String  {
    let a: char = (fcc >> 24) as u8 as char;
    let b: char = (fcc >> 16) as u8 as char;
    let c: char = (fcc >> 8) as u8 as char;
    let d: char = fcc as u8 as char;

    format!("'{}{}{}{}'", a, b, c, d)
}

fn main()
{
    let unit = AudioOutputUnit::create(|c| {
        let desc = c.description();
        if let Ok(desc) = desc {
            println!("type = {}, sub_type = {}, manufacturer = {}, flags = {:x}",
                     format_ostype(desc.kind()),
                     format_ostype(desc.sub_kind()),
                     format_ostype(desc.manufacturer()),
                     desc.flags());
        }
        false
    });
    assert!(unit.is_ok());
}
