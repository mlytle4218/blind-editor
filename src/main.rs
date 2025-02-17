extern crate ffmpeg_next as ffmpeg;

use std::env;


use ffmpeg::media::Type;


fn main() {
    ffmpeg::init().unwrap();
    match ffmpeg::format::input(&env::args().nth(1).expect("missing input file name")) {
        Ok(input_file) => {
            print!("something");
            let input = input_file
            .streams().best(Type::Audio).unwrap();
        let audio_stream_index = input.index();
        print!("\n\n");
        print!("{audio_stream_index}");

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
        let mut decoder = context_decoder.decoder().audio();

            
            // match ffmpeg::decoder::find_by_name(input_file) {
            //     Some(codec) => {
            //         print!("things");
            //     },
            //     None => {
            //         print!("Nada");
            //     }
                
            // }
        },
        Error => {  
            print!("Nada");
        }
    }

    // for arg in env::args().skip(1) {
    //     // print!("{arg}\n");
    //     // let bob = ffmpeg::decoder::find_by_name(&arg);
    //     match  ffmpeg::decoder::find_by_name(&arg) {
    //         Some(codec) => {
    //             print!("things");
    //         },
    //         None => {
    //             print!("Nada");
    //         }
    //     } 
        
    //     // if let Some(codec) = ffmpeg::decoder::find_by_name(&arg) {
    //     //     println!("type: decoder");
    //     //     println!("\t id: {:?}", codec.id());
    //     //     println!("\t name: {}", codec.name());
    //     //     println!("\t description: {}", codec.description());
    //     //     println!("\t medium: {:?}", codec.medium());
    //     //     println!("\t capabilities: {:?}", codec.capabilities());

    //     //     if let Some(profiles) = codec.profiles() {
    //     //         println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
    //     //     } else {
    //     //         println!("\t profiles: none");
    //     //     }
    //     // }
    // }
}
