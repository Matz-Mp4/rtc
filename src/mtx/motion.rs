pub trait Motion: Sized {
    fn move_front(&mut self, value: f64) -> Self;
    fn move_back(&mut self, value: f64) -> Self;
    fn move_left(&mut self, value: f64) -> Self;
    fn move_right(&mut self, value: f64) -> Self;
    fn move_up(&mut self, value: f64) -> Self;
    fn move_down(&mut self, value: f64) -> Self;

    fn look_left(&mut self,degree: f64) -> Self;
    fn look_right(&mut self,degree: f64) -> Self;
    fn look_up(&mut self,degree: f64) -> Self;
    fn look_down(&mut self,degree: f64) -> Self;
}
