# rust_grep
multithreaded file searching using channels

Apparently the first thing everybody does after reading through the rust tutorials is create a grep clone, here is my attempt.
This one works by creating a thread for each bufferred file reader and sending any found data back on the channel receiver.
Includes line numbers, but no colors

TODO: regex matching, currently only does contains()
