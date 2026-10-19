# Bevy Spaceship Game

![Game Image](https://terra.hackclub-assets.com/b089d7cf-8c4f-4878-9162-d0ac826cafd7.png)

A work in progress 2d spaceship simulation game, inspired by games like endless sky and space engineers.

I plan to add many different parts you can use to build your ships, and ship damage on an individual system level.

Current features include:
Movement - WASD
Weapons - Click
Static asteroids to destroy
Enemy ship that can destroy you

### Roadmap:

- Add ship upgrades menu
- Add grid-based ship builder
- Add smart controller for ally and enemy npc ships
- Chose a cohesive art style instead of using placeholder assets

### Engine

I built this game in rust using the `bevy` game engine and `avian2d` for physics.
Bevy uses an ECS (Entity Component System) model that shold allow the game to stay performant even while simulating hundreds of ships at once. This is important because I want each ship to be incrementally destroyed, with the ability to destroy individual engines, weapons, and hull segments one at a time. This will require lots of individual game objects and colliders, so the ability to do lots of data processing every physics step is important to this project's success. I also wanted to try writing a game entirely in rust, and while Godot offers rust binding, the developer experience is subpar, and feels very disconnected from the typical Godot/Gdscript workflow.
