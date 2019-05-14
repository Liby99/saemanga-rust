# Scheduler Thoughts

## Structure

Separate Python Server Periodically Sending Priviledged Signal to Rust Server

This enables the Rust server to be consistent. We can easily disable python
scheduler by controlling child process. Python also allows fast iteration.

## Tasks

1. Update Existing Manga (oldest 50, rotating). Per Hour.
2. Update Overall Genre Manga. Per Hour.
3. Update All Other Genres. Per 4 Hours.
4. Remove Unused User Session. Per Day.