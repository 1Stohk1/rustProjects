use oop_proj::{ Screen, Button };
use rgb::RGB;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(
                Button{
                    width: 20, 
                    height: 20, 
                    label: String::from("button"),
                    color: RGB{r: 255, g: 0, b: 0},
                    border_width: 2,
                    x_pos: 40.0,
                    y_pos: 10.0,
                }
            ),
            Box::new(
                Button{
                    width: 20, 
                    height: 20, 
                    label: String::from("button"),
                    color: RGB{r: 0, g: 255, b: 0},
                    border_width: 2,
                    x_pos: 10.0,
                    y_pos: 10.0,
                }
            )
        ],
    };
    screen.run(100, 100);
}
