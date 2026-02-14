#[derive(Clone)]
pub struct ModelTexture {
    pub id: u32, // Unique ID for hashing and comparison
    pub diffuse_bind_group: wgpu::BindGroup,
    pub shine_damper: f32,
    pub relfectivity: f32,
    pub number_of_rows: u32,
}

impl ModelTexture {
    pub fn new(id: u32, bind_group: wgpu::BindGroup) -> Self {
        Self {
            id,
            diffuse_bind_group: bind_group,
            shine_damper: 1.0,
            relfectivity: 0.0,
            number_of_rows: 1,
        }
    }

    pub fn get_texture_id(&self) -> u32 {
        self.id
    }

    pub fn get_shine(&self) -> f32 {
        self.shine_damper.clone()
    }

    pub fn get_reflectivity(&self) -> f32 {
        self.relfectivity.clone()
    }

    pub fn get_number_of_rows(&self) -> u32 {
        self.number_of_rows.clone()
    }

    pub fn clone(&self) -> ModelTexture {
        let mut model_texture = ModelTexture::new(self.id, self.diffuse_bind_group.clone());
        model_texture.shine_damper = self.get_shine();
        model_texture.relfectivity = self.get_reflectivity();
        model_texture.number_of_rows = self.get_number_of_rows();

        model_texture
    }
    /*
        pub fn cleanup(&self, gl: &GlFns) {
    // to do
        }
        */
}
