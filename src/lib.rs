



use colored::*;



pub enum LogColor {
    Blue,
    Green,
    Red,
    Black,
    White,
    Purple,
    Yellow,
    Custom {r:u8,g:u8,b:u8}
    
}

impl LogColor {
    
   fn get_message_of_color(&self, input_message:String) -> ColoredString {
       
       
       match self {
           
           
           Self::Blue => input_message.blue(),
           Self::Green => input_message.green(),
           Self::Red => input_message.red(),
           Self::Black => input_message.black(),
           Self::White => input_message.white(),
           Self::Purple => input_message.purple(),
           Self::Yellow => input_message.yellow(),  
           Self::Custom {r,g,b} => input_message.truecolor(r.clone(), g.clone(), b.clone())  
           
           
       }
       
   } 
    
    
}


pub trait DegenLogStyle {
    
    fn get_log_color( &self ) -> LogColor ;
    
    fn bold( &self ) -> bool; 
    
    fn show(  &self ) -> bool;
    
    
}

pub fn log<T:DegenLogStyle>( message:String , style: T   ) {
    
    
    let log_color = style.get_log_color();
    
    if style.show() {
        match style.bold() {        
            true =>  println!( "{}" , log_color.get_message_of_color(message).bold()) ,
            false =>  println!( "{}" , log_color.get_message_of_color(message) )              
        
        }
    }
    
  
}



pub fn log_str<T:DegenLogStyle>( message:&str , style: T   ) {
    
    
    let log_color = style.get_log_color();
    
    if style.show() {
        match style.bold() {        
            true =>  println!( "{}" , log_color.get_message_of_color(message.into()).bold()) ,
            false =>  println!( "{}" , log_color.get_message_of_color(message.into()) )              
        
        }
    }
    
  
}