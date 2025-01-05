use macroquad::prelude::*;

#[derive(Clone)]
struct Planet {
    x: f32,
    y: f32,
    x_vel: f32,
    y_vel: f32,
    mass: f32,
}

impl Planet {
    pub fn new(x: f32, y: f32, x_vel: f32, y_vel: f32, mass: f32) -> Self {
        Self {
            x: x,
            y: y,
            x_vel: x_vel,
            y_vel: y_vel,
            mass: mass,
        }
    }

    pub fn equals(&self, other_planet: &Planet) -> bool {
        let mut same_coords = false;
        let mut same_vel = false;
        let mut same_mass = false;

        same_coords = self.x == other_planet.x && self.y == other_planet.y;
        same_vel = self.x_vel == other_planet.x_vel && self.y_vel == other_planet.y_vel;
        same_mass = self.mass == other_planet.mass;

        return same_coords && same_vel && same_mass;
    }

    pub fn set_mass(&mut self, new_mass: f32) {
        self.mass = new_mass;
    }

    pub fn set_velocity(&mut self, newx: f32, newy: f32) {
        self.x_vel = newx;
        self.y_vel = newy;
    }

    pub fn updateForces(&mut self, other_planet: &Planet) {
        let dx = self.x - other_planet.x;
        let dy = self.y - other_planet.y;

        let dist_sq = (dx * dx) + (dy * dy);

        let f = (39.5 * other_planet.mass) / (dist_sq * (dist_sq + 0.15).sqrt());

        self.x_vel -= dx * f;
        self.y_vel -= dy * f;
    }

    pub fn updatePos(&mut self, dt: f32) {
        self.x += self.x_vel * dt;
        self.y += self.y_vel * dt;
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.mass, BLACK);
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Gravity".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let mut mouse_x: f32;
    let mut mouse_y: f32;
    let mut last_click_x: f32 = 0f32;
    let mut last_click_y: f32 = 0f32;
    let mut mouse_clicked: u8 = 0;

    let mut planets: Vec<Planet> = Vec::new();

    // let planet = Planet::new(screen_width / 2f32, screen_height / 2f32, 0f32, 0f32, 20f32);
    // planets.push(planet);

    loop {
        // variables
        (mouse_x, mouse_y) = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_clicked += 1;
        }

        clear_background(WHITE);

        // update planet graphics
        for planet in planets.iter_mut() {
            planet.draw();
        }

        // new planet
        if mouse_clicked == 0 {
            // update planets
            for planet in planets.iter_mut() {
                planet.updatePos(get_frame_time());
            }

            for i in 0..planets.len() {
                for j in 0..planets.len() {
                    // don't apply forces to self
                    if i != j {
                        let other_planet = planets[j].clone();
                        planets[i].updateForces(&other_planet);
                    }
                }
            }
        } else if mouse_clicked == 1 {
            let planet = Planet::new(mouse_x, mouse_y, 0f32, 0f32, 20f32);
            planets.push(planet);

            last_click_x = mouse_x;
            last_click_y = mouse_y;

            mouse_clicked += 1;
        } else if mouse_clicked == 2 {
            if let Some(planet) = planets.last_mut() {
                let distance_from = ((mouse_x - last_click_x).powf(2f32)
                    + (mouse_y - last_click_y).powf(2f32))
                .sqrt();
                planet.set_mass(distance_from / 2f32);
            }
        } else if mouse_clicked == 3 {
            draw_line(last_click_x, last_click_y, mouse_x, mouse_y, 5f32, RED);
        } else if mouse_clicked == 4 {
            if let Some(planet) = planets.last_mut() {
                let mut new_x_vel = mouse_x - last_click_x;
                let mut new_y_vel = mouse_y - last_click_y;

                if new_x_vel < 10f32 && new_x_vel > -10f32 {
                    new_x_vel = 0f32;
                }
                if new_y_vel < 10f32 && new_y_vel > -10f32 {
                    new_y_vel = 0f32;
                }

                planet.set_velocity(new_x_vel, new_y_vel);
            }

            mouse_clicked += 1;
        } else {
            mouse_clicked = 0;
        }

        next_frame().await;
    }
}
