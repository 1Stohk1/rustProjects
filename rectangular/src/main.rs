#[derive(Debug)] // posso anche importare Copy e Clone da qui
struct Point {
    x: f32,
    y: f32
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
#[derive(Debug)]
struct Rectangular {
    top_right: Point,
    bottom_left: Point
}

fn calcolo_area (r1: &Rectangular) -> f32 {
    let top_rx: &Point = &r1.top_right; 
    let bott_sx: &Point = &r1.bottom_left; 
    
    let width: f32 = top_rx.x - bott_sx.x;
    let height: f32 = top_rx.y - bott_sx.y;

    width * height
}

fn main() {
    let punto_a = Point{
        x:3.0, 
        y:10.5
    };
    let punto_b = Point{
        x:0.0, 
        y:1.0
    };


    let mut rettangolo2 = Rectangular{
        top_right:punto_a.clone(),
        bottom_left:punto_b.clone()
    };

    rettangolo2.top_right = Point{x: 23.5, y:24.2};

    println!("Questo è lo stesso rettangolo {rettangolo2:?}");

    let rettangolo = Rectangular{
        // Questa linea prende in prestito il punto che non può essere preso di nuovo in prestito, perchè non ha più il possesso
        top_right: punto_a.clone(), // aggiungiamo il clone per evitare di prendere in prestito il path
        bottom_left: punto_b
    };

    // ora questo punto mai instanziato chiede il possesso dei punti, poco dopo potrei creare il rettangolo senza problemi 
    let Point {x: left_edge, y:right_edge} = punto_a;

    println!("Ho inventato un punto usando la destrutturazione con x: {left_edge} e y: {right_edge}");

    println!("Il rettangolo avente le seguenti coordinate {rettangolo:#?} ha un area di {}", calcolo_area(&rettangolo))
}
