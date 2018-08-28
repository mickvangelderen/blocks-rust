use gl;

#[derive(Debug)]
pub struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Viewport {
    #[inline]
    pub fn new(width: i32, height: i32) -> Self {
        Viewport {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    #[inline]
    pub fn with_position(x: i32, y: i32, width: i32, height: i32) -> Self {
        Viewport {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    pub unsafe fn update(&mut self) -> ViewportUpdate {
        ViewportUpdate(self)
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    pub fn aspect(&self) -> f32 {
        (self.width as f32 / self.height as f32).abs()
    }
}

pub struct ViewportUpdate<'a>(&'a mut Viewport);

impl<'a> ViewportUpdate<'a> {
    #[inline]
    pub fn x(&mut self, x: i32) -> &mut Self {
        self.0.x = x;
        self
    }

    #[inline]
    pub fn y(&mut self, y: i32) -> &mut Self {
        self.0.y = y;
        self
    }

    #[inline]
    pub fn width(&mut self, width: i32) -> &mut Self {
        self.0.width = width;
        self
    }

    #[inline]
    pub fn height(&mut self, height: i32) -> &mut Self {
        self.0.height = height;
        self
    }
}

impl<'a> Drop for ViewportUpdate<'a> {
    fn drop(&mut self) {
        unsafe {
            gl::Viewport(self.0.x, self.0.y, self.0.width, self.0.height);
        }
    }
}
