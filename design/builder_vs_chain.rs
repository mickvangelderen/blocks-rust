struct Window {
    title: String,
    width: usize,
    height: usize,
}

struct Builder {
    title: Option<String>,
    width: Option<usize>,
    height: Option<usize>,
}

impl Builder {
    #[inline]
    fn new() -> Self {
        Builder { title: None, width: None, height: None }
    }

    #[inline]
    fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    #[inline]
    fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }
}

impl From<Builder> for Window {
    #[inline]
    fn from(builder: Builder) -> Self {
        Window {
            title: builder.title.unwrap_or_default(),
            width: builder.width.unwrap_or_default(),
            height: builder.height.unwrap_or_default(),
        }
    }
}

trait Chain<T> {
    fn drain<F: FnMut(T)>(self, f: F);
}

struct Link<T, C: Chain<T>> {
    value: T,
    next: C,
}

impl<T, C: Chain<T>> Chain<T> for Link<T, C> {
    #[inline]
    fn drain<F: FnMut(T)>(self, mut f: F) {
        let Link { value, next } = self;
        f(value);
        next.drain(f);
    }
}

struct End;

impl<T> Chain<T> for End {
    #[inline]
    fn drain<F: FnMut(T)>(self, _: F) {}
}

enum Attribute {
    Title(String),
    Width(usize),
    #[allow(unused)]
    Height(usize),
}

impl<C: Chain<Attribute>> From<C> for Window {
    fn from(attributes: C) -> Self {
        let mut window = Window {
            title: String::new(),
            width: 0,
            height: 0,
        };

        attributes.drain(|attribute| {
            match attribute {
                Attribute::Title(title) => window.title = title,
                Attribute::Width(width) => window.width = width,
                Attribute::Height(height) => window.height = height,
            }
        });

        window
    }
}

#[inline(never)]
fn build_window_from_builder() {
    let mut builder = Builder::new();

    builder
        .title(String::from("Hello, World!"))
        .width(1024);

    // ptr, len, cap.
    assert_eq!(std::mem::size_of::<String>(), 24);

    // Some/None memory optimized by using String's internal pointer instead of an extra tag?
    assert_eq!(std::mem::size_of::<Option<String>>(), 24);

    assert_eq!(std::mem::size_of::<Option<usize>>(), 16);
    assert_eq!(std::mem::size_of::<Builder>(), 56);

    let window = Window::from(builder);

    assert_eq!(window.title, String::from("Hello, World!"));
    assert_eq!(window.width, 1024);
    assert_eq!(window.height, 0);
}

#[inline(never)]
fn build_window_from_chain() {
    let attributes = Link {
        value: Attribute::Title(String::from("Hello, World!")),
        next: Link {
            value: Attribute::Width(1024),
            next: End,
        }
    };

    assert_eq!(std::mem::size_of::<Attribute>(), 32);
    assert_eq!(std::mem::size_of_val(&attributes), std::mem::size_of::<[Attribute; 2]>());

    let window = Window::from(attributes);

    assert_eq!(window.title, String::from("Hello, World!"));
    assert_eq!(window.width, 1024);
    assert_eq!(window.height, 0);
}

fn main() {
    let _ = build_window_from_builder();
    let _ = build_window_from_chain();
    println!("Success!");
}
