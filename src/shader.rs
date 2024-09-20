use eframe::egui::{Color32, Rect};
use std::collections::HashMap;

#[derive(Clone)]
pub struct BoxRegion {
    pub relative_rect: Rect,  // ウィンドウに対して相対的な座標
    pub is_selected: bool,
}

#[derive(Default)]
pub struct GradientCache {
    cache: HashMap<(Color32, Color32), Vec<Color32>>,
    num_steps: usize,
}

impl GradientCache {
    pub fn new(num_steps: usize) -> Self {
        Self {
            cache: HashMap::new(),
            num_steps,
        }
    }

    // グラデーションのキャッシュを取得、または作成
    pub fn get_or_create_gradient(&mut self, top_color: Color32, bottom_color: Color32) -> Vec<Color32> {
        if let Some(gradient) = self.cache.get(&(top_color, bottom_color)) {
            return gradient.clone();
        }

        // キャッシュにない場合、グラデーションを生成してキャッシュに保存
        let mut gradient = Vec::with_capacity(self.num_steps);
        for i in 0..self.num_steps {
            let t = i as f32 / (self.num_steps as f32 - 1.0);
            let r = (1.0 - t) * top_color.r() as f32 + t * bottom_color.r() as f32;
            let g = (1.0 - t) * top_color.g() as f32 + t * bottom_color.g() as f32;
            let b = (1.0 - t) * top_color.b() as f32 + t * bottom_color.b() as f32;
            gradient.push(Color32::from_rgb(r as u8, g as u8, b as u8));
        }

        self.cache.insert((top_color, bottom_color), gradient.clone());
        gradient
    }
}