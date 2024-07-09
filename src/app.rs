use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app(initial_camera_scale: f32) -> App {
    let mut app = App::new();
    app.add_systems(Startup, add_player);
    let add_camera_fun = move |mut commands: Commands| {
        let mut bundle = Camera2dBundle::default();
        bundle.projection.scale = initial_camera_scale;
        commands.spawn(bundle);
    };
    app.add_systems(Startup, add_camera_fun);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(128.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    return query.iter(app.world_mut()).len();
}

#[cfg(test)]
fn get_camera_scale(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<&OrthographicProjection>();
    let projection = query.single(app.world());
    projection.scale
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec3 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec3 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale
}

#[cfg(test)]
fn has_camera(app: &App) -> bool {
    for c in app.world().components().iter() {
        if c.name() == "bevy_render::camera::camera::Camera" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        let initial_camera_scale = 1.0;
        create_app(initial_camera_scale);
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_setup_player_adds_a_player() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
        app.add_systems(Startup, add_player);
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let initial_camera_scale = 1.0;
        let mut app = create_app(initial_camera_scale);
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_get_player_coordinat() {
        let initial_camera_scale = 1.0;
        let mut app = create_app(initial_camera_scale);
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_get_player_scale() {
        let initial_camera_scale = 1.0;
        let mut app = create_app(initial_camera_scale);
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec3::new(128.0, 32.0, 0.0));
    }

    #[test]
    fn test_empty_app_has_no_camera() {
        let mut app = App::new();
        app.update();
        assert!(!has_camera(&app));
    }

    #[test]
    fn test_app_has_a_camera() {
        let initial_camera_scale = 1.0;
        let mut app = create_app(initial_camera_scale);
        app.update();
        assert!(has_camera(&app));
    }

    #[test]
    fn test_get_camera_scale() {
        let initial_camera_scale = 1.2;
        let mut app = create_app(initial_camera_scale);
        app.update();
        assert_eq!(get_camera_scale(&mut app), initial_camera_scale);
    }
}
