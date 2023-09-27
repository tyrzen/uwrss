@echo off
setlocal

for /f "tokens=1,2 delims==" %%a in ('type .env') do (
    set %%a=%%b
)

start uwrss.exe --interval %INTERVAL% --query "title:((\"Project manager\") OR (\"project management\") OR (\"project coordinator\") OR (\"scrum master\") OR (\"product manager\") OR (\"product management\"))" --smtp-server %SMTP_SERVER% --smtp-port %SMTP_PORT% --smtp-username %SMTP_USERNAME% --smtp-password %SMTP_PASSWORD% --recipient %RECIPIENT%
start uwrss.exe --interval %INTERVAL% --query "title:((\"Business analyst\") OR (\"Requirements manager\") OR (\"Business Analysis\") OR (\"Requirements Creator\"))" --smtp-server %SMTP_SERVER% --smtp-port %SMTP_PORT% --smtp-username %SMTP_USERNAME% --smtp-password %SMTP_PASSWORD% --recipient %RECIPIENT%
start uwrss.exe --interval %INTERVAL% --query "title:((\"QA\") OR (\"QC\") OR (\"Tester\") OR (\"Test Engineer\") OR (\"QA Engineer\") OR (\"Quality Engineer\") OR (\"Applications Tester\") OR (\"Quality control\") OR (\"Quality assurance\"))" --smtp-server %SMTP_SERVER% --smtp-port %SMTP_PORT% --smtp-username %SMTP_USERNAME% --smtp

endlocal