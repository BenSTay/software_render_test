use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Buffer {
    pub rows: Vec<Row>,
    height: usize,
    width: usize
}

#[derive(Clone)]
pub struct Row {
    data: Vec<u8>,
    bytes: usize
}

impl Buffer {
    pub fn new (width: usize, height: usize) -> Self {
        let row = Row::new(width);
        let rows = vec![row; height];
        Buffer { rows, height, width }
    }

    pub fn render (&self, texture: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for n in 0..self.height {
            let rect = Rect::new(0, n as i32, self.width as u32, 1);
            texture.update(rect, &self.rows[n].data, self.rows[n].bytes)
                .expect("Couldn't update texture");
        }
        canvas.copy(texture, None, None)?;
        Ok(())
    }
}

impl Row {
    fn new (width: usize) -> Self {
        let bytes = width * 3;
        let data = vec![0u8; bytes];
        Row { data, bytes }
    }

    pub fn set_pixel(&mut self, x: usize, color: &[u8; 3]) {
        let index = x * 3;
        self.data[index..index+3].clone_from_slice(color);
    }
}
