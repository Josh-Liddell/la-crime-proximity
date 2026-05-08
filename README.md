# LA Crime Proximity API
Input a coordinate to return information on the nearest historical Los Angeles crimes to that location.

On startup the server parses 2020-2024 data.gov data into an [r*-tree](https://en.wikipedia.org/wiki/R*-tree) data structure which allows for efficent access of spacial data.
