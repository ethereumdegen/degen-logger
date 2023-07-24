


mod degenlogger;



enum CustomLogStyle {
    
    Warn,
    Error,
    Hidden
    
} 


impl degenlogger::DegenLogStyle for CustomLogStyle {
    
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
    
    fn get_log_color( &self ) -> degenlogger::LogColor {
       
         
          match self {
            Self::Warn =>   degenlogger::LogColor::Blue ,
            Self::Error =>   degenlogger::LogColor::Red ,
            Self::Hidden =>   degenlogger::LogColor::Black 
        }
     }
    
}


fn main() {
   
    
    
    degenlogger::log(  "hello world", CustomLogStyle::Warn  );
    
    degenlogger::log(  "foo bar", CustomLogStyle::Error  );
    
     degenlogger::log(  "this is hidden now ", CustomLogStyle::Hidden  );
    
}
