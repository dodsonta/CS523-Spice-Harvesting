# CS523-Spice-Harvesting
Taite Dodson 2025, tdodson@pdx.edu

This is a repository for my final project in CS523 at Portland State University. 

## License
This project is licensed under the [MIT License](LICENSE).

## About The Project
This project is an idle game similar to Cookie Clicker and based on the book series Dune by Frank Herbert. The project uses the GGEZ game engine library to generate the game and run it dynamically. It throws up a windowed screen where you can enter commands by typing them, and clicking increases the spice amount. The game uses a save system that, on successful exit or when a "save" command is entered, creates or updates a file named savegame.json in the current folder, which is loaded the next time the game starts. 

## Development 
The process was pretty smooth, and I ended up increasing the project's scope. I originally planned to keep the project in the terminal and use normal I/O to enter the commands and increase spice levels. I realized as I worked on it that this wound up being very simple, with the most complicated part being the save system and figuring out how I/O works in Rust. My solution for the I/O was very janky, and I'm sure it would have broken if I'd continued with it. My save system was originally made by hand, with me attempting to create a JSON file by hand and reading it line by line. After a few hours dealing with this, I realized there had to be a better solution and found out about the serde and serde_json libraries. 

After I finished setting up the basics with this terminal system, I was disappointed by how simple the program itself was, so I initially started looking for a way to get the spice count to update dynamically. This led me to find the GGEZ game engine, which I could use to dynamically update the game and have it pop out into its own application. It took me a long time to get GGEZ figured out, and I had to continually look at their provided examples and the Game of Life example in our class repository during development. I am very happy with how my program ended up, and there's nothing glaring I'd want to change. 

## What Was Learned
I obviously learned a lot about coding in Rust through this project. This project was more in-depth than anything else we did in this class, and I feel much more comfortable with the language. 

On top of learning about Rust programming, I also learned a lot about game and app development. I work in IT and CyberSecurity, so a lot of what I program is scripts, and I have only a limited amount of app development experience from undergrad. I learned a lot about how game engines can support development and how games are essentially an endless loop. 