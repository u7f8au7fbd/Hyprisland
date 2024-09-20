use eframe::egui::{self, Color32, Rect, Stroke, Area};
mod shader;

#[derive(Default)]
struct MyApp {
    regions: Vec<shader::BoxRegion>,
    gradient_cache: shader::GradientCache, // グラデーションキャッシュ
    margin: f32, // 余白サイズ
    texts: Vec<String>, // 各ボックスに対応するテキスト
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            regions: Vec::new(),
            gradient_cache: shader::GradientCache::new(20), // 段階数は20
            margin: 10.0, // 四方の余白を10pxに設定
            texts: vec![], // テキストの初期化
        };

        // 初期のボックスにデフォルトのテキストを追加
        app.regions.push(shader::BoxRegion {
            relative_rect: Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            is_selected: true,
        });
        app.texts.push("Initial Box".to_string());

        app
    }

    fn split_selected_region(&mut self, screen_rect: Rect) {
        if let Some(selected_index) = self.regions.iter().position(|r| r.is_selected) {
            let selected_region = self.regions[selected_index].clone();
            // ウィンドウサイズも考慮して横長か縦長かを判断
            let is_horizontal_split = (selected_region.relative_rect.width() * screen_rect.width())
                > (selected_region.relative_rect.height() * screen_rect.height());

            let (first_rect, second_rect) = if is_horizontal_split {
                // 左右に分割
                let mid_x = selected_region.relative_rect.center().x;
                let left_rect = Rect::from_min_max(
                    selected_region.relative_rect.min,
                    egui::pos2(mid_x, selected_region.relative_rect.max.y),
                );
                let right_rect = Rect::from_min_max(
                    egui::pos2(mid_x, selected_region.relative_rect.min.y),
                    selected_region.relative_rect.max,
                );
                (left_rect, right_rect)
            } else {
                // 上下に分割
                let mid_y = selected_region.relative_rect.center().y;
                let top_rect = Rect::from_min_max(
                    selected_region.relative_rect.min,
                    egui::pos2(selected_region.relative_rect.max.x, mid_y),
                );
                let bottom_rect = Rect::from_min_max(
                    egui::pos2(selected_region.relative_rect.min.x, mid_y),
                    selected_region.relative_rect.max,
                );
                (top_rect, bottom_rect)
            };

            // 元のボックスを分割された2つのボックスに置き換える
            self.regions[selected_index] = shader::BoxRegion {
                relative_rect: first_rect,
                is_selected: true,
            };
            self.texts[selected_index] = format!("Box {}", selected_index + 1); // テキストの更新
            self.regions.push(shader::BoxRegion {
                relative_rect: second_rect,
                is_selected: false,
            });
            self.texts.push(format!("Box {}", self.regions.len())); // 新しいボックスにもテキストを追加
        }
    }

    // ウィンドウサイズに合わせてボックスのレイアウトを再計算
    fn calculate_absolute_rect(&self, relative_rect: Rect, screen_rect: Rect) -> Rect {
        Rect::from_min_max(
            egui::pos2(
                screen_rect.min.x + relative_rect.min.x * screen_rect.width(),
                screen_rect.min.y + relative_rect.min.y * screen_rect.height(),
            ),
            egui::pos2(
                screen_rect.min.x + relative_rect.max.x * screen_rect.width(),
                screen_rect.min.y + relative_rect.max.y * screen_rect.height(),
            ),
        )
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 初期状態のボックスがない場合は画面全体を1つのボックスとして追加
        if self.regions.is_empty() {
            self.regions.push(shader::BoxRegion {
                relative_rect: Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                is_selected: true,
            });
            self.texts.push("Initial Box".to_string()); // 初期テキスト
        }

        // 現在のウィンドウサイズを取得
        let screen_rect = ctx.available_rect();

        // 分割キー "Q" のチェック
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Q) {
                self.split_selected_region(screen_rect);
            }
        });

        // ボックスの絶対座標を先に計算しておく
        let absolute_rects: Vec<(Rect, bool, String)> = self
            .regions
            .iter()
            .enumerate()
            .map(|(index, region)| {
                let absolute_rect = self.calculate_absolute_rect(region.relative_rect, screen_rect).shrink(self.margin);
                (absolute_rect, region.is_selected, self.texts[index].clone()) // テキストも含める
            })
            .collect();

        // クリック、ドラッグで選択状態を更新
        let pointer_pos = ctx.pointer_hover_pos();
        let mut selected_region_index = None; // クリックされたボックスを追跡

        // クリックまたはドラッグ中の選択処理
        if ctx.input(|i| i.pointer.any_pressed()) {
            if let Some(pointer_pos) = pointer_pos {
                for (index, (absolute_rect, _, _)) in absolute_rects.iter().enumerate() {
                    // ポインタが矩形内にある場合、そのボックスを選択
                    if absolute_rect.contains(pointer_pos) {
                        selected_region_index = Some(index);
                    }
                }
            }
        }

        // ボックスの選択状態を更新
        if let Some(index) = selected_region_index {
            for region in &mut self.regions {
                region.is_selected = false;
            }
            self.regions[index].is_selected = true;
        }

        // 各ボックスを描画
        egui::CentralPanel::default()
        .frame(egui::Frame::none()) // フレームなし
        .show(ctx, |ui| {
            let painter = ui.painter();

            // 透過された半透明の背景を描画
            painter.rect_filled(
                ui.max_rect(),
                0.0,
                Color32::from_rgba_premultiplied(0, 0, 0, 64), // 背景に透過を追加
            );

            for (rect, selected, text) in absolute_rects {
                // 選択された場合のグラデーション
                let selected_top_color = Color32::from_rgb(255, 0, 255);  // 上部の色
                let selected_bottom_color = Color32::from_rgb(0, 255, 255);  // 下部の色

                // 選択されていない場合のグラデーション
                let unselected_top_color = Color32::from_rgb(64, 0, 64);  // 上部の色
                let unselected_bottom_color = Color32::from_rgb(0, 64, 64);  // 下部の色

                // 選択状態によるグラデーションの色選択
                let (top_color, bottom_color) = if selected {
                    (selected_top_color, selected_bottom_color)
                } else {
                    (unselected_top_color, unselected_bottom_color)
                };

                // グラデーションをキャッシュから取得
                let gradient = self.gradient_cache.get_or_create_gradient(top_color, bottom_color);

                let step_height = rect.height() / gradient.len() as f32; // 各ステップの高さを計算
                for (i, &current_color) in gradient.iter().enumerate() {
                    let y1 = rect.top() + i as f32 * step_height;
                    let y2 = y1 + step_height;

                    // 左右のグラデーション描画
                    painter.line_segment(
                        [egui::pos2(rect.left(), y1), egui::pos2(rect.left(), y2)],
                        Stroke::new(3.0, current_color),
                    );
                    painter.line_segment(
                        [egui::pos2(rect.right(), y1), egui::pos2(rect.right(), y2)],
                        Stroke::new(3.0, current_color),
                    );
                }

                // 上部と下部の直線描画
                painter.line_segment(
                    [rect.left_top(), rect.right_top()],
                    Stroke::new(3.0, top_color),
                );
                painter.line_segment(
                    [rect.left_bottom(), rect.right_bottom()],
                    Stroke::new(3.0, bottom_color),
                );

                // Areaにテキストを配置
                Area::new(format!("area_{}", text).into())
                    .fixed_pos(rect.min) // ボックスの位置に固定
                    .show(ctx, |ui| {
                        ui.label(text); // テキストを描画
                    });
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_shown(false)
            .with_titlebar_buttons_shown(false)
            .with_inner_size([1280.0, 1280.0])
            .with_resizable(true)
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native("HyprIsland", options, Box::new(|_cc| Ok(Box::new(MyApp::new()))))
}
