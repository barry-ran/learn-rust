macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

macro_rules! tracef {
    ($( $x:expr )?) => {        
        $(
            let _tracef_ra = Raii::new($x, function!());
        )?
    };
}

struct Raii {
    target: String,
    tips: String
}

impl Raii {
    fn new(target: &str, tips: &str) -> Raii {
        let tips = tips.to_string();
        let target = target.to_string();
        let ra = Raii {
            target,
            tips
        };
        ra.begin();
        ra
    }

    fn begin(&self) {
        println!("{} {} begin", self.target, self.tips);
    }
}

impl Drop for Raii {
    fn drop(&mut self) {
        println!("{} {} end", self.target, self.tips);
    }
}

fn main() {
    test_fun();    
}

fn test_fun() {
    tracef!("mod_a");
    println!("test_fun");
}
