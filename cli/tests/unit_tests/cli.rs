#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_command() {
        let args = ["app", "SET", "mykey", "myvalue"];
        let cli_args = CliArgs::parse_from(&args);

        match cli_args.operation {
            CliOperation::SET { key, value } => {
                assert_eq!(key, "mykey");
                assert_eq!(value, "myvalue");
            }
            _ => panic!("Expected SET operation"),
        }
    }

    #[test]
    fn test_get_command() {
        let args = ["app", "GET", "mykey"];
        let cli_args = CliArgs::parse_from(&args);

        match cli_args.operation {
            CliOperation::GET { key } => {
                assert_eq!(key, "mykey");
            }
            _ => panic!("Expected GET operation"),
        }
    }

    #[test]
    fn test_delete_command() {
        let args = ["app", "DELETE", "mykey"];
        let cli_args = CliArgs::parse_from(&args);

        match cli_args.operation {
            CliOperation::DELETE { key } => {
                assert_eq!(key, "mykey");
            }
            _ => panic!("Expected DELETE operation"),
        }
    }
}