#![allow(non_snake_case)]

use std::{thread::sleep, time};
#[link(name = "bass")]
extern{
    fn BASS_Init(device: i32, freq: i32, flags: i32, win: u32, CLSID: u32) -> bool;
    fn BASS_Start() -> bool;
    fn BASS_Stop() -> bool;
    fn BASS_StreamCreateFile(mem : bool, f: *const u8, offset1 : i32, offset2 : i32, length1 : i32, length2: i32, flags: i32) -> i64;
    fn BASS_ChannelPlay(handle: i64, restart: bool);
}
fn main() 
{
    
    unsafe
    {
        BASS_Init(-1, 44100, 0, 0, 0);
        BASS_Start();
        let stream: i64 = BASS_StreamCreateFile(false, "tomp3.cc - Mumei  VirusMV_256kbps.mp3".as_ptr(), 0, 0, 0, 0, 0x4);
        print!("BASS IS STARTED!!!");
        BASS_ChannelPlay(stream, false);
        loop
        {
            let ten_millis = time::Duration::from_millis(4800);
                
            sleep(ten_millis);
        };
    };
}
