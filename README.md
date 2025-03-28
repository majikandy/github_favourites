A Github user’s most popular repositories
You are designing and implementing a CLI tool to help you find out a user's most popular
repositories using the Github v3 API.
In your CLI, build out the following functionality
1. Add a command that accepts a Github username and displays the top 10 repositories
for that user sorted by number of stars.
For each repository displayed, show the following fields:
a. Repository name
b. Repository URL
c. Repository description
d. Star count
2. Cache these results so that you only make a network request when there is no
cached data, or a flag is passed to explicitly clear the cache
3. Allow the user to pass a flag to specify a file format and path that the results will be
saved to. The user may decide to save both formats to different paths in the same
call. Silence the terminal display in this case. Support the following two formats:
a. JSON
b. Toml
4. Colour and format the output displayed to the terminal so it is easier on the eyes
a. Use a different colour for the field name and field value
b. Align the separator between the field name and field value
5. Add an option that allows you to pass in an Access Token to increase Github API
limits
6. Allow passing in multiple usernames from an argument or input configuration file of
your preferred format and display the top 10 entries after merging the lists of
repositories
7. Make the requests to Github concurrently and add a rate limiter to limit the requests
to a maximum of 2 per 10 seconds every time the program is run.
8. Allow the rate limit to persist across program runs.