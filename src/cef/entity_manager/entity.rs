use super::{TEXTURE_HEIGHT, TEXTURE_WIDTH};
use classicube_sys::{
    cc_bool, Bitmap, Entity, EntityVTABLE, Entity_Init, Entity_SetModel, Gfx_UpdateTexturePart,
    LocationUpdate, Model_Render, OwnedGfxTexture, OwnedString, PackedCol, PACKEDCOL_WHITE,
};
use std::{mem, pin::Pin};

pub struct CefEntity {
    // We don't need to Pin entity because all the ffi operations
    // are temporary, they never store our pointer
    pub entity: Entity,

    v_table: Pin<Box<EntityVTABLE>>,
    texture: OwnedGfxTexture,
}

impl CefEntity {
    pub fn register() -> Self {
        let entity = unsafe { mem::zeroed() };

        let v_table = Box::pin(EntityVTABLE {
            Tick: Some(Self::tick),
            Despawn: Some(Self::despawn),
            SetLocation: Some(Self::set_location),
            GetCol: Some(Self::get_col),
            RenderModel: Some(Self::render_model),
            RenderName: Some(Self::render_name),
        });

        let mut pixels: Vec<u8> = vec![255; 4 * TEXTURE_WIDTH * TEXTURE_HEIGHT];

        let mut bmp = Bitmap {
            Scan0: pixels.as_mut_ptr(),
            Width: TEXTURE_WIDTH as i32,
            Height: TEXTURE_HEIGHT as i32,
        };

        let texture = OwnedGfxTexture::create(&mut bmp, true, false);

        let mut this = Self {
            entity,
            v_table,
            texture,
        };

        unsafe {
            this.register_entity();
        }

        this
    }

    unsafe extern "C" fn tick(_entity: *mut Entity, _delta: f64) {
        // println!("Tick");
    }

    unsafe extern "C" fn despawn(_entity: *mut Entity) {
        // println!("Despawn");
    }

    unsafe extern "C" fn set_location(
        _entity: *mut Entity,
        _update: *mut LocationUpdate,
        _interpolate: cc_bool,
    ) {
        // println!("SetLocation");
    }

    unsafe extern "C" fn get_col(_entity: *mut Entity) -> PackedCol {
        // println!("GetCol");

        PACKEDCOL_WHITE
    }

    unsafe extern "C" fn render_model(entity: *mut Entity, _delta_time: f64, _t: f32) {
        let entity = &mut *entity;

        // println!("RenderModel");

        Model_Render(entity.Model, entity);
    }

    unsafe extern "C" fn render_name(_entity: *mut Entity) {
        // println!("RenderName");
    }

    unsafe fn register_entity(&mut self) {
        let CefEntity {
            entity,
            v_table,
            texture,
            ..
        } = self;

        Entity_Init(entity);

        let model_name = OwnedString::new("cef");
        Entity_SetModel(entity, model_name.as_cc_string());

        entity.VTABLE = v_table.as_mut().get_unchecked_mut();
        entity.Velocity.set(0.0, 0.0, 0.0);
        entity.RotX = 180.0;
        entity.TextureId = texture.resource_id;

        entity.Position.set(0.0, 0.0, 0.0);
    }

    pub fn update_texture(&mut self, mut part: Bitmap) {
        let CefEntity { texture, .. } = self;

        unsafe {
            Gfx_UpdateTexturePart(texture.resource_id, 0, 0, &mut part, 0);
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        let CefEntity { entity, .. } = self;

        entity.ModelScale.set(scale, scale, 1.0);
    }
}