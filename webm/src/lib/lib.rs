
extern crate webm_sys as ffi;

pub mod mux {        
    use ffi;
    use std::os::raw::c_void;
    use std::fs::{File, OpenOptions};
    use std::path::Path;

    use std::io::{Write, Seek};    
    
    pub struct WebmWriter
    {
        webm_writer: ffi::mux::WriterMutPtr,
        file: Box<File>,        
        chunk_cb: fn(&str),
        chunk_count: u32,
        base_name: String,
        chunk_name: String,
        file_path: String,
    }

    //unsafe impl<T: Send + Write + Seek > Send for Writer<T> {}

    impl WebmWriter
    {
        pub fn new(file_path: &str, base_name: &str, chunk_cb: fn(&str)) -> Box<WebmWriter> {
            use std::io::SeekFrom;
            use std::slice::from_raw_parts;
            use std::mem::transmute;

            let chunk_count = 0;
            let chunk_name = format!("{}_{}.webm", base_name, chunk_count);
            let mut path = Path::new(file_path);
            let path = path.join(&chunk_name);
            let file = Box::new(OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path).unwrap());

            let mut w = Box::new(WebmWriter {                
                webm_writer: 0 as ffi::mux::WriterMutPtr,
                file,
                chunk_cb,
                chunk_count,
                base_name: base_name.to_string(),
                chunk_name,
                file_path: file_path.to_string(),
            });
            
            extern "C" fn write_fn(dest: *mut c_void,
                                      buf: *const c_void,
                                      len: usize) -> bool                
            {
                let writer: &mut WebmWriter = unsafe { transmute(dest) };

                let buf = unsafe {
                    from_raw_parts(buf as *const u8, len as usize)
                };                
                writer.file.write(buf).is_ok()
            }
            extern "C" fn get_pos_fn(dest: *mut c_void) -> u64                
            {
                let writer: &mut WebmWriter = unsafe { transmute(dest) };
                
                let pos = writer.file.seek(SeekFrom::Current(0))
                    .unwrap();                
                pos
            }
            extern "C" fn set_pos_fn(dest: *mut c_void,
                                        pos: u64) -> bool                
            {
                let writer: &mut WebmWriter = unsafe { transmute(dest) };                
                writer.file.seek(SeekFrom::Start(pos)).is_ok()
            }
            extern "C" fn element_start_notify_fn(dest: *mut c_void,
                element_id: u64, pos: i64) -> ()                
            {
                
                unsafe {                    
                    if (element_id == ffi::mux::ELEMENT_ID_MKVCLUSTE as u64) {
                        let writer: &mut WebmWriter = unsafe { transmute(dest) };                
                        writer.update_chunk();
                    }                     
                }                
            }            

            w.webm_writer = unsafe {
                ffi::mux::new_writer(Some(write_fn),
                                     Some(get_pos_fn),
                                     Some(set_pos_fn),
                                     Some(element_start_notify_fn),
                                     transmute(&mut *w))
            };
            debug_assert!(w.webm_writer != 0 as *mut _);
            w
        }        

        fn update_chunk(&mut self) {
            let mut path = Path::new(&self.file_path);
            let path = path.join(&self.chunk_name);

            (self.chunk_cb)(path.to_str().unwrap());

            self.chunk_count = self.chunk_count + 1;
            self.chunk_name = format!("{}_{}.webm", self.base_name, self.chunk_count);
            self.file = Box::new(OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.chunk_name).unwrap());
        }

        fn webm_writer(&self) -> ffi::mux::WriterMutPtr {
            self.webm_writer
        }
    }

    impl Drop for WebmWriter {
        fn drop(&mut self) {
            unsafe {
                ffi::mux::delete_writer(self.webm_writer);
            }            
        }
    }

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub struct VideoTrack(ffi::mux::SegmentMutPtr,
                          ffi::mux::VideoTrackMutPtr);
    #[derive(Eq, PartialEq, Clone, Copy)]
    pub struct AudioTrack(ffi::mux::SegmentMutPtr,
                          ffi::mux::AudioTrackMutPtr);

    unsafe impl Send for VideoTrack {}
    unsafe impl Send for AudioTrack {}

    pub trait Track {
        fn is_audio(&self) -> bool { false }
        fn is_video(&self) -> bool { false }

        fn add_frame(&mut self, data: &[u8], timestamp_ns: u64, keyframe: bool) -> bool {
            unsafe {
                ffi::mux::segment_add_frame(self.get_segment(),
                                            self.get_track(),
                                            data.as_ptr(),
                                            data.len() as usize,
                                            timestamp_ns, keyframe)
            }
        }

        #[doc(hidden)]
        fn get_segment(&self) -> ffi::mux::SegmentMutPtr;

        #[doc(hidden)]
        fn get_track(&self) -> ffi::mux::TrackMutPtr;
    }
    impl VideoTrack {
        pub fn set_color(&mut self, bit_depth: u8, subsampling: (bool, bool), full_range: bool) -> bool {
            let (sampling_horiz, sampling_vert) = subsampling;
            fn to_int(b: bool) -> i32 { if b {1} else {0}};
            unsafe {
                ffi::mux::mux_set_color(self.get_track(), bit_depth.into(), to_int(sampling_horiz), to_int(sampling_vert), to_int(full_range)) != 0
            }
        }
    }
    impl Track for VideoTrack {
        fn is_video(&self) -> bool { true }

        #[doc(hidden)]
        fn get_segment(&self) -> ffi::mux::SegmentMutPtr { self.0 }
        #[doc(hidden)]
        fn get_track(&self) -> ffi::mux::TrackMutPtr {
            unsafe { ffi::mux::video_track_base_mut(self.1) }
        }
    }
    impl Track for AudioTrack {
        fn is_audio(&self) -> bool { true }

        #[doc(hidden)]
        fn get_segment(&self) -> ffi::mux::SegmentMutPtr { self.0 }
        #[doc(hidden)]
        fn get_track(&self) -> ffi::mux::TrackMutPtr {
            unsafe { ffi::mux::audio_track_base_mut(self.1) }
        }
    }

    #[derive(Eq, PartialEq, Clone, Copy, Debug)]
    pub enum AudioCodecId {
        Opus,
        Vorbis,
    }
    impl AudioCodecId {
        fn get_id(&self) -> u32 {
            match self {
                &AudioCodecId::Opus => ffi::mux::OPUS_CODEC_ID,
                &AudioCodecId::Vorbis => ffi::mux::VORBIS_CODEC_ID,
            }
        }
    }
    #[derive(Eq, PartialEq, Clone, Copy, Debug)]
    pub enum VideoCodecId {
        VP8,
        VP9,
    }
    impl VideoCodecId {
        fn get_id(&self) -> u32 {
            match self {
                &VideoCodecId::VP8 => ffi::mux::VP8_CODEC_ID,
                &VideoCodecId::VP9 => ffi::mux::VP9_CODEC_ID,
            }
        }
    }

    //unsafe impl<W: Send> Send for Segment<W> {}

    pub struct Segment {
        ffi: ffi::mux::SegmentMutPtr,
        _writer: Box<WebmWriter>,
    }

    impl Segment {
        /// Note: the supplied writer must have a lifetime larger than the segment.
        pub fn new(dest: Box<WebmWriter>) -> Option<Self>            
        {
            let ffi = unsafe { ffi::mux::new_segment() };
            let success = unsafe {
                let ret = ffi::mux::initialize_segment(ffi, dest.webm_writer());
                ffi::mux::segment_set_mode(ffi, ffi::mux::SEGMENT_MODE_LIVE);
                ret
            };
            if !success { return None; }

            Some(Segment {
                ffi: ffi,
                _writer: dest,
            })
        }

        pub fn set_app_name(&mut self, name: &str) {
            use std::ffi::CString;
            unsafe {
                ffi::mux::mux_set_writing_app(self.ffi, CString::new(name).unwrap().as_ptr());
            }
        }

        pub fn add_video_track(&mut self, width: u32, height: u32,
                               id: Option<i32>, codec: VideoCodecId) -> VideoTrack
        {
            let vt = unsafe {
                ffi::mux::segment_add_video_track(self.ffi, width as i32, height as i32,
                                                  id.unwrap_or(0), codec.get_id())
            };
            VideoTrack(self.ffi, vt)
        }
        pub fn add_audio_track(&mut self, sample_rate: i32, channels: i32,
                               id: Option<i32>, codec: AudioCodecId) -> AudioTrack {
            let at = unsafe {
                ffi::mux::segment_add_audio_track(self.ffi, sample_rate, channels,
                                                  id.unwrap_or(0), codec.get_id())
            };
            AudioTrack(self.ffi, at)
        }

        /// After calling, all tracks are freed (ie you can't use them).
        pub fn finalize(self, duration: Option<u64>) -> bool {
            let result = unsafe {
                ffi::mux::finalize_segment(self.ffi, duration.unwrap_or(0))
            };
            unsafe {
                ffi::mux::delete_segment(self.ffi);
            }
            result
        }
    }
}
