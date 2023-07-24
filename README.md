

## Degen Logger 


Logging utility for rust console 


```
cargo add degen-logger 
```

## Overview 

Sick and tired of constantly editing your println!() statements in your code?  Degen Logger is here to save you.  Simply define a custom enum in your code like this (probably off in some library/util folder) : 

```



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


```

Then, in your code, log with that style like so: 

```
 degen_logger::log(   format!("Listening on: {}", "localhost".to_string()), CustomLogStyle::Warn  );
    
    
```


Now all of your log statements have a CustomLogStyle.  You can edit the implemented functions of this style in order to show/hide the messages of that style, change their print color, and determine boldness.  



### Example usage 

See main.rs 
