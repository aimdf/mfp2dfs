use std::{string, time::{Duration, Instant}};
const tick_count: u64 = 5; // Количество тиков в секунду
const TICK_RATE: Duration = Duration::from_millis(1000 / tick_count); // 5 ticks per second
const VELOCITY_DAMPING: f32 = 0.05 / tick_count as f32; // 5% от скорости в секунду, деленное на количество тиков в секунду
const RADIUS: f32 = 0.5; // Радиус шарика для более точного отображения столкновений
struct Ball {
    x: f32,  
    y: f32,
    vx: f32,
    vy: f32,
}
struct wall {
    coords: Vec<(u32, u32)>,
}
impl wall {
    fn new() -> Self {
        wall { coords: Vec::new() }
    }
    
    fn add(&mut self, x: u32, y: u32) {
        self.coords.push((x, y));
    }
}
impl Ball {
    fn update(&mut self, width: f64, height: f64, wwall: &wall) {
        self.x = (self.x  + self.vx);
        self.y = (self.y  + self.vy);
        if self.x >= width as f32 - RADIUS || self.x <= 0.0 ||
   wwall.coords.contains(&(self.x.round() as u32, self.y.round() as u32)) {
            self.x -= self.vx; // Корректируем позицию после отражения, чтобы избежать застревания в стене
            self.vx = -self.vx; // Отражение по горизонтали
            self.x = self.x.clamp(RADIUS, width as f32 - RADIUS); // Убедиться, что шарик не выходит за границы после отражения
        }
        if self.y >= height as f32 - RADIUS || self.y <= 0.0 ||
   wwall.coords.contains(&(self.x.round() as u32, self.y.round() as u32))  {
            self.y -= self.vy; // Корректируем позицию после отражения, чтобы избежать застревания в стене
            self.vy = -self.vy; // Отражение по вертикали
            self.y = self.y.clamp(RADIUS, height as f32 - RADIUS); // Убедиться, что шарик не выходит за границы после отражения
        }
        if self.vx > 0.0{
        self.vx = (self.vx  - VELOCITY_DAMPING);}
        else if self.vx <= 0.02 && self.vx >= -0.02 {
            self.vx = 0.0;} // Остановить движение, если скорость уже нулевая
         else {
             self.vx = (self.vx  + VELOCITY_DAMPING);
        }
        if self.vy > 0.0{
        self.vy = (self.vy  - VELOCITY_DAMPING);}
        else if self.vy <= 0.02 && self.vy >= -0.02 {
            self.vy = 0.0; // Остановить движение, если скорость уже нулевая
        }
         else {
             self.vy = (self.vy + VELOCITY_DAMPING);
         }

    }

fn render(&self, width: f64 , height: f64, frame: &mut String, wwall: &wall) {

    
    // Собираем кадр в строку
    for y in 0..height  as u32 {
        for x in 0..width as u32 {
        if self.x.round() as u32 == x && self.y.round() as u32 == y {
                frame.push('●');
                frame.push(' '); // Разделитель (опционально)
        } else if wwall.coords.contains(&(x, y)) {
                frame.push('▨');
                frame.push(' '); // Разделитель (опционально)
        }
        else {
                frame.push('⬚');
                frame.push(' '); // Разделитель (опционально)
        }
            frame.push(' '); // Разделитель (опционально)
        }
        frame.push('\n');
    }
    frame.push('\n');
    frame.push_str(&return_status(self)); // Добавляем статус в конец кадра
    frame.push_str("                                                "); // Добавляем отступ для лучшей читаемости

}
}
fn clear_screen(width: f64, height: f64) {
    print!("\x1B[1;1H"); 
}
fn return_status(ball: &Ball) -> String {
     format!(
    "Position: ({}, {}), Velocity: ({:.2}, {:.2})", ball.x.round(), ball.y.round(), ball.vx, ball.vy)
}
fn main() {
    let mut previous_time = Instant::now();
    let (width, height) = (20.0, 20.0); // Размеры игрового поля    
    let mut ball = Ball { x: 10.0, y: 10.0, vx: 2.0, vy: 3.0 };
    let mut frame = String::new();;
    let mut wwall = wall { coords: Vec::new() };
    wwall.add(10, 11);
    wwall.add(10, 9);
    wwall.add(9, 10);
    wwall.add(11, 9);
    wwall.add(11, 10);
    print!("\x1B[2J\x1B[1;1H"); // Очистка экрана и установка курсора в верхний левый угол
    loop {
        let current_time = Instant::now();
        let elapsed_time = current_time - previous_time;
    if elapsed_time >= TICK_RATE {
        ball.update( width, height, &wwall);
        clear_screen(width, height);
        frame.clear();
        ball.render( width, height, &mut frame, &wwall);
        print!("{}", frame);
        previous_time = previous_time + TICK_RATE; //  Учитываем точное время тика
    }
        
    let sleep_time = TICK_RATE.saturating_sub(elapsed_time);
    std::thread::sleep(sleep_time);
    }

}