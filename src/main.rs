



use degen_logger;



enum CustomLogStyle {
    
    Warn,
    Error,
    Hidden
    
} 


impl degen_logger::DegenLogStyle for CustomLogStyle {
    
    fn bold(&self) -> bool {
        match self {
            Self::Warn => true ,
            Self::Error => false ,
            Self::Hidden => false 
        }
    }
    
     fn show(&self) ->  bool {
        match self {
            Self::Warn => true ,
            Self::Error => true ,
            Self::Hidden => false 
        }
    }
    
    fn get_log_color( &self ) -> degen_logger::LogColor {
       
         
          match self {
            Self::Warn =>   degen_logger::LogColor::Blue ,
            Self::Error =>   degen_logger::LogColor::Red ,
            Self::Hidden =>   degen_logger::LogColor::Black 
        }
     }
    
}


fn main() {
   
    
    
    degen_logger::log(   format!("Listening on: {}", "localhost".to_string()), CustomLogStyle::Warn  );
    
    
    degen_logger::log_str(  "hello world", CustomLogStyle::Warn  );
    
    degen_logger::log_str(  "foo bar", CustomLogStyle::Error  );
    
     degen_logger::log_str(  "this is hidden now ", CustomLogStyle::Hidden  );
    
}
