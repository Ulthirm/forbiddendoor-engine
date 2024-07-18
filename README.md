# Forbidden Door Engine


Requirements:
- [ ] User control to the server
  - [ ] Master/Slave, Owner/Pet, Dom/Sub, other relationship status control
  - [ ] Dom/Master/Owner/etc account nickname control
- [ ] Sends a message/action to users based on status (EG: Meows at users that join the Meow VC)
- [ ] BDSM Test linking
- [ ] Discord OAUTH instead of anything custom
- [ ] Can generate temporary voice channels
    - [ ] Private VCs
    - [ ] Public VCs
- [ ] Mod roles can view all channels
- [ ] Completely ephemeral (No data storage in the long term)
- [ ] Links to the private site for the bot 
  - [ ] User config
  - [ ] Linking


[//]: ![Flowchart](Images/Flowchart.jpg)

# Crates used
[Serenity](https://crates.io/crates/serenity)
[tracing](https://crates.io/crates/tracing)
[Chrono](https://crates.io/crates/chrono)
[Tokio](https://crates.io/crates/tokio)