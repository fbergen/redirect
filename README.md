# 301

I have been looking around where i can get a simple redirect, over https that doesnt include conplec config, i couldnt find somethjng simple 

this is a simple 301 redirect to a domain of choice, run on kraft.cloud and all requests are cold starts within single digit ms

## Deploy

`kraft cloud deploy -0 -p 443:8080 -d meet.fberge.com -e REDIR=https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2w2hCzte8YuK2dQyKyRB1LYUktZ8HFSIjHfadmCOrDpsO9nHCp-6LjOuwNBY2IJuiCGHdIbLHz .`

