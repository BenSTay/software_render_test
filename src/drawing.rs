use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

pub struct Buffer {
    data: Vec<Vec<u8>>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let row = vec![0u8; width * 3];
        let data = vec![row; height];
        Buffer { data }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &[u8; 3]) {
        let index = x * 3;
        self.data[y][index..index+3].clone_from_slice(color);
    }

    pub fn render(&self, texture: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let row_bytes = self.data[0].len();
        let width = row_bytes as u32 / 3;

        for row in self.data.iter().enumerate() {
            let rect = Rect::new(0, row.0 as i32, width, 1);
            texture
                .update(rect, row.1, row_bytes)
                .expect("Couldn't update texture");
        }
        
        canvas.copy(texture, None, None)?;
        Ok(())
    }
}
