use crate::domain::exceptions::DomainException;

pub struct Parser;
pub enum CommandTypes {
    WEATHER,
    FX
}

pub struct ParsedPurpose {
    pub command: CommandTypes,
    pub args: Vec<String>
}

impl TryFrom<u32> for CommandTypes {
    type Error = DomainException;

    fn try_from(value: u32) -> Result<CommandTypes, Self::Error> {
        match value {
            0 => Ok(CommandTypes::WEATHER),
            1 => Ok(CommandTypes::FX),
            _ => Err(DomainException::UnknownCommand)
        }
    }
}

impl TryFrom<&str> for CommandTypes {
    type Error = DomainException;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "WEATHER" => Ok(CommandTypes::WEATHER),
            "FX" => Ok(CommandTypes::FX),
            _ => Err(DomainException::UnknownCommand)
        }
    }

}

impl ParsedPurpose {
    pub fn new(command: CommandTypes, args: Vec<String>) -> ParsedPurpose {
        ParsedPurpose {
            command,
            args
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self 
    }

    pub fn parse(&self, data: String) -> Result<ParsedPurpose, DomainException> {
        let args = shlex::split(&data)
            .ok_or(DomainException::UnknownCommand)?;

        match args.first() {
            Some(command) => {
                let command_ = CommandTypes::try_from(command.as_str());

                if let Ok(com) = command_ {
                    Ok(ParsedPurpose::new(com, args[1..].to_vec())) 
                } else {
                    Err(DomainException::UnknownCommand)
                }
            }

            None => {
                Err(DomainException::ProcessingError("no command".to_string()))
            }
        }
    }
}
