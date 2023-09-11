use raylib::core::math::Vector2;

use rand::distributions::{Distribution, Uniform};
use raylib::prelude::*;

pub struct Agent {
    pub position: Vector2,
    pub velocity: Vector2,
    pub walls: [i32; 2],
    pub max_speed: f32,
    pub max_turn_rate: f32,
    pub size: f32,
    pub colour: Color,
    pub trail_locations: Vec<Vector2>,
    pub trail_length: i32,
}

impl Agent {
    pub fn update(
        &mut self,
        dt: f32,
        is_left_pointer_pressed: bool,
        is_right_pointer_pressed: bool,
        pointer_position: Vector2,
    ) {
        if is_left_pointer_pressed {
            self.point_to_pointer(pointer_position);
        } else if is_right_pointer_pressed {
            self.point_from_pointer(pointer_position);
        }
        self.handle_walls();
        self.walk(dt);
    }
    fn walk(&mut self, dt: f32) {
        self.position += self.velocity * dt * self.max_speed;
        self.trail_locations.push(self.position);
        if self.trail_locations.len() > self.trail_length as usize {
            self.trail_locations.remove(0);
        }
    }
    fn handle_walls(&mut self) {
        let mut rng = rand::thread_rng();
        let range: Uniform<f32> = Uniform::from(-0.1..0.1);
        if self.position.x <= 0.0 {
            self.position.x = 0.0;
            self.velocity.x *= -1.0 + range.sample(&mut rng);
            self.velocity.y += range.sample(&mut rng);
            self.velocity.normalize();
        }
        if self.position.x >= self.walls[0] as f32 {
            self.position.x = self.walls[0] as f32;
            self.velocity.x *= -1.0 + range.sample(&mut rng);
            self.velocity.y += range.sample(&mut rng);
            self.velocity.normalize();
        }
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y *= -1.0 + range.sample(&mut rng);
            self.velocity.x += range.sample(&mut rng);
            self.velocity.normalize();
        }
        if self.position.y >= self.walls[1] as f32 {
            self.position.y = self.walls[1] as f32;
            self.velocity.y *= -1.0 + range.sample(&mut rng);
            self.velocity.x += range.sample(&mut rng);
            self.velocity.normalize();
        }
    }

    fn get_triangle_points(&self) -> [Vector2; 3] {
        //returns an array of three vectors representing the points of a triangle
        //takes into acount the direction the velocity vector is heading, and points the triangle in that direction
        let mut angle = (self.velocity.y / self.velocity.x).atan();
        if self.velocity.x < 0.0 {
            angle += std::f32::consts::PI;
        }

        let point_1 = Vector2::new(
            // point along the velocity vector
            self.position.x + (self.size + self.size * 0.1) * angle.cos(),
            self.position.y + (self.size + self.size * 0.1) * angle.sin(),
        );
        let point_2 = Vector2::new(
            // point at the right of the velocity vector
            self.position.x + self.size * (angle + 2.0943951023931953).cos(),
            self.position.y + self.size * (angle + 2.0943951023931953).sin(),
        );
        let point_3 = Vector2::new(
            // point at the left of the velocity vector
            self.position.x + self.size * (angle - 2.0943951023931953).cos(),
            self.position.y + self.size * (angle - 2.0943951023931953).sin(),
        );
        let points = [point_1, point_2, point_3];
        // println!("{:?}, {:?}", self.position, points);
        points
    }

    fn point_to_pointer(&mut self, pointer_position: Vector2) {
        let vector_to_pointer = pointer_position - self.position;
        self.velocity = self.velocity.lerp(vector_to_pointer, self.max_turn_rate);
        self.velocity.normalize();
    }

    fn point_from_pointer(&mut self, pointer_position: Vector2) {
        let vector_to_pointer = pointer_position - self.position;
        self.velocity = self.velocity.lerp(-vector_to_pointer, self.max_turn_rate);
        self.velocity.normalize();
    }

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        debug_points: bool,
        debug_vectors: bool,
        pointer_position: Vector2,
    ) {
        let points = self.get_triangle_points();
        // d.draw_triangle_lines(points[0], points[1], points[2], self.colour);
        d.draw_triangle(points[1], points[0], points[2], self.colour);

        let mut i = self.trail_locations.len();
        for p in &self.trail_locations {
            d.draw_pixel_v(
                p,
                Color::new(
                    self.colour.r,
                    self.colour.g,
                    self.colour.b,
                    self.colour.a - i as u8,
                ),
            );
            i -= 1;
        }
        if debug_points {
            d.draw_circle_v(self.position, 10.0, Color::RED);
            d.draw_circle_v(points[0], 5.0, Color::GREEN);
            d.draw_circle_v(points[1], 5.0, Color::GREEN);
            d.draw_circle_v(points[2], 5.0, Color::GREEN);
            d.draw_text(
                &format!("1"),
                points[0].x as i32,
                points[0].y as i32,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!("2"),
                points[1].x as i32,
                points[1].y as i32,
                20,
                Color::WHITE,
            );
            d.draw_text(
                &format!("3"),
                points[2].x as i32,
                points[2].y as i32,
                20,
                Color::WHITE,
            );
        }

        if debug_vectors {
            d.draw_line_v(
                self.position,
                (self.velocity * 100.0) + self.position,
                Color::WHITE,
            );
            d.draw_line_v(self.position, pointer_position, Color::SKYBLUE);
            let mut vector = self
                .velocity
                .lerp(pointer_position - self.position, self.max_turn_rate);
            vector.normalize();
            d.draw_line_v(
                self.position,
                self.position + vector * self.max_speed,
                Color::RED,
            );
        }
    }
}
