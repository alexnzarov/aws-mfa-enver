# aws-mfa-enver

An application that fetches AWS session credentials and stores them in the Windows environment.

## Configuration

When the application is run the first time, it will ask you for your IAM credentials. It stores them in a config.toml file in the program's root directory. 
The region is not prompted and the default value 'eu-central-1' is used. You can change it manually in the config file.

```toml
aws_access_key_id = '<access_key_id>'
aws_secret_access_key = '<secret_access_key>'
mfa_serial_number = '<mfa_serial_number>'
region = '<aws_region>'
```

## How to use it

Run the application and enter a temporary MFA code when you're asked to do it. If everything went fine, you will see a message that environment variables were updated.

Environment variables that hold session credentials:
```
AWS_ACCESS_KEY_ID
AWS_SECRET_ACCESS_KEY
AWS_SESSION_TOKEN
```
