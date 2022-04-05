# Sloth
... because opening a GUI is too inconvenient compared to a CLI.

Sloth is a CLI Networking API testing APp including a Client and a mock Server.


# Module

Sloth is divided in modules: client and Server.

## Server

`server` can always replaced by `s`

command | desc |
------- | ---- |
`server run/r <filename>` | Run the server |
`server new/n <filename> <port>` | Create a new 
`server add/a <filename>  <name> <path> <method> <status?>` | Will add a new route at the end of the list. Default status is 200.
`server list/l <filename>` | Return the list of Route on a table.
`server delete/d <filename>` | Delete the Server file.
`server delete_route/dr <filename> <od>` | Will delete the route.
`server reorganize/rg <filename>` | Will reorganize the od of the Routes. Useful after deleting routes or modifying the file.