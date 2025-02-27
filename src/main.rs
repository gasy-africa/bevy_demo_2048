#![windows_subsystem = "windows"]

mod config;
use config::*;

mod gameRule;
use gameRule::*;

use rand::Rng;

use bevy::asset::{Asset, HandleId};
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::text::Text2dBounds;
use bevy::window::PresentMode;
use bevy::input::touch::{TouchPhase, TouchInput};

fn main() {
	// create a "constructor" closure, which can initialize
    // our data and move it into a closure that bevy can run as a system
    let mut config = TouchEvent {
		ev: TouchInput { 
				phase: TouchPhase::Started
			, position: Vec2 { x: 0.0, y: 0.0 }
			, force: None, id: 0 
		}
	};

	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Bevy 2048".to_string(),
				position: WindowPosition::Centered,
				width: WINDOW_WIDTH,
				height: WINDOW_HEIGHT,
				present_mode: PresentMode::AutoNoVsync,
				resizable: false,
				..default()
			},
			..default()
		}))
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_state(VICTORY_or_DEFEAT::NONE)
		// .add_startup_system(setup)
		.add_system_set(SystemSet::on_enter(VICTORY_or_DEFEAT::NONE).with_system(setup))
		.add_system_set(
			SystemSet::on_update(VICTORY_or_DEFEAT::NONE)
				.label("input")
				.with_system(keyboard_input)
				.with_system(move |
					events: EventReader<TouchInput>, 
					asset_server: Res<AssetServer>,
					cell_value_save: ResMut<CELL_VALUE_SAVE>,
					text_query: Query< &mut Text, With<CellValue> >,
					score_query: Query< &mut Text, Without<CellValue> >,
					materials: ResMut< Assets<ColorMaterial> >,
					app_state: ResMut< State<VICTORY_or_DEFEAT> >,
					| {
					// call our function from inside the closure
					handle_touches(
						events,
						asset_server,
						cell_value_save,
						text_query,
						score_query,
						materials,
						app_state,
						&mut config,
					);
				})
		)
		.add_system_set(SystemSet::on_enter(VICTORY_or_DEFEAT::DEFEAT).with_system(DefeatFunction))
		.add_system_set(SystemSet::on_enter(VICTORY_or_DEFEAT::VICTORY).with_system(VictoryFunction))
		// .add_system(keyboard_input)
		.run();
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
	match cell_value {
		2 => COLOR_CELL_2.clone(),
		4 => COLOR_CELL_4.clone(),
		8 => COLOR_CELL_8.clone(),
		16 => COLOR_CELL_16.clone(),
		32 => COLOR_CELL_32.clone(),
		64 => COLOR_CELL_64.clone(),
		128 => COLOR_CELL_128.clone(),
		256 => COLOR_CELL_256.clone(),
		512 => COLOR_CELL_512.clone(),
		1024 => COLOR_CELL_1024.clone(),
		2048 => COLOR_CELL_2048.clone(),
		_ => COLOR_CELL_NULL.clone()
	}
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>
) {

	// 初始化存储数组
	let mut cell_value_save_temp: Vec<Vec<u32>> = Init_cell_value_save();
	let mut cell_background_save: Vec<HandleId> = Vec::new();
	// 计算左上方格偏移
	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let mut y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;


	commands.spawn(Camera2dBundle::default());

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into()).into(),
		material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
		transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
		..default()
	});

	// 初始化文字信息
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: side_length / 2.0,
		color: COLOR_BROWN,
	};
	let box_size = Vec2::new(side_length, side_length);

	for i in 0..CELL_SIDE_NUM {
		for j in 0..CELL_SIDE_NUM {

			// 格中显示内容
			let mut text = "";
			if cell_value_save_temp[i as usize][j as usize] == 2 {
				text = "2";
			}

			let materialColor = materials.add(ColorMaterial::from(cell_color(cell_value_save_temp[i as usize][j as usize])));
			cell_background_save.push(materialColor.id());
			// 绑定格，根据数字来确定格的颜色
			commands.spawn(
				MaterialMesh2dBundle {
					mesh: meshes.add(shape::Box::new(side_length, side_length, 0.0).into()).into(),
					material: materialColor,
					transform: Transform::from_xyz(
						x_offset + (j as f32) * (side_length + CELL_SPACE),
						y_offset - (i as f32) * (side_length + CELL_SPACE),
						0.0),
					..default()
				}
			);

			// 绑定数字
			commands.spawn((
				Text2dBundle {
					text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
					text_2d_bounds: Text2dBounds {
						// Wrap text in the rectangle
						size: box_size,
					},
					transform: Transform::from_xyz(
						x_offset + (j as f32) * (side_length + CELL_SPACE),
						y_offset - (i as f32) * (side_length + CELL_SPACE),
						1.0),
					..default()
				},
				CellValue
			));
		}
	}

	// 将存储数组设为资源
	commands.insert_resource(
		CELL_VALUE_SAVE{
			valueSave: cell_value_save_temp.clone(),
			cellBackGround: cell_background_save,
			score: 0
		}
	);

	commands.spawn(
		Text2dBundle {
			text: Text::from_sections(
				[
					TextSection::new("SCORE\n", text_style.clone()),
					TextSection::new("0", text_style.clone()),
				]

			),
			text_2d_bounds: Text2dBounds {
				// Wrap text in the rectangle
				size: box_size,
			},
			transform: Transform::from_xyz(
				-WINDOW_WIDTH / 2.0,
				WINDOW_HEIGHT / 2.0,
				0.0,
			),
			..default()
		}
	);
}

fn keyboard_input(
	keyboard_input: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	mut cell_Value_Save: ResMut<CELL_VALUE_SAVE>,
	mut text_query: Query<(&mut Text), (With<CellValue>)>,
	mut score_query: Query<(&mut Text), (Without<CellValue>)>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut app_state: ResMut<State<VICTORY_or_DEFEAT>>,
) {
	let mut moved = MOVE_DIRECTION::NONE;
	if keyboard_input.just_pressed(KeyCode::Up) {
		moved = MOVE_DIRECTION::UP;
	}
	if keyboard_input.just_pressed(KeyCode::Down) {
		moved = MOVE_DIRECTION::DOWN;
	}
	if keyboard_input.just_pressed(KeyCode::Right) {
		moved = MOVE_DIRECTION::RIGHT;
	}
	if keyboard_input.just_pressed(KeyCode::Left) {
		moved = MOVE_DIRECTION::LEFT;
	}
	motion(moved, asset_server, cell_Value_Save, text_query, score_query, materials, app_state);
}

fn motion(moved: MOVE_DIRECTION,
	mut asset_server: Res<AssetServer>,
	mut cell_value_save: ResMut<CELL_VALUE_SAVE>,
	mut text_query: Query< &mut Text, With<CellValue> >,
	mut score_query: Query< &mut Text, Without<CellValue> >,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut app_state: ResMut<State<VICTORY_or_DEFEAT>>
) {
	match moved {
		MOVE_DIRECTION::NONE => return,
		_ => {
		let mut i = 0;
		Move_Value(moved, &mut cell_value_save);
		score_query.single_mut().sections[1].value = cell_value_save.score.to_string();
		let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
		let font = asset_server.load("fonts/FiraSans-Bold.ttf");
		let mut text_style = TextStyle {
			font,
			font_size: side_length / 2.0,
			color: COLOR_BROWN,
		};
		for mut text in text_query.iter_mut() {
			let cell_value_temp = cell_value_save.valueSave[i / 4][i % 4];
			if cell_value_temp > 4 {
				text_style.color = COLOR_WHITE;
			} else {
				text_style.color = COLOR_BROWN;
			}
			if cell_value_temp != 0 {
				text.sections[0].style = text_style.clone();
				text.sections[0].value = cell_value_save.valueSave[i / 4][i % 4].to_string();
			} else {
				text.sections[0].value = "".to_string();
			}
			materials.set_untracked(cell_value_save.cellBackGround[i], ColorMaterial::from(cell_color(cell_value_save.valueSave[i / 4][i % 4])));
			i += 1;
		}
		let result = check_result(&mut cell_value_save);
		match result {
			VICTORY_or_DEFEAT::VICTORY => {
				println!("victory");
				app_state.overwrite_set(VICTORY_or_DEFEAT::VICTORY);
			},
			VICTORY_or_DEFEAT::DEFEAT => {
				println!("defeat");
				app_state.overwrite_set(VICTORY_or_DEFEAT::DEFEAT);
			},
			VICTORY_or_DEFEAT::NONE => println!("none")
		}
	}
	}
}

fn DefeatFunction(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut cell_Value_Save: ResMut<CELL_VALUE_SAVE>,
	mut app_state: ResMut<State<VICTORY_or_DEFEAT>>,
	entities: Query<Entity, Without<Camera>>
) {
	for entityQuery in &entities {
		commands.entity(entityQuery).despawn();
	}
	let box_size = Vec2::new(WINDOW_HEIGHT, WINDOW_HEIGHT);
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: WINDOW_HEIGHT / 5.0,
		color: COLOR_BROWN,
	};

	let mut text = String::from("YOU  LOST\nSCORE: ");
	text.push_str(&cell_Value_Save.score.to_string());
	commands.spawn((
		Text2dBundle {
			text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
			text_2d_bounds: Text2dBounds {
				size: box_size,
			},
			..default()
		}
	));
}

fn VictoryFunction(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut cell_Value_Save: ResMut<CELL_VALUE_SAVE>,
	mut app_state: ResMut<State<VICTORY_or_DEFEAT>>,
	entities: Query<Entity, Without<Camera>>
) {
	for entityQuery in &entities {
		commands.entity(entityQuery).despawn();
	}
	let box_size = Vec2::new(WINDOW_HEIGHT, WINDOW_HEIGHT);
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font,
		font_size: WINDOW_HEIGHT / 5.0,
		color: COLOR_BROWN,
	};

	let mut text = String::from("WINNER\nSCORE: ");
	text.push_str(&cell_Value_Save.score.to_string());
	commands.spawn((
		Text2dBundle {
			text: Text::from_section(text, text_style.clone()).with_alignment(TextAlignment::CENTER),
			text_2d_bounds: Text2dBounds {
				size: box_size,
			},
			..default()
		}
	));
}

struct TouchEvent {
    ev: TouchInput,
}

fn handle_touches(
    mut events: EventReader<TouchInput>,
	asset_server: Res<AssetServer>,
	mut cell_value_save: ResMut<CELL_VALUE_SAVE>,
	mut text_query: Query<(&mut Text), (With<CellValue>)>,
	mut score_query: Query<(&mut Text), (Without<CellValue>)>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut app_state: ResMut<State<VICTORY_or_DEFEAT>>,
    state: &mut TouchEvent,
) {
	let mut moved = MOVE_DIRECTION::NONE;

    for ev in events.iter() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
                state.ev = *ev;
            }
            TouchPhase::Moved => {
                println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
				moved = move_calculation(*ev, state);
            }
            TouchPhase::Cancelled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }
    }
	motion(moved, asset_server, cell_value_save, text_query, score_query, materials, app_state);

}

fn move_calculation(
	ev: TouchInput,
	state: &mut TouchEvent,
) -> MOVE_DIRECTION {
	println!("State {} ended at: {:?}", state.ev.id, state.ev.position);
	let x = ev.position.x - state.ev.position.x;
	let y = ev.position.y - state.ev.position.y;
	let mut moved = MOVE_DIRECTION::NONE;
	if x.abs() > y.abs() {
		// x has biggest move
		if x.is_sign_positive() {
			moved = MOVE_DIRECTION::RIGHT
		} else {
			moved = MOVE_DIRECTION::LEFT			
		}
	} else {
		// y has biggest move
		if y.is_sign_positive() {
			moved = MOVE_DIRECTION::DOWN
		} else {
			moved = MOVE_DIRECTION::UP			
		}
	}
	state.ev = TouchInput { 
		phase: TouchPhase::Started
		, position: Vec2 { x: 0.0, y: 0.0 }
		, force: None, id: 0 
	};
	moved
}
