use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub mod background;
pub mod camera;
pub mod geometry;
pub mod shader;
pub mod shader_background;
pub mod shader_stl;
pub mod stl;
pub mod texture;
pub mod textures;

use background::Background;
use camera::Camera;
use shader_background::ShaderBackground;
use shader_stl::ShaderStl;
use stl::Stl;
use textures::StaticTextures;

pub struct Renderer {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    stls: Vec<Stl>,
    background: Background,
    shader_stl: ShaderStl,
    shader_background: ShaderBackground,
    camera: Camera,

    pub resolution: (u32, u32),

    dirty: bool,
    last_render_time: f32,
}

#[derive(Debug)]
pub enum RendererError {
    NoGl,
    NoContext,
    GeometryError(geometry::GeometryError),
    ShaderError(shader::ShaderError),
    TextureError(texture::TextureError),
}

impl From<geometry::GeometryError> for RendererError {
    fn from(err: geometry::GeometryError) -> Self {
        Self::GeometryError(err)
    }
}
impl From<texture::TextureError> for RendererError {
    fn from(err: texture::TextureError) -> Self {
        Self::TextureError(err)
    }
}
impl From<shader::ShaderError> for RendererError {
    fn from(err: shader::ShaderError) -> Self {
        Self::ShaderError(err)
    }
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, RendererError> {
        let gl = get_gl_context(&canvas).map_err(|_| RendererError::NoContext)?;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        if gl.is_null() {
            return Err(RendererError::NoGl);
        }

        let background = Background::new(&gl)?;
        let mut shader_stl = ShaderStl::new(&gl)?;
        let mut shader_background = ShaderBackground::new(&gl)?;
        let textures = StaticTextures::new(&gl)?;

        shader_stl.image_matcap = Some(textures.stl_matcap.clone());
        shader_background.image_matcap = Some(textures.stl_matcap.clone());

        let camera = Camera::new();

        Ok(Self {
            canvas,
            gl,
            stls: Vec::new(),
            background,
            shader_stl,
            shader_background,
            camera,
            resolution: (100, 100),
            dirty: true,
            last_render_time: 0.0,
        })
    }

    pub fn add_stl(&mut self, stl_data: &[u8]) -> Result<(), RendererError> {
        self.stls.push(Stl::new(&self.gl, &stl_data)?);
        Ok(())
    }

    /// Checks if the canvas has been resized. If it has been, update
    /// the openGL context size, update the camera and mark as needing
    /// re-rendering
    fn check_resize(&mut self) {
        let client_width = self.canvas.client_width();
        let client_height = self.canvas.client_height();
        let canvas_width = self.canvas.width() as i32;
        let canvas_height = self.canvas.height() as i32;

        if client_width != canvas_width || client_height != canvas_height {
            self.gl.viewport(0, 0, client_width, client_height);
            let client_width = client_width as u32;
            let client_height = client_height as u32;

            self.canvas.set_width(client_width);
            self.canvas.set_height(client_height);
            self.camera.aspect = (client_width as f32) / (client_height as f32);
            self.resolution = (client_width, client_height);

            self.dirty = true;

            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }

    /// Update the renderer. Note that it doesn't always render - only
    /// when the dirty flag is true will it actually render.
    pub fn update(&mut self) {
        self.check_resize();
        let now = window().unwrap().performance().unwrap().now();
        let time = (now / 1000.0) as f32;

        let time_since_render = time - self.last_render_time;
        if time_since_render > 0.2 {
            self.dirty = true;
        }

        if self.dirty {
            self.render();
            self.dirty = false;
            self.last_render_time = time;
        }
    }

    /// Perform a render immediately.
    fn render(&mut self) {
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let (world_to_camera, camera_to_screen) = self.camera.to_matrices();

        {
            // Render the background
            self.shader_background
                .setup(&self.gl, world_to_camera, camera_to_screen);
            self.background.render(&self.gl, &self.shader_background);
        }

        {
            // Render the models
            self.shader_stl
                .setup(&self.gl, world_to_camera, camera_to_screen);
            for stl in self.stls.iter() {
                stl.render(&self.gl, &self.shader_stl);
            }
        }
    }

    /// Orbit the view
    pub fn drag_view(&mut self, delta: (f32, f32)) {
        self.camera.azimuth += delta.0;
        self.camera.elevation += delta.1;
        self.camera.elevation = f32::min(f32::max(self.camera.elevation, -1.4), 1.4);
        self.dirty = true;
    }
}

fn get_gl_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, JsValue> {
    Ok(canvas.get_context("webgl2")?.unwrap().dyn_into()?)
}
