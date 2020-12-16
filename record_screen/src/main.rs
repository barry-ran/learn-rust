use std::fmt;
use std::fs::{File, OpenOptions};
use std::{io, thread};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use scrap::{Capturer, Display};
use vpx_encode;
use cpal;
use webm::mux;
use webm::mux::Track;

mod convert;
mod sound;

fn main() -> io::Result<()> {        
    // 获取显示器
    let displays = Display::all()?;

    let i = if displays.is_empty() {
        error("No displays found.");
        return Ok(());
    } else if displays.len() == 1 {
        0
    } else {
        // 多个显示器：让用户选择显示器
        let names: Vec<_> = displays
            .iter()
            .enumerate()
            .map(
                |(i, display)| format!("Display {} [{}x{}]", i, display.width(), display.height(),),
            )
            .collect();

        quest::ask("Which display?\n");
        let i = quest::choose(Default::default(), &names)?;
        i
    };

    let display = displays.into_iter().nth(i).unwrap();

    // 获取麦克风
    let mics: Vec<_> = cpal::input_devices().collect();
    let mic = if mics.is_empty() {
        None
    } else {
        let mut names = vec!["None".into()];
        names.extend(mics.iter().map(|m| m.name()));
        println!("{:?}", names);

        quest::ask("Which audio source?\n");
        let i = quest::choose(Default::default(), &names)?;
        println!();

        if i == 0 {
            None
        } else {
            Some(mics.into_iter().nth(i - 1).unwrap())
        }
    };

    // 创建采集器
    let mut capturer = Capturer::new(display)?;
    let width = capturer.width() as u32;
    let height = capturer.height() as u32;

    // 创建编码器
    let vpx_codec = vpx_encode::VideoCodecId::VP9;
    let bv = 20000; // Video bitrate in kilobits per second
    let mut vpx = vpx_encode::Encoder::new(vpx_encode::Config {
        width: width,
        height: height,
        timebase: [1, 1000],
        bitrate: bv,
        codec: vpx_codec,
    }).unwrap();

    // 创建webm封装器
    let mux_codec = mux::VideoCodecId::VP9;

    // 创建webm文件
    let filename = "chunk";

    let chunk_cb = |chunk_file_name: &str|{
        println!("chunk cb: {}", chunk_file_name);
    };

    let mut writer = mux::WebmWriter::new("", filename, chunk_cb);
    
    let mut webm =
        mux::Segment::new(writer).expect("Could not initialize the multiplexer.");
    let mut vt = webm.add_video_track(width, height, None, mux_codec);
            
    // 开始录制
    let start = Instant::now();
    let stop = Arc::new(AtomicBool::new(false));

    // 采集声音
    let ba = 96;
    if let Some(mic) = mic {
        if let Err(e) = sound::run(stop.clone(), mic, &mut webm, ba) {
            error(e);
        }
    } else {
        error("mic invalid");
    }

    thread::spawn({
        let stop = stop.clone();
        move || {
            let _ = quest::ask("Recording! Press ⏎ to stop.");
            let _ = quest::text();
            stop.store(true, Ordering::Release);
        }
    });
    

    // 录制时间
    let duration = Some(Duration::from_secs(180));
    
    let fps = 30;
    let spf = Duration::from_nanos(1_000_000_000 / fps);
    let mut yuv = Vec::new();

    while !stop.load(Ordering::Acquire) {
        let now = Instant::now();
        let time = now - start;

        if Some(true) == duration.map(|d| time > d) {
            break;
        }

        match capturer.frame() {
            Ok(frame) => {                
                let ms = time.as_secs() * 1000 + time.subsec_millis() as u64;

                convert::argb_to_i420(width as usize, height as usize, &frame, &mut yuv);

                for frame in vpx.encode(ms as i64, &yuv).unwrap() {                    
                    vt.add_frame(frame.data, frame.pts as u64 * 1_000_000, frame.key);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {                
                // Wait.
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }

        let dt = now.elapsed();
        if dt < spf {
            thread::sleep(spf - dt);
        }
    }

    // 结束

    let mut frames = vpx.finish().unwrap();
    while let Some(frame) = frames.next().unwrap() {
        vt.add_frame(frame.data, frame.pts as u64 * 1_000_000, frame.key);
    }

    let _ = webm.finalize(None);

    Ok(())
}

// 红色打印错误信息
fn error<S: fmt::Display>(s: S) {
    println!("\u{1B}[1;31m{}\u{1B}[0m", s);
}
